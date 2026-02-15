use std::env;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_url: String,
    pub app_port: u16,
    pub cpu_count: usize,
    pub database_url: String,
    pub app_secret: String,
    pub ollama_url: String,
}

pub static ENV: LazyLock<Config> = LazyLock::new(|| {
    let port = env::var("APP_PORT").unwrap_or("8080".to_string());
    let cpu_count = env::var("CPU_COUNT").unwrap_or(num_cpus::get().to_string());

    let app_url = env::var("APP_URL").unwrap_or("http://localhost".to_string());
    let app_port = port.parse::<u16>().unwrap_or_else(|_| {
        eprintln!("ERROR: APP_PORT must be a valid u16 number");
        std::process::exit(1);
    });
    let cpu_count = cpu_count.parse::<usize>().unwrap_or_else(|_| {
        eprintln!("ERROR: CPU_COUNT must be a valid number");
        std::process::exit(1);
    });
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("ERROR: DATABASE_URL must be set");
        std::process::exit(1);
    });
    let app_secret = env::var("APP_SECRET").unwrap_or_else(|_| {
        eprintln!("ERROR: APP_SECRET must be set");
        std::process::exit(1);
    });
    let ollama_url = env::var("OLLAMA_URL").unwrap_or("http://localhost:11434".to_string());

    Config {
        app_url,
        app_port,
        cpu_count,
        database_url,
        app_secret,
        ollama_url,
    }
});
