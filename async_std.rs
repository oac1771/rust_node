// could use this type of structure to be able to setup contract address after deploying node
// // main.rs
// #[macro_use]
// extern crate rocket;

// use async_std::task;
// use rocket::tokio::sync::mpsc;
// use rocket::tokio::time::Duration;
// use rocket::State;
// use std::sync::Arc;

// // Rocket's shared state
// #[rocket::get("/")]
// fn index(state: &State<Arc<mpsc::Sender<u32>>>) -> String {
//     let sender = state.inner().clone();
//     task::spawn(async move {
//         // Send a value to the async-std task
//         sender.send(42).await.expect("send error");
//     });

//     "Hello, Rocket!".to_string()
// }

// #[launch]
// fn rocket() -> _ {
//     let (sender, receiver) = mpsc::channel(10);
//     let sender = Arc::new(sender);

//     // Spawn an async-std task
//     task::spawn(async move {
//         while let Some(value) = receiver.recv().await {
//             println!("Received value: {}", value);
//         }
//     });

//     rocket::build().manage(sender)
// }
