use dotenv::dotenv;
use mysql::{OptsBuilder, Pool};

pub fn install_mysql() -> Result<(), mysql::Error> {
    // 加载 .env 文件
    dotenv().ok();

    // 获取数据库配置
    let db_host = std::env::var("DATABASE_HOST").unwrap();
    let db_port = std::env::var("DATABASE_PORT").unwrap();
    let db_user = std::env::var("DATABASE_USER").unwrap();
    let db_pass = std::env::var("DATABASE_PASS").unwrap();
    let db_data = std::env::var("DATABASE_DATA").unwrap();

    // 拼接数据库连接字符串
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_data
    );

    // 建立数据库连接池
    let mut opts_builder = OptsBuilder::new();
    opts_builder.url(db_url);
    let pool = mysql::Pool::new(connection_string).unwrap();

    // 获取数据库连接
    let mut conn = pool.get_conn()?;
    let sql_content = std::fs::read_to_string("database/20240301.sql").unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(sql_content).unwrap();
    Ok(conn)
}
