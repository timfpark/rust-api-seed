use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::spawn;
use tokio::time;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("OK".into()))
}

async fn ping_server(endpoint: &str) {
    use std::time::Instant;
    let now = Instant::now();

    let _response_result = reqwest::get(endpoint).await;

    let duration = now.elapsed();
    println!("http call to {} took {}", endpoint, duration.as_millis())

}

async fn ping_loop() {
    let endpoints_string = env::var("ENDPOINTS").unwrap_or("http://localhost:8080/healthz".to_string()).to_string();
    let endpoints_split  = endpoints_string.split(",");
    let endpoints: Vec<String> = endpoints_split.map(String::from).collect();

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;

            for endpoint in endpoints.iter() {
                ping_server(endpoint).await;
            }
        }
    });
}

#[tokio::main]
async fn main() {
    let port_string = env::var("PORT").unwrap_or("8080".to_string());
    let port = port_string.parse::<u16>().unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    ping_loop().await;

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
