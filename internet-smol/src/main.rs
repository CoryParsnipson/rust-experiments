use async_io::Timer;
use handlebars::{handlebars_helper, Handlebars, RenderError};
use rand::Rng;
use serde_json::json;
use smol::Task;
use std::cmp::max;
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;
use substring::Substring;

const MAX_DELAY: i64 = 3000; // in milliseconds
const MIN_RESPONDERS: usize = 3;
const MAX_RESPONDERS: usize = 10;

fn main() {
    println!("Starting internet simulator...");
    loop {
        print!("Name a thing: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = String::from(input.trim());

        let has_sucks_in_it = input.to_lowercase().contains("suck");
        let topic = if has_sucks_in_it {
            let start_pos = input.to_lowercase().find("suck").unwrap();
            let mut end_pos = start_pos;

            loop {
                if let Some(c) = input.chars().nth(end_pos) {
                    if !c.is_whitespace() {
                        end_pos += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            input.substring(0, start_pos).to_owned() + input.substring(end_pos, input.len())
        } else {
            String::from(input)
        };

        let num_responders = {
            let mut rng = rand::thread_rng();
            let num = rng.gen::<usize>() % MAX_RESPONDERS;

            max(MIN_RESPONDERS, num)
        };

        let d = Arc::new(Dialogue::new());
        let mut handles: Vec<Task<()>> = vec![];

        for tid in 0..num_responders {
            let topic = topic.clone();
            let d = d.clone();

            handles.push(smol::spawn(async move {
                let delay_amount = {
                    // calculate delay this way because I want them to be grouped in clusters
                    let mut rng = rand::thread_rng();
                    let delay = (rng.gen::<i64>() % (MAX_DELAY / 1000)) * 1000;
                    let jitter = (rng.gen::<u64>() % 1000) as i64 - 500;
                    let delay = delay + jitter;

                    (if delay > 0 { delay } else { 0 }) as u64
                };

                Timer::after(Duration::from_millis(delay_amount)).await;

                if has_sucks_in_it {
                    if tid == 0 {
                        println!("{}", d.render("good1", &json!({"topic": topic})).unwrap());
                    } else {
                        println!("{}", d.get_good_response(&json!({"topic": topic})));
                    }
                } else {
                    println!("{}", d.get_bad_response(&json!({"topic": topic})));
                }
            }));
        }

        for handle in handles {
            smol::block_on(handle);
        }

        println!(""); // add blank line after all responses
    }
}

pub struct Dialogue {
    data: Handlebars<'static>,
    good_templates: Vec<String>,
    bad_templates: Vec<String>,
}

impl Dialogue {
    pub fn new() -> Dialogue {
        let mut d = Dialogue {
            data: Handlebars::new(),
            good_templates: vec![],
            bad_templates: vec![],
        };

        handlebars_helper!(sucks: |topic: String| {
            let sucks_literals = ["sucks", "SUCKS", "SUCKS!", "SUCKS!!"];
            let idx = {
                let mut rng = rand::thread_rng();
                rng.gen::<usize>() % sucks_literals.len()
            };

            format!("{} {}", topic, sucks_literals[idx])
        });
        d.data.register_helper("sucks", Box::new(sucks));

        d.register_bad_responses();
        d.register_good_responses();

        d
    }

    pub fn render(&self, template: &str, data: &serde_json::Value) -> Result<String, RenderError> {
        self.data.render(template, data)
    }

    pub fn get_good_response(&self, data: &serde_json::Value) -> String {
        let idx = {
            let mut rng = rand::thread_rng();
            rng.gen::<usize>() % self.good_templates.len()
        };

        self.render(&self.good_templates[idx], data).unwrap()
    }

    pub fn get_bad_response(&self, data: &serde_json::Value) -> String {
        let idx = {
            let mut rng = rand::thread_rng();

            // make the "x SUCKS" response more likely
            if rng.gen::<usize>() % 10 > 7 {
                0
            } else {
                rng.gen::<usize>() % self.bad_templates.len()
            }
        };

        self.render(&self.bad_templates[idx], data).unwrap()
    }

    pub fn register_bad_responses(&mut self) -> &mut Self {
        let data = vec![
            ("bad1", "{{sucks topic}}"),
            ("bad2", "Man, I Hate {{topic}}"),
            ("bad3", "{{topic}} is for BABIES!!"),
            ("bad4", "I don't like {{topic}}"),
            ("bad5", "{{topic}} is mid"),
        ];

        for (id, template) in data {
            self.data.register_template_string(id, template).unwrap();
            self.bad_templates.push(String::from(id));
        }

        self
    }

    pub fn register_good_responses(&mut self) -> &mut Self {
        let data = vec![
            ("good1", "Actually, I really like {{topic}}"),
            ("good2", "What are you talking about? {{topic}} rules!"),
            ("good3", "{{topic}} is okay."),
            ("good4", "{{topic}} isn't bad."),
            ("good5", "Yeah, what is he talking about?"),
        ];

        for (id, template) in data {
            self.data.register_template_string(id, template).unwrap();
            self.good_templates.push(String::from(id));
        }

        self
    }
}
