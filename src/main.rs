pub mod singleton;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, post, put, State};
use rocket::http::Status;

use crate::singleton::{User, SINGLETON};

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
fn create_user(user: Json<User>) -> Result<Json<User>, Status> {
    let singleton = SINGLETON.lock().unwrap();
    let user_detail = singleton.get_data().create_user(user.into_inner());
    match user_detail {
        Some(_) => {let user_return = user_detail.unwrap();
                    Ok(Json(user_return))},
        None => Err(Status::InternalServerError),
    }
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
        routes![
            hello,
            create_user,
            read_user,
            update_user,
            delete_user,
            all_users
        ],
    )
}

#[cfg(test)]
mod test {
    use super::rocket;
    use assert_json_diff::assert_json_include;
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
        use rocket::local::blocking::LocalResponse;
        use assert_json_diff::assert_json_eq;
        use rocket::serde::json::Json;
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::create_user))
            .header(ContentType::JSON)
            .body(
                r##"{
            "id": "456",
            "name": "tutu",
            "email": "tutu@tutu.fr"
        }"##,
            )
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let formatter = serde_json::ser::CompactFormatter;
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(& mut buf, formatter);
        let response: LocalResponse = response;
        let j = Json(response.into_json::<crate::singleton::User>());

        // 
        //let retour = response.into_string();
        let comp: String =
            String::from("{\"id\": 456,\"name\": \"tutu\",\"email\": \"tutu@tutu.fr\"}");
           //assert_json_eq!(j, comp);
    }
}
