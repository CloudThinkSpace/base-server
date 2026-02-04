use std::sync::Arc;

///
/// 认证中间件
/// 通过请求路径查看是否可以通过认证
///
///
///
use axum::{extract::Request, http::header, middleware::Next, response::Response};
use common::{error::auth::AuthError, jwt::JwtService};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, AuthError> {
    // 跳过公开路径（可选）
    let path = req.uri().path();
    // 请求方法
    let method = req.method().clone();

    // 获取jwt服务
    let service = req
        .extensions()
        .get::<Arc<JwtService>>()
        .ok_or(AuthError::MissingToken)?;

    // 判断是否为跟目录
    if path.starts_with("/") {
        return Ok(next.run(req).await);
    }

    // 判断是否需要认证
    if service.is_ignore_uri(path, method.as_str()) {
        return Ok(next.run(req).await);
    }

    // 获取认证token
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AuthError::MissingToken)?;

    // 验证 JWT
    let claims = service.verify_token(token)?;

    // 为了在提取器中减少开销
    req.extensions_mut().insert(Arc::new(claims));

    Ok(next.run(req).await)
}
