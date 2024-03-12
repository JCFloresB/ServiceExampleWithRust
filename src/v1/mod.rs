use actix_web::web::{self, ServiceConfig};

mod users;

const PATH: &str = "/v1";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope(PATH).configure(users::service));
}
