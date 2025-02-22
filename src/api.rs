use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

use crate::repository;
use crate::util;

#[derive(Deserialize)]
struct ShortenUrl {
    shorten_id: Option<String>,
    original_url: Option<String>,
}

#[get("/search")]
async fn search_url(query: web::Query<ShortenUrl>) -> impl Responder {
    // get arguments
    let ShortenUrl { shorten_id, original_url } = query.into_inner();

    // got shorten id, return original url
    if let Some(shorten_id) = shorten_id {
        // TODO: Implement search logic
        return HttpResponse::Ok().body(format!("Get shorten ID: {:?}, return original URL", shorten_id));
    }

    // get original url, return shorten id
    if let Some(original_url) = original_url {
        // TODO: Implement search logic
        return HttpResponse::Ok().body(format!("Get original URL: {:?}, return shorten ID", original_url));
    }

    // did not provide any arguments
    HttpResponse::BadRequest().body("Both shorten_id and original_url are missing")
}

#[post("/shorten")]
async fn create_shorten_url(body: web::Json<ShortenUrl>) -> Result<String> {
    // get arguments
    let ShortenUrl { shorten_id, original_url } = body.into_inner();

    // check input (required fields)
    if original_url.is_none() {
        return Err(actix_web::error::ErrorBadRequest("Original URL is required"));
    }

    // unwarp fields
    let shorten_id = match shorten_id {
        Some(shorten_id) => shorten_id,
        None => util::gen_random_id(6), // create one if not provided
    };
    let original_url = original_url.unwrap();

    // TODO: add to database

    // return response
    let response = format!("Shortened {:?} to {:?}", original_url, shorten_id);
    Ok(response)
}

#[delete("/shorten")]
async fn delete_shorten_url(body: web::Json<ShortenUrl>) -> Result<String> {
    // get arguments
    let ShortenUrl { shorten_id, original_url } = body.into_inner();

    // check input (can only provide one of {shorten id, original url})
    if shorten_id.is_none() && original_url.is_none() { // both not provided
        return Err(actix_web::error::ErrorBadRequest("Either shorten_id or original_url is required"));
    } else if shorten_id.is_none() == false && original_url.is_none() == false { // both provided
        return Err(actix_web::error::ErrorBadRequest("Cannot provide both shorten_id and original_url"));
    }

    // TODO: delete from database
    if let Some(shorten_id) = shorten_id {
        // TODO: Implement delete logic for shorten id
        return Ok(format!("Delete shorten URL {:?}", shorten_id));
    } else if let Some(original_url) = original_url {
        // TODO: Implement delete logic for original url
        return Ok(format!("Delete original URL {:?}", original_url));
    }

    // return response
    panic!("should not reach here")
}

#[get("/list")]
async fn list_shorten_url() -> impl Responder {
    // TODO: get all shorten URLs from database

    // TODO: return response
    HttpResponse::Ok().body("List Shorten URL")
}

#[get("/{shorten_id}")]
async fn redirect_to_original(path: web::Path<(String,)>) -> impl Responder {
    // get arguments
    let (shorten_id,) = path.into_inner();

    // TODO: get data from database

    // TODO: redirect to original URL
    web::Redirect::to("https://google.com").permanent()
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(search_url)
            .service(create_shorten_url)
            .service(delete_shorten_url)
            .service(list_shorten_url)
            .service(redirect_to_original)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
