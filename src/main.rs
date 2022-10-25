use std::collections::HashMap;
use std::error::Error;
use rocket::{get, main, routes};

// #[get("/")]
// async fn index() -> String {
//     "hello rust!".to_string()
// }
//
// #[allow(unused)]
// #[main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     rocket::build()
//         .mount("/", routes![hello])
//         .launch().await?;
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert("receive_id", "oc_51c8c6584bf4c1a3b01723bdec14d2da");
    body.insert("msg_type", "text");
    body.insert("content", "{\"text\":\"111\"}");
    body.insert("uuid", "1");

    let res = client.post("https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type=chat_id")
        .json(&body)
        .header("Authorization", "Bearer t-g104aplaOIZDEKG3VMILKIKZBMCDKSMOEDBGIPPF")
        .header("Content-Length", "application/json")
        .send()
        .await?
        .text()
        .await?;

    println!("res: {:#?}", res);

    Ok(())
}