//!
//! Support for Slack Files API methods
//!

use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SlackClientSession;
use crate::*;
use crate::{ClientResult, SlackClientHttpConnector};
use futures::future::{BoxFuture, FutureExt};
use slack_morphism_models::*;

impl<'a, SCHC> SlackClientSession<'a, SCHC>
where
    SCHC: SlackClientHttpConnector + Send,
{
    ///
    /// https://api.slack.com/methods/files.info
    ///
    pub async fn files_info(
        &self,
        req: &SlackApiFilesInfoRequest,
    ) -> ClientResult<SlackApiFilesInfoResponse> {
        self.http_api
            .http_get(
                "files.info",
                &vec![
                    ("file", Some(req.file.value())),
                    ("count", req.count.map(|x| x.to_string()).as_ref()),
                    ("cursor", req.cursor.as_ref().map(|x| x.value())),
                    ("limit", req.limit.map(|x| x.to_string()).as_ref()),
                    ("page", req.page.map(|x| x.to_string()).as_ref()),
                ],
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/files.list
    ///
    pub async fn files_list(
        &self,
        req: &SlackApiFilesListRequest,
    ) -> ClientResult<SlackApiFilesListResponse> {
        self.http_api
            .http_get(
                "files.list",
                &vec![
                    ("channel", req.channel.as_ref().map(|x| x.value())),
                    ("count", req.count.map(|x| x.to_string()).as_ref()),
                    ("page", req.page.map(|x| x.to_string()).as_ref()),
                    (
                        "show_files_hidden_by_limit",
                        req.show_files_hidden_by_limit
                            .map(|x| x.to_string())
                            .as_ref(),
                    ),
                    ("team_id", req.team_id.as_ref().map(|x| x.value())),
                    ("ts_from", req.ts_from.as_ref().map(|x| x.value())),
                    ("ts_to", req.ts_to.as_ref().map(|x| x.value())),
                    ("user", req.user.as_ref().map(|x| x.value())),
                    ("count", req.count.map(|v| v.to_string()).as_ref()),
                    (
                        "types",
                        req.types
                            .as_ref()
                            .map(|xs| {
                                xs.iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",")
                            })
                            .as_ref(),
                    ),
                ],
            )
            .await
    }

    ///
    /// https://api.slack.com/methods/files.comments.delete
    ///
    pub async fn files_comments_delete(
        &self,
        _req: &SlackApiFilesCommentsDeleteRequest,
    ) -> ClientResult<SlackApiFilesCommentsDeleteResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.delete
    ///
    pub async fn files_delete(
        &self,
        _req: &SlackApiFilesDeleteRequest,
    ) -> ClientResult<SlackApiFilesDeleteResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.revokePublicURL
    ///
    pub async fn files_revoke_public_url(
        &self,
        _req: &SlackApiFilesRevokePublicURLRequest,
    ) -> ClientResult<SlackApiFilesRevokePublicURLResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.sharedPublicURL
    ///
    pub async fn files_shared_public_url(
        &self,
        _req: &SlackApiFilesSharedPublicURLRequest,
    ) -> ClientResult<SlackApiFilesSharedPublicURLResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.upload
    ///
    pub async fn files_upload(
        &self,
        _req: &SlackApiFilesUploadRequest,
    ) -> ClientResult<SlackApiFilesUploadResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.remote.add
    ///
    pub async fn files_remote_add(
        &self,
        _req: &SlackApiFilesRemoteAddRequest,
    ) -> ClientResult<SlackApiFilesRemoteAddResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.remote.info
    ///
    pub async fn files_remote_info(
        &self,
        _req: &SlackApiFilesRemoteInfoRequest,
    ) -> ClientResult<SlackApiFilesRemoteInfoResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.remote.list
    ///
    pub async fn files_remote_list(
        &self,
        _req: &SlackApiFilesRemoteListRequest,
    ) -> ClientResult<SlackApiFilesRemoteListResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.remote.remove
    ///
    pub async fn files_remote_remove(
        &self,
        _req: &SlackApiFilesRemoteRemoveRequest,
    ) -> ClientResult<SlackApiFilesRemoteRemoveResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.remote.share
    ///
    pub async fn files_remote_share(
        &self,
        _req: &SlackApiFilesRemoteShareRequest,
    ) -> ClientResult<SlackApiFilesRemoteShareResponse> {
        unimplemented!()
    }

    ///
    /// https://api.slack.com/methods/files.remote.update
    ///
    pub async fn files_remote_update(
        &self,
        _req: &SlackApiFilesRemoteUpdateRequest,
    ) -> ClientResult<SlackApiFilesRemoteUpdateResponse> {
        unimplemented!()
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesInfoRequest {
    pub file: SlackFileId,
    pub count: Option<u16>,
    pub cursor: Option<SlackCursorId>,
    pub limit: Option<u16>,
    pub page: Option<u16>,
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesInfoResponse {
    pub file: Option<SlackFile>,
}

impl<SCHC> SlackApiScrollableRequest<SCHC> for SlackApiFilesInfoRequest
where
    SCHC: SlackClientHttpConnector + Send + Sync + Clone + 'static,
{
    type ResponseType = SlackApiFilesInfoResponse;
    type CursorType = SlackCursorId;
    type ResponseItemType = SlackUser;

    fn with_new_cursor(&self, new_cursor: Option<&Self::CursorType>) -> Self {
        self.clone().opt_cursor(new_cursor.cloned())
    }

    fn scroll<'a, 's>(
        &'a self,
        session: &'a SlackClientSession<'s, SCHC>,
    ) -> BoxFuture<'a, ClientResult<Self::ResponseType>> {
        async move { session.files_info(self).await }.boxed()
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesListRequest {
    pub channel: Option<SlackChannelId>,
    pub count: Option<u16>,
    pub page: Option<u16>,
    pub show_files_hidden_by_limit: Option<bool>,
    pub team_id: Option<SlackTeamId>,
    pub ts_from: Option<SlackTs>,
    pub ts_to: Option<SlackTs>,
    pub types: Option<Vec<String>>,
    pub user: Option<SlackUserId>,
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesListResponse {
    pub files: Option<Vec<SlackFile>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesCommentsDeleteRequest {
    pub file: SlackFileId,
    pub id: SlackCommentId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesCommentsDeleteResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesDeleteRequest {
    pub file: SlackFileId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesDeleteResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRevokePublicURLRequest {
    pub file: SlackFileId,
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRevokePublicURLResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesSharedPublicURLRequest {
    pub file: SlackFileId,
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesSharedPublicURLResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadRequest {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesUploadResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteAddRequest {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteAddResponse {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteInfoRequest {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteInfoResponse {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteListRequest {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteListResponse {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteRemoveRequest {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteRemoveResponse {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteShareRequest {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteShareResponse {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteUpdateRequest {}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackApiFilesRemoteUpdateResponse {}
