// // src/main.rs
// use cloud_storage::Client;
// use firebase_rs::*;
// use serde::{Deserialize, Serialize};
// use std::env;
// use tokio;

// #[derive(Serialize, Deserialize, Debug)]
// struct Product {
//     name: String,
//     description: String,
//     price: f64,
//     quantity: u32,
// }
// #[derive(Serialize, Deserialize, Debug)]
// struct Response {
//     name: String,
// }
// async fn create_product(fiebase_client: &Firebase, product: &Product) -> Response {
//     let firebase = firebase_client.at("products");
//     let product_res = firebase.set::<Product>(product).await;

//     string_to_response(&product_res.unwrap().data);
// }

// fn string_to_response(s: &str) -> Response {
//     serde_json::from_str(s).unwrap()
// }

// #[tokio::main]
// async fn main() {
//     // Load environment variables (if using dotenv)
//     let _ = dotenv::dotenv().ok();
// }

//------------------

// use std::collections::HashMap;

// use firebase_rs::*;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     name: String,
//     age: u32,
//     email: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Response {
//     name: String,
// }

// #[tokio::main]
// async fn main() {
//     // Create the user
//     let user = User {
//         name: "Jhon Doe".to_string(),
//         age: 25,
//         email: "jhon.doe@mail.com".to_string(),
//     };

//     // Create the Firebase Instance
//     let firebase = Firebase::new("https://busines-management-system-default-rtdb.asia-southeast1.firebasedatabase.app").unwrap();

//     // Create the user
//     let response = set_user(&firebase, &user).await;

//     // Get all users
//     let users = get_users(&firebase).await;
//     println!("{:?}", users);
//     // Get the user
//     let mut user = get_user(&firebase, &response.name).await;
//     println!("{:?}", user);

//     // Update the user
//     user.email = "updated.mail@gmail.com".to_string();
//     let updated_user = update_user(&firebase, &response.name, &user).await;
//     println!("{:?}", updated_user);

//     // // Delete the user
//     // delete_user(&firebase, &response.name).await;
//     // println!("User deleted");
// }

// // Create a user
// async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
//     let firebase = firebase_client.at("users");
//     let _users = firebase.set::<User>(&user).await;
//     return string_to_reponse(&_users.unwrap().data);
// }

// // Get All users
// async fn get_users(firebase_client: &Firebase) -> HashMap<String,User> {
//     let firebase = firebase_client.at("users");
//     let users = firebase.get::<HashMap<String, User>>().await;
//     println!("{:?}", users);
//     return users.unwrap();
// }

// // Get a user
// async fn get_user(firebase_client: &Firebase, id: &String) -> User {
//     let firebase = firebase_client.at("users").at(&id);
//     let user = firebase.get::<User>().await;
//     return user.unwrap();
// }

// // Update a user
// async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
//     let firebase = firebase_client.at("users").at(&id);
//     let _user = firebase.update::<User>(&user).await;
//     return string_to_user(&_user.unwrap().data);
// }

// async fn delete_user(firebase_client: &Firebase, id: &String) {
//     let firebase = firebase_client.at("users").at(&id);
//     let _result = firebase.delete().await;
// }

// // Convert a string to a Response
// fn string_to_reponse(s: &str) -> Response {
//     serde_json::from_str(s).unwrap()
// }

// // Convert a string to a User
// fn string_to_user(s: &str) -> User {
//     serde_json::from_str(s).unwrap()
// }

fn main() {
// https://cgarciarosales97.medium.com/crud-operations-with-rust-and-firebase-realtime-database-1841f66f3fa0    
}


// register response .

// {
//     "tokeeeeeee": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2OWQ4YzkzMGI2MjczYzBmODk1ZGFiMjEiLCJlbWFpbCI6ImFkbWluQHNocmVlbmFuZGkuY29tIiwicm9sZSI6ImFkbWluIiwiZXhwIjoxNzc1OTAxMzYwLCJpYXQiOjE3NzU4MTQ5NjB9.v_xcE9WI9QrE1OchzCBF4eWw62mtsODHfMo2gFclkWc",
//     "user": {
//         "id": "69d8c930b6273c0f895dab21",
//         "email": "admin@shreenandi.com",
//         "name": "Super Admin",
//         "role": "admin",
//         "is_active": true,
//         "created_at": "2026-04-10T09:56:00.866022265Z"
//     }
// }


// login response


// {
//     "tokeeeeeee": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2OWQ4YzkzMGI2MjczYzBmODk1ZGFiMjEiLCJlbWFpbCI6ImFkbWluQHNocmVlbmFuZGkuY29tIiwicm9sZSI6ImFkbWluIiwiZXhwIjoxNzc1OTAxNDQ0LCJpYXQiOjE3NzU4MTUwNDR9.uwwpVg3Qna3vq8meHst6-HHLAwammkjuUXCdvmaw0To",
//     "user": {
//         "id": "69d8c930b6273c0f895dab21",
//         "email": "admin@shreenandi.com",
//         "name": "Super Admin",
//         "role": "admin",
//         "is_active": true,
//         "created_at": "2026-04-10T09:56:00.866022265Z"
//     }
// }
