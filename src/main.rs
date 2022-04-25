#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello from the Streetpass Server!!"
}

#[get("/version")]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, version])
}
