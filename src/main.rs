use std::path::Path;
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

#[post("/{id}/{titleid}/upload")]
async fn upload(path: web::Path<(String, String)>, body: web::Bytes, req: HttpRequest) -> impl Responder {
    let (id, titleid) = path.into_inner();

    println!("Console ID is {}", id);
    println!("Title ID is {}", titleid);

    let path = String::from("./sp-data/") + &id + "/" + &titleid;

    if !Path::new(&path).exists() {
        std::fs::create_dir_all(Path::new(&path)).unwrap();
    }

    println!("Body {:?}", body);
    println!("Headers {:?}", req.headers());

    //let f = File::create(path + "/msg.bin").unwrap();
    //let tokio_f = tokio::fs::File::from_std(f);
//
    //data.open(512.kibibytes())
    //    .stream_to(tokio_f)
    //    .await?;

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(version)
            .service(download)
            .service(upload)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
