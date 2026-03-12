use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    error::AppError,
    github::{events::GitHubEvent, webhook},
    state::AppState,
};

pub async fn serve(state: AppState) -> Result<(), AppError> {
    let bind_addr = state.config.bind_addr;
    let app = router(state);
    let listener = TcpListener::bind(bind_addr)
        .await
        .map_err(|source| AppError::Internal(source.to_string()))?;

    tracing::info!(address = %bind_addr, "starting webhook server");
    axum::serve(listener, app)
        .await
        .map_err(|source| AppError::Internal(source.to_string()))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/webhooks/github", post(github_webhook))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state))
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

async fn github_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let signature = required_header(&headers, "x-hub-signature-256")?;
    let event_name = required_header(&headers, "x-github-event")?;
    let delivery_id = headers
        .get("x-github-delivery")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown");

    webhook::verify_signature(&state.config.github.webhook_secret, signature, &body)?;
    let event = GitHubEvent::parse(event_name, &body)?;

    tracing::info!(delivery_id, event_name, "accepted GitHub webhook delivery");

    match event {
        GitHubEvent::Ping => Ok((StatusCode::OK, Json(json!({ "message": "pong" })))),
        GitHubEvent::Issues(payload) => {
            if payload.action == "opened" {
                state.issue_to_pr.handle_issue_opened(payload).await?;
                Ok((StatusCode::ACCEPTED, Json(json!({ "queued": true }))))
            } else {
                Ok((
                    StatusCode::ACCEPTED,
                    Json(json!({ "queued": false, "reason": "issue action ignored" })),
                ))
            }
        }
    }
}

fn required_header<'a>(headers: &'a HeaderMap, name: &'static str) -> Result<&'a str, AppError> {
    headers
        .get(name)
        .ok_or_else(|| AppError::BadRequest(format!("missing header {name}")))?
        .to_str()
        .map_err(|_| AppError::BadRequest(format!("invalid header {name}")))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::util::ServiceExt;

    use crate::{
        config::{AppConfig, GitHubConfig},
        state::AppState,
        workflows::issue_to_pr::NoopIssueToPrWorkflow,
    };

    use super::router;

    #[tokio::test]
    async fn health_route_returns_ok() {
        let app = router(AppState::new(
            AppConfig {
                bind_addr: "127.0.0.1:3000".parse().expect("socket addr"),
                log_filter: "info".to_owned(),
                github: GitHubConfig {
                    webhook_secret: "secret".to_owned(),
                    app_id: 1,
                    installation_id: Some(2),
                    private_key_pem: "pem".to_owned(),
                },
                workspace_root: std::env::temp_dir().join("konjo-tests"),
            },
            Arc::new(NoopIssueToPrWorkflow),
        ));

        let response = app
            .oneshot(Request::builder().uri("/healthz").body(Body::empty()).expect("request"))
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.expect("body").to_bytes();
        assert!(std::str::from_utf8(&body).expect("utf8").contains("ok"));
    }
}