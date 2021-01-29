use actix_web::web::{scope, ServiceConfig};

mod files;
mod projects;
mod users;

pub fn init(cfg: &mut ServiceConfig) {
    info!("Configuring services for {:?}...", std::thread::current().id());
    cfg.service(scope("/api")
        .service(users::get_users)
        .service(users::get_users_by_id)
        .service(projects::get_projects_by_uid)
        .service(projects::get_projects_by_id)
        .service(files::save_file)
    );
}