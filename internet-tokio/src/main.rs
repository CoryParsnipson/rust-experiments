use rand::Rng;
use std::cmp::max;
use std::io::{self, Write};
use tokio::io::Result;
use tokio::runtime::Handle;
use tokio::time::{sleep, Duration};

const MAX_DELAY: i64 = 3000; // in milliseconds
const MIN_RESPONDERS: usize = 1;
const MAX_RESPONDERS: usize = 10;

#[tokio::main]
async fn main() -> Result<()> {
    let tokio_h = Handle::current();

    println!("Starting internet simulator...");
    loop {
        print!("Name a thing: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = String::from(input.trim());
        let handle = tokio_h.clone();

        let th = tokio_h.spawn(async move {
            let num_responders = {
                let mut rng = rand::thread_rng();
                let num = rng.gen::<usize>() % MAX_RESPONDERS;

                max(MIN_RESPONDERS, num)
            };

            response(&handle, input.as_str(), num_responders).await;
        });

        let _ = th.await;
    }
}

async fn response(handle: &Handle, topic: &str, num_responders: usize) {
    let mut handles = Vec::new();
    handles.reserve(num_responders);

    for _ in 0..num_responders {
        let topic = String::from(topic);

        handles.push(handle.spawn(async move {
            let delay_amount = {
                // calculate delay this way because I want them to be grouped in clusters
                let mut rng = rand::thread_rng();
                let delay = (rng.gen::<i64>() % (MAX_DELAY / 1000)) * 1000;
                let jitter = (rng.gen::<u64>() % 1000) as i64 - 500;
                let delay = delay + jitter;

                (if delay > 0 { delay } else { 0 }) as u64
            };

            sleep(Duration::from_millis(delay_amount)).await;
            println!("{} SUCKS!!", topic);
        }));
    }

    for handle in handles {
        let _ = handle.await.unwrap();
    }

    // put in whitespace at the very end
    println!("");
}
