use rocket::{get, main, routes};

#[get("/")]
async fn hello() -> String {
    "hello rust!".to_string()
}

#[allow(unused)]
#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/hello", routes![hello])
        .launch().await?;

    Ok(())
}