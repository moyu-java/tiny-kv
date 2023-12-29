use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tiny_kv::Result;

#[derive(Parser)]
#[command(author, version, about = "A tiny redis client.", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1:6379")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut stream = TcpStream::connect(cli.addr).await.expect("服务器连接失败");
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // 发送用户输入的命令给服务端
                stream.write_all(line.as_bytes()).await?;

                // 读取并打印服务端的响应
                let mut buffer = [0; 512];
                let response = match stream.read(&mut buffer).await {
                    Ok(0) => {
                        // 服务端断开连接
                        println!("server disconnected.");
                        break;
                    }
                    Ok(data) => data,
                    Err(e) => {
                        // 数据读取错误
                        eprintln!("Failed: {:?}", e);
                        break;
                    }
                };
                let response = String::from_utf8_lossy(&buffer[..response]);
                println!("{}", response.to_string());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}


