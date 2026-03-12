use std::{fs, path::PathBuf};

use tempfile::TempDir;

use crate::error::AppError;

#[derive(Clone)]
pub struct WorkspaceManager {
    root: PathBuf,
}

impl WorkspaceManager {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn create_job_workspace(&self, job_key: &str) -> Result<JobWorkspace, AppError> {
        fs::create_dir_all(&self.root)
            .map_err(|source| AppError::Workspace(source.to_string()))?;

        let dir = tempfile::Builder::new()
            .prefix(&format!("konjo-{job_key}-"))
            .tempdir_in(&self.root)
            .map_err(|source| AppError::Workspace(source.to_string()))?;

        Ok(JobWorkspace { dir })
    }
}

pub struct JobWorkspace {
    dir: TempDir,
}

impl JobWorkspace {
    pub fn path(&self) -> &std::path::Path {
        self.dir.path()
    }
}