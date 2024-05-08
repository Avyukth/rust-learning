#[derive(PartialEq, Eq, Debug)]
pub enum LoginRole{
    Admin,
    User,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LoginAction{
    Granted(LoginRole),
    Denied,
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username: String  = username.to_lowercase();
    if username == "admin" && password == "password" {
        LoginAction::Granted(LoginRole::Admin)
    } else if username == "subha" && password == "subhapass" {
        LoginAction::Granted(LoginRole::User)
    } else {
        LoginAction::Denied
    }
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
        assert_eq!(login("admin", "password"), LoginAction::Granted(LoginRole::Admin));
        assert_eq!(login("ADMIN", "password"),LoginAction::Granted(LoginRole::Admin));
        assert_eq!(login("subha", "subhapass"), LoginAction::Granted(LoginRole::User));
        assert_eq!(login("admin2", "password"), LoginAction::Denied);
    }
}
