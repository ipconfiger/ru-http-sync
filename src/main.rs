use axum::{routing::{get_service, get, post}, Router, extract::{Query, Multipart}, response::{Html, Redirect}};
use tower_http::{services::{ServeDir, ServeFile}};
use std::net::UdpSocket;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use futures_util::stream::StreamExt;
use tokio::fs;

#[tokio::main]
async fn main() {
    let static_service = get_service(ServeDir::new(".").not_found_service(ServeFile::new("assets/not_found.html")));

    let app = Router::new()
        .route("/", get(list_dir_request))
        .route("/", post(upload_file))
        .fallback_service(static_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PathQuery{
    pub path: Option<String>
}

pub async fn upload_file(
    Query(mut query): Query<PathQuery>,
    mut multipart: Multipart
)-> Redirect {
    let mut current_dir = env::current_dir().unwrap();
    let path = query.path.unwrap_or("".to_string());
    for dir in path.clone().split("/").collect::<Vec<&str>>(){
        current_dir.push(dir.to_string())
    }
    println!("saving dir path:{current_dir:?}");
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        //let data = field.bytes().await.unwrap();
        let filename = field.file_name().unwrap().to_string();
        let mut saving_path = PathBuf::from(current_dir.clone());
        saving_path.push(filename);
        if let Err(e) = fs::write(saving_path, field.bytes().await.unwrap()).await {
            println!("保存文件错误:{e:?}");
        }

    }
    Redirect::to(format!("/?path={path}").as_str())
}

pub async fn list_dir_request(
    Query(mut query): Query<PathQuery>
) -> Html<String> {
    let current_dir = env::current_dir().unwrap();
    println!("current: {current_dir:?}");
    let path = query.path.unwrap_or("".to_string());
    let dir_path = PathBuf::from(format!("{}{path}", &current_dir.into_os_string().into_string().unwrap()));

    println!("path:{} base_path:{:?}", &path, &dir_path);
    let mut html_buf:Vec<String> = Vec::new();
    html_buf.push("<ul>".to_string());
    if !path.is_empty(){
        let path_seq = path.split("/").collect::<Vec::<&str>>();
        let parrant = path_seq[0..path_seq.len()-1].join("/");
        html_buf.push(format!("<li> <a href=\"/?path={}\">../</a> </li>", parrant));
    }
    for entry in std::fs::read_dir(dir_path.clone()).unwrap(){
        if let Ok(item) = entry{
            if let Ok(metadata) = std::fs::metadata(&item.path()){
                let file_name = item.file_name().to_os_string().into_string().unwrap();
                if metadata.is_dir() {
                    let path_param = format!("{}/{}", path.clone(), file_name);
                    html_buf.push(format!("<li><a href=\"/?path={}\">{}</a></li>", path_param, file_name));
                }else{
                    let file_name = item.file_name().to_os_string().into_string().unwrap();
                    let down_path = format!("{path}/{file_name}");
                    println!("down path:{}", down_path);
                    html_buf.push(format!("<li><a href=\"{down_path}\" target=\"__blank\">{file_name}</></li>"));
                }
            }

        }
    }
    html_buf.push("</ul>".to_string());
    let form_html = format!(r#"
        <form action="/?path={}" method="post" enctype="multipart/form-data">
            <label>
                上传文件：
                <input type="file" name="axum_rs_file">
            </label>
            <button type="submit">上传文件</button>
        </form>
        <hr/>
    "#, path.clone());
    let c_dir = if path.is_empty() { "/".to_string() } else { path };

    let resp = format!("{form_html}\n<h1>当前目录：{c_dir}</h1>\n{}", html_buf.join(""));
    Html(resp)
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