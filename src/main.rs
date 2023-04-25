use actix_files::Files;
use actix_web::middleware::ErrorHandlers;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use module::middlewares::handle_errors;
use module::routes::main::routes;
use module::{config, database::initialize_db_pool, print_logo};
use std::time::Duration;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config();
    let tera = Tera::new("templates/**/*").unwrap();
    let pool = initialize_db_pool();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("static/", "static").show_files_listing())
            .configure(routes)
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(ErrorHandlers::new().default_handler(handle_errors)
            )
    })
    .keep_alive(Duration::from_secs(75))
    .workers(4)
    .bind(&config.address)?
    .run();
    print_logo();
    println!("\x1b[32;49;1m-->Connected to database! \x1b[0m");
    println!(
        "\x1b[32;49;1m-->Server Started \x1b[0maccess link http://{}",
        &config.address
    );
    server.await
}
