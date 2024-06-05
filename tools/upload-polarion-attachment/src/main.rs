use anyhow::Result;
use colored::Colorize;
use documents::Document;
use file_system::{get_root_dir, guess_document_and_name};
use networking::{create_client, is_attachment_existing, patch_attachment, post_attachment};
use reqwest::blocking::Client;
use std::env;
use std::io::stdin;
use std::path::PathBuf;
use std::str::{self, FromStr};

mod api_types;
mod documents;
mod file_system;
mod networking;

/// The ... well ... main function
fn main() {
    let root_dir = get_root_dir().expect("Should be called from within the git repository");

    let token = env::var("POLARION_ACCESS_TOKEN")
        .expect("Should have a POLARION_ACCESS_TOKEN environment variable");
    let client = create_client(&token).expect("Should be able to create network client");

    for arg in std::env::args().skip(1) {
        println!("Processing {}", arg);
        if let Err(error) = process_arg(arg.as_str(), &root_dir, &client) {
            let msg = format!("[Error]: {}", error);
            eprintln!("{}", msg.red());
        }
    }
}

/// Processes a single command line argument
fn process_arg(arg: &str, root_dir: &PathBuf, client: &Client) -> Result<()> {
    let path = PathBuf::from_str(arg)?.canonicalize()?;
    let (document, name) = guess_document_and_name(&path, root_dir)?;
    let is_existing = is_attachment_existing(&document, &name, client)?;

    confirm_operation(&document, &name, is_existing)?;

    if is_existing {
        patch_attachment(&path, &document, &name, client)?;
    } else {
        post_attachment(&path, &document, &name, client)?;
    }

    Ok(())
}

/// Asks the user to confirm uploading or replacing the attachment
fn confirm_operation(document: &Document, name: &str, is_existing: bool) -> Result<()> {
    let message = if is_existing {
        format!(
            "Attachment {} already exists in {}. Hit <Enter> to {} it.",
            name.bold(),
            document.prefix().bold(),
            "overwrite".bold()
        )
    } else {
        format!(
            "Attachment {} doesn't exist in {}. Hit <Enter> to {} it.",
            name.bold(),
            document.prefix().bold(),
            "upload".bold()
        )
    };
    println!("{}", message.cyan());

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(())
}
