// api.rs
mod login;

use axum::{
    extract::Form,
    response::{Json, IntoResponse},
};
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}


pub async fn status() -> impl IntoResponse {
    const MESSAGE: &str = "ilolicon IDC API Online";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn login(Form(frm): Form<LoginForm>) -> impl IntoResponse {
    // 调用登录函数进行用户验证并获取令牌
    if let Some(token) = login::login_and_get_token(&frm.username, &frm.password) {
        // 如果验证成功，返回带有令牌的成功响应
        let json_response = json!({
            "status": "success",
            "message": "Login successful",
            "token": token
        });

        Json(json_response)
    } else {
        // 如果验证失败，返回相应的错误状态码和消息
        Json(json!({"status": "error", "message": "Invalid username or password"}))
    }
}
