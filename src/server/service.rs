use crate::server::models::User;

pub struct UserService;

impl UserService {
    pub async fn get_user(id: u64) -> Option<User> {
        Some(User {
            id: id,
            name: String::from("Mike Anthony"),
        })
    }
}
