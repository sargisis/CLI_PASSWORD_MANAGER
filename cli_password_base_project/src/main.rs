mod commands;
mod db;
mod models;


use std::env;
use commands::Commands;
use db::DataBase;

fn main() {
    // собираем аргументы от CLI
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage:");
        println!(" add <service> <login> <password>");
        println!(" get <service>");
        println!(" list");
        println!(" delete <service>");
        return;
    }

    // пытаемся распарсить команду
    let cmd = match Commands::parse_command(args) {
        Ok(c) => c,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let mut db = DataBase::new();

    db.execute(cmd);
}