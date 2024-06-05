use crate::documents::Document;
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Command;
use std::str::{self, FromStr};

/// Determines the git repository root starting from the current working directory
pub(crate) fn get_root_dir() -> Result<PathBuf> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout)?.trim();
        let path = PathBuf::from_str(stdout)?.canonicalize()?;
        return Ok(path);
    }

    let stderr = str::from_utf8(&output.stderr)?.to_string();
    Err(anyhow!(stderr))
}

/// Tries to determine the corresponding Polarion document and attachment name from a path
pub(crate) fn guess_document_and_name(
    path: &PathBuf,
    root_dir: &PathBuf,
) -> Result<(Document, String)> {
    let total = path.components().count();
    let mut remainder = total - 1;

    while remainder > 0 {
        let head = path.components().take(remainder).collect::<PathBuf>();
        if &head == root_dir {
            break;
        }

        if let Ok(document) = Document::try_from(&head) {
            let tail = path.components().skip(remainder).collect::<PathBuf>();
            let name = tail.to_string_lossy().replace('/', "___");
            return Ok((document, name));
        }

        remainder -= 1;
    }

    Err(anyhow!(
        "Could not determine Polarion document for {:?}",
        path
    ))
}
