use std::sync::{PoisonError, RwLock};

use chrono::Utc;
use tracing::instrument;
// use futures::{future::BoxFuture, FutureExt};
use uuid::Uuid;

use crate::user::User;
use async_trait::async_trait;
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

pub type RepositoryResult<T> = Result<T, RepositoryError>;
#[cfg_attr(test, mockall::automock)]
// cuando se trate de un test, se creará automáticamente el mock del trait Repository añadiendo el prefijo mock (MockRepository)
#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn get_user(&self, user_id: &Uuid) -> RepositoryResult<User>;
    async fn create_user(&self, user: &User) -> RepositoryResult<User>;
    async fn update_user(&self, user: &User) -> RepositoryResult<User>;
    async fn delete_user(&self, user_id: &uuid::Uuid) -> RepositoryResult<Uuid>;
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

#[async_trait]
impl Repository for MemoryRepository {
    #[instrument(skip(self))]
    async fn get_user(&self, user_id: &uuid::Uuid) -> RepositoryResult<User> {
        let users = self.users.read()?;
        tracing::debug!("Get user: {:?}", users);
        users
            .iter()
            .find(|u| &u.id == user_id)
            .cloned()
            .ok_or_else(|| RepositoryError::InvalidId)
    }

    #[instrument(skip(self))]
    async fn create_user(&self, user: &User) -> RepositoryResult<User> {
        if self.get_user(&user.id).await.is_ok() {
            return Err(RepositoryError::AlreadyExists);
        }
        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());
        let mut users = self.users.write().unwrap();
        users.push(new_user.clone());
        tracing::debug!("User created!!");
        Ok(new_user)
    }

    #[instrument(skip(self))]
    async fn update_user(&self, user: &User) -> RepositoryResult<User> {
        if let Ok(old_user) = self.get_user(&user.id).await {
            let mut updated_user = user.to_owned();
            updated_user.created_at = old_user.created_at;
            updated_user.updated_at = Some(Utc::now());
            let mut users = self.users.write().unwrap();
            users.retain(|x| x.id != updated_user.id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
            users.push(updated_user.clone()); // se inserta el elemento a actualizar, esto esdespues de que se ha quitado previamente
            tracing::debug!("User with id {} correctly updated!!", user.id);
            Ok(updated_user)
        } else {
            tracing::error!("User {} not exist!", user.id);
            return Err(RepositoryError::DoesNotExists);
        }
    }

    #[instrument(skip(self))]
    async fn delete_user(&self, user_id: &uuid::Uuid) -> RepositoryResult<Uuid> {
        let mut users = self.users.write()?;
        users.retain(|x| x.id != *user_id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
        tracing::debug!("User {:?} was correctly eliminated", user_id);
        Ok(user_id.to_owned())
    }
}
