use axum::{routing::{get_service, get, post}, Router, extract::{Query, Multipart, DefaultBodyLimit}, response::{Html, Redirect}};
use tower_http::{services::{ServeDir, ServeFile}};
use std::net::UdpSocket;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use tokio::fs;
use tera::{Tera, Context};

#[tokio::main]
async fn main() {
    // 初始化 Tera 模板引擎
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("模板解析错误: {}", e);
            std::process::exit(1);
        }
    };

    let static_service = get_service(ServeDir::new(".").not_found_service(ServeFile::new("assets/not_found.html")));

    let app = Router::new()
        .route("/", get(list_dir_request))
        .route("/", post(upload_file))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .fallback_service(static_service)
        .with_state(tera);
    let my_ip = get_ip().unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9527").await.unwrap();
    println!("Copy url: http://{my_ip}:9527");
    axum::serve(listener, app).await.unwrap();

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathQuery{
    pub path: Option<String>
}

// 移除 Template derive，改用 Tera

#[derive(Serialize, Debug)]
pub struct FileItem {
    pub name: String,
    pub url: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub file_type: String,
    pub is_image: bool,
    pub file_extension: String,
}

pub async fn upload_file(
    Query(query): Query<PathQuery>,
    mut multipart: Multipart
)-> Redirect {
    let mut current_dir = env::current_dir().unwrap();
    let path = query.path.unwrap_or("".to_string());
    for dir in path.clone().split("/").collect::<Vec<&str>>(){
        current_dir.push(dir.to_string())
    }
    println!("saving dir path:{current_dir:?}");
    while let Ok(Some(field)) = multipart.next_field().await {
        //let data = field.bytes().await.unwrap();
        let filename = field.file_name().unwrap().to_string();
        let mut saving_path = PathBuf::from(current_dir.clone());
        saving_path.push(filename);
        if let Ok(write_bytes) = field.bytes().await {
            let bs_size = write_bytes.len();
            println!("获取上传文件大小：{bs_size}");
            if let Err(e) = fs::write(saving_path, write_bytes).await {
                println!("保存文件错误:{e:?}");
            }
            break;
        }
    }
    Redirect::to(format!("/?path={path}").as_str())
}

pub async fn list_dir_request(
    Query(query): Query<PathQuery>,
    axum::extract::State(tera): axum::extract::State<Tera>
) -> Result<Html<String>, axum::response::Response> {
    let current_dir = env::current_dir().unwrap();
    println!("current: {current_dir:?}");
    let path = query.path.unwrap_or("".to_string());
    let dir_path = PathBuf::from(format!("{}{path}", &current_dir.into_os_string().into_string().unwrap()));

    println!("path:{} base_path:{:?}", &path, &dir_path);
    
    let mut items: Vec<FileItem> = Vec::new();
    
    // 读取目录内容
    if let Ok(entries) = std::fs::read_dir(&dir_path) {
        for entry in entries {
            if let Ok(item) = entry {
                if let Ok(metadata) = std::fs::metadata(&item.path()) {
                    let file_name = item.file_name().to_os_string().into_string().unwrap();
                    let file_extension = get_file_extension(&file_name);
                    let is_image = is_image_file(&file_extension);
                    
                    if metadata.is_dir() {
                        let path_param = format!("{}/{}", path.clone(), file_name);
                        items.push(FileItem {
                            name: file_name,
                            url: format!("/?path={}", path_param),
                            is_dir: true,
                            is_file: false,
                            file_type: "folder".to_string(),
                            is_image: false,
                            file_extension: file_extension,
                        });
                    } else {
                        let down_path = format!("{path}/{file_name}");
                        items.push(FileItem {
                            name: file_name,
                            url: down_path,
                            is_dir: false,
                            is_file: true,
                            file_type: if is_image { "image".to_string() } else { "file".to_string() },
                            is_image,
                            file_extension,
                        });
                    }
                }
            }
        }
    }
    
    // 按类型和名称排序：文件夹在前，然后按名称排序
    items.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    
    // 计算父目录路径
    let parent_path = if !path.is_empty() {
        let path_seq: Vec<&str> = path.split("/").collect();
        if path_seq.len() > 1 {
            path_seq[0..path_seq.len()-1].join("/")
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };
    
    let current_dir_display = if path.is_empty() { "/".to_string() } else { path.clone() };
    
    // 创建 Tera 上下文
    let mut context = Context::new();
    context.insert("current_dir", &current_dir_display);
    context.insert("current_path", &path);
    context.insert("parent_path", &parent_path);
    context.insert("items", &items);
    
    match tera.render("directory.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(e) => {
            eprintln!("模板渲染错误: {}", e);
            Err(axum::response::Response::builder()
                .status(500)
                .body("Internal Server Error".into())
                .unwrap())
        }
    }
}

fn get_file_extension(filename: &str) -> String {
    if let Some(dot_pos) = filename.rfind('.') {
        filename[dot_pos + 1..].to_lowercase()
    } else {
        String::new()
    }
}

fn is_image_file(extension: &str) -> bool {
    matches!(extension, "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" | "tiff" | "tif")
}

pub fn get_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };
    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };
    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}