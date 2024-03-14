use actix_web::web::{self, ServiceConfig};

use crate::repository::Repository;

mod users;

const PATH: &str = "/v1";

pub fn service<R: Repository>(cfg: &mut ServiceConfig) {
    cfg.service(web::scope(PATH).configure(users::service::<R>));
}
