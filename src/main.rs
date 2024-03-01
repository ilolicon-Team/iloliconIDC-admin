mod check;

fn main() {
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
                "run" => run(),
                _ => println!("错误的参数"),
            }
        }
        None => println!("help 输出帮助信息"),
    }
}

fn run() {

}