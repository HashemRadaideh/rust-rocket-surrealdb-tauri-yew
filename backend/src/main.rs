use rocket::serde::{json::Json, Serialize};

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/jsoned", routes![jsoned])
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    hello: String,
}

#[get("/jsoned")]
async fn jsoned() -> Json<Message> {
    Json(Message {
        hello: "World!".to_string(),
    })
}
