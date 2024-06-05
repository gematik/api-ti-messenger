use serde::Serialize;

/// Request object for `resource` part of
///
///     POST /projects/{projectId}/spaces/{spaceId}/documents/{documentName}/attachments
///
/// See https://pet-gematikde.msappproxy.net/polarion/rest/v1#operations-Document_Attachments-postDocumentItemAttachments
/// for further details.
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Resource {
    data: Vec<Item>,
}

impl Resource {
    /// Creates a resource object for uploading a single file
    pub(crate) fn create(name: &str) -> Self {
        Self {
            data: vec![Item {
                type_field: "document_attachments".to_string(),
                attributes: Attributes {
                    file_name: name.to_string(),
                    title: name.to_string(),
                },
            }],
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Item {
    #[serde(rename = "type")]
    type_field: String,
    attributes: Attributes,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Attributes {
    #[serde(rename = "fileName")]
    pub(crate) file_name: String,
    pub(crate) title: String,
}
