# Konjo

Konjo is a Rust-first GitHub AI contributor bot. The initial implementation in this repository is the production-oriented service scaffold: webhook ingress, event verification, workflow orchestration boundaries, a provider-agnostic LLM interface, and an issue-to-PR execution path stub.

## Architecture

The service is designed around a GitHub App runtime:

```text
GitHub webhook
	-> Axum ingress
	-> signature verification
	-> event parsing
	-> workflow dispatch
	-> LLM planning layer
	-> isolated repo workspace
	-> GitHub PR/comment actions
```

The first milestone intentionally prioritizes the runtime boundaries over full repository mutation. That means the code currently accepts verified issue webhooks, routes opened issues into the issue-to-PR workflow, allocates an isolated workspace, produces a stub plan through an LLM provider abstraction, and records the intended PR action through the GitHub client layer.

## Current Modules

```text
src/
	config.rs            Environment-based runtime configuration
	error.rs             Shared application error model
	github/
		client.rs          GitHub client wrapper and action stubs
		events.rs          Supported webhook payload models
		webhook.rs         HMAC verification for GitHub signatures
	llm.rs               Provider trait and mock planner implementation
	repo.rs              Ephemeral per-job workspace management
	server.rs            Axum routes and webhook handling
	state.rs             Shared application state
	workflows/
		issue_to_pr.rs     First workflow boundary and implementation stub
```

## Required Configuration

Set these environment variables before starting the service:

```bash
export GITHUB_WEBHOOK_SECRET="..."
export GITHUB_APP_ID="123456"
export GITHUB_PRIVATE_KEY_PEM="-----BEGIN PRIVATE KEY-----..."
export GITHUB_INSTALLATION_ID="123456789"   # optional for now
export KONJO_BIND_ADDR="0.0.0.0:3000"       # optional
export KONJO_LOG="info,konjo=debug"         # optional
export KONJO_WORKSPACE_ROOT="/tmp/konjo"    # optional
```

## Run

```bash
cargo run
```

Health endpoint:

```text
GET /healthz
```

GitHub webhook endpoint:

```text
POST /webhooks/github
```

## What Is Implemented

- GitHub webhook signature verification using `X-Hub-Signature-256`
- Parsing for `ping` and `issues` webhook events
- Dispatch for `issues.opened` into an issue-to-PR workflow
- Provider-agnostic LLM planning interface with a mock implementation
- Isolated per-job workspace creation under a configurable root
- GitHub client boundary for later PR, comment, and review operations
- Health route and basic HTTP tracing

## What Is Not Implemented Yet

- GitHub App JWT generation and installation token exchange
- Repository cloning, file editing, validation command execution, and commits
- PR creation, PR review automation, CI triage, dependency updates, and maintenance workflows
- Delivery deduplication, persistent job state, and retry queues

## Next Steps

1. Add GitHub App authentication and installation token exchange.
2. Implement repository cloning and controlled command execution in isolated workspaces.
3. Convert the PR creation stub into a real branch, commit, push, and pull request flow.
4. Add policy enforcement for editable paths, command allowlists, and approval gates.
5. Extend the event model and orchestration layer for PR review and CI triage.

## License

This project is licensed under the MIT License.
