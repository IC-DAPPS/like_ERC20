#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Deployment process is starting..."

# Run Cargo tests to generate Candid file
echo "Running Cargo tests to generate Candid file..."
cargo test -p like_erc20_backend generate_candid
sleep 5

# Deploy Backend Canister
echo "Deploying Backend Canister..."
dfx deploy like_erc20_backend
# sleep 15

# # Generate declarations
# echo "Generating declarations..."
# dfx generate like_erc20_backend

# Deploy frontend
# echo "Deploying frontend..."
# npm install
# dfx deploy frontend

# Inform the user that the deployment completed successfully
echo "Deployment completed successfully. You can now run 'npm run dev' to start your local development server"
