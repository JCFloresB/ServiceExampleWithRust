use crate::user::User;

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String>;
}
pub struct MemoryRepository {
    users: Vec<User>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: vec![User::new(String::from("Juan Carlos"), (1984, 02, 14))],
        }
    }
}

impl Repository for MemoryRepository {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String> {
        self.users
            .iter()
            .find(|u| &u.id == user_id)
            .map(|u| u.clone()) // se usa el map para clonar el objeto, ya que de no hacerse devuelve una referencia y se necesita un Usuario
            .ok_or_else(
                || String::from("User not found, invalid id"), // .clone()
            )
    }
}
