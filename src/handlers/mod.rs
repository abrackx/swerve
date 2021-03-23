use actix_web::web::{scope, ServiceConfig};
use crate::config::Pool;

mod files;
mod projects;
mod users;

pub fn init(cfg: &mut ServiceConfig) {
    info!("Configuring services for {:?}...", std::thread::current().id());
    cfg.service(scope("/api")
        /*
        GET:
        /api/users/1
        /api/users/projects/1
        /api/projects
        /api/projects/1
        /api/projects/1/tags
        /api/tags
        POST:
        /api
         */
        .service(users::get_users)
        .service(users::get_users_by_id)
        .service(projects::get_projects_by_uid)
        .service(projects::get_projects_by_id)
        .service(files::save_file)
    );
}