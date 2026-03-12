use std::sync::Arc;

use konjo::{
    config::AppConfig,
    github::client::GitHubClient,
    llm::MockLlmProvider,
    logging,
    repo::WorkspaceManager,
    server,
    state::AppState,
    workflows::issue_to_pr::DefaultIssueToPrWorkflow,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env()?;
    logging::init(&config.log_filter);

    let github_client = Arc::new(GitHubClient::new(config.github.clone())?);
    let llm_provider = Arc::new(MockLlmProvider::default());
    let workspace_manager = Arc::new(WorkspaceManager::new(config.workspace_root.clone()));
    let issue_workflow = Arc::new(DefaultIssueToPrWorkflow::new(
        llm_provider,
        github_client,
        workspace_manager,
    ));

    let state = AppState::new(config, issue_workflow);
    server::serve(state).await?;
    Ok(())
}