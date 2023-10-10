use std::path::PathBuf;
use crate::*;
use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionType};

#[get("/{filename:.txt}")]
async fn static_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = NamedFile::open(path)?;
    Ok(file.use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}