use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};

// ── Types ──

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumUser {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumPost {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: ForumUser,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub view_count: i64,
    pub comment_count: i64,
    pub is_pinned: bool,
    pub is_locked: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumComment {
    pub id: i32,
    pub post_id: i32,
    pub author: ForumUser,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ForumMessage {
    pub id: i32,
    pub sender: ForumUser,
    pub recipient: ForumUser,
    pub content: String,
    pub created_at: String,
    pub is_read: bool,
}

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub recipient_id: i32,
    pub content: String,
}

#[derive(Deserialize)]
pub struct PostsQuery {
    pub category: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

// ── Helper: get current user from auth header ──

async fn get_current_user(pool: &Pool<Postgres>, user_id: i32) -> Result<ForumUser, StatusCode> {
    let row = sqlx::query("SELECT id, name, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => {
            let uid: i32 = r.get("id");
            Ok(ForumUser {
                id: uid,
                name: r.get("name"),
                avatar: "👤".to_string(),
                is_admin: uid == 1 || uid == 2,
            })
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

// ── List Posts ──

async fn list_posts(
    State(pool): State<Pool<Postgres>>,
    Query(q): Query<PostsQuery>,
) -> Json<Vec<ForumPost>> {
    let rows = if let Some(ref cat) = q.category {
        if cat == "all" || cat.is_empty() {
            sqlx::query(
                "SELECT p.*, u.name as author_name, u.email as author_email,
                        (SELECT COUNT(*) FROM forum_comments WHERE post_id = p.id) as comment_count
                 FROM forum_posts p
                 JOIN users u ON u.id = p.author_id
                 ORDER BY p.is_pinned DESC, p.created_at DESC"
            )
            .fetch_all(&pool)
            .await
        } else {
            sqlx::query(
                "SELECT p.*, u.name as author_name, u.email as author_email,
                        (SELECT COUNT(*) FROM forum_comments WHERE post_id = p.id) as comment_count
                 FROM forum_posts p
                 JOIN users u ON u.id = p.author_id
                 WHERE p.category = $1
                 ORDER BY p.is_pinned DESC, p.created_at DESC"
            )
            .bind(cat)
            .fetch_all(&pool)
            .await
        }
    } else {
        sqlx::query(
            "SELECT p.*, u.name as author_name, u.email as author_email,
                    (SELECT COUNT(*) FROM forum_comments WHERE post_id = p.id) as comment_count
             FROM forum_posts p
             JOIN users u ON u.id = p.author_id
             ORDER BY p.is_pinned DESC, p.created_at DESC"
        )
        .fetch_all(&pool)
        .await
    };

    let rows = rows.unwrap_or_default();
    let posts: Vec<ForumPost> = rows
        .iter()
        .map(|r| {
            let author_id: i32 = r.get("author_id");
            ForumPost {
                id: r.get("id"),
                title: r.get("title"),
                content: r.get("content"),
                author: ForumUser {
                    id: author_id,
                    name: r.get("author_name"),
                    avatar: "👤".to_string(),
                    is_admin: author_id == 1 || author_id == 2,
                },
                category: r.get("category"),
                tags: r.get::<Vec<String>, _>("tags"),
                created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                view_count: r.get("view_count"),
                comment_count: r.get("comment_count"),
                is_pinned: r.get("is_pinned"),
                is_locked: r.get("is_locked"),
            }
        })
        .collect();

    Json(posts)
}

// ── Get Single Post ──

async fn get_post(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<ForumPost>, StatusCode> {
    // Increment view count
    let _ = sqlx::query("UPDATE forum_posts SET view_count = view_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    let row = sqlx::query(
        "SELECT p.*, u.name as author_name, u.email as author_email,
                (SELECT COUNT(*) FROM forum_comments WHERE post_id = p.id) as comment_count
         FROM forum_posts p
         JOIN users u ON u.id = p.author_id
         WHERE p.id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => {
            let author_id: i32 = r.get("author_id");
            Ok(Json(ForumPost {
                id: r.get("id"),
                title: r.get("title"),
                content: r.get("content"),
                author: ForumUser {
                    id: author_id,
                    name: r.get("author_name"),
                    avatar: "👤".to_string(),
                    is_admin: author_id == 1 || author_id == 2,
                },
                category: r.get("category"),
                tags: r.get::<Vec<String>, _>("tags"),
                created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                view_count: r.get("view_count"),
                comment_count: r.get("comment_count"),
                is_pinned: r.get("is_pinned"),
                is_locked: r.get("is_locked"),
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// ── Create Post ──

async fn create_post(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<ForumPost>, StatusCode> {
    // TODO: extract real user_id from JWT token in header
    // For now use user_id=1 as placeholder
    let user_id: i32 = 1;
    let user = get_current_user(&pool, user_id).await?;
    let category = payload.category.unwrap_or_else(|| "general".to_string());
    let tags = payload.tags.unwrap_or_default();

    let row = sqlx::query(
        "INSERT INTO forum_posts (title, content, author_id, category, tags)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, created_at, updated_at"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(user_id)
    .bind(&category)
    .bind(&tags)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id: i32 = row.get("id");
    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

    Ok(Json(ForumPost {
        id,
        title: payload.title,
        content: payload.content,
        author: user,
        category,
        tags,
        created_at: created_at.to_rfc3339(),
        updated_at: created_at.to_rfc3339(),
        view_count: 0,
        comment_count: 0,
        is_pinned: false,
        is_locked: false,
    }))
}

// ── Delete Post ──

async fn delete_post(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM forum_posts WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// ── Toggle Pin ──

async fn toggle_pin(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<ForumPost>, StatusCode> {
    sqlx::query("UPDATE forum_posts SET is_pinned = NOT is_pinned WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    get_post(State(pool), Path(id)).await
}

// ── Toggle Lock ──

async fn toggle_lock(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<ForumPost>, StatusCode> {
    sqlx::query("UPDATE forum_posts SET is_locked = NOT is_locked WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    get_post(State(pool), Path(id)).await
}

// ── List Comments ──

async fn list_comments(
    State(pool): State<Pool<Postgres>>,
    Path(post_id): Path<i32>,
) -> Json<Vec<ForumComment>> {
    let rows = sqlx::query(
        "SELECT c.*, u.name as author_name
         FROM forum_comments c
         JOIN users u ON u.id = c.author_id
         WHERE c.post_id = $1
         ORDER BY c.created_at ASC"
    )
    .bind(post_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let comments: Vec<ForumComment> = rows
        .iter()
        .map(|r| {
            let author_id: i32 = r.get("author_id");
            ForumComment {
                id: r.get("id"),
                post_id: r.get("post_id"),
                author: ForumUser {
                    id: author_id,
                    name: r.get("author_name"),
                    avatar: "👤".to_string(),
                    is_admin: author_id == 1 || author_id == 2,
                },
                content: r.get("content"),
                created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
            }
        })
        .collect();

    Json(comments)
}

// ── Create Comment ──

async fn create_comment(
    State(pool): State<Pool<Postgres>>,
    Path(post_id): Path<i32>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<Json<ForumComment>, StatusCode> {
    let user_id: i32 = 1; // TODO: extract from JWT
    let user = get_current_user(&pool, user_id).await?;

    // Check post exists and not locked
    let post = sqlx::query("SELECT is_locked FROM forum_posts WHERE id = $1")
        .bind(post_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match post {
        Some(r) => {
            let locked: bool = r.get("is_locked");
            if locked {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        None => return Err(StatusCode::NOT_FOUND),
    }

    let row = sqlx::query(
        "INSERT INTO forum_comments (post_id, author_id, content) VALUES ($1, $2, $3)
         RETURNING id, created_at"
    )
    .bind(post_id)
    .bind(user_id)
    .bind(&payload.content)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id: i32 = row.get("id");
    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

    Ok(Json(ForumComment {
        id,
        post_id,
        author: user,
        content: payload.content,
        created_at: created_at.to_rfc3339(),
    }))
}

// ── Delete Comment ──

async fn delete_comment(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM forum_comments WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// ── List Messages ──

async fn list_messages(
    State(pool): State<Pool<Postgres>>,
) -> Json<Vec<ForumMessage>> {
    let user_id: i32 = 1; // TODO: extract from JWT

    let rows = sqlx::query(
        "SELECT m.*, 
                s.name as sender_name, 
                r.name as recipient_name
         FROM forum_messages m
         JOIN users s ON s.id = m.sender_id
         JOIN users r ON r.id = m.recipient_id
         WHERE m.sender_id = $1 OR m.recipient_id = $1
         ORDER BY m.created_at DESC"
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let messages: Vec<ForumMessage> = rows
        .iter()
        .map(|r| {
            let sender_id: i32 = r.get("sender_id");
            let recipient_id: i32 = r.get("recipient_id");
            ForumMessage {
                id: r.get("id"),
                sender: ForumUser {
                    id: sender_id,
                    name: r.get("sender_name"),
                    avatar: "👤".to_string(),
                    is_admin: sender_id == 1 || sender_id == 2,
                },
                recipient: ForumUser {
                    id: recipient_id,
                    name: r.get("recipient_name"),
                    avatar: "👤".to_string(),
                    is_admin: recipient_id == 1 || recipient_id == 2,
                },
                content: r.get("content"),
                created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                is_read: r.get("is_read"),
            }
        })
        .collect();

    Json(messages)
}

// ── Send Message ──

async fn send_message(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ForumMessage>, StatusCode> {
    let user_id: i32 = 1; // TODO: extract from JWT
    let sender = get_current_user(&pool, user_id).await?;
    let recipient = get_current_user(&pool, payload.recipient_id).await?;

    let row = sqlx::query(
        "INSERT INTO forum_messages (sender_id, recipient_id, content)
         VALUES ($1, $2, $3)
         RETURNING id, created_at"
    )
    .bind(user_id)
    .bind(payload.recipient_id)
    .bind(&payload.content)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id: i32 = row.get("id");
    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");

    Ok(Json(ForumMessage {
        id,
        sender,
        recipient,
        content: payload.content,
        created_at: created_at.to_rfc3339(),
        is_read: false,
    }))
}

// ── Mark Message Read ──

async fn mark_message_read(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> StatusCode {
    let _ = sqlx::query("UPDATE forum_messages SET is_read = true WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    StatusCode::NO_CONTENT
}

// ── Mark Conversation Read ──

async fn mark_conversation_read(
    State(pool): State<Pool<Postgres>>,
    Path(partner_id): Path<i32>,
) -> StatusCode {
    let user_id: i32 = 1; // TODO: extract from JWT
    let _ = sqlx::query(
        "UPDATE forum_messages SET is_read = true
         WHERE sender_id = $1 AND recipient_id = $2 AND is_read = false"
    )
    .bind(partner_id)
    .bind(user_id)
    .execute(&pool)
    .await;

    StatusCode::NO_CONTENT
}

// ── Search Posts ──

async fn search_posts(
    State(pool): State<Pool<Postgres>>,
    Query(q): Query<SearchQuery>,
) -> Json<Vec<ForumPost>> {
    let query = match q.q {
        Some(ref s) if !s.trim().is_empty() => s.trim().to_lowercase(),
        _ => return Json(vec![]),
    };

    let pattern = format!("%{}%", query);

    let rows = sqlx::query(
        "SELECT p.*, u.name as author_name, u.email as author_email,
                (SELECT COUNT(*) FROM forum_comments WHERE post_id = p.id) as comment_count
         FROM forum_posts p
         JOIN users u ON u.id = p.author_id
         WHERE LOWER(p.title) LIKE $1 OR LOWER(p.content) LIKE $1
         ORDER BY p.is_pinned DESC, p.created_at DESC"
    )
    .bind(&pattern)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let posts: Vec<ForumPost> = rows
        .iter()
        .map(|r| {
            let author_id: i32 = r.get("author_id");
            ForumPost {
                id: r.get("id"),
                title: r.get("title"),
                content: r.get("content"),
                author: ForumUser {
                    id: author_id,
                    name: r.get("author_name"),
                    avatar: "👤".to_string(),
                    is_admin: author_id == 1 || author_id == 2,
                },
                category: r.get("category"),
                tags: r.get::<Vec<String>, _>("tags"),
                created_at: r.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                updated_at: r.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                view_count: r.get("view_count"),
                comment_count: r.get("comment_count"),
                is_pinned: r.get("is_pinned"),
                is_locked: r.get("is_locked"),
            }
        })
        .collect();

    Json(posts)
}

// ── Router ──

pub fn forum_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/api/forum/posts", get(list_posts).post(create_post))
        .route("/api/forum/posts/search", get(search_posts))
        .route("/api/forum/posts/{id}", get(get_post).delete(delete_post))
        .route("/api/forum/posts/{id}/pin", put(toggle_pin))
        .route("/api/forum/posts/{id}/lock", put(toggle_lock))
        .route("/api/forum/posts/{id}/comments", get(list_comments).post(create_comment))
        .route("/api/forum/comments/{id}", delete(delete_comment))
        .route("/api/forum/messages", get(list_messages).post(send_message))
        .route("/api/forum/messages/{id}/read", put(mark_message_read))
        .route("/api/forum/messages/conversation/{partner_id}/read", put(mark_conversation_read))
}
