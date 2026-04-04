use axum::{response::Html, routing::get, Router};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建数据库
    let pool = PgPoolOptions::new()
        .connect("postgresql://postgres:@localhost/postgres")  // 替换为你的数据库URL
        .await?;

    sqlx::query("CREATE DATABASE IF NOT EXISTS users")
        .execute(&pool)
        .await?;

    println!("PostgreSQL数据库已创建！");

    // 创建axum路由
    let app = Router::new()
        .route("/", get(root_handler));

    // 启动服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("服务器启动在 http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
