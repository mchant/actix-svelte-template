use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

fn return_page(_req: HttpRequest, page: &str) -> Result<NamedFile> {
    let path: PathBuf = page.parse()?;
    Ok(NamedFile::open(path)?)
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    return_page(req, "./dist/index.html")
}

pub async fn run(host: String, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        let pairs: [(&str, &str); 2] = [("/_app", "./dist/_app"), ("/images", "./dist/images")];
        let app = App::new()
            .route("/", web::get().to(index))
            .service(actix_files::Files::new(pairs[0].0, pairs[0].1).show_files_listing())
            .service(actix_files::Files::new(pairs[1].0, pairs[1].1).show_files_listing());
        app
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
