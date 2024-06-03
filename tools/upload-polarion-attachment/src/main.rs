use anyhow::{anyhow, Result};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::env;
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::{self, FromStr};

#[derive(Deserialize, Debug)]
struct PolarionAttachments {
    data: Vec<PolarionAttachment>,
    links: PolarionAttachmentsLinks
}

#[derive(Debug,Deserialize)]
struct PolarionAttachmentsLinks {
    #[serde(rename = "self")]
    self_field: String,
    first: String,
    prev: Option<String>,
    next: Option<String>,
    last: String
}

#[derive(Debug,Deserialize)]
struct PolarionAttachment {
    #[serde(rename = "type")]
    type_field: String,
    id: String,
    links: PolarionAttachmentLinks
}

#[derive(Debug,Deserialize)]
struct PolarionAttachmentLinks {
    #[serde(rename = "self")]
    self_field: String,
    content: String
}

#[derive(Debug,Serialize)]
struct PolarionPostAttachmentResources {
    data: Vec<PolarionPostAttachmentResource>
}


#[derive(Debug,Serialize)]
struct PolarionPostAttachmentResource {
    #[serde(rename = "type")]
    type_field: String,
    attributes: PolarionPostAttachmentAttributes
}

#[derive(Debug,Serialize)]
struct PolarionPostAttachmentAttributes {
    #[serde(rename = "fileName")]
    file_name: String,
    title: String
}

#[derive(Debug,Serialize)]
struct PolarionPatchAttachmentResource {
    #[serde(rename = "type")]
    type_field: String,
    id: String,
    attributes: PolarionPatchAttachmentAttributes
}

#[derive(Debug,Serialize)]
struct PolarionPatchAttachmentAttributes {
    title: String
}

enum Document {
    Basis,
    ePA,
    Pro
}

impl Document {
    fn project(&self) -> &str {
        match self {
            Document::Basis => "Mainline_OPB1",
            Document::ePA => "Mainline_OPB1",
            Document::Pro => "Mainline_OPB1"
        }
    }

    fn space(&self) -> &str {
        match self {
            Document::Basis => "Spezifikation",
            Document::ePA => "Spezifikation",
            Document::Pro => "Spezifikation"
        }    
    }

    fn document(&self) -> &str {
        match self {
            Document::Basis => "gemSpec_TI-M_Basis",
            Document::ePA => "gemSpec_TI-M_ePA",
            Document::Pro => "gemSpec_TI-M_Pro"
        }
    }
}

/// The ... well ... main function
fn main() -> Result<()> {
    let token = env::var("POLARION_ACCESS_TOKEN")?;
    let root_dir = get_root_dir()?;

    for arg in std::env::args().skip(1) {
        println!("Processing {}", arg);
        if let Err(error) = process_arg(arg.as_str(), &root_dir, &token) {
            println!("Error: {}", error);
        }
    }

    Ok(())
}

/// Tries to determine the git repository root starting from the current working directory
fn get_root_dir() -> Result<PathBuf> {
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

/// Processes a single command line argument
fn process_arg(arg: &str, root_dir: &PathBuf, token: &str) -> Result<()> {
    let path = PathBuf::from_str(&arg)?.canonicalize()?;
    let (document, id) = guess_document_and_id(&path, root_dir)?;
    
    if is_attachment_existing(&document, &id, token)? {
        println!("Attachment {} already exists. Do you want to override it?", id);
    } else {
        println!("Attachment {} doesn't exists yet. Do you want to upload it?", id);
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let attachments = upload_attachment(&path, &document, &id, token);
        println!("{:?}", attachments);
    }
    
    Ok(())
}

/// Tries to determine the corresponding Polarion document and attachment ID from a path
fn guess_document_and_id(path: &PathBuf, root_dir: &PathBuf) -> Result<(Document, String)> {
    let total = path.components().count();
    let mut remainder = total - 1;

    while remainder > 0 {
        let head = path.components().take(remainder).collect::<PathBuf>();
        if &head == root_dir {
            break;
        }

        if let Ok(document) = guess_document_from_last_path_component(&head) {
            let tail = path.components().skip(remainder).collect::<PathBuf>();
            let sanitized = tail.to_string_lossy().replace("/", "___");
            let id = format!("{}/{}/{}/{}", document.project(), document.space(), document.document(), sanitized);
            return Ok((document, id));
        }

        remainder -= 1;
    }

    Err(anyhow!("Could not determine document from {:?}", path))
}

/// Tries to determine the corresponding Polarion document from a path's last component
fn guess_document_from_last_path_component(path: &Path) -> Result<Document> {
    let folder = path.file_name().map_or(None, |n| n.to_str());
    match folder {
        Some("TI-M_Basis") => Ok(Document::Basis),
        Some("TI-M_ePA") => Ok(Document::ePA),
        Some("TI-M_Pro") => Ok(Document::Pro),
        _ => Err(anyhow!("Could not determine matching from last component of {:?}", folder))
    }
}

/// Checks if an attachment already exists in Polarion
fn is_attachment_existing(document: &Document, id: &str, token: &str) -> Result<bool> {
    let attachments = fetch_attachments(&document, token)?;
    Ok(attachments.iter().any(|a| a.id == id))
}

/// Fetches all existing attachments for a given document
fn fetch_attachments(document: &Document, token: &str) -> Result<Vec<PolarionAttachment>> {
    let mut url = Some(format!("https://pet.gematik.de/polarion/rest/v1/projects/{}/spaces/{}/documents/{}/attachments", document.project(), document.space(), document.document()));

    let client = reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mut attachments: Vec<PolarionAttachment> = vec![];

    while url.is_some() {
        let mut response = client
            .get(url.unwrap())
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()?
            .json::<PolarionAttachments>()?;
        attachments.append(&mut response.data);
        url = response.links.next;
    }
    
    Ok(attachments)
}

fn upload_attachment(path: &PathBuf, document: &Document, id: &str, token: &str) -> Result<Vec<PolarionAttachment>> {
    let url = format!("https://pet.gematik.de/polarion/rest/v1/projects/{}/spaces/{}/documents/{}/attachments", document.project(), document.space(), document.document());

    let client = reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let mut form = reqwest::blocking::multipart::Form::new();

// TODO: For whatever reason this crap doesn't work (missing field `data` at line 1 column 161))

    let resources = PolarionPostAttachmentResources {
        data: vec![PolarionPostAttachmentResource {
            type_field: "document_attachments".to_string(),
            attributes: PolarionPostAttachmentAttributes {
                file_name: id.to_string(),
                title: id.to_string()
            }
        }]
    };
    form = form.text("resource", to_string(&resources)?);
    form = form.part("files[]", reqwest::blocking::multipart::Part::file(path)?.file_name(id.to_string()));
    
    let response = client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .multipart(form)
        .send()?
        .json::<PolarionAttachments>()?;

    Ok(response.data)
}
