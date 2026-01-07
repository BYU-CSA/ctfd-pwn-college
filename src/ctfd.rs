// Starter code derived from https://github.com/jordanbertasso/ctfd-solve-announcer-discord

use chrono::{DateTime, Utc};
use reqwest::header;
use serde::{Deserialize, Serialize};

pub struct CTFdClient {
    client: reqwest::Client,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub success: bool,
    pub errors: Option<Vec<String>>,
    pub data: Option<T>,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Challenge {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct NewChallenge {
    pub name: String,
    pub category: String,
    pub description: String,
    pub value: String,
    pub state: String,
    #[serde(rename = "type")] // cuz it's stupid
    pub typestr: String,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    pub content: String,
    pub data: String,
    #[serde(rename = "type")]
    pub typestr: String,
    pub challenge: i64,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: String, // the actual username of the user
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub fields: Vec<Field>,
    pub id: i64,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChallengeSolver {
    pub account_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreboardEntry {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Submission {
    pub challenge_id: i64,
    pub user_id: i64,
    pub provided: String,
    #[serde(rename = "type")]
    pub typestr: String,
    pub date: DateTime<Utc>,
}

impl CTFdClient {
    pub fn new(url: String, api_key: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        let auth_value = format!("Token {}", api_key);
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&auth_value).unwrap(),
        );

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            url,
        }
    }

    pub async fn get_challenges(&self) -> Result<Vec<Challenge>, reqwest::Error> {
        let url = format!("{}/api/v1/challenges", self.url);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<APIResponse<Vec<Challenge>>>()
            .await?;

        Ok(response.data.unwrap())
    }

    pub async fn get_challenges_of_category(
        &self,
        category: &str,
    ) -> Result<Vec<Challenge>, reqwest::Error> {
        let url = format!("{}/api/v1/challenges?category={}", self.url, category);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<APIResponse<Vec<Challenge>>>()
            .await?;

        Ok(response.data.unwrap())
    }

    async fn get_challenge_id_by_name(
        &self,
        name: &str,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/challenges?name={}", self.url, name);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<APIResponse<Vec<Challenge>>>()
            .await?;

        match response.data.unwrap().first() {
            Some(challenge) => return Ok(challenge.id),
            None => return Err("No challenge found matching name".into()),
        }
    }

    async fn new_flag_for_challenge(
        &self,
        id: i64,
        flag: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let new_flag = Flag::new(id, &flag);

        let url = format!("{}/api/v1/flags", self.url);
        let response = self.client.post(&url).json(&new_flag).send().await?;

        if response.status() == 200 {
            Ok(())
        } else {
            Err("failed to upload flag for challenge".into())
        }
    }

    pub async fn new_challenge(
        &self,
        name: &str,
        category: &str,
        flag: &str,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Ensure that no challenge gets double-created
        let challenge_exists = self.get_challenge_id_by_name(&name).await;
        match challenge_exists {
            Ok(id) => {
                eprintln!("Challenge {} already exists with id {}", &name, id);
                return Ok(id);
            }
            _ => {}
        }

        let url = format!("{}/api/v1/challenges", self.url);
        let new_challenge = NewChallenge::new(category, name);

        // send the request to make the challenge
        let response = self.client.post(&url).json(&new_challenge).send().await?;

        if response.status() != 200 {
            return Err("Failed to create new challenge".into());
        }

        let id = self.get_challenge_id_by_name(&name).await.unwrap();

        self.new_flag_for_challenge(id, &flag).await.unwrap();

        Ok(id)
    }

    /// Fetches all the user objects from CTFd
    /// I've set it up such that there's a custom field called PWN College username
    /// This function fetches those values for all users for which it is configured
    pub async fn get_users(&self) -> Result<Vec<String>, reqwest::Error> {
        // Add ?view=admin to get all users
        let url = format!("{}/api/v1/users?view=admin", self.url);
        let response = self
            .client
            .get(&url)
            .send()
            .await? //.text().await?;
            .json::<APIResponse<Vec<User>>>()
            .await?;

        // I can only do this cuz there's only one field
        Ok(response
            .data
            .unwrap()
            .iter()
            .filter(|user| user.fields.len() > 0)
            .map(|user| user.fields.get(0).unwrap().value.clone())
            .collect())
        // Ok(Vec::new())
    }

    pub async fn get_user_id_for_user(&self, username: &str) -> Result<i64, reqwest::Error> {
        let url = format!("{}/api/v1/users?view=admin", self.url);
        let response = self
            .client
            .get(&url)
            .send()
            .await? //.text().await?;
            .json::<APIResponse<Vec<User>>>()
            .await?;

        let matched_users: Vec<User> = response
            .data
            .unwrap()
            .into_iter()
            .filter(|user| user.fields.len() > 0 && user.fields.get(0).unwrap().value == username.to_string())
            .collect();

        if matched_users.len() < 1 {
            todo!("Return an error here");
        }

        Ok(matched_users.get(0).expect("Somehow had no entries").id)
    }

    pub async fn post_submission(
        &self,
        challenge_id: i64,
        username: &str,
        flag: &str,
    ) -> Result<(), String> {
        let real_user_id = self
            .get_user_id_for_user(&username)
            .await
            .map_err(|e| format!("Failed to get user id for user: {e:?}").to_string())?;
        let submission = Submission::new(challenge_id, real_user_id, flag);

        let url = format!("{}/api/v1/submissions", self.url);
        let response = self.client.post(&url).json(&submission).send().await
            .map_err(|e| format!("Failed to send post request: {e:?}").to_string())?;

        if response.status() != 200 {
            dbg!(response.text().await.unwrap());
            return Err("Failed to create new challenge".into());
        }

        Ok(())
    }
}

impl Challenge {
    pub async fn get_solves(
        &self,
        client: &CTFdClient,
    ) -> Result<Vec<ChallengeSolver>, reqwest::Error> {
        let url = format!("{}/api/v1/challenges/{}/solves", client.url, self.id);
        let response = client
            .client
            .get(&url)
            .send()
            .await?
            .json::<APIResponse<Vec<ChallengeSolver>>>()
            .await?;

        Ok(response.data.unwrap())
    }
}

impl NewChallenge {
    pub fn new(challenge_category: &str, challenge_name: &str) -> Self {
        Self {
            name: challenge_name.to_string(),
            category: challenge_category.to_string(),
            description: String::from(format!(
                "This challenge will be on PWN College in the {} module and will auto-complete here when you solve it there",
                &challenge_category
            )),
            value: 100.to_string(),
            state: String::from("visible"),
            typestr: String::from("standard"),
        }
    }
}

impl Submission {
    pub fn new(challenge_id: i64, user_id: i64, flag: &str) -> Self {
        Self {
            challenge_id,
            user_id,
            provided: flag.to_string(),
            typestr: String::from("string"),
            date: Utc::now(),
        }
    }
}

impl Flag {
    pub fn new(challenge_id: i64, flag: &str) -> Self {
        Self {
            content: flag.to_string(),
            data: String::from("case_insensitive"),
            typestr: String::from("static"),
            challenge: challenge_id,
        }
    }
}
