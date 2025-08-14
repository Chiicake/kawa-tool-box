use std::{fs, thread, sync::Mutex};
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json;

// 配置结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub excel_path: String,
    pub target_path: String,
}

// 全局配置互斥锁
static CONFIG_LOCK: Mutex<()> = Mutex::new(());
const CONFIG_PATH: &str = "config.json";

// 读取配置
pub fn read_config() -> Config {
    let _lock = CONFIG_LOCK.lock().unwrap();
    match fs::read_to_string(CONFIG_PATH) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

// 保存配置
fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let _lock = CONFIG_LOCK.lock().unwrap();
    let content = serde_json::to_string_pretty(config)?;
    fs::write(CONFIG_PATH, content)?;
    Ok(())
}

impl Default for Config {
    fn default() -> Self {
        Config {
            excel_path: String::new(),
            target_path: String::new(),
        }
    }
}

// 解析HTTP请求方法和路径
fn parse_request(request: &str) -> (&str, &str, &str) {
    let mut lines = request.lines();
    let first_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    
    if parts.len() >= 3 {
        (parts[0], parts[1], parts[2])
    } else {
        ("", "", "")
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    let mut temp = [0; 1024];
    loop {
        let n = stream.read(&mut temp).unwrap();
        if n == 0 {
            break;
        }
        buffer.extend_from_slice(&temp[..n]);
        if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    let request = String::from_utf8_lossy(&buffer);

    // 解析请求方法和路径
    let (method, path, _) = parse_request(&request);

    // 处理不同端点
    let (status_line, content_type, response_body) = match (method, path) {
        ("GET", "/config") => {
            let contents = fs::read_to_string("./src/config.html").unwrap_or_else(|_| {
                "<h1>404 Not Found</h1>".to_string()
            });
            ("200 OK", "text/html", contents)
        }
        ("GET", "/get_config") => {
            let config = read_config();
            let json = serde_json::to_string(&config).unwrap_or_else(|e| {
                format!("{{\"error\": \"Failed to serialize config: {}\"}}", e)
            });
            ("200 OK", "application/json", json)
        }
        ("POST", "/save_config") => {
            // 检查Content-Type头
            let (headers_part, _) = request.split_once("\r\n\r\n").unwrap_or((&request, ""));
            let mut content_type_valid = false;
            
            for header in headers_part.lines().skip(1) {
                if header.starts_with("Content-Type:") {
                    let content_type = header.split(":").nth(1).map(|s| s.trim()).unwrap_or("");
                    if content_type.eq_ignore_ascii_case("application/json") {
                        content_type_valid = true;
                    }
                    break;
                }
            }
            
            if !content_type_valid {
                (
                    "415 Unsupported Media Type", 
                    "application/json", 
                    "{\"error\": \"Unsupported Media Type: expected application/json\"}".to_string()
                );
            }
            
            // 解析请求体中的JSON
            let separator_pos = request.find("\r\n\r\n");
            if separator_pos.is_none() {
                (
                    "400 Bad Request", 
                    "application/json", 
                    "{\"error\": \"Invalid request format: missing header separator\"}".to_string()
                );
            }
            let body_start = separator_pos.unwrap() + 4;
            let body = &request[body_start..];
            
            // 清除可能的空字符和BOM
            let body = body.trim_start_matches(|c: char| c.is_whitespace() || c == '\u{FEFF}');
            
            // 检查body是否为空
            if body.is_empty() {
                (
                    "400 Bad Request", 
                    "application/json", 
                    "{\"error\": \"Empty request body\"}".to_string()
                );
            }
            
            match serde_json::from_str::<Config>(body) {
                Ok(config) => {
                    if save_config(&config).is_ok() {
                        ("200 OK", "application/json", "{\"status\": \"success\"}".to_string())
                    } else {
                        ("500 Internal Server Error", "application/json", 
"{\"error\": \"Failed to save config\"}".to_string())
                    }
                }
                Err(e) => {
                    (
                        "400 Bad Request", 
                        "application/json", 
                        format!("{{\"error\": \"Invalid JSON: {}\"}}", e)
                    )
                }
            }
        }
        _ => {
            let contents = fs::read_to_string("./src/404.html").unwrap_or_else(|_| {
                "<h1>404 Not Found</h1>".to_string()
            });
            ("404 Not Found", "text/html", contents)
        }
    };

    // 构建响应
    let response = format!(
        "HTTP/1.1 {}
Content-Type: {}
Content-Length: {}

{}",
        status_line,
        content_type,
        response_body.len(),
        response_body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
