use rand::Rng;
use tokio::io::Result;
use tokio::time::{sleep, Duration};

const MAX_DELAY: i64 = 3000; // in milliseconds

#[tokio::main]
async fn main() -> Result<()> {
    let mut handles = vec![];

    for tid in 0..10 {
        handles.push(tokio::spawn(async move {
            let delay_amount = {
                // calculate delay this way because I want them to be grouped in clusters
                let mut rng = rand::thread_rng();
                let delay = (rng.gen::<i64>() % (MAX_DELAY / 1000)) * 1000;
                let jitter = (rng.gen::<u64>() % 1000) as i64 - 500;
                let delay = delay + jitter;

                (if delay > 0 { delay } else { 0 }) as u64
            };

            println!("Thread {tid}: starting...");
            sleep(Duration::from_millis(delay_amount)).await;
            println!("Thread {tid}: delayed for {delay_amount}ms!!");
        }));
    }

    // keep main thread alive until the spawned threads execute
    for handle in handles {
        let _ = handle.await.unwrap();
    }

    println!("Hello, world!");
    Ok(())
}
