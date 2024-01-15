use actix_web::web;
use crate::handlers::user::{create_user, delete_user, get_user_details, update_user};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/user")
            .route("/register", web::post().to(create_user))
            .route("/{telephone}", web::put().to(update_user))
            .route("/{telephone}", web::delete().to(delete_user))
            .route("/{telephone}", web::get().to(get_user_details))
        );
}