use lazy_static::lazy_static;

use prometheus::{IntCounter, IntGauge, Registry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::spawn;
use tokio::time;
use warp::{Filter, Rejection, Reply};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
}

#[derive(Serialize, Deserialize, Debug)]
struct ServiceMetadata {
    cluster_name: String,
    cloud: String,
    cloud_region: String
}

async fn ping_server(endpoint: &str) -> Result<(), reqwest::Error> {
    use std::time::Instant;
    let now = Instant::now();

    let response_result = reqwest::get(endpoint).await;
    let duration = now.elapsed();

    match response_result {
        Ok(response) => {
            if response.status() == 200 {
                let remote_service_metadata: ServiceMetadata = response.json().await?;
                println!("http call to {} took {}", remote_service_metadata.cluster_name, duration.as_millis());

                /*
                let metric = &metrics[&endpoint.name];

                metric.set(duration.as_millis() as i64);
                */

            } else {
                println!("http call failed with status code {}", response.status())
            }
            Ok(())
        }
        Err(error) => {
            /*
            let error_metric = &error_metrics[&endpoint.name];
            error_metric.inc();
            */

            println!("http call failed with error {}", error);

            Err(error)
        }
    }
}

async fn ping_loop(endpoint: String /*, metrics: HashMap<String, IntGauge>, error_metrics: HashMap<String, IntCounter>*/) {
    spawn(async move {
        let mut interval = time::interval(Duration::from_millis(100));
        loop {
            interval.tick().await;

            ping_server(&endpoint).await;
        }
    });
}

/*
fn parse_endpoints() -> Vec<Endpoint> {
    let endpoint_names_string = env::var("ENDPOINT_NAMES").unwrap_or("".to_string()).to_string();
    let endpoint_urls_string = env::var("ENDPOINT_URLS").unwrap_or("".to_string()).to_string();

    let names: Vec<String> = endpoint_names_string.split(",").map(String::from).collect();
    let urls: Vec<String> = endpoint_urls_string.split(",").map(String::from).collect();

    if names.len() != urls.len() {
        panic!("ENDPOINT_NAMES not same length as ENDPOINT_URLS");
    }

    let mut endpoints: Vec<Endpoint> = vec![];
    for (index, _) in names.iter().enumerate() {
        let name = names[index].clone();
        let url = urls[index].clone();

        if name.len() == 0  || url.len() == 0 {
            continue;
        }

        endpoints.push(
            Endpoint {
                name,
                url
            }
        );
    }

    endpoints

}

fn create_metrics(my_name: &str, endpoints: &Vec<Endpoint>) -> (HashMap<String, IntGauge>, HashMap<String, IntCounter>) {
    let mut endpoint_metrics: HashMap<String, IntGauge> = HashMap::new();
    let mut endpoint_error_metrics: HashMap<String, IntCounter> = HashMap::new();

    for endpoint in endpoints {
        let gauge_name = format!("{}_{}_latency", my_name, endpoint.name);
        let help_message = format!("Latency (ms) between {} and {}", my_name, endpoint.name);
        let gauge = IntGauge::new(gauge_name.clone(), help_message).expect("metric can't be created");

        println!("creating metric {} for endpoint {}", gauge_name, endpoint.name);

        endpoint_metrics.insert(endpoint.name.clone(), gauge.clone());
        REGISTRY.register(Box::new(gauge.clone())).expect("gauge failed to be registered");

        let error_counter_name = format!("{}_{}_errors", my_name, endpoint.name);
        let error_help_message = format!("Errors between {} and {}", my_name, endpoint.name);
        let counter = IntCounter::new(error_counter_name, error_help_message).expect("metric can't be created");

        endpoint_error_metrics.insert(endpoint.name.clone(), counter.clone());
        REGISTRY.register(Box::new(counter.clone())).expect("counter failed to be registered");
    }

    (endpoint_metrics, endpoint_error_metrics)
}
*/

async fn metrics_handler() -> Result<impl Reply, Rejection> {
    use prometheus::Encoder;
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
    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_custom);
    Ok(res)
}



#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cluster_name = env::var("CLUSTER_NAME").unwrap_or("UNKNOWN".to_string());
    let cloud = env::var("CLOUD").unwrap_or("UNKNOWN".to_string());
    let cloud_region = env::var("CLOUD_REGION").unwrap_or("UNKNOWN".to_string());
    let ring = env::var("RING").unwrap_or("UNKNOWN".to_string());

    let metadata = ServiceMetadata {
        cluster_name,
        cloud,
        cloud_region
    };

    let metadata_response = serde_json::to_string(&metadata).unwrap();

    let endpoint = env::var("ENDPOINT").unwrap_or("https://cluster-agent.nublada.cloud/healthz".to_string());
    let port_string = env::var("PORT").unwrap_or("8080".to_string());
    let port = port_string.parse::<u16>().unwrap();

    // let (metrics, error_metrics) = create_metrics(&my_name, &endpoints);

    ping_loop(endpoint).await;

    // GET /healthz => 200 OK with body "OK"
    let healthz_route = warp::path!("healthz")
        .map(move || metadata_response.clone());

    let metrics_route = warp::path!("metrics").and_then(metrics_handler);
    let api = metrics_route.or(healthz_route);
    let routes = api.with(warp::log("cluster-agent"));

    println!("{}", port);

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}
