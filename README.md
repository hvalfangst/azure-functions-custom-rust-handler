# Azure Functions in Rust using Axum with CI/CD

Azure Function with custom handler invoking a binary compiled from code written in Rust with the Axum framework.
This axum project is built and deployed automatically on repository pushes with GitHub Actions Workflows.
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


### 2. Access Azure Portal

- Open your browser and navigate to the Azure Portal.

### 3. Microsoft Entra ID Setup

- Navigate to 'Microsoft Entra ID' using the search bar.
- Click on 'Add' and choose 'App registration'.
- Set name to 'hvalfangst-axum-function', account option to 'Single tenant', and click 'Register'.
- Go to the overview of the newly created app registration 'hvalfangst-axum-function'.
- Copy values associated with Application (client) ID and Directory (tenant) ID for future use.
- Navigate to 'Certificates & secrets' under 'Manage'.
- Click the button titled 'New client secret' under 'Client secrets'.
- Pick a fitting description and keep default expiry. Click on the 'Add' button to finish.
- Copy the associated value with the newly created secret.

### 4. Subscription Setup

- Navigate to your subscription, which holds your provisioned resource group.
- Click on 'Add' under 'Access control (IAM)' and pick 'role assignment'.
- Choose the role 'Contributor' under 'Privileged administrator roles'.
- Navigate to the 'Members' section and click 'Select members'.
- Search for the app registration 'hvalfangst-axum-function' and assign the role.

### 5. GitHub Repository Secrets

- Open the 'Settings' tab of your GitHub repository.
- Click on 'Actions' under 'Security' -> 'Secrets and variables'.
- Create the following repository secrets:
    - AZURE_SP_APP_ID : Value copied in step #3
    - AZURE_SP_PASSWORD: Value copied in step #3
    - AZURE_TENANT_ID : Value copied in step #4

### 5. Deploy Workflow

The GitHub Actions workflow 'cicd' will automatically build our axum binary and deploy the azure function customer handler on repository pushes.