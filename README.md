# Azure Functions App with customer handler in Rust utilizing Axum

Azure Function programmed in Rust with the Axum framework, 
providing endpoints for arithmetic operations.
A CI/CD pipeline has been implemented with GitHub Actions. 
It will build, package and deploy this application to Azure Function Apps
on any repository pushes. Azure resources are provisioned with Terraform.

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

### 2. Access Azure Portal

- Open your browser and navigate to the Azure Portal.

### 3. Configure Deployment Credentials

- Navigate to your newly created Function App 'hvalfangstfunctionapp'.
- Download publish profile
- create new secret 'PUBLISH_PROFILE' in GitHub repository