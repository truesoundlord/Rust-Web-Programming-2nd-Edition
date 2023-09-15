mod login;
mod logout;

use actix_web::web::{ServiceConfig, get, scope};


pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("views/auth") // 'v1' doesn't exists in the filesystem  
            .route("login", get().to(login::login))
            .route("logout", get().to(logout::logout))
    );
}
