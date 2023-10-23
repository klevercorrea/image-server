use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::io::Cursor;

#[get("/img/{name}/w_{width}/h_{height}")]
async fn image_handler(info: web::Path<(String, Option<u32>, Option<u32>)>) -> impl Responder {
    let name = &info.0;
    let width = info.1.unwrap_or(0);
    let height = info.2.unwrap_or(0);

    let img = ImageReader::open(format!("images/{}", name))
        .unwrap()
        .decode()
        .unwrap();

    let (img_width, img_height) = img.dimensions();

    let width = if width == 0 {
        img_width
    } else if width < 200 || width > img_width {
        img_width
    } else {
        width
    };

    let height = if height == 0 {
        img_height
    } else if height < 200 || height > img_height {
        img_height
    } else {
        height
    };

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
    HttpServer::new(|| App::new().service(image_handler))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}