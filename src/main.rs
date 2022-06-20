use rocket::fs::NamedFile;
use rocket::futures::executor::block_on;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, version, download])
}
