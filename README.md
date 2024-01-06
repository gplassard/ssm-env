# SSM-Env

Expose values persisted in AWS Parameter Store as environment variables

Inspired by [aws-vault](https://github.com/99designs/aws-vault)


Example usage
```bash
aws-vault exec admin -- cargo run -- exec -- powershell Get-ChildItem Env:
```
