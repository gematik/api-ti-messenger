use std::path::PathBuf;

/// Polarion document identifier
pub(crate) enum Document {
    Basis,
    Epa,
    Pro,
}

impl Document {
    /// Gets the document's project ID
    pub(crate) fn project(&self) -> &str {
        match self {
            Document::Basis => "Mainline_OPB1",
            Document::Epa => "Mainline_OPB1",
            Document::Pro => "Mainline_OPB1",
        }
    }

    /// Gets the document's space ID
    pub(crate) fn space(&self) -> &str {
        match self {
            Document::Basis => "Spezifikation",
            Document::Epa => "Spezifikation",
            Document::Pro => "Spezifikation",
        }
    }

    /// Gets the document's document ID
    pub(crate) fn document(&self) -> &str {
        match self {
            Document::Basis => "gemSpec_TI-M_Basis",
            Document::Epa => "gemSpec_TI-M_ePA",
            Document::Pro => "gemSpec_TI-M_Pro",
        }
    }

    /// Builds the document's prefix by slash-separating its project, space and document ID
    pub(crate) fn prefix(&self) -> String {
        format!("{}/{}/{}", self.project(), self.space(), self.document())
    }

    /// Builds an attachment's ID from its name
    pub(crate) fn get_attachment_id(&self, name: &str) -> String {
        format!("{}/{}", self.prefix(), name)
    }
}

impl TryFrom<&PathBuf> for Document {
    type Error = &'static str;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let folder = value.file_name().and_then(|n| n.to_str());
        match folder {
            Some("TI-M_Basis") => Ok(Document::Basis),
            Some("TI-M_ePA") => Ok(Document::Epa),
            Some("TI-M_Pro") => Ok(Document::Pro),
            _ => Err("Last path component doesn't match a known Polarion document"),
        }
    }
}
