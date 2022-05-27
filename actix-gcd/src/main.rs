use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(get_index))
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
