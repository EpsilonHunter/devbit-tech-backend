use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use rand;
use chrono::{Utc,Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use dotenv::dotenv;
mod database;
#[derive(Serialize)]
struct User {
    id:i32,
    name: String,
    email: String,
}
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    code: String,
    password: String,
}
#[derive(Serialize)]
struct CreateUserResponse {
    name: String,
    email: String,
    id:i32,
}
#[derive(Deserialize)]
struct SendCodeRequest {
    email: String,
}
#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}
#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: User,
}
async fn create_user(
    pool: State<Pool<Postgres>>,
    payload: Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {

    let temp:String = sqlx::query("SELECT code FROM verify_code WHERE email = $1")
        .bind(&payload.email).fetch_one(&*pool).await.unwrap().get(0);
    if temp != payload.code{
        return Json(CreateUserResponse {
            name: payload.name.clone(),
            email: payload.email.clone(),
            id: 0,
        })
    }
    println!("接收到前端json，开始将用户数据插入数据库");
    let row = sqlx::query("INSERT INTO users (name, email,password) VALUES ($1, $2, $3) RETURNING id")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.password)
        .fetch_one(&*pool)
        .await;
    println!("插入成功!");
    Json(CreateUserResponse {
        name: payload.name.clone(),
        email: payload.email.clone(),
        id: row.unwrap().get(0),
    })
}
async fn login_check(pool: State<Pool<Postgres>>,payload:Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let row = sqlx::query("SELECT password, id, name FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_one(&*pool)
        .await.unwrap();
    let db_password: String = row.get(0);
    let user_id: i32 = row.get(1);
    let user_name: String = row.get(2);
    if db_password == payload.password {
        let token = generate_token(user_id, &user_name);
        Ok(Json(LoginResponse {
            token,
            user: User {
                id: user_id,
                name: user_name,
                email: payload.email.clone(),
            },
        }))
    }
    else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
async fn send_verification_code(pool:State<Pool<Postgres>>,req:Json<SendCodeRequest>) {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)"
    )
        .bind(&req.email)
        .fetch_one(&*pool)
        .await.unwrap();
    if exists {

    }
    else {
        println!("接收到前端json，开始发送验证码");
        let code = rand::random_range(100000..=999999);
        sqlx::query("INSERT INTO verify_code (email, code) VALUES ($1, $2)")
            .bind(&req.email)
            .bind(&code)
            .execute(&*pool)
            .await
            .unwrap();
        let email = Message::builder()
            .from(Mailbox::new(Some("devbit".to_owned()), "2043399410@qq.com".parse().unwrap()))
            .to(Mailbox::new(Some("client".to_owned()), req.email.parse().unwrap()))
            .subject("devbit")
            .header(ContentType::TEXT_PLAIN)
            .body(format!("[devbit]验证码:{},有效期5分钟,如非本人操作，请忽略.",code))
            .unwrap();

        let creds = Credentials::new("2043399410@qq.com".to_owned(), "raaukatcqjxydiaa".to_owned());

        let mailer = SmtpTransport::relay("smtp.qq.com")
            .unwrap()
            .port(465)
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }

}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = database::db_init().await?;
    let app = Router::new()
        .route("/register", post(create_user))
        .route("/register/send_code",post(send_verification_code))
        .route("/login",post(login_check))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    email: String,
    exp: usize,
}

fn generate_token(user_id: i32, email: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_bytes()),
    )
        .unwrap()
}
