# Pelaika-TUI

An agentic coding assistant for the terminal — it runs on your local AI models via the Pelaika API.

> Built in Rust as a learning project. Today it authenticates and lists available
> models; streaming chat and the agentic tool-running loop (the part that makes it a
> *coding* assistant) are in progress. See the roadmap below.

## What it is

A terminal client that talks to the Pelaika API to drive your own
local LLMs. The goal is a Claude-Code-style agent: stream the model's reply, let it
call tools, run those tools locally, and loop until the task is done — all from the
terminal.

## Requirements

- A [Rust toolchain](https://rustup.rs/) (stable, 2024 edition) — gives you `cargo`.
- Access to a running Pelaika API and a Keycloak account to mint a token against.

## Setup

Configuration is read from environment variables (loaded from a `.env` file in the
crate root via [`dotenvy`](https://crates.io/crates/dotenvy)). Copy the example and
fill it in:

```bash
cp .env.example .env
```

| Variable                    | What it is                                              |
|-----------------------------|--------------------------------------------------------|
| `PELAIKA_API_BASE_URL`      | Base URL of the Pelaika API (e.g. `https://…`)         |
| `PELAIKA_KC_TOKEN_ENDPOINT` | Keycloak realm token endpoint (for the password grant) |
| `PELAIKA_KC_CLIENT_ID`      | Keycloak client id (`pelaika`)                          |
| `PELAIKA_USERNAME`          | Username for the password grant                         |
| `PELAIKA_PASSWORD`          | Password for that account                               |

> `.env` holds a secret and is git-ignored — never commit it. The token is minted
> per-run and lasts ~5 minutes; there's no refresh yet, so just re-run when it expires.

## Run

```bash
cargo run
```

Right now this mints a bearer token and prints the models available from the API.

## Roadmap

The build follows a milestone path:

- [x] **M1.5** — mint a bearer token via the Keycloak password grant
- [x] **M2** — list available models (`GET /models`)
- [ ] **M3** — stream a chat reply (`POST /chats/incognito`, SSE)
- [ ] **M4** — interactive REPL with a transcript
- [ ] **M5** — terminal polish (colors, spinner, raw mode)
- [ ] **M6** — full Keycloak auth (PKCE + refresh) and saved chats
- [ ] **M7** — the agentic tool loop (`POST /chats/agent`): model-driven `read_file` / `write_file` / `run_bash`
