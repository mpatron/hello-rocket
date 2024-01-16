use lazy_static::lazy_static;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

pub fn json2string(user: User) -> String {
    let formatter = serde_json::ser::CompactFormatter;
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    let user_json = Json(user);
    user_json.serialize(&mut ser).unwrap();
    String::from_utf8(buf).unwrap()
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
        match self.users.insert(user.id, user.clone()) {
            Some(_) => Some(user.clone()),
            None => Some(user.clone()),
        }
    }

    pub fn read_user(&self, id: u32) -> Option<User> {
        match self.users.get(&id) {
            Some(user) => Some(user.clone()),
            None => None,
        }
    }

    pub fn update_user(&mut self, id: u32, user: User) -> Option<User> {
        match self.users.get_mut(&id) {
            Some(existing_user) => {
                *existing_user = user.clone();
                Some(existing_user.clone())
            }
            None => None,
        }
    }

    pub fn delete_user(&mut self, id: u32) -> Option<User> {
        match self.users.remove(&id) {
            Some(user) => Some(user.clone()),
            None => None,
        }
    }

    pub fn list_user(&mut self) -> Option<String> {
        let les_users = self.users.clone();
        let mylist = les_users.into_values().collect::<Vec<User>>();
        let myjson = serde_json::to_string(&mylist).unwrap();
        match self.users.values().count() {
            0 => Some(myjson.clone()),
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
