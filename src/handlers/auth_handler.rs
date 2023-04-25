use crate::{
    database::{init_db_pool::DbPool, User},
    services::{auth_services::{login_user, register_user}, dir_services::create_dir}, middlewares::close_session, tools::{hash_password, rand_key},
};
use actix_session::Session;
use actix_web::{web::*, HttpResponse, Result, error::{ErrorUnauthorized, ErrorConflict} };
use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize)]
pub struct SignupForm {
    username: String,
    email: String,
    password: String,
}
#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    //pub remember_me: String,
}
pub async fn render_signup(tera: Data<Tera>) -> Result<HttpResponse> {
    let mut context: Context = Context::new();
    context.insert("title", "Registre-se");
    let rendered = tera.render("pages/signup.tera", &context).unwrap();

    Ok(HttpResponse::Ok().body(rendered))
}
pub async fn handle_signup(
    session: Session,
    pool: Data<DbPool>,
    signup_form: Form<SignupForm>,
) -> Result<HttpResponse> {
    let key = &rand_key();
    let user: User = User {
        id: 1,
        name: signup_form.username.clone(),
        email: signup_form.email.clone(),
        role: String::from("user"),
        password: Some(hash_password(&signup_form.password)),
        dir_id: (&key).to_string()
    };
    match register_user(pool, &user).await {
        Ok(_) => {
            create_dir((&key).to_string()).expect("Failed to create use folder");
            Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish())},
        Err(err) => Err(ErrorConflict(err))
    }
}

pub async fn render_login(tera: Data<Tera>) -> Result<HttpResponse> {
    let mut context: Context = Context::new();
    context.insert("title", "Login");
    let rendered = tera.render("pages/login.tera", &context).unwrap();
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn handle_login(
    session: Session,
    pool: Data<DbPool>,
    login_form: Form<LoginForm>,
) -> Result<HttpResponse> {
    let credentials: LoginForm = LoginForm {
        email: login_form.email.clone(),
        password: login_form.password.clone(),
        //remember_me: login_form.remember_me.clone(),
    };
    match login_user(session, pool, &credentials).await {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish()),
        Err(err) => Err(ErrorUnauthorized(err))
    }
}

pub async fn handle_logout(
    session: Session,
    pool: Data<DbPool>
) -> Result<HttpResponse> {
    close_session(pool,session).await;
    Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
}

