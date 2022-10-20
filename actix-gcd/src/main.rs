use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// Placing a #[derive(Deserialize)] attribute above a type definition tells the serde crate to
// examine the type when the program is compiled and automatically generate code to parse a value
// of this type from data in the format that HTML forms use for POST requests. In fact, that attribute
// is sufficient to let you parse a GcdParameters value from almost any sort of structured data:
//JSON, YAML, TOML, or any one of a number of other textual and binary formats. The serde crate also
// provides a Serialize attribute that generates code to do the reverse, taking Rust values and
// writing them out in a structured format.
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");

    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action = "/gcd" method="post">
                <input type = "text" name="n"/>
                <input type = "text" name="m"/>
                <button type = "submit">Compute GCD</button>
                </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} \
                                    is <b>{}</b>\n", form.n, form.m, gcd(form.m, form.n));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}


fn gcd(mut n:u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m=n;
            n=t;
        }
        m=m%n;
    }
    n
}
