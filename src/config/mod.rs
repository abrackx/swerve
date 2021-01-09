use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn migrate_and_config_db(url: &str) -> Pool {
    info!("Migrating database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    match embedded_migrations::run(&pool.get().expect("Could not connect to pool.")) {
        Ok(()) => pool,
        Err(e) => {
            panic!("Failed to run migration: {:?}", e)
        }
    }
}