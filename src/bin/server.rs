use std::{env, fs};
use clap::{Parser, ValueEnum};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tiny_kv::engine::KvEngine;
use tiny_kv::engine::sled::SledEngine;
use tiny_kv::{KvError, Result};

#[derive(Parser)]
#[command(author, version, about = "A tiny kv server.", long_about = None)]
struct Cli {
    /// Sets the listening address
    #[arg(short, long, default_value_t = 6379)]
    port: u16,

    // Sets the Key-Value Storage Engine
    // #[arg(short, long, value_enum, default_value = Engine.Sled)]
    // engine: Engine,
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// enum Engine {
//     /// In-Memory Key-Value Storage Engine
//     Mem,
//     /// Sled-based Key-Value Storage Engine
//     Sled,
// }

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let engine = SledEngine::new(sled::open(env::current_dir()?)?);
    // 监听本地端口，准备接收连接。
    let listener = TcpListener::bind(format!("127.0.0.1:{}", cli.port)).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        let client_addr = socket.peer_addr()?;
        println!("client connected: {}", client_addr);
        let engine = engine.clone();
        // 对于新的连接，启动一个新的异步任务
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                // 读取客户端数据
                let data = match socket.read(&mut buffer).await {
                    Ok(0) => {
                        // 客户端断开连接
                        println!("client disconnected: {}", client_addr);
                        return;
                    }
                    Ok(data) => data,
                    Err(e) => {
                        // 数据读取错误
                        eprintln!("Failed to read from socket: {:?}", e);
                        return;
                    }
                };

                // 解析命令。
                let command = String::from_utf8_lossy(&buffer[..data]);

                println!("{}: {}",client_addr, command.to_string());

                let parts: Vec<&str> = command.trim().split_whitespace().collect();
                let response = match parts.as_slice() {
                    ["set", key, value] => {
                        engine.set(key.to_string(), value.to_string()).unwrap();
                        "Ok".to_string()
                    }
                    ["get", key] => {
                        engine.get(key.to_string()).unwrap().unwrap()
                    }
                    ["remove", key] => {
                        engine.remove(key.to_string()).unwrap();
                        "Ok".to_string()
                    }
                    _ => String::from("unsupported command"),
                };

                // 发送响应。
                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    eprintln!("Failed to write to socket: {:?}", e);
                    return;
                }
            }
        });
    }
}