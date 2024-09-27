#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Deployment process shuru ho raha hai..."

# Run Cargo tests to generate Candid file
echo "Candid file generate karne ke liye Cargo tests chala rahe hain..."
cargo test -p like_erc20_backend generate_candid
sleep 5

# Deploy Backend Canister
echo "Backend Canister deploy kar rahe hain..."
dfx deploy like_erc20_backend
sleep 15

# Generate declarations
echo "Declarations generate kar rahe hain..."
dfx generate like_erc20_backend

# Deploy frontend
# echo "Frontend deploy kar rahe hain..."
# npm install
# dfx deploy frontend

# Inform the user that the deployment completed successfully
echo "Deployment safaltapurvak pura ho gaya. Ab aap 'npm run dev' chala kar apna local development server shuru kar sakte hain"
