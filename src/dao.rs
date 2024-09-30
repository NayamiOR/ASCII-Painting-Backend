use crate::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::{Decode, Encode, Execute, Postgres, QueryBuilder, Type};

pub(crate) async fn save_painting(pool: &PgPool, painting: Painting) -> Result<i32, Error> {
    let returning = sqlx::query!(
        r#" 
        insert into paintings (name,author_id,content,created_at) 
        VALUES ($1, $2, $3, $4 ) 
        RETURNING id
        "#,
        painting.name,
        painting.author_id,
        painting.content,
        painting.created_at
    )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))?;

    Ok(returning.id)
}

pub(crate) async fn save_user(pool: &PgPool, user: &User) -> Result<i32, Error> {
    let pwd = bcrypt::hash(&user.password, 12).unwrap();
    let returning = sqlx::query!(
        r#" insert into users (username,email,password,created_at) 
        VALUES ($1, $2, $3, $4) 
        RETURNING id
        "#,
        user.username,
        user.email,
        pwd,
        user.created_at
    )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))?;

    Ok(returning.id)
}

pub(crate) async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, Error> {
    sqlx::query_as!(User, r#" select * from users where email = $1 "#, email)
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))
}

pub(crate) async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<User, Error> {
    sqlx::query_as!(User, r#" select * from users where id = $1 "#, id)
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))
}

pub(crate) async fn user_email_exist(pool: &PgPool, email: &str) -> Result<bool, Error> {
    let result = get_user_by_email(pool, email).await;

    check_result(result)
}

pub(crate) async fn user_id_exist(pool: &PgPool, id: i32) -> Result<bool, Error> {
    let result = get_user_by_id(pool, id).await;

    check_result(result)
}

pub(crate) async fn painting_exist(pool: &PgPool, id: i32) -> Result<bool, Error> {
    let result = get_painting_by_id(pool, id).await;

    check_result(result)
}

fn check_result<T>(result: Result<T, Error>) -> Result<bool, Error> {
    match result {
        Ok(_) => Ok(true),
        Err(Error::Database(e)) => {
            // 直接匹配 sqlx::Error::RowNotFound
            if matches!(*e, sqlx::Error::RowNotFound) {
                return Ok(false);
            }
            Err(Error::Database(e)) // 处理其他数据库错误
        }
        Err(e) => Err(e), // 处理非数据库错误
    }
}

pub(crate) async fn set_user_name(pool: &PgPool, user_id: i32, name: &str) -> Result<i32, Error> {
    let returning = sqlx::query!(
        r#" update users set username = $1 where id = $2 RETURNING id "#,
        name,
        user_id
    )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))?;

    Ok(returning.id)
}

pub(crate) async fn get_painting_by_id(pool: &PgPool, id: i32) -> Result<Painting, Error> {
    sqlx::query_as!(Painting, r#" select * from paintings where id = $1 "#, id)
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))
}

pub(crate) async fn get_paintings_by_author_id(
    pool: &PgPool,
    author_id: i32,
) -> Result<Vec<Painting>, Error> {
    sqlx::query_as!(
        Painting,
        r#" select * from paintings where author_id = $1 "#,
        author_id,
    ).fetch_all(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))
}

pub(crate) async fn like_painting(
    pool: &PgPool,
    painting_id: i32,
    user_id: i32,
    cancel_like: bool,
) -> Result<(), Error> {
    // insert user action
    // update painting like_num
    todo!()
}

pub(crate) async fn favorite_painting(
    pool: &PgPool,
    painting_id: i32,
    user_id: i32,
    cancel_favorite: bool,
) -> Result<(), Error> {
    // insert user action
    // update painting favorite_num
    todo!()
}

pub(crate) async fn filter_paintings(
    pool: &PgPool,
    filter: PaintingFilter,
) -> Result<Vec<Painting>, Error> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM paintings ");
    if let Some(sort) = filter.sort {
        query_builder.push("ORDER BY ");
        match sort {
            PaintingSort::Default => query_builder.push("created_at"),
            PaintingSort::CreatedAt => query_builder.push("created_at"),
            PaintingSort::LikeNum => query_builder.push("like_num"),
            PaintingSort::FavoriteNum => query_builder.push("favorite_num"),
        };
    }
    query_builder.push(" WHERE 1=1 ");
    if let Some(state) = filter.state {
        query_builder.push("AND state = ").push_bind(state as i32);
    }
    if let Some(time) = filter.time {
        query_builder.push("AND created_at > ").push_bind(time);
    }
    if let Some(page) = filter.page {
        query_builder.push("LIMIT 10 OFFSET ").push_bind(page * 10);
    }

    let query = query_builder
        .build()
        .sql();
    let paintings = sqlx::query_as(query)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(Box::new(e)))?;

    Ok(paintings)
}

pub(crate) async fn update_avatar(pool: &PgPool, user_id: i32, avatar: &str) -> Result<(), Error> {
    todo!()
}

pub(crate) async fn delete_painting_by_id(pool: &PgPool, painting_id: i32) -> Result<(), Error> {
    todo!()
}

pub(crate) struct PaintingFilter {
    pub(crate) page: Option<i32>,
    pub(crate) sort: Option<PaintingSort>,
    pub(crate) time: Option<DateTime<Utc>>,    // 时间戳格式
    pub(crate) state: Option<PaintingState>,
}

impl Default for PaintingFilter {
    fn default() -> Self {
        Self {
            page: Some(1),
            sort: Some(PaintingSort::Default),
            time: Some(Utc::now()),
            state: Some(PaintingState::Unreviewed),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) enum PaintingSort {
    Default,
    CreatedAt,
    LikeNum,
    FavoriteNum,
}

impl From<i32> for PaintingSort {
    fn from(value: i32) -> Self {
        match value {
            0 => PaintingSort::Default,
            1 => PaintingSort::CreatedAt,
            2 => PaintingSort::LikeNum,
            3 => PaintingSort::FavoriteNum,
            _ => panic!("Invalid value for PaintingSort"),
        }
    }
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub(crate) struct Painting {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) content: String,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) author_id: i32,
    pub(crate) favorite_num: i32,
    pub(crate) like_num: i32,
    pub(crate) state: PaintingState,
}

#[derive(Debug, Clone, PartialEq, Type, Deserialize, Serialize)]
pub(crate) enum PaintingState {
    Unreviewed,
    Passed,
    NotPassed,
}

impl From<i32> for PaintingState {
    fn from(value: i32) -> Self {
        match value {
            0 => PaintingState::Unreviewed,
            1 => PaintingState::Passed,
            2 => PaintingState::NotPassed,
            _ => panic!("Invalid value for PaintingState"),
        }
    }
}

#[derive(sqlx::FromRow, Clone)]
pub(crate) struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::{get_random_string, init_pool};
    use lazy_static::lazy_static;
    use rand::random;
    use sqlx::types::chrono::Utc;
    use std::env;
    use std::sync::Once;

    lazy_static! {
        // 通过 `Once` 来确保只设置一次
        static ref INIT: Once = Once::new();
    }

    fn init_env_vars() {
        INIT.call_once(|| {
            dotenv::dotenv().ok();
        });
    }

    #[tokio::test]
    async fn test_save_get_user() {
        // 测试save user和get user by email
        init_env_vars();
        let pool = init_pool().await;
        let url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
        let random_id = random::<i32>();
        let random_name = get_random_string(16);
        let email: String = format!("{}@example.com", random_name);
        let user = User {
            id: random_id,
            username: random_name.clone(),
            email: email.clone(),
            created_at: Utc::now().naive_utc(),
            password: get_random_string(64),
        };

        let returning = save_user(&pool, &user).await.unwrap();

        let get_user = get_user_by_email(&pool, email.as_str()).await.unwrap();
        assert_eq!(get_user.username, random_name);
        assert_eq!(get_user.email, email);
        assert!(bcrypt::verify(&user.password, &get_user.password).unwrap());

        let get_user = get_user_by_id(&pool, returning).await.unwrap();
        assert_eq!(get_user.username, random_name);
        assert_eq!(get_user.email, email);
        assert!(bcrypt::verify(&user.password, &get_user.password).unwrap());
    }

    #[tokio::test]
    async fn test_paintings() {
        init_env_vars();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL MUST BE SET");
        let painting = Painting {
            id: random::<i32>(),
            name: get_random_string(64),
            content: "test".to_string(),
            created_at: Utc::now().naive_utc(),
            author_id: 1,
            favorite_num: 0,
            like_num: 0,
            state: PaintingState::Unreviewed,
        };

        let returning = save_painting(&init_pool().await, painting.clone())
            .await
            .unwrap();

        let get_painting = get_painting_by_id(&init_pool().await, returning).await;
        dbg!(&get_painting);
        let get_painting = get_painting.unwrap();

        assert_eq!(get_painting.name, painting.name);
        assert_eq!(get_painting.content, painting.content);
        assert_eq!(get_painting.author_id, painting.author_id);
        assert_eq!(get_painting.favorite_num, painting.favorite_num);
        assert_eq!(get_painting.like_num, painting.like_num);
    }
}
