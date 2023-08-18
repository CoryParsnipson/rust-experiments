use handlebars::{handlebars_helper, Handlebars};
use rand::Rng;
use serde_json::json;
use std::cmp::max;
use std::io::{self, Write};
use substring::Substring;
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

    let mut reg = Handlebars::new();

    handlebars_helper!(sucks: |topic: String| {
        let sucks_literals = ["sucks", "SUCKS", "SUCKS!", "SUCKS!!"];
        let idx = {
            let mut rng = rand::thread_rng();
            rng.gen::<usize>() % sucks_literals.len()
        };

        format!("{} {}", topic, sucks_literals[idx])
    });
    reg.register_helper("sucks", Box::new(sucks));
    reg.register_template_string("sucks", "{{sucks topic}}")
        .unwrap();

    reg.register_template_string("bad1", "Man, I hate {{topic}}")
        .unwrap();
    reg.register_template_string("bad2", "{{topic}} is for BABIES!!")
        .unwrap();
    reg.register_template_string("bad3", "I don't like {{topic}}")
        .unwrap();
    reg.register_template_string("bad4", "{{topic}} is mid")
        .unwrap();

    reg.register_template_string("good1", "Actually, I really like {{topic}}")
        .unwrap();
    reg.register_template_string("good2", "What are you talking about? {{topic}} rules!")
        .unwrap();
    reg.register_template_string("good3", "{{topic}} is okay.")
        .unwrap();
    reg.register_template_string("good4", "{{topic}} isn't bad.")
        .unwrap();
    reg.register_template_string("good5", "Yeah, what is he talking about?")
        .unwrap();

    for tid in 0..num_responders {
        let reg = reg.clone();
        let has_sucks_in_it = topic.to_lowercase().contains("suck");
        let topic = if has_sucks_in_it {
            let start_pos = topic.to_lowercase().find("suck").unwrap();
            let mut end_pos = start_pos;

            loop {
                if let Some(c) = topic.chars().nth(end_pos) {
                    if !c.is_whitespace() {
                        end_pos += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            topic.substring(0, start_pos).to_owned() + topic.substring(end_pos, topic.len())
        } else {
            topic.to_string()
        }
        .trim()
        .to_string();

        if has_sucks_in_it && tid == 0 {
            println!("{}", reg.render("good1", &json!({"topic": topic})).unwrap());
        } else {
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

                let response_types = if has_sucks_in_it {
                    vec!["good2", "good3", "good4", "good5"]
                } else {
                    vec![
                        "sucks", "sucks", "sucks", "sucks", "bad1", "bad2", "bad3", "bad4",
                    ]
                };
                let idx = {
                    let mut rng = rand::thread_rng();
                    rng.gen::<usize>() % response_types.len()
                };

                println!(
                    "{}",
                    reg.render(response_types[idx], &json!({"topic": topic}))
                        .unwrap()
                );
            }));
        }
    }

    for handle in handles {
        let _ = handle.await.unwrap();
    }

    // put in whitespace at the very end
    println!("");
}
