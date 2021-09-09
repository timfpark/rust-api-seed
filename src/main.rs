use axum::{
    handler::{get},
    Router,
};
use lazy_static::lazy_static;
use prometheus::{Encoder, IntCounter, Registry};
use std::env;
use std::net::SocketAddr;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref WELCOMES_SERVED: IntCounter = IntCounter::new("welcomes_served", "Number of Welcomes served").expect("welcomes_served can't be created");
    pub static ref WELCOME: String = env::var("WELCOME").unwrap_or("Welcome Visitor!".to_string());
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let port_string = env::var("PORT").unwrap_or("8080".to_string());
    let port = port_string.parse::<u16>().unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/metrics", get(metrics));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    REGISTRY.register(Box::new(WELCOMES_SERVED.clone())).expect("welcomes_served failed to be registered");

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn metrics() -> String {
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };

    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };

    let res_prom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_prom);

    res
}

async fn root() -> &'static str {
    WELCOMES_SERVED.inc();

    &WELCOME
}
