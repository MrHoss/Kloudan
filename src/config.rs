use std::env;
use dotenv::dotenv;

pub struct Config{
    pub database_url:String,
    pub address:String,
}
pub fn config()->Config{
    dotenv().ok();
    let config:Config = Config{
        database_url: env::var("DATABASE_URL").expect("\x1b[31;1mDATABASE_URL \x1b[31mnot defined\x1b[0m"),
        address: env::var("ADDRESS").expect("\x1b[31;1mADDRESS \x1b[31mnot defined\x1b[0m")
    };
    return config;
}




pub fn print_logo(){
    println!("\x1b[34;1m _____     _____     _____     _____     _____     _____   ");
    println!("/\\    \\   /\\    \\   /\\    \\   /\\    \\   /\\    \\   /\\    \\  ");
    println!("\\ \\    \\  \\ \\    \\  \\ \\    \\  \\ \\____\\  \\ \\____\\  \\ \\    \\ ");
    println!(" \\ \\    \\  \\ \\    \\  \\ \\    \\  \\/____/   \\/____/   \\ \\    \\");
    println!("  \\ \\____\\  \\ \\____\\  \\ \\____\\           ___        \\ \\____\\");
    println!("   \\/____/   \\/____/   \\/____/          /\\__\\        \\/____/");
    println!("Server initialized                      \\/__/               Danmark\x1b[0m");
}