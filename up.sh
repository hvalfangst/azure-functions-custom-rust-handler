#!/bin/sh

# Exits immediately if a command exits with a non-zero status
set -e

echo "Initializing Terraform..."
terraform init;

echo "Planning Azure resource provisioning..."
terraform plan;

echo "Applying planned Azure resource provisioning..."
terraform apply;