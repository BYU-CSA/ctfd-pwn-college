use serde::{Deserialize, Serialize};
use reqwest::header;

pub struct PWNCollegeClient {
    client: reqwest::Client,
    url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub errors: Option<Vec<String>>, // idk
    pub modules: Option<T>
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub challenges: Vec<Challenge>, // there's more, but this is all that matters
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub name: String,
}

impl PWNCollegeClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));

        Self {
            client: reqwest::Client::builder().default_headers(headers).build().unwrap(),
            url: "https://pwn.college/".to_string(),
        }
    }

    pub async fn get_modules_from_dojo(&self, dojo: &str) -> Result<Vec<Module>, reqwest::Error> {
        let url = format!("{}/pwncollege_api/v1/dojos/{}/modules", self.url, &dojo);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<APIResponse<Vec<Module>>>()
            .await?;
        
        Ok(response.modules.unwrap())
    }

    pub async fn get_challenges_for_module(&self, module: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let modules = self.get_modules_from_dojo("intro-to-cybersecurity").await.unwrap(); // we're only using the orange belt
        let target_module = modules.iter().find(|m| m.id == module);

        match target_module {
            Some(module) => {
                Ok(module.challenges.iter().map(|c| c.name.clone()).collect())
            },
            None => Err("Couldn't find module".into())
        }
    }
}