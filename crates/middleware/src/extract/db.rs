#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use common::error::db::DbError;
use sqlx::PgPool;

pub struct DbPool(pub Arc<PgPool>);
/// > 自定义提取器，提取数据库连接池对象
///
impl<S> FromRequestParts<S> for DbPool
where
    S: Send + Sync,
{
    type Rejection = DbError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = parts
            .extensions
            .get::<Arc<PgPool>>()
            .ok_or(DbError::PoolIsNotExistError)?;
        Ok(DbPool(pool.clone()))
    }
}
