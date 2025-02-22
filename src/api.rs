use actix_web::{App, HttpResponse, HttpServer, Responder, Result, delete, get, post, web};
use serde::{Deserialize, Serialize};
use std::env;

use crate::util;

#[derive(Deserialize, Serialize)]
struct ShortenUrl {
    shorten_id: Option<String>,
    original_url: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct ShortenUrlVec {
    result: Vec<ShortenUrl>,
}

#[derive(Deserialize, Serialize)]
struct StringVec {
    result: Vec<String>,
}

#[get("/search")]
async fn search_url(query: web::Query<ShortenUrl>) -> impl Responder {
    // get arguments
    let ShortenUrl {
        shorten_id,
        original_url,
    } = query.into_inner();

    // got shorten id, return original url
    if let Some(shorten_id) = shorten_id {
        // get repository
        let repo = match util::get_repo().await {
            Ok(repo) => repo,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error connectiong to database: {:?}", e));
            }
        };
        // find and return data
        return match repo.find_by_shorten_id(&shorten_id).await {
            Ok(model) => match model {
                Some(model) => HttpResponse::Ok().body(model.original_url),
                None => {
                    HttpResponse::NotFound().body(format!("Shorten ID {:?} not found", shorten_id))
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
        };
    }

    // get original url, return shorten id
    if let Some(original_url) = original_url {
        // TODO: Implement search logic
        // get repository
        let repo = match util::get_repo().await {
            Ok(repo) => repo,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Error connectiong to database: {:?}", e));
            }
        };
        // find and return data
        return match repo.find_all_by_original_url(&original_url).await {
            Ok(model_vec) => HttpResponse::Ok().json(StringVec {
                result: {
                    // make it a original_url string vector
                    model_vec
                        .iter()
                        .map(|model| model.shorten_id.clone())
                        .collect()
                },
            }),
            Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {:?}", e)),
        };
    }

    // did not provide any arguments
    HttpResponse::BadRequest().body("Both shorten_id and original_url are missing")
}

#[post("/shorten")]
async fn create_shorten_url(body: web::Json<ShortenUrl>) -> Result<String> {
    // get arguments
    let ShortenUrl {
        shorten_id,
        original_url,
    } = body.into_inner();

    // check input (required fields)
    if original_url.is_none() {
        return Err(actix_web::error::ErrorBadRequest(
            "Original URL is required",
        ));
    }

    // unwarp fields
    let shorten_id = match shorten_id {
        Some(shorten_id) => shorten_id,
        None => util::gen_random_id(6), // create one if not provided
                                        // TODO: regenerate if shorten_id already exists
    };
    let original_url = original_url.unwrap();

    // get repository
    let repo = match util::get_repo().await {
        Ok(repo) => repo,
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Error connecting to database: {:?}",
                e
            )));
        }
    };

    // create and insert data
    match repo.create(&shorten_id, &original_url).await {
        Ok(_) => (),
        Err(e) => {
            if e.to_string().contains("Shorten ID already exists") {
                return Err(actix_web::error::ErrorBadRequest(format!(
                    "Shorten ID {:?} already exists",
                    shorten_id
                )));
            }
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Database error: {:?}",
                e
            )));
        }
    };

    // return response
    let response = format!("Shortened {:?} to {:?}", original_url, shorten_id);
    Ok(response)
}

#[delete("/shorten")]
async fn delete_shorten_url(body: web::Json<ShortenUrl>) -> Result<String> {
    // get arguments
    let ShortenUrl {
        shorten_id,
        original_url,
    } = body.into_inner();

    // check input (can only provide one of {shorten id, original url})
    if shorten_id.is_none() && original_url.is_none() {
        // both not provided
        return Err(actix_web::error::ErrorBadRequest(
            "Either shorten_id or original_url is required",
        ));
    } else if shorten_id.is_none() == false && original_url.is_none() == false {
        // both provided
        return Err(actix_web::error::ErrorBadRequest(
            "Cannot provide both shorten_id and original_url",
        ));
    }

    // get repository
    let repo = match util::get_repo().await {
        Ok(repo) => repo,
        Err(e) => {
            return Err(actix_web::error::ErrorInternalServerError(format!(
                "Error connecting to database: {:?}",
                e
            )));
        }
    };

    // delete from database
    if let Some(shorten_id) = shorten_id {
        // delete by shorten id
        repo.delete_by_shorten_id(&shorten_id).await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database error: {:?}", e))
        })?;
        return Ok(format!("Deleted shorten URL {:?}", shorten_id));
    } else if let Some(original_url) = original_url {
        // delete by original url
        repo.delete_by_original_url(&original_url)
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Database error: {:?}", e))
            })?;
        return Ok(format!("Deleted original URL {:?}", original_url));
    }

    panic!("Should not reach here")
}

#[get("/list")]
async fn list_shorten_url() -> impl Responder {
    // get repo
    let repo = match util::get_repo().await {
        Ok(repo) => repo,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error connecting to database: {:?}", e));
        }
    };

    // get all shorten URLs from database
    let model_vec = match repo.get_all().await {
        Ok(model_vec) => model_vec,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {:?}", e));
        }
    };

    // Return the list of shortened URLs as a JSON response
    HttpResponse::Ok().json(ShortenUrlVec {
        result: model_vec
            .iter()
            .map(|model| ShortenUrl {
                shorten_id: Some(model.shorten_id.clone()),
                original_url: Some(model.original_url.clone()),
            })
            .collect(),
    })
}

#[get("/{shorten_id}")]
async fn redirect_to_original(path: web::Path<(String,)>) -> impl Responder {
    // get arguments
    let (shorten_id,) = path.into_inner();

    // get repo
    let repo = match util::get_repo().await {
        Ok(repo) => repo,
        Err(_) => return web::Redirect::to(format!("/search?shorten_id={}", shorten_id)), // error connecting to database
    };

    // get data from database
    let model = match repo.find_by_shorten_id(&shorten_id).await {
        Ok(model_ops) => match model_ops {
            Some(model) => model,
            None => return web::Redirect::to(format!("/search?shorten_id={}", shorten_id)), // not found
        },
        Err(_) => return web::Redirect::to(format!("/search?shorten_id={}", shorten_id)), // database error
    };

    // redirect to original URL
    web::Redirect::to(model.original_url).permanent()
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let host: String = env::var("HOST").unwrap_or("localhost".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .service(search_url)
            .service(create_shorten_url)
            .service(delete_shorten_url)
            .service(list_shorten_url)
            .service(redirect_to_original)
    })
    .bind((host, port))?
    .run()
    .await
}
