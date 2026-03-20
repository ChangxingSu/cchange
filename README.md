# cchange

Switch between Claude Code profiles (subscription, API keys, custom endpoints).

## Install

```sh
cargo install --path .
```

## Quick Start

```sh
# Create a config file
cchange init

# Edit ~/.config/cchange/config.toml with your keys
# Then launch claude with a profile
cchange company
```

## Usage

```
cchange <profile> [claude args...]
cchange list
cchange init
```

Everything after the profile name is passed directly to `claude`:

```sh
# Resume mode with company key
cchange company -r

# One-shot prompt with personal key
cchange personal -p "explain this error"

# Allowed tools
cchange company -r --allowedTools Edit,Bash

# No profile = uses default from config
cchange -p "hello"
```

## Config

Default path: `~/.config/cchange/config.toml`

Override with `CCHANGE_CONFIG` env var:

```sh
export CCHANGE_CONFIG=~/my-cchange.toml
```

### Format

```toml
default = "pro"

[profiles.pro]
# No api_key = use subscription auth (Pro/Max plan)

[profiles.company]
api_key = "sk-ant-api03-COMPANY_KEY"
api_url = "https://api-proxy.company.com/v1"

[profiles.personal]
api_key = "sk-ant-api03-PERSONAL_KEY"
```

- `api_key` omitted → unsets `ANTHROPIC_API_KEY` (subscription auth)
- `api_key` set → sets `ANTHROPIC_API_KEY`
- `api_url` set → sets `ANTHROPIC_BASE_URL`
- `api_url` omitted → unsets `ANTHROPIC_BASE_URL` (default endpoint)

## Commands

| Command | Description |
|---------|-------------|
| `cchange <profile>` | Launch claude with the given profile |
| `cchange list` | Show profiles with masked keys |
| `cchange init` | Create config file with sample template |

## How It Works

`cchange` uses Unix `exec` to replace itself with `claude`, setting the appropriate env vars. No subprocess, no signal proxying — claude gets the full TTY.
