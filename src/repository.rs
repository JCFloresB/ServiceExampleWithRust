use std::sync::{PoisonError, RwLock};

use chrono::Utc;
use uuid::Uuid;

use crate::user::User;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("PoisonError '{0}'")]
    LockError(String),
    #[error("This entity already exists")]
    AlreadyExists,
    #[error("This entity does not exist")]
    DoesNotExists,
    #[error("The id format is not valid")]
    InvalidId,
}

impl<T> From<PoisonError<T>> for RepositoryError {
    fn from(poison_error: PoisonError<T>) -> Self {
        RepositoryError::LockError(poison_error.to_string())
    }
}

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &Uuid) -> Result<User, RepositoryError>;
    fn create_user(&self, user: &User) -> Result<User, RepositoryError>;
    fn update_user(&self, user: &User) -> Result<User, RepositoryError>;
    fn delete_user(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Uuid, RepositoryError>;
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
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, RepositoryError> {
        let users = self.users.read()?;
        users
            .iter()
            .find(|u| &u.id == user_id)
            .map(|u| u.clone()) // se usa el map para clonar el objeto, ya que de no hacerse devuelve una referencia y se necesita un Usuario
            .ok_or_else(
                || RepositoryError::InvalidId, // .clone()
            )
    }

    fn create_user(&self, user: &User) -> Result<User, RepositoryError> {
        if self.get_user(&user.id).is_ok() {
            return Err(RepositoryError::AlreadyExists);
        }
        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());
        let mut users = self.users.write()?;
        users.push(new_user.clone());
        Ok(new_user)
    }

    fn update_user(&self, user: &User) -> Result<User, RepositoryError> {
        if self.get_user(&user.id).is_err() {
            return Err(RepositoryError::DoesNotExists);
        }
        let mut updated_user = user.to_owned();
        updated_user.updated_at = Some(Utc::now());
        let mut users = self.users.write()?;
        users.retain(|x| x.id != updated_user.id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
        users.push(updated_user.clone()); // se inserta el elemento a actualizar, esto esdespues de que se ha quitado previamente
        Ok(updated_user)
    }

    fn delete_user(
        &self,
        user_id: &uuid::Uuid,
    ) -> Result<Uuid, RepositoryError> {
        let mut users = self.users.write()?;
        users.retain(|x| x.id != *user_id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
        Ok(user_id.to_owned())
    }
}
