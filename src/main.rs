use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

async fn home() -> impl Responder {
    HttpResponse::Ok().body(
        "<h3>恭喜你，部署 Consul 服务成功！</h3><br><br><br><br>记得给笔者 痴者工良 点一下赞哟~~~",
    )
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut contents = String::new();

    let mut ip = File::open("option.txt")?;

    let size = ip.read_to_string(&mut contents)?;

    println!("欢迎使用 痴者工良 编写的 demo 程序，程序使用 Rust 语言，基于 actix_web 框架开发。");
    println!("读取配置文件成功，文件字节大小 {} bytes，查询到的配置信息为", size);
    println!("{}",contents);
    println!("开始启动 Web 服务！请使用浏览器打开网页测试!");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(contents.trim())?
    .run()
    .await
}
