#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
struct Task<'r> {
    description: &'r str,
    complete: bool
}

#[post("/foo", data = "<task>")]
fn foo(task: Json<Task<'_>>) {
    println!("Description: {}, Complete: {}", task.description, task.complete)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, foo])
}