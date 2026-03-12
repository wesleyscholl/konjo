use octocrab::Octocrab;

use crate::{config::GitHubConfig, error::AppError};

#[derive(Clone)]
pub struct GitHubClient {
    inner: Octocrab,
    auth: GitHubConfig,
}

impl GitHubClient {
    pub fn new(auth: GitHubConfig) -> Result<Self, AppError> {
        let inner = Octocrab::builder().build()?;
        Ok(Self { inner, auth })
    }

    pub fn app_id(&self) -> u64 {
        self.auth.app_id
    }

    pub fn installation_id(&self) -> Option<u64> {
        self.auth.installation_id
    }

    pub fn client(&self) -> &Octocrab {
        &self.inner
    }

    pub async fn create_pull_request_stub(
        &self,
        repository: &str,
        issue_number: u64,
        branch_name: &str,
        title: &str,
        body: &str,
    ) -> Result<(), AppError> {
        tracing::info!(
            repository,
            issue_number,
            branch_name,
            title,
            body,
            app_id = self.app_id(),
            installation_id = ?self.installation_id(),
            "PR creation is not implemented yet; recording intended action"
        );
        Ok(())
    }
}