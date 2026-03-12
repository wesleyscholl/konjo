use std::sync::Arc;

use crate::{config::AppConfig, workflows::issue_to_pr::IssueToPrWorkflow};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub issue_to_pr: Arc<dyn IssueToPrWorkflow>,
}

impl AppState {
    pub fn new(config: AppConfig, issue_to_pr: Arc<dyn IssueToPrWorkflow>) -> Self {
        Self {
            config,
            issue_to_pr,
        }
    }
}