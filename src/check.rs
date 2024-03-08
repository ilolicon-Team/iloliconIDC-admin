use dotenv::dotenv;
use std::env;
use std::path::Path;
use ansi_term::Color;

mod install_sqlite3;
mod install_mysql;

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
    } else if database_base == "mysql" {
        install_mysql::install_mysql();
    } else {
        println!("{} Install Failed!\nUnknown database type", Color::Red.paint("[Install]"));
    }
}