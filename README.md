Here’s the provided text in markdown format:

```md
# Like ERC20 Token Project on Internet Computer

This project implements an ERC20-like token system on the Internet Computer (IC) platform. It includes a Rust-based backend canister that manages token operations and a local ledger for testing purposes.

## Prerequisites

Before you begin, ensure you have the following installed:
- DFINITY Canister SDK (dfx)
- Rust and Cargo
- Node.js and npm

## Getting Started

1. Clone the repository and navigate to the project directory:

   ```
   git clone https://github.com/your-username/like_ERC20.git
   cd like_ERC20
   ```

2. Start the local Internet Computer replica:

   ```
   dfx start --clean
   ```

## Deployment

The project includes two main deployment scripts:

### 1. Deploy Local Ledger

This script deploys the local ledger canister:
```
./scripts/deploy-local-ledger.sh
```

### 2. Deploy Backend

This script builds and deploys the Like ERC20 backend canister:

```
./scripts/deploy-local.sh
```

During deployment, you'll be prompted to enter an initialization argument for the maximum token supply.

## Usage

After deployment, you can interact with the canister using `dfx canister call` commands. Here are some example operations:

1. Check maximum supply:
   ```
   dfx canister call like_erc20_backend max_supply
   ```

2. Mint tokens:
   ```
   dfx canister call like_erc20_backend mint '(principal "YOUR_PRINCIPAL_ID", 50)'
   ```

3. Check balance:
   ```
   dfx canister call like_erc20_backend balance_of '(principal "YOUR_PRINCIPAL_ID")'
   ```

4. Approve spending:
   ```
   dfx canister call like_erc20_backend approve '(principal "YOUR_PRINCIPAL_ID", principal "SPENDER_PRINCIPAL_ID", 20)'
   ```

5. Transfer tokens:
   ```
   dfx canister call like_erc20_backend transfer '(principal "FROM_PRINCIPAL_ID", principal "TO_PRINCIPAL_ID", 10)'
   ```

Replace `YOUR_PRINCIPAL_ID`, `SPENDER_PRINCIPAL_ID`, `FROM_PRINCIPAL_ID`, and `TO_PRINCIPAL_ID` with actual principal IDs.

## Project Structure

- `src/like_erc20_backend/`: Contains the Rust code for the backend canister
- `scripts/`: Contains deployment and build scripts
- `dfx.json`: Configuration file for the Internet Computer project

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
```

You can use this markdown text as-is for documentation or sharing purposes!