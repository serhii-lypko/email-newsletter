use actix_http::StatusCode;
use actix_web::{dev, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

// A type implements the Responder trait if it can be converted into a HttpResponse -
// it is implemented off the shelf for a variety of common types (e.g. strings, status codes,
// bytes, HttpResponse, etc.) and we can roll our own implementations if needed.
async fn test(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("msg").unwrap_or("World");
    println!("{}", name);
    format!("OK")
}

async fn json_test(_req: HttpRequest) -> Result<HttpResponse> {
    #[derive(Serialize)]
    struct Sample {
        foo: String,
    }

    let sample = Sample {
        foo: "bar".to_string(),
    };

    Ok(HttpResponse::build(StatusCode::OK).json(sample))
}

// actix-web accepts a broad range of signatures as request handlers
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<dev::Server, std::io::Error> {
    // HttpServer handles all transport level concerns
    let server = HttpServer::new(|| {
        // App iterates over all registered endpoints until it finds a matching one (both path
        // template and guards are satisfied) and passes over the request object to the handler.
        App::new()
            .route("/test/{msg}", web::get().to(test))
            .route("/json-test", web::get().to(json_test))
            .route("/health-check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}

/*
    -- Embeded tests --

    An embedded test module has privileged access to the code living next to it: it can interact with
    structs, methods, fields and functions that have not been marked as public and would normally not
    be available to a user of our code if they were to import it as a dependency of their own project.

    It is good for unit testing.
*/
#[cfg(test)]
mod tests {}
