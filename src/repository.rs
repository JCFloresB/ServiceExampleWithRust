use std::sync::{PoisonError, RwLock};

use chrono::Utc;
use futures::{future::BoxFuture, FutureExt};
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

type RepositoryResultOutput<T> = Result<T, RepositoryError>;
type RepositoryResult<'a, T> = BoxFuture<'a, RepositoryResultOutput<T>>;

pub trait Repository: Send + Sync + 'static {
    fn get_user<'a>(&'a self, user_id: &'a Uuid) -> RepositoryResult<'a, User>;
    fn create_user<'a>(&'a self, user: &'a User) -> RepositoryResult<'a, User>;
    fn update_user<'a>(&'a self, user: &'a User) -> RepositoryResult<'a, User>;
    fn delete_user<'a>(
        &'a self,
        user_id: &'a uuid::Uuid,
    ) -> RepositoryResult<'a, Uuid>;
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
    fn get_user<'a>(
        &'a self,
        user_id: &'a uuid::Uuid,
    ) -> RepositoryResult<'a, User> {
        async move {
            let users = self.users.read()?;
            print!("Get user: {:?}", users);
            users
                .iter()
                .find(|u| &u.id == user_id)
                .cloned()
                .ok_or_else(|| RepositoryError::InvalidId)
        }
        .boxed()
    }

    fn create_user<'a>(&'a self, user: &'a User) -> RepositoryResult<'a, User> {
        async move {
            if self.get_user(&user.id).await.is_ok() {
                return Err(RepositoryError::AlreadyExists);
            }
            let mut new_user = user.to_owned();
            new_user.created_at = Some(Utc::now());
            let mut users = self.users.write().unwrap();
            users.push(new_user.clone());
            Ok(new_user)
        }
        .boxed()
    }

    fn update_user<'a>(&'a self, user: &'a User) -> RepositoryResult<'a, User> {
        async move {
            if let Ok(old_user) = self.get_user(&user.id).await {
                let mut updated_user = user.to_owned();
                updated_user.created_at = old_user.created_at;
                updated_user.updated_at = Some(Utc::now());
                let mut users = self.users.write().unwrap();
                users.retain(|x| x.id != updated_user.id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
                users.push(updated_user.clone()); // se inserta el elemento a actualizar, esto esdespues de que se ha quitado previamente
                Ok(updated_user)
            } else {
                return Err(RepositoryError::DoesNotExists);
            }
        }
        .boxed()
    }

    fn delete_user<'a>(
        &'a self,
        user_id: &'a uuid::Uuid,
    ) -> RepositoryResult<'a, Uuid> {
        async move {
            let mut users = self.users.write()?;
            users.retain(|x| x.id != *user_id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
            Ok(user_id.to_owned())
        }
        .boxed()
        // let mut users = self.users.write().unwrap();
        // users.retain(|x| x.id != *user_id); // el vector se queda con todos los elementos que sean diferentes al user id entrante
        // Ok(user_id.to_owned())
        // Box::pin(std::future::ready(Ok(user_id.to_owned())))
    }
}
