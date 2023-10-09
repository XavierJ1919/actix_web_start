use std::path::PathBuf;
use crate::*;
use actix_files::NamedFile;

async fn static_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}