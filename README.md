# fire-a-rocket

`rocket.rs`

### Launching 的两种方式
```rust
// * launch方式 (官方推荐?)
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![hello])
}

// * rocket::build方式 (i prefer)
#[rocket::main]
async fn main() {
    // ...
}
```