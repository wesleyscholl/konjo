use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    error::AppError,
    github::{client::GitHubClient, events::IssuesEvent},
    llm::{IssuePrompt, LlmProvider},
    repo::WorkspaceManager,
};

#[async_trait]
pub trait IssueToPrWorkflow: Send + Sync {
    async fn handle_issue_opened(&self, event: IssuesEvent) -> Result<(), AppError>;
}

pub struct DefaultIssueToPrWorkflow {
    llm: Arc<dyn LlmProvider>,
    github: Arc<GitHubClient>,
    workspace_manager: Arc<WorkspaceManager>,
}

impl DefaultIssueToPrWorkflow {
    pub fn new(
        llm: Arc<dyn LlmProvider>,
        github: Arc<GitHubClient>,
        workspace_manager: Arc<WorkspaceManager>,
    ) -> Self {
        Self {
            llm,
            github,
            workspace_manager,
        }
    }
}

#[async_trait]
impl IssueToPrWorkflow for DefaultIssueToPrWorkflow {
    async fn handle_issue_opened(&self, event: IssuesEvent) -> Result<(), AppError> {
        let plan = self
            .llm
            .plan_issue_to_pr(IssuePrompt {
                repository: event.repository.full_name.clone(),
                issue_number: event.issue.number,
                issue_title: event.issue.title.clone(),
                issue_body: event.issue.body.clone(),
            })
            .await?;

        let job_key = format!("issue-{}", event.issue.number);
        let workspace = self.workspace_manager.create_job_workspace(&job_key)?;

        tracing::info!(
            repository = event.repository.full_name,
            default_branch = event.repository.default_branch,
            issue_number = event.issue.number,
            issue_url = ?event.issue.html_url,
            sender = event.sender.login,
            installation_id = event.installation.as_ref().map(|installation| installation.id),
            workspace = %workspace.path().display(),
            branch_name = plan.branch_name,
            "prepared issue-to-PR workflow execution"
        );

        self.github
            .create_pull_request_stub(
                &event.repository.full_name,
                event.issue.number,
                &plan.branch_name,
                &plan.pr_title,
                &plan.pr_body,
            )
            .await
    }
}

pub struct NoopIssueToPrWorkflow;

#[async_trait]
impl IssueToPrWorkflow for NoopIssueToPrWorkflow {
    async fn handle_issue_opened(&self, _event: IssuesEvent) -> Result<(), AppError> {
        Ok(())
    }
}