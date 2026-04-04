use axum::{response::Html, routing::get, Router};
use sqlx::{Pool, Postgres};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::<Postgres>::connect("postgres://postgres:@localhost:5432/postgres").await?;
    println!("Hello, world!");
    match sqlx::query("CREATE DATABASE users")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("✓ 数据库 users 创建成功"),
        Err(e) => {println!("go on")
        }
    }

    let app = Router::new()
        .route("/", get(root_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
