use std::{path::Path, fs::{File, ReadDir, DirEntry}, io::Write};
use actix_web::{error, get, post, web, Responder, Result, HttpServer, App, HttpResponse, HttpRequest, http::header::HeaderValue};
use actix_files::NamedFile;
use rand::seq::IteratorRandom;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from the Streetpass Server!!")
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[get("/{titleid}/download")]
async fn download(path: web::Path<String>, req: HttpRequest) -> Result<NamedFile> {
    let titleid = path.into_inner();
    let console_id = match req.headers().get("3ds-id") {
        Some(id) => id.to_str().unwrap(),
        None => {
            return Err(error::ErrorBadRequest("Console ID not provided"))
        }
    };
    println!("Title ID is {}", titleid);
    println!("Console ID is {}", console_id);

    let title_path = Path::new("./sp-data").join(&titleid);

    if title_path.try_exists().is_err() {
        println!("No files available for {}", &titleid);
        return Err(error::ErrorNotFound("No files found"));
    }

    if title_path.try_exists().unwrap() == false {
        println!("No files available for {}", &titleid);
        return Err(error::ErrorNotFound("No files found"));
    }

    let console_paths = std::fs::read_dir(title_path).unwrap();

    let filtered_paths: Vec<DirEntry> = console_paths.filter(|p| {
        p.as_ref().unwrap().file_name() != console_id
    }).map(|p| {
        p.unwrap()
    }).collect();

    if filtered_paths.len() == 0 {
        println!("No files available for {}", &titleid);
        return Err(error::ErrorNotFound("No files found"));
    }

    println!("{:?}", filtered_paths);

    let mut message_paths = std::fs::read_dir(filtered_paths[0].path()).unwrap();

    // todo - better message file selection
    let message_path = message_paths.next().unwrap().unwrap().path();

    Ok(NamedFile::open(message_path)?)
}

#[post("/{titleid}/upload/{filename}")]
async fn upload(path: web::Path<(String, String)>, body: web::Bytes, req: HttpRequest) -> Result<impl Responder> {
    let (titleid, filename) = path.into_inner();

    let console_id = match req.headers().get("3ds-id") {
        Some(id) => id.to_str().unwrap(),
        None => {
            return Err(error::ErrorBadRequest("Console ID not provided"))
        }
    };

    println!("Console ID is {}", console_id);
    println!("Title ID is {}", titleid);
    println!("Filename is {}", filename);

    let folder_path = String::from("./sp-data/") + &titleid + "/" + &console_id;

    if !Path::new(&folder_path).exists() {
        std::fs::create_dir_all(Path::new(&folder_path)).unwrap();
    }

    let mut f = File::create(folder_path + "/" + &filename).unwrap();

    f.write_all(&body).unwrap();

    Ok(HttpResponse::Ok())
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
