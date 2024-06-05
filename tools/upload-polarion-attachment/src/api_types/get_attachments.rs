use serde::Deserialize;

/// HTTP 200 response object for
///
///     GET /projects/{projectId}/spaces/{spaceId}/documents/{documentName}/attachments
///
/// See https://pet-gematikde.msappproxy.net/polarion/rest/v1#operations-Document_Attachments-getDocumentAttachments
/// for further details.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct Response {
    pub(crate) data: Vec<Attachment>,
    pub(crate) links: Option<PaginationLinks>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Attachment {
    #[serde(rename = "type")]
    pub(crate) type_field: String,
    pub(crate) id: String,
    pub(crate) links: ContentLinks,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ContentLinks {
    #[serde(rename = "self")]
    pub(crate) self_field: String,
    pub(crate) content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct PaginationLinks {
    #[serde(rename = "self")]
    pub(crate) self_field: String,
    pub(crate) first: String,
    pub(crate) prev: Option<String>,
    pub(crate) next: Option<String>,
    pub(crate) last: String,
}
