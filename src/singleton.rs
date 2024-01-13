use lazy_static::lazy_static;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Users {
    users: HashMap<u32, User>,
}

impl Users {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn create_user(&mut self, user: User) -> Option<User> {
        /// self.users.insert(user.id, user.clone())
        /// let les_users = self.users.clone();
        /// let mut inerted_user: les_users.insert(user.id, user.clone());
        /// inerted_user
        /// match self.users.insert(user.id, user.clone()) {
        ///     Some(inserted_user) => Some(inserted_user.clone()),
        ///     None => None,
        /// }
        self.users.insert(user.id, user.clone())
    }

    pub fn read_user(&self, id: u32) -> Option<Json<User>> {
        match self.users.get(&id) {
            Some(user) => Some(Json(user.clone())),
            None => None,
        }
    }

    pub fn update_user(&mut self, id: u32, user: User) -> Option<Json<User>> {
        match self.users.get_mut(&id) {
            Some(existing_user) => {
                *existing_user = user.clone();
                Some(Json(existing_user.clone()))
            }
            None => None,
        }
    }

    pub fn delete_user(&mut self, id: u32) -> Option<Json<User>> {
        match self.users.remove(&id) {
            Some(user) => Some(Json(user.clone())),
            None => None,
        }
    }

    pub fn list_user(&mut self) -> Option<Json<String>> {
        let les_users = self.users.clone();
        let mut mylist = les_users.into_values().collect::<Vec<User>>();
        let myjson = serde_json::to_string(&mylist).unwrap();
        match self.users.values().count() {
            0 => Some(Json(myjson.clone())),
            _ => None,
        }
    }
}

pub struct Singleton {
    users: Users,
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            users: Users::new(),
        }
    }

    pub fn get_data(&self) -> Users {
        self.users.clone()
    }
}

lazy_static! {
    pub static ref SINGLETON: Mutex<Singleton> = Mutex::new(Singleton::new());
}
