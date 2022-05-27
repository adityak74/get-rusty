use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Cannot compute GCD for 0");
    }

    let response = 
        format!("The GCD of numbers {} and {} \
        is <br>{}</br>", 
        form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    web::Json(json!({ "message": format!("Hello {}!", name) }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
            .service(greet)
    });

    println!("Serving on localhost:3000....");
    server
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

async fn get_index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Calc GCD</button>
                </form>
            "#,
        )
}
