use actix_web::{
    error::PathError,
    get,
    web::{self, PathConfig, ServiceConfig},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::repository::RepositoryInjector;

const PATH: &str = "/user";

pub fn service(cfg: &mut ServiceConfig) {
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
        web::scope(PATH).service(
            web::resource("/{user_id}")
                .app_data(
                    PathConfig::default().error_handler(path_config_handler),
                )
                .route(web::get().to(get_user)),
        ),
    );
}
/* #region get user method 1*/
#[get("/user/{user_id}")]
async fn get(
    user_id: web::Path<String>,
    repo: web::Data<RepositoryInjector>,
) -> HttpResponse {
    if let Ok(parsed_user_id) = Uuid::parse_str(&user_id) {
        match repo.get_user(&parsed_user_id) {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("Not found"),
        }
    } else {
        HttpResponse::BadRequest().body("Invalid UUID")
    }
}
/* #endregion */

/* #region get user method 2 */
#[warn(dead_code)]
async fn get_user(
    user_id: web::Path<Uuid>,
    // user_id: web::Path<String>,
    repo: RepositoryInjector,
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

fn path_config_handler(err: PathError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::ErrorBadRequest(err)
}
