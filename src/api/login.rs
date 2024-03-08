use crate::db;

pub fn login_and_get_token(username: &str, password: &str) -> Option<String> {
    // 调用数据库函数进行用户验证并获取令牌
    db::read_only("users", "token", &format!("username='{}' AND password='{}'", username, password))
        .iter()
        .map(|row| row.get("token").unwrap())
        .next()
}
