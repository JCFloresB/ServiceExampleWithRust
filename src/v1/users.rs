use actix_web::{
    error::PathError,
    web::{self, PathConfig, ServiceConfig},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::{repository::Repository, user::User};

const PATH: &str = "/user";

pub fn service<R: Repository>(cfg: &mut ServiceConfig) {
    // cfg.service(web::scope("/user").service(getuser)); //se puede poner el scope, sin embargo en este caso get tiene la dirección correcta
    // cfg.service(get); //get user method 1
    // cfg.service(
    //     web::scope(PATH)
    //         .app_data(PathConfig::default())
    //         .route("/{user_id}", web::get().to(get_user)),
    // ); //get user method 2

    // se utiliza el estractor para manejar el error al mandar una petición con un Uuid incorrecto, específicamente un error 400 (bad request)
    // esto es, por medio de una conficuración (PathConfig) y se hace de esta forma para que solo afecte al get (/user/{user_id})
    // despues de la coma "," se puede hacer otro método, por ejemplo un post y se anexarian al método las configuraciones pertinentes y que solo
    // afecten a este.
    cfg.service(
        web::scope(PATH)
            .app_data(PathConfig::default().error_handler(path_config_handler))
            // GET
            .route("/{user_id}", web::get().to(get_user::<R>))
            // POST
            .route("/", web::post().to(post::<R>))
            // PUT
            .route("/", web::put().to(put::<R>))
            // DELETE
            .route("/{user_id}", web::delete().to(delete::<R>)),
    );
}
/* #region get user method 1*/
// #[get("/user/{user_id}")]
// async fn get<R: Repository>(
//     user_id: web::Path<String>,
//     repo: web::Data<R>,
//     // repo: web::Data<RepositoryInjector>,
// ) -> HttpResponse {
//     if let Ok(parsed_user_id) = Uuid::parse_str(&user_id) {
//         match repo.get_user(&parsed_user_id) {
//             Ok(user) => HttpResponse::Ok().json(user),
//             Err(_) => HttpResponse::NotFound().body("Not found"),
//         }
//     } else {
//         HttpResponse::BadRequest().body("Invalid UUID")
//     }
// }
/* #endregion */

/* #region get user method 2 */
#[warn(dead_code)]
async fn get_user<R: Repository>(
    user_id: web::Path<Uuid>,
    // user_id: web::Path<String>,
    repo: web::Data<R>,
    // repo: RepositoryInjector,
    // repo: web::Data<RepositoryInjector>,
) -> HttpResponse {
    // if let Ok(parsed_user_id) = Uuid::parse_str(&user_id) {
    match repo.get_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
    // } else {
    //     HttpResponse::BadRequest().body("Invalid UUID")
    // }
}
/* #endregion */

async fn post<R: Repository>(
    user: web::Json<User>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_user(&user) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Someting went wrong: {}", e)),
    }
}

async fn put<R: Repository>(
    user: web::Json<User>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.update_user(&user) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            HttpResponse::NotFound().body(format!("Someting went wrong: {}", e))
        }
    }
}

async fn delete<R: Repository>(
    user_id: web::Path<Uuid>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.delete_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Someting went wrong: {}", e)),
    }
}

fn path_config_handler(err: PathError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::ErrorBadRequest(err)
}

#[cfg(test)]
mod test_user {

    use crate::user::User;

    use super::*;
    use actix_web::body::MessageBody;
    use actix_web::http::StatusCode;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        CustomRepo {}
        impl Repository for CustomRepo {
            fn get_user(&self, user_id: &uuid::Uuid)-> Result<User, String>;
            fn create_user(&self, user: &User) -> Result<User, String>;
            fn update_user(&self, user: &User) -> Result<User, String>;
            fn delete_user(&self, user_id: &uuid::Uuid) -> Result<Uuid, String>;
        }
    }

    #[actix_rt::test]
    async fn it_workrs() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "Juan Carlos";

        let mut repo = MockCustomRepo::default();
        repo.expect_get_user().returning(move |id| {
            let mut user = User::new(String::from(user_name), (1984, 02, 14));
            user.id = *id;
            Ok(user)
        });

        let result =
            get_user(web::Path::from(user_id), web::Data::new(repo)).await;

        assert_eq!(result.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn user_id_equeals() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "Juan Carlos";

        let mut repo = MockCustomRepo::default();
        repo.expect_get_user().returning(move |id| {
            let mut user = User::new(String::from(user_name), (1984, 02, 14));
            user.id = *id;
            Ok(user)
        });

        let result =
            get_user(web::Path::from(user_id), web::Data::new(repo)).await;

        let body = result.into_body().try_into_bytes().unwrap();

        let user: User = serde_json::from_slice(&body).unwrap();

        assert_eq!(user.id, user_id);
    }

    #[actix_rt::test]
    async fn user_name_is_equals() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "Juan Carlos";

        let mut repo = MockCustomRepo::default();
        repo.expect_get_user().returning(move |id| {
            let mut user = User::new(String::from(user_name), (1984, 02, 14));
            user.id = *id;
            Ok(user)
        });

        let result =
            get_user(web::Path::from(user_id), web::Data::new(repo)).await;

        assert_eq!(result.status(), StatusCode::OK);

        let body = result.into_body().try_into_bytes().unwrap();

        let user: User = serde_json::from_slice(&body).unwrap();

        assert_eq!(user.name, user_name);
    }

    #[actix_rt::test]
    async fn user_name_is_different() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "Juan Carlos";

        let mut repo = MockCustomRepo::default();
        repo.expect_get_user().returning(move |id| {
            let mut user = User::new(String::from("Pancho"), (1984, 02, 14));
            user.id = *id;
            Ok(user)
        });

        let result =
            get_user(web::Path::from(user_id), web::Data::new(repo)).await;

        assert_eq!(result.status(), StatusCode::OK);

        let body = result.into_body().try_into_bytes().unwrap();

        let user: User = serde_json::from_slice(&body).unwrap();

        assert_ne!(user.name, user_name);
    }
}
