pub mod singleton;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, post, put, State};

use crate::singleton::{SINGLETON, User, };



#[get("/users")]
fn all_users() -> std::option::Option<Json<std::string::String>> {
    let singleton = SINGLETON.lock().unwrap();
  singleton.get_data().list_user()
}

#[get("/users/<id>", format = "application/json")]
fn read_user(id: u32) -> Option<Json<User>> {
    let singleton = SINGLETON.lock().unwrap();
    singleton.get_data().read_user(id)
}

#[post("/users", format = "application/json", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    let singleton = SINGLETON.lock().unwrap();
    singleton.get_data().create_user(user.into_inner())
}

#[put("/users/<id>", format = "application/json", data = "<user>")]
fn update_user(id: u32, user: Json<User>) -> Option<Json<User>> {
    let singleton = SINGLETON.lock().unwrap();
    singleton.get_data().update_user(id, user.into_inner())
}

#[delete("/users/<id>", format = "application/json")]
fn delete_user(id: u32) -> Option<Json<User>> {
    let singleton = SINGLETON.lock().unwrap();
    singleton.get_data().delete_user(id)
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\n"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![hello, create_user, read_user, update_user, delete_user, all_users], 
    )
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_hello() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::hello)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!\n");
    }

    #[test]
    fn test_create_user() {
        use rocket::http::ContentType;
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!(super::create_user)).header(ContentType::JSON).body(r##"{
            "id": "456",
            "name": "tutu",
            "email": "tutu@tutu.fr"
        }"##).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "{\"id\": 456,\"name\": \"tutu\",\"email\": \"tutu@tutu.fr\"}");
    }
}
