use reqwest::header;
use serde::{Deserialize, Serialize};

pub struct PWNCollegeClient {
    client: reqwest::Client,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleAPIResponse<T> {
    pub success: bool,
    pub errors: Option<Vec<String>>, // idk
    pub modules: Option<T>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SolvesAPIResponse<T> {
    pub success: bool,
    pub errors: Option<Vec<String>>, // idk
    pub solves: Option<T>,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Solve {
    pub module_id: String,
    pub challenge_id: String,
}

impl PWNCollegeClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            url: "https://pwn.college/".to_string(),
        }
    }

    async fn get_modules_from_dojo(&self, dojo: &str) -> Result<Vec<Module>, reqwest::Error> {
        let url = format!("{}/pwncollege_api/v1/dojos/{}/modules", self.url, &dojo);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ModuleAPIResponse<Vec<Module>>>()
            .await?;

        Ok(response.modules.unwrap())
    }

    pub async fn get_challenges_for_module(
        &self,
        module: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let modules = self
            .get_modules_from_dojo("intro-to-cybersecurity")
            .await
            .unwrap(); // we're only using the orange belt
        let target_module = modules.iter().find(|m| m.id == module);

        match target_module {
            Some(module) => Ok(module.challenges.iter().map(|c| c.name.clone()).collect()),
            None => Err("Couldn't find module".into()),
        }
    }

    pub async fn pretty_print_module(
        &self,
        module: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let modules = self
            .get_modules_from_dojo("intro-to-cybersecurity")
            .await
            .unwrap(); // we're only using the orange belt
        let target_module = modules.iter().find(|m| m.id == module);

        match target_module {
            Some(module) => Ok(module.name.clone()),
            None => Err("Couldn't find module".into()),
        }
    }

    pub async fn get_solves_by_user_for_module(
        &self,
        dojo: &str,
        module: &str,
        username: &str,
    ) -> Result<Vec<String>, reqwest::Error> {
        let url = format!(
            "{}/pwncollege_api/v1/dojos/{}/solves?username={}",
            self.url, &dojo, &username
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<SolvesAPIResponse<Vec<Solve>>>()
            .await?;

        // dbg!(&response.solves.unwrap());
        let solves = response.solves.unwrap();
        let target_module_challenges: Vec<&Solve> =
            solves.iter().filter(|c| c.module_id == module).collect();

        Ok(target_module_challenges
            .iter()
            .map(|c| c.challenge_id.clone())
            .collect())
    }
}
