pub mod config;
pub use config::*;

pub mod database{
    pub mod init_db_pool;
    pub mod models;
    pub mod schema;
    pub use init_db_pool::initialize_db_pool;
    pub use models::*;
    pub use schema::*;
}
pub mod tools{
    pub mod encrypt;
    pub use encrypt::*;

}
pub mod middlewares{
    pub mod sessions;
    pub mod handle_errors;
    pub mod auth;
    pub use sessions::*;
    pub use handle_errors::*;
}
pub mod services{
    pub mod auth_services{
        pub mod register_user;
        pub mod login_user;
        pub use register_user::*;
        pub use login_user::*;
    }
    pub mod dir_services{
        pub mod create_dirs;
        pub use create_dirs::*;
    }
    pub mod user_services{
        pub mod show_user;
        pub use show_user::*;
    }

}

pub mod handlers{
    pub mod errors;
    pub mod auth_handler;
    pub mod dir_handlers;
    pub mod index_handlers;
    pub use errors::*;
    pub use auth_handler::*;
    pub use dir_handlers::*;
    pub use index_handlers::*;
}

pub mod routes{
    pub mod main;
    pub mod auth_routes;
    pub mod dir_routes;
    pub mod user_routes;
    pub mod index_routes;
    pub use main::routes;
    pub use auth_routes::*;
    pub use dir_routes::*;
    pub use user_routes::*;
    pub use index_routes::*;
}