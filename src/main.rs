use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}
#[derive(Debug, Serialize)]
struct CreateUserResponse {
    name: String,
    email: String,
    id: i32,
}
async fn create_user(
    pool: State<Pool<Postgres>>,
    payload: Json<CreateUserRequest>,
) -> Json<CreateUserResponse> {
    let row = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id")
        .bind(&payload.name)
        .bind(&payload.email)
        .fetch_one(&*pool)
        .await;
    Json(CreateUserResponse {
        id: row.unwrap().get(0),
        name: payload.name.clone(),
        email: payload.email.clone(),
    })
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::<Postgres>::connect("postgres://postgres:@localhost:5432/postgres").await?;
    println!("Hello, world!");
    match sqlx::query("CREATE DATABASE users").execute(&pool).await {
        Ok(_) => println!("数据库users 创建成功."),
        Err(_) => {
            println!("数据库users已存在.")
        }
    }
    let pool = Pool::<Postgres>::connect("postgres://postgres:@localhost:5432/users").await?;
    match sqlx::query("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE)")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("表users 创建成功."),
        Err(_) => {println!("表users已存在.")
        }
    }
    let app = Router::new()
        .route("/", post(create_user).get(|| async { "Hello, World!" }))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
