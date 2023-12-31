use std::time::Duration;

use colored::Colorize;
use log;
use question::{Answer, Question};
use reqwest::blocking::{Client, Response};
use serde_json::json;
use spinners::{Spinner, Spinners};

use crate::ai::completions::prompts;
use crate::cli::utils;
use crate::Config;

pub struct GptyCompletions {
    pub system_prompt: prompts::Prompt,
    pub config: Config,
}

impl GptyCompletions {
    pub fn new(config: Config) -> Self {
        Self {
            system_prompt: prompts::Prompt::new(),
            config,
        }
    }

    pub fn refine_loop(
        self: &Self,
        prompt: prompts::Prompt,
        mut should_refine: bool,
        spinner: &mut Spinner,
    ) -> String {
        let mut refined_prompts = vec![prompt.clone()];
        let mut response_prompt = self.call(refined_prompts.clone(), spinner);
        spinner.stop_and_persist(
            "✔".green().to_string().as_str(),
            "Got some results!".green().to_string(),
        );
        while should_refine {
            crate::pprint(&response_prompt.content, "bash");
            should_refine =
                utils::ask_for_confirmation(">> Refine the prompt? [y/N]", Some(Answer::NO));
            refined_prompts.push(response_prompt.clone());
            if should_refine {
                let user_refinement = Question::new(">> Enter your refinement: ").ask();
                let mut spinner = Spinner::new(Spinners::BouncingBar, "Refining...".into());
                let new_prompt = match user_refinement.unwrap() {
                    Answer::RESPONSE(value) => prompts::Prompt::build("user".to_string(), value),
                    _ => {
                        println!("Invalid response");
                        std::process::exit(1);
                    }
                };
                refined_prompts.push(new_prompt);
                response_prompt = self.call(refined_prompts.clone(), &mut spinner);
                spinner.stop_and_persist(
                    "✔".green().to_string().as_str(),
                    "Refined result!".green().to_string(),
                );
            }
        }
        return response_prompt.content;
    }

    pub fn call(
        self: &Self,
        prompt: Vec<prompts::Prompt>,
        spinner: &mut Spinner,
    ) -> prompts::Prompt {
        let client = Client::new();
        let api_addr = format!("{}/chat/completions", self.config.api_base);
        //let max_tokens = self.cli.token_limit.unwrap_or(self.config.max_tokens);
        let max_tokens = self.config.max_tokens;
        let mut group_prompts = vec![self.system_prompt.clone()];
        group_prompts.extend(prompt.clone());

        log::debug!("Calling OpenAI API: {}", api_addr);
        let response = client
            .post(api_addr)
            .timeout(Duration::from_secs(60 * 3))
            .json(&json!({
                "top_p": 1,
                "temperature": 0,
                "max_tokens": max_tokens,
                "presence_penalty": 0,
                "frequency_penalty": 0,
                "model": "gpt-3.5-turbo",
                "messages": group_prompts,
            }))
            .header("Authorization", format!("Bearer {}", &self.config.api_key))
            .send()
            .unwrap();

        let validated_response = self.validate_response(response, spinner);
        let message =
            serde_json::from_str::<serde_json::Value>(&validated_response.text().unwrap()).unwrap();

        let response_prompt = prompts::Prompt::build(
            message["choices"][0]["message"]["role"]
                .as_str()
                .unwrap()
                .to_string(),
            message["choices"][0]["message"]["content"]
                .as_str()
                .unwrap()
                .to_string(),
        );

        return response_prompt;
    }

    fn validate_response(self: &Self, response: Response, spinner: &mut Spinner) -> Response {
        let status_code = response.status();
        if status_code.is_client_error() {
            let response_body = response.json::<serde_json::Value>().unwrap();
            let error_message = response_body["error"]["message"].as_str().unwrap();
            spinner.stop_and_persist(
                "✖".red().to_string().as_str(),
                format!("API error: \"{error_message}\"").red().to_string(),
            );
            std::process::exit(1);
        } else if status_code.is_server_error() {
            spinner.stop_and_persist(
                "✖".red().to_string().as_str(),
                format!("OpenAI is currently experiencing problems. Status code: {status_code}")
                    .red()
                    .to_string(),
            );
            std::process::exit(1);
        }
        return response;
    }
}
