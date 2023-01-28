use std::{path::Path, fs::File, io::Write};
use actix_web::{get, post, web, Responder, Result, HttpServer, App, HttpResponse, HttpRequest};
use actix_files::NamedFile;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from the Streetpass Server!!")
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[get("/{titleid}/download")]
async fn download(path: web::Path<String>) -> Result<NamedFile> {
    let titleid = path.into_inner();
    println!("Title ID is {}", titleid);
    let path = "./sp-data/".to_owned() + &titleid + "/NT9DANjYeugA=";
    Ok(NamedFile::open(path)?)
}

#[post("/{titleid}/upload/{filename}")]
async fn upload(path: web::Path<(String, String)>, body: web::Bytes, req: HttpRequest) -> impl Responder {
    let (titleid, filename) = path.into_inner();

    let id = req.headers().get("3ds-id").unwrap().to_str().unwrap();

    println!("Console ID is {}", id);
    println!("Title ID is {}", titleid);
    println!("Filename is {}", filename);

    let folder_path = String::from("./sp-data/") + &titleid + "/" + &id;

    if !Path::new(&folder_path).exists() {
        std::fs::create_dir_all(Path::new(&folder_path)).unwrap();
    }

    let mut f = File::create(folder_path + "/" + &filename).unwrap();

    f.write_all(&body).unwrap();

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8000");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(version)
            .service(download)
            .service(upload)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
