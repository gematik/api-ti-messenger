use serde::Serialize;

/// Request object for `resource` part of
///
///     PATCH /projects/{projectId}/spaces/{spaceId}/documents/{documentName}/attachments/{attachmentId}
///
/// See https://pet-gematikde.msappproxy.net/polarion/rest/v1#operations-Document_Attachments-patchDocumentAttachment
/// for further details.
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Resource {
    pub(crate) data: Item,
}

impl Resource {
    /// Creates a resource object for replacing a single file
    pub(crate) fn create(id: &str, name: &str) -> Self {
        Self {
            data: Item {
                type_field: "document_attachments".to_string(),
                id: id.to_string(),
                attributes: Attributes {
                    title: name.to_string(),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Item {
    #[serde(rename = "type")]
    pub(crate) type_field: String,
    pub(crate) id: String,
    pub(crate) attributes: Attributes,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Attributes {
    pub(crate) title: String,
}
