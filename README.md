# Bitwarden secrets

CLI to help to manage secrets for shell session or code compilation.

## Installation

```
cargo install bitwarden-secrets
```

## Usage

To use the secrets in your shell session as environment variables, you can use the following commands:

### PowerShell

```powershell
iex (bitwarden-secrets | Out-String)
```

### Bash

```bash
source <(bitwarden-secrets)
```
