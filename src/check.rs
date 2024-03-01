use dotenv::dotenv;
use std::env;
use std::path::Path;
use ansi_term::Color;

mod install_sqlite3;

pub fn check_sqlite3() {
    dotenv().ok();
    let database_base = env::var("DATABASE_BASE").unwrap();
    if database_base == "sqlite3" {
        let path = Path::new("data.db");
        if path.exists() {
            println!("{} Installed Skip!", Color::Red.paint("[Check]"));
        } else {
            match install_sqlite3::install_sqlite3() {
                Ok(_) => println!("{} Installed!", Color::Red.paint("[Install]")),
                Err(err) => {
                    println!("{} Install Failed!\nError: {}", Color::Red.paint("[Install]"), err);
                }
            }
        }
    } else {
        println!("{} 暂时无法自动安装非Sqlite3数据库!", Color::Red.paint("[Failed]"));
    }
}