# sample_simple_discord_bot_rust

This repository contains a simple Discord bot implemented in Rust for AWS Lambda. It is set up to respond to the `/ask` command.

## Build Environment

The execution environment is assumed to be [provided.al2 (Amazon Linux 2)/arm64](https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html). Therefore, the default build target is `aarch64-unknown-linux-musl`.
If you are building on macOS, use [FiloSottile/homebrew-musl-cross](https://github.com/FiloSottile/homebrew-musl-cross) or [emk/rust-musl-builder](https://github.com/emk/rust-musl-builder).

## Build

The `discord` crate can be built using the following command:

```bash
cargo build --release
```

## Deployment

Please follow the steps in the [official documentation's Getting Started guide](https://discord.com/developers/docs/getting-started#installing-slash-commands)
for deployment procedures.

AWS Lambda needs to refer to the raw http body for signature verification, so you need to have [Proxy Integration](https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html)
enabled.

### Environment Variables

The following tokens are required for execution. For more details, please see [Adding Credentials](https://discord.com/developers/docs/getting-started#adding-credentials).

```
DISCORD_PUBLIC_KEY
DISCORD_TOKEN
```

### Discord Command Registration

After deployment, register the slash command. For more details, please see [Installing slash commands](https://discord.com/developers/docs/getting-started#installing-slash-commands).

```bash
cd discord_register
APP_ID=*** cargo run
```

### IAM

Configure the Execution role of Lambda functions as follows:

#### Permissions

Grant the following permissions to enable access to the related services:

- AWSLambdaBasicExecutionRole

#### Trust relationships

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
```
