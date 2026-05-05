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
    match sqlx::query("CREATE TABLE verify_code (email TEXT NOT NULL, code VARCHAR(6) NOT NULL)")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("表verify_code创建成功."),
        Err(_) => println!("表verify_code已存在."),
    }
    match sqlx::query("ALTER TABLE users ADD COLUMN avatar VARCHAR(255) NOT NULL DEFAULT ''")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("avatar列添加成功."),
        Err(_) => println!("avatar列已存在."),
    }

    match sqlx::query("ALTER TABLE users ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT FALSE")
        .execute(&pool)
        .await
    {
        Ok(_) => println!("is_admin列添加成功."),
        Err(_) => println!("is_admin列已存在."),
    }
    match sqlx::query(
        "CREATE TABLE posts (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        author_id INT NOT NULL REFERENCES users(id),
        category TEXT NOT NULL,
        tags TEXT[] NOT NULL DEFAULT '{}',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        view_count INT NOT NULL DEFAULT 0,
        comment_count INT NOT NULL DEFAULT 0,
        like_count INT NOT NULL DEFAULT 0,
        is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
        is_locked BOOLEAN NOT NULL DEFAULT FALSE
    )"
    )
        .execute(&pool)
        .await
    {
        Ok(_) => println!("posts表格添加成功."),
        Err(_) => println!("posts表格已存在."),
    }
    Ok(pool)
}
