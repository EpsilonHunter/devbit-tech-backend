use sqlx::{Pool, Postgres};
pub async fn db_init() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = Pool::<Postgres>::connect("postgres://postgres:@localhost:5432/postgres").await?;
    println!("Hello, world!");
    match sqlx::query("CREATE DATABASE users").execute(&pool).await {
        Ok(_) => println!("数据库users创建成功."),
        Err(_) => println!("数据库users已存在."),

    }
    let pool = Pool::<Postgres>::connect("postgres://postgres:@localhost:5432/users").await?;
    match sqlx::query("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE)")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("表users创建成功."),
        Err(_) => println!("表users已存在."),

    }
    match sqlx::query("ALTER TABLE users ADD COLUMN password VARCHAR(255) NOT NULL")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("密码列添加成功."),
        Err(_) => println!("密码列已存在."),

    }
    match sqlx::query("CREATE TABLE verify_code (email TEXT NOT NULL, code VARCHAR(6) NOT NULL")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("表verify_code创建成功."),
        Err(_) => println!("表verify_code已存在."),
    }
    Ok(pool)
}
