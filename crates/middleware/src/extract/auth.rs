use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use server_common::{error::auth::AuthError, jwt::Claims};

pub struct Auth(pub Arc<Claims>);

/// > 定义用户信息提取器，提取用户编号和角色信息
///
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Arc<Claims>>()
            .ok_or(AuthError::MissingToken)?;
        // 拼装对象,不使用Claims，减少开销
        let auth_info = Auth(claims.clone());
        Ok(auth_info)
    }
}
