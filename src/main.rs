use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::env;

#[get("/img/{name}")]
async fn original_image_handler(info: web::Path<String>) -> impl Responder {
    let name = &info;
    let images_path = env::var("IMAGES_PATH").unwrap_or_else(|_| String::from("images"));

    let img = ImageReader::open(format!("{}/{}", images_path, name))
        .unwrap()
        .decode()
        .unwrap();

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(buf.into_inner())
}

#[get("/img/{name}/w_{width}/h_{height}")]
async fn resized_image_handler(info: web::Path<(String, u32, u32)>) -> impl Responder {
    let name = &info.0;
    let width = info.1;
    let height = info.2;
    let images_path = env::var("IMAGES_PATH").unwrap_or_else(|_| String::from("images"));

    let img = ImageReader::open(format!("{}/{}", images_path, name))
        .unwrap()
        .decode()
        .unwrap();

    if width > img.width() || height > img.height() {
        return HttpResponse::BadRequest().body("Requested dimensions exceed original image size");
    }

    let img = img.resize(width, height, image::imageops::FilterType::Triangle);

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(buf.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(original_image_handler).service(resized_image_handler))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}