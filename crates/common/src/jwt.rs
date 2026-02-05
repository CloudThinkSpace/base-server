use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use server_config::{jwt::JwtConfig, uri::IgnoreUri};

use crate::error::auth::AuthError;

// 定义 JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub exp: i64,    // 过期时间
    pub iat: i64,    // 签发时间
    pub data: Option<String>,
}

// Token 生成和验证
pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(mut config: JwtConfig) -> Self {
        if let Some(data) = &mut config.ignore_uris {
            JwtService::methods_to_uppercase(data);
        }
        Self { config }
    }

    // 生成 Token
    pub fn generate_token(
        &self,
        user_info: &str,
        data: Option<String>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.expiration_hours);

        let claims = Claims {
            sub: user_info.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            data,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_ref()),
        )
    }

    // 验证 Token
    pub fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        // 检查是否过期
        let now = Utc::now().timestamp();
        if token_data.claims.exp < now {
            return Err(AuthError::ExpiredToken);
        }

        Ok(token_data.claims)
    }

    // 获取加密钥匙
    pub fn get_secret(&self) -> String {
        self.config.secret.clone()
    }

    // 获取过期时间戳
    pub fn get_expiration(&self) -> i64 {
        self.config.expiration_hours
    }

    // 获取忽略uri信息
    pub fn is_ignore_uri(&self, uri: &str, method: &str) -> bool {
        if let Some(data) = &self.config.ignore_uris {
            for item in data.iter() {
                // 判断是否在跳过验证路径中
                if uri.starts_with(&item.path) && item.method.contains(&method.to_uppercase()) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    // 将请求方法转大写
    pub fn methods_to_uppercase(data: &mut [IgnoreUri]) {
        for item in data.iter_mut() {
            for method in item.method.iter_mut() {
                *method = method.to_uppercase();
            }
        }
    }
}
