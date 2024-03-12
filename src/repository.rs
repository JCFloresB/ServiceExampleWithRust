use actix_web::FromRequest;

use crate::user::User;
use std::{
    future::{ready, Ready},
    ops::Deref,
    sync::Arc,
};

pub trait Repository: Send + Sync + 'static {
    fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, String>;
}

// Se crea un struct de estado de la app, en este caso solo contiene el trait de Repository(interfaz)
//envuelta en un box para evitar el desconocimiento del tamaño del Repository
pub struct RepositoryInjector {
    repository: Arc<Box<dyn Repository>>,
}

//Se hace una implementación del trait Repository con dos métodos, el new para crear la tupla del estado
//y el segundo que genera el web::Data que se necesita para pasar el dato a la petición y poder manejarla
impl RepositoryInjector {
    pub fn new(repo: impl Repository) -> Self {
        Self {
            repository: Arc::new(Box::new(repo)),
        }
    }

    // pub fn new_shared(repo: impl Repository) -> web::Data<Self> {
    //     web::Data::new(Self::new(repo))
    // }
}

// Se crea la implementación para poder clonar el repository injector,
// recordando que se se ha creado apartir de una tupla nombrada como
// "repository", por tanto accedemos a ella por nombre
impl Clone for RepositoryInjector {
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
        }
    }
}

// Se crea la implementación FromRequest para el repository injector, con la finalidad
// de poder enviar por request nuestro injector.
impl FromRequest for RepositoryInjector {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // intenta obtener el repository injector de la petición, si lo encuentra lo devuelve, de
        // lo contrario devuelve un error, indicando que no se ha encontrado.
        if let Some(injector) = req.app_data::<Self>() {
            let injector_owned = injector.to_owned();
            ready(Ok(injector_owned))
        } else {
            ready(Err(actix_web::error::ErrorBadRequest(
                "No repository injector was found in the request",
            )))
        }
    }
}

//Se hace una derreferenciación para que sea transparente la tupla del stado, y de esta forma poder
//acceder al método getUser del trait Repository
impl Deref for RepositoryInjector {
    type Target = dyn Repository;

    fn deref(&self) -> &Self::Target {
        self.repository.as_ref().as_ref()
    }
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
