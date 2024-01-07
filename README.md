# SSM-Env

Expose values persisted in AWS Parameter Store as environment variables

Inspired by [aws-vault](https://github.com/99designs/aws-vault)

# Installation

```bash
cargo install --path .
```

# Usage

```bash
# display help
ssm-env --help
ssm-env exec --help

# execute with credentials retrieved from aws-vault
aws-vault exec admin -- ssm-env exec powershell Get-ChildItem Env:
```

Running from cargo
```bash
aws-vault exec admin -- cargo run -- exec powershell Get-ChildItem Env:
```
