// Starter code derived from https://github.com/jordanbertasso/ctfd-solve-announcer-discord

use std::collections::HashMap;

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
pub struct ScoreboardEntry{
    pub id: i64,
    pub name: String,
}



pub(crate) type TeamId = i64;
pub(crate) type TeamPosition = i64;

impl CTFdClient {
    pub fn new(url: String, api_key: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));

        let auth_value = format!("Token {}", api_key);
        headers.insert("Authorization", header::HeaderValue::from_str(&auth_value).unwrap());

        Self {
            client: reqwest::Client::builder().default_headers(headers).build().unwrap(),
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

    pub async fn get_challenges_of_category(&self, category: &str) -> Result<Vec<Challenge>, reqwest::Error> {
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

    async fn get_challenge_id_by_name(&self, name: &str) -> Result<i64, Box<dyn std::error::Error>> {
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

    async fn new_flag_for_challenge(&self, id: i64, flag: &str) -> Result<(), reqwest::Error> {
        let new_flag = Flag::new(id, &flag);

        let url = format!("{}/api/v1/flags", self.url);
        let response = self
            .client
            .post(&url)
            .json(&new_flag)
            .send()
            .await?;
        
        if response.status() == 200 {
            Ok(())
        } else {
            panic!();
        }
    }

    pub async fn new_challenge(&self, name: &str, value: i64, category: &str, flag: &str) -> Result<Vec<Challenge>, reqwest::Error> {
        let url = format!("{}/api/v1/challenges", self.url);
        let new_challenge = NewChallenge::new(category, name, value);

        let json_text = serde_json::to_string(&new_challenge).unwrap();
        println!("JSON body: {}", json_text);

        // send the request to make the challenge
        let response = self
            .client
            .post(&url)
            .json(&new_challenge)
            .send()
            .await?;
        
        if response.status() != 200 {
            eprintln!("Failed to post");
            dbg!(&response);
            return Ok(Vec::<Challenge>::new());
        }

        let id = self.get_challenge_id_by_name(&name).await.unwrap();
        dbg!(id);

        self.new_flag_for_challenge(id, &flag).await.unwrap();
        
        // probably will need to process the data, get the new challenge id, and then update the flag through the flag function

        Ok(Vec::<Challenge>::new())
    }

    pub async fn get_team(&self, team_id: i64) -> Result<Team, reqwest::Error>{
        let url = format!("{}/api/v1/teams/{}", self.url, team_id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<APIResponse<Team>>()
            .await?;

        Ok(response.data.unwrap())
    }

    pub async fn get_top_10_teams(&self) -> Result<HashMap<TeamId, TeamPosition>, reqwest::Error> {
        let url = format!("{}/api/v1/scoreboard/top/10", self.url);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<APIResponse<HashMap<i64, ScoreboardEntry>>>()
            .await?;

        let mut teams = HashMap::new();

        for (i, team) in response.data.unwrap() {
            teams.insert(team.id, i);
        }

        Ok(teams)
    }
}

impl Challenge {
    pub async fn get_solves(&self, client: &CTFdClient) -> Result<Vec<ChallengeSolver>, reqwest::Error> {
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
    pub fn new(challenge_category: &str, challenge_name: &str, value: i64) -> Self {
        NewChallenge {
            name: challenge_name.to_string(),
            category: challenge_category.to_string(),
            description: String::from(format!("This challenge will be on PWN College in the {} module and will auto-complete here when you solve it there", &challenge_category)),
            value: value.to_string(),
            state: String::from("visible"),
            typestr: String::from("standard"),
        }
    }
}

impl Flag {
    pub fn new(challenge_id: i64, flag: &str) -> Self {
        Flag {
            content: flag.to_string(),
            data: String::from("case_insensitive"),
            typestr: String::from("static"),
            challenge: challenge_id
        }
    }
}