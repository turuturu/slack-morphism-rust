use bytes::Buf;
use futures::future::TryFutureExt;
use futures::future::{BoxFuture, FutureExt};
use hyper::body::HttpBody;
use hyper::client::*;
use hyper::http::StatusCode;
use hyper::{Body, Request, Response, Uri};
use hyper_rustls::HttpsConnector;
use mime::Mime;
use rvstruct::ValueStruct;
use slack_morphism::errors::*;
use slack_morphism::signature_verifier::SlackEventAbsentSignatureError;
use slack_morphism::signature_verifier::SlackEventSignatureVerifier;
use slack_morphism::{ClientResult, SlackApiToken, SlackClientHttpConnector, SlackEnvelopeMessage};
use slack_morphism_models::{SlackClientId, SlackClientSecret};
use std::collections::HashMap;
use std::io::Read;
use url::Url;

#[derive(Clone, Debug)]
pub struct SlackClientHyperConnector {
    hyper_connector: Client<HttpsConnector<HttpConnector>>,
}

impl SlackClientHyperConnector {
    pub fn new() -> Self {
        let https_connector = HttpsConnector::with_native_roots();
        let http_client = Client::builder().build::<_, hyper::Body>(https_connector);
        Self {
            hyper_connector: http_client,
        }
    }

    pub(crate) fn parse_query_params(request: &Request<Body>) -> HashMap<String, String> {
        request
            .uri()
            .query()
            .map(|v| {
                url::form_urlencoded::parse(v.as_bytes())
                    .into_owned()
                    .collect()
            })
            .unwrap_or_else(HashMap::new)
    }

    pub(crate) fn hyper_redirect_to(
        url: &str,
    ) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        Response::builder()
            .status(hyper::http::StatusCode::FOUND)
            .header(hyper::header::LOCATION, url)
            .body(Body::empty())
            .map_err(|e| e.into())
    }

    fn setup_token_auth_header(
        request_builder: hyper::http::request::Builder,
        token: Option<&SlackApiToken>,
    ) -> hyper::http::request::Builder {
        if token.is_none() {
            request_builder
        } else {
            let token_header_value = format!("Bearer {}", token.unwrap().token_value.value());
            request_builder.header(hyper::header::AUTHORIZATION, token_header_value)
        }
    }

    pub(crate) fn setup_basic_auth_header(
        request_builder: hyper::http::request::Builder,
        username: &str,
        password: &str,
    ) -> hyper::http::request::Builder {
        let header_value = format!(
            "Basic {}",
            base64::encode(format!("{}:{}", username, password))
        );
        request_builder.header(hyper::header::AUTHORIZATION, header_value)
    }

    pub(crate) fn create_http_request(
        url: Url,
        method: hyper::http::Method,
    ) -> hyper::http::request::Builder {
        let uri: Uri = url.as_str().parse().unwrap();
        hyper::http::request::Builder::new()
            .method(method)
            .uri(uri)
            .header("accept-charset", "utf-8")
    }

    async fn http_body_to_string<T>(body: T) -> ClientResult<String>
    where
        T: HttpBody,
        T::Error: std::error::Error + Sync + Send + 'static,
    {
        let http_body = hyper::body::aggregate(body).await?;
        let mut http_reader = http_body.reader();
        let mut http_body_str = String::new();
        http_reader.read_to_string(&mut http_body_str)?;
        Ok(http_body_str)
    }

    fn http_response_content_type<RS>(response: &Response<RS>) -> Option<Mime> {
        let http_headers = response.headers();
        http_headers.get(hyper::header::CONTENT_TYPE).map(|hv| {
            let hvs = hv.to_str().unwrap();
            hvs.parse::<Mime>().unwrap()
        })
    }

    pub(crate) async fn send_webapi_request<RS>(&self, request: Request<Body>) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let http_res = self.hyper_connector.request(request).await?;
        let http_status = http_res.status();
        let http_content_type = Self::http_response_content_type(&http_res);
        let http_body_str = Self::http_body_to_string(http_res).await?;

        match http_status {
            StatusCode::OK
                if http_content_type.iter().all(|response_mime| {
                    response_mime.type_() == mime::APPLICATION
                        && response_mime.subtype() == mime::JSON
                }) =>
            {
                let slack_message: SlackEnvelopeMessage =
                    serde_json::from_str(http_body_str.as_str())?;
                if slack_message.error.is_none() {
                    let decoded_body = serde_json::from_str(http_body_str.as_str())?;
                    Ok(decoded_body)
                } else {
                    Err(SlackClientError::ApiError(
                        SlackClientApiError::new(slack_message.error.unwrap())
                            .opt_warnings(slack_message.warnings)
                            .with_http_response_body(http_body_str),
                    )
                    .into())
                }
            }
            StatusCode::OK => serde_json::from_str("{}").map_err(|e| e.into()),
            _ => Err(SlackClientError::HttpError(
                SlackClientHttpError::new(http_status.as_u16())
                    .with_http_response_body(http_body_str),
            )
            .into()),
        }
    }

    pub(crate) async fn decode_signed_response(
        req: Request<Body>,
        signature_verifier: &SlackEventSignatureVerifier,
    ) -> ClientResult<String> {
        let headers = &req.headers().clone();
        let req_body = req.into_body();
        match (
            headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_HASH_HEADER),
            headers.get(SlackEventSignatureVerifier::SLACK_SIGNED_TIMESTAMP),
        ) {
            (Some(received_hash), Some(received_ts)) => {
                Self::http_body_to_string(req_body)
                    .and_then(|body| async {
                        signature_verifier
                            .verify(
                                received_hash.to_str().unwrap(),
                                &body,
                                received_ts.to_str().unwrap(),
                            )
                            .map(|_| body)
                            .map_err(|e| e.into())
                    })
                    .await
            }
            _ => Err(Box::new(SlackEventAbsentSignatureError::new())),
        }
    }
}

impl SlackClientHttpConnector for SlackClientHyperConnector {
    fn http_get_uri<'a, RS>(
        &'a self,
        full_uri: Url,
        token: Option<&'a SlackApiToken>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send,
    {
        async move {
            let base_http_request = Self::create_http_request(full_uri, hyper::http::Method::GET);

            let http_request = Self::setup_token_auth_header(base_http_request, token);

            let body = self
                .send_webapi_request(http_request.body(Body::empty())?)
                .await?;

            Ok(body)
        }
        .boxed()
    }

    fn http_get_with_client_secret<'a, RS>(
        &'a self,
        full_uri: Url,
        client_id: &'a SlackClientId,
        client_secret: &'a SlackClientSecret,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + 'a + Send,
    {
        async move {
            let http_request = Self::setup_basic_auth_header(
                Self::create_http_request(full_uri, hyper::http::Method::GET),
                client_id.value(),
                client_secret.value(),
            )
            .body(Body::empty())?;

            self.send_webapi_request(http_request).await
        }
        .boxed()
    }

    fn http_post_uri<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        request_body: &'a RQ,
        token: Option<&'a SlackApiToken>,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a,
    {
        async move {
            let post_json = serde_json::to_string(&request_body)?;

            let base_http_request = Self::create_http_request(full_uri, hyper::http::Method::POST)
                .header("content-type", "application/json; charset=utf-8");

            let http_request = Self::setup_token_auth_header(base_http_request, token);

            let response_body = self
                .send_webapi_request(http_request.body(post_json.into())?)
                .await?;

            Ok(response_body)
        }
        .boxed()
    }
}
