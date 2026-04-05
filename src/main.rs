use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use rand;
use resend_rs::types::{CreateEmailBaseOptions, Tag};
use resend_rs::{Resend, Result};
mod database;
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    code: String,
    password: String,
    _confirm_password: String,
}
#[derive(Serialize)]
struct CreateUserResponse {
    status_code: u16,
    name: String,
    email: String,
    id:i32,
}
#[derive(Debug, Deserialize)]
struct _LoginRequest {
    email: String,
    password: String,
}
struct _LoginResponse {
    status_code: StatusCode,
}
async fn create_user(
    pool: State<Pool<Postgres>>,
    payload: Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {
    let temp:String = sqlx::query("SELECT code FROM verify_code WHERE email = $1")
        .bind(&payload.email).fetch_one(&*pool).await.unwrap().get(0);
    if temp != payload.code{
        return Json(CreateUserResponse {
            status_code: 401,
            name: payload.name.clone(),
            email: payload.email.clone(),
            id: 0,
        })
    }
    let row = sqlx::query("INSERT INTO users (name, email,password) VALUES ($1, $2, $3) RETURNING id")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.password)
        .fetch_one(&*pool)
        .await;

    Json(CreateUserResponse {
        status_code: 200,
        name: payload.name.clone(),
        email: payload.email.clone(),
        id: row.unwrap().get(0),
    })
}
async fn _login_check(_pool: State<Pool<Postgres>>,_payload:Json<_LoginRequest>) -> Json<CreateUserResponse> {
    todo!()
}
async fn send_verification_code(pool:State<Pool<Postgres>>,email:String) {
    let code = rand::random_range(100000..=999999);
    sqlx::query("INSERT INTO verify_code (email, code) VALUES ($1, $2)")
        .bind(&email)
        .bind(&code)
        .execute(&*pool)
        .await
        .unwrap();
    let text = format!("【devbit】验证码：{}，有效期5分钟，如非本人操作，请忽略。",code);
    let resend = Resend::default();

    let from = "onboarding@resend.dev";
    let to = [email];
    let subject = "devbit";

    let email = CreateEmailBaseOptions::new(from, to, subject)
        .with_text(&text)
        .with_tag(Tag::new("dev", "bit"));

    let _id = resend.emails.send(email).await.unwrap();
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = database::db_init().await?;
    let app = Router::new()
        .route("/register", post(create_user))
        .route("/register/send_code",post(send_verification_code))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
