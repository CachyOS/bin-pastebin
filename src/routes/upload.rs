use rocket::data::{Data, ToByteUnit};

use std::{fs, path::Path};

use crate::get_parsed_args;
use crate::models::paste_id::PasteId;

// Currently I know only these, if you need another file type feel free to add them
// and possibly make a PR
const SUPPORTED_MIMETYPES: [&str; 4] = [
    "application/json",
    "application/xml",
    "application/mbox",
    "application/x-shellscript",
];

// text/x-tex: Rust detect PDF file as this one.
const BLACKLIST: [&str; 1] = ["text/x-tex"];

#[post("/", data = "<paste>")]
pub async fn upload(mut paste: Data<'_>) -> Result<String, &str> {
    let args = get_parsed_args();
    let file_partial = paste
        .peek(args.binary_upload_limit.mebibytes().as_u64() as usize)
        .await;
    let mime = tree_magic::from_u8(file_partial);
    println!("{}", mime);

    if BLACKLIST.contains(&mime.as_str())
        || (!mime.contains("text")
            && !SUPPORTED_MIMETYPES.contains(&mime.as_str()))
    {
        return Err("UNSUPPORTED_MIMETYPE");
    }

    let result = paste
        .open(args.binary_upload_limit.mebibytes())
        .into_bytes()
        .await;
    if result.is_err() {
        return Err("FILE_UPLOAD_FAILED");
    }

    let file = result.unwrap().value;
    let id = PasteId::new(7, &file);
    let filepath = Path::new(&args.upload).join(format!("{id}", id = id));
    let url = format!("/p/{id}", id = id);
    if filepath.is_file() {
        return Ok(url);
    }
    if fs::write(&filepath, file).is_err() {
        return Err("FILE_UPLOAD_FAILED");
    }
    Ok(url)
}
