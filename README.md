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

# execute with credentials retrieved from AWS Parameter Store at /app/ssm-env/env/<ENV_VARIABLE>
aws-vault exec admin -- ssm-env exec -- <command>
# execute with credentials retrieved from AWS Parameter Store at /app/ssm-env/env/context-1/<ENV_VARIABLE> and /app/ssm-env/env/context-2/<ENV_VARIABLE>
aws-vault exec admin -- ssm-env exec --contexts context-1 --contexts context-2 -- <command>
# execute with credentials retrieved from AWS Parameter Store at /my/prefix-1/<ENV_VARIABLE> and /my/prefix-2/<ENV_VARIABLE>
aws-vault exec admin -- ssm-env exec --ssm-path-prefixes /my/prefix-1/ --ssm-path-prefixes /my/prefix-2/ -- <command>
# execute with specific SSM parameters mapped to environment variables
aws-vault exec admin -- ssm-env exec --param-map /app/db/password=DB_PASSWORD --param-map /app/api/key=API_KEY -- <command>
# execute with credentials retrieved from AWS Parameter Store at /app/ssm-env/ansible-vault and put in a temporary file compatible with ansible vault
aws-vault exec admin -- ssm-env  exec-ansible-vault-mode -- <command>
```

Running from cargo (windows)
```bash
aws-vault exec admin -- cargo run -- exec powershell Get-ChildItem Env:
aws-vault exec admin -- cargo run -- exec-ansible-vault-mode powershell Get-ChildItem Env:
```

Running from cargo (unix)
```bash
aws-vault exec admin -- cargo run -- exec env
aws-vault exec admin -- cargo run -- exec-ansible-vault-mode env
```
