use handlebars::{handlebars_helper, Handlebars, RenderError};
use rand::Rng;
use serde_json::json;
use std::io::{self, Write};

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

        smol::block_on(async move {
            let d = Dialogue::new();
            println!("{}", d.get_bad_response(&json!({"topic": input})));
        });
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
