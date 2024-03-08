mod handler;
mod model;
mod response;
mod route;
mod check;
mod db;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use route::create_router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    // 跳过程序名称
    args.next();

    match args.next() {
        Some(arg) => {
            match arg.as_str() {
                "help" => {
                    println!("help 显示帮助信息");
                    println!("install 安装数据库");
                    println!("run 启动程序");
                }
                "install" => check::check_sqlite3(),
                "run" => {
                    let cors = CorsLayer::new()
                        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                        .allow_credentials(true)
                        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
                
                    let app = create_router().layer(cors);
                
                    println!("🚀 Server started successfully");
                    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
                    axum::serve(listener, app).await.unwrap();
                },
                _ => println!("错误的参数"),
            }
        }
        None => println!("help 输出帮助信息"),
    }
}