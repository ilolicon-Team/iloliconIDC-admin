extern crate dotenv;
extern crate mysql;

use std::env;
use std::fs;
use dotenv::dotenv;
use mysql as my;
use my::prelude::Queryable;
use ansi_term::Color;

pub fn install_mysql() {
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

    let sql_content = fs::read_to_string("database/20240301.sql").expect("Failed to read SQL file");

    for query in sql_content.split(';') {
        if !query.trim().is_empty() {
            let query = query.trim();
            conn.query_drop(query).expect("Failed to execute query");
        }
    }
    println!("{} Installed!", Color::Red.paint("[Install]"));
}