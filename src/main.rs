use rocket::fs::NamedFile;
use rocket::futures::executor::block_on;
use rocket::data::{Data, ToByteUnit};
use rocket::tokio;
use std::fs::File;
use std::path::Path;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello from the Streetpass Server!!"
}

#[get("/version")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[get("/<titleid>/download")]
fn download(titleid: String) -> Result<NamedFile, std::io::Error> {
    println!("Title ID is {}", titleid);
    let path = "./sp-data/".to_owned() + &titleid + "/NT9DANjYeugA=";
    block_on(NamedFile::open(path))
}

#[post("/<id>/<titleid>/upload", data = "<data>")]
async fn upload(id: String, titleid: String, data: Data<'_>) -> std::io::Result<()> {
    println!("Console ID is {}", id);
    println!("Title ID is {}", titleid);

    let path = id + "/" + &titleid;

    if !Path::new(&path).exists() {
        std::fs::create_dir_all(Path::new(&path)).unwrap();
    }

    let f = File::create(path + "/msg.bin").unwrap();
    let tokio_f = tokio::fs::File::from_std(f);

    data.open(512.kibibytes())
        .stream_to(tokio_f)
        .await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, version, download, upload])
}
