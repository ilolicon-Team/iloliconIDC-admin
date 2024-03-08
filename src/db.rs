extern crate dotenv;
extern crate mysql;
extern crate ansi_term;

use std::env;
use dotenv::dotenv;
use mysql as my;
use my::prelude::Queryable;

pub fn read_only(tables: &str, data: &str, sqlwhere: &str) -> Vec<my::Row> {
    dotenv().ok();

    let db_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let db_port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let db_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let db_pass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
    let db_data = env::var("DATABASE_DATA").expect("DATABASE_DATA must be set");

    let opts_builder = my::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port.parse::<u16>().expect("Invalid port"))
        .user(Some(db_user))
        .pass(Some(db_pass))
        .db_name(Some(db_data));

    let pool = my::Pool::new(opts_builder).expect("Failed to create pool");
    
    let mut conn = pool.get_conn().expect("Failed to get connection from pool");

    // 构建查询语句
    let query = format!("SELECT {} FROM {} WHERE {}", data, tables, sqlwhere);

    conn.query_iter(query)
        .expect("Failed to execute query")
        .filter_map(Result::ok)
        .collect()
}
