use std::sync::RwLock;

use chrono::Utc;
use uuid::Uuid;

use crate::user::User;

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &Uuid) -> Result<User, String>;
    fn create_user(&self, user: &User) -> Result<User, String>;
    fn update_user(&self, user: &User) -> Result<User, String>;
    fn delete_user(&self, user_id: &uuid::Uuid) -> Result<Uuid, String>;
}
pub struct MemoryRepository {
    //se agrega RwLock como envoltorio ya que es un mutex, con la finalidad de poder seguir utilizando self como una referencia inmutable
    users: RwLock<Vec<User>>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: RwLock::new(vec![]),
            // users: vec![User::new(String::from("Juan Carlos"), (1984, 02, 14))],
        }
    }
}

impl Repository for MemoryRepository {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String> {
        let users = self.users.read().map_err(|_| "Unlock error")?;
        users
            .iter()
            .find(|u| &u.id == user_id)
            .map(|u| u.clone()) // se usa el map para clonar el objeto, ya que de no hacerse devuelve una referencia y se necesita un Usuario
            .ok_or_else(
                || String::from("User not found, invalid id"), // .clone()
            )
    }

    fn create_user(&self, user: &User) -> Result<User, String> {
        if self.get_user(&user.id).is_ok() {
            return Err(String::from("This user already exist"));
        }
        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());
        let mut users = self.users.write().map_err(|_| "Unlock error")?;
        users.push(new_user.clone());
        Ok(new_user)
    }

    fn update_user(&self, user: &User) -> Result<User, String> {
        if self.get_user(&user.id).is_err() {
            return Err(String::from("This user does not exist"));
        }
        let mut updated_user = user.to_owned();
        updated_user.updated_at = Some(Utc::now());
        let mut users = self.users.write().map_err(|_| "Unlock error")?;
        users.retain(|x| x.id != updated_user.id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
        users.push(updated_user.clone()); // se inserta el elemento a actualizar, esto esdespues de que se ha quitado previamente
        Ok(updated_user)
    }

    fn delete_user(&self, user_id: &uuid::Uuid) -> Result<Uuid, String> {
        let mut users = self.users.write().map_err(|_| "Unlock error")?;
        users.retain(|x| x.id != *user_id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
        Ok(user_id.to_owned())
    }
}
