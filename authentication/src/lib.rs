use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
            role,
        }
    }
}

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("subha", "subhapass", LoginRole::User),
//     ]
// }
// pub fn get_admin_users() {
//     let users: Vec<String> = get_users()
//         .into_iter()
//         .filter(|u| u.role == LoginRole::Admin)
//         .map(|u| u.username)
//         .collect();
// }

pub fn get_default_users() -> HashMap<String, User> {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "subha".to_string(),
        User::new("subha", "subhapass", LoginRole::User),
    );
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        let users: HashMap<String, User> = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

// pub fn login(username: &str, password: &str) -> Option<LoginAction> {

//     let users: [User; 2] = get_user();

//     if let Some(user) = users.iter().find(|user| user.username == username){
//         if user.password == password {
//             return Some(LoginAction::Granted(user.role.clone()))
//         }else{
//             return Some(LoginAction::Denied)
//         }
//     }
//     None
// let username: String  = username.to_lowercase();
// if username == "admin" && password == "password" {
//     LoginAction::Granted(LoginRole::Admin)
// } else if username == "subha" && password == "subhapass" {
//     LoginAction::Granted(LoginRole::User)
// } else {
//     LoginAction::Denied
// }
// }

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    if let Some(user) = users.get(username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }

    // if let Some(user) = users.iter().find(|user| user.username == username) {
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     } else {
    //         return Some(LoginAction::Denied);
    //     }
    // }
    None
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Subhrajit", greet_user("Subhrajit"));
    }

    #[test]
    fn test_login() {
        // assert_eq!(login("admin", "password"), LoginAction::Granted(LoginRole::Admin));
        // assert_eq!(login("ADMIN", "password"),LoginAction::Granted(LoginRole::Admin));
        // assert_eq!(login("subha", "subhapass"), LoginAction::Granted(LoginRole::User));
        // assert_eq!(login("admin2", "password"), LoginAction::Denied);
    }
}
