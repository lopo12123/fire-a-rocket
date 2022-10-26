use rocket::{get, post, put, delete, routes};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: usize,
    name: String,
}

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

#[get("/json1")]
fn test_json1() -> Value {
    json!({"name": "lopo"})
}

#[get("/json2")]
fn test_json2() -> Option<Json<Task>> {
    Some(Json(Task {
        id: 1,
        name: "lopo".to_string(),
    }))
}

#[post("/body", format = "json", data = "<task>")]
fn test_body(task: Json<Task>) -> Json<Task> {
    let task = task.into_inner();
    println!("{:#?}", task);

    Json(task)
}

#[allow(unused)]
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/test", routes![
            test_get, test_post, test_put, test_delete,
            test_json1, test_json2,
            test_body,
        ])
        .launch().await?;

    Ok(())
}