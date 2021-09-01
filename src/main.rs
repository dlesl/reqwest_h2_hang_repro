use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use reqwest::Client;

const WORKERS: usize = 50;
const URL: &'static str = "https://localhost:8443/hello";
const TIMEOUT: Option<u64> = None;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_ = stop.clone();
    ctrlc::set_handler(move || stop_.store(true, Ordering::Relaxed)).unwrap();
    let mut builder = Client::builder().danger_accept_invalid_certs(true);
    if let Some(timeout) = TIMEOUT {
        builder = builder.timeout(Duration::from_secs(timeout));
    }
    let client = builder.build().unwrap();
    let start = Instant::now();
    let workers: Vec<_> = (1..=WORKERS)
        .map(|n| {
            let join_handle =
                tokio::runtime::Handle::current().spawn(worker(n, client.clone(), stop.clone()));
            (n, join_handle)
        })
        .collect();
    let mut count = 0;
    for (n, join_handle) in workers {
        println!("worker {} stopped!", n);
        count += join_handle.await.unwrap();
    }
    let elapsed = start.elapsed().as_secs();
    println!(
        "{} succesful reqs in {} sec, {} req/s",
        count,
        elapsed,
        count / elapsed
    );
    Ok(())
}

async fn worker(n: usize, client: Client, stop: Arc<AtomicBool>) -> u64 {
    let mut count = 0;
    while !stop.load(Ordering::Relaxed) {
        if let Err(e) = request(&client).await {
            println!("worker {}: {:?}", n, e);
        } else {
            count += 1;
        }
    }
    count
}

async fn request(client: &Client) -> reqwest::Result<()> {
    let _ = client.get(URL).send().await?.bytes().await?;
    Ok(())
}
