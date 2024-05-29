# Bitwarden secrets

CLI to help to manage secrets for shell session or code compilation.

## Installation

```
cargo install bitwarden-secrets
```

And also install [Bitwarden's CLI](https://bitwarden.com/download/#downloads-command-line-interface).

## Usage

To use the secrets in your shell session as environment variables, you can use the following commands:

### PowerShell

```powershell
iex (bw-secrets | Out-String)
```

### Bash

```bash
source <(bw-secrets)
```
