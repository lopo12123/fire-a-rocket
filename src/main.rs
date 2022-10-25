use rocket::{get, post, put, delete, routes};

#[get("/")]
fn test_get() -> String {
    "re: get".to_string()
}

#[post("/")]
fn test_post() -> String {
    "re: post".to_string()
}

#[put("/<id>")]
fn test_put(id: usize) -> String {
    format!("re: put <{}>", id)
}

#[delete("/<id>")]
fn test_delete(id: usize) -> String {
    format!("re: delete <{}>", id)
}

#[allow(unused)]
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/test", routes![test_get, test_post, test_put, test_delete])
        .launch().await?;

    Ok(())
}