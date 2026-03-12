use serde::Deserialize;

use crate::error::AppError;

#[derive(Clone, Debug)]
pub enum GitHubEvent {
    Ping,
    Issues(IssuesEvent),
}

impl GitHubEvent {
    pub fn parse(event_name: &str, payload: &[u8]) -> Result<Self, AppError> {
        match event_name {
            "ping" => Ok(Self::Ping),
            "issues" => Ok(Self::Issues(
                serde_json::from_slice(payload)
                    .map_err(|source| AppError::BadRequest(format!("invalid issues event: {source}")))?,
            )),
            other => Err(AppError::UnsupportedEvent(other.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct IssuesEvent {
    pub action: String,
    pub issue: Issue,
    pub repository: Repository,
    pub installation: Option<Installation>,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub html_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Repository {
    pub full_name: String,
    pub default_branch: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Installation {
    pub id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub login: String,
}