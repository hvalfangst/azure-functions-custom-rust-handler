# Azure Functions App with customer handler in Rust utilizing Axum

Azure Function with customer handler invoking a binary compiled from code written in Rust with the Axum framework.
The binary 'handler' and associated files (function/host.json) are deployed to Azure Function App via 'func azure functionapp publish'
Azure resources are provisioned with Terraform.

## Requirements

* x86-64
* Linux/Unix
* [Rust](https://www.rust-lang.org/tools/install)

## Creating resources

The shell script 'up' allocates Azure resources with Terraform.

## Deleting resources

The shell script 'down' deallocates Azure resources.


## Guide

### 1. Provision Azure Resources

- Run the 'up' script to provision Azure resources.

### 2. Build binary

- sudo apt install musl-tools
- rustup target add x86_64-unknown-linux-musl
- cargo build --release --target=x86_64-unknown-linux-musl
- cp target/x86_64-unknown-linux-musl/release/handler .


### 3. Deploy Custom Binary Azure Function

- func azure functionapp publish hvalfangstlinuxfunctionapp --custom