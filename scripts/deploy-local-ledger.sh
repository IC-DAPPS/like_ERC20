#########################################################################################
########################### Deploy local ledger canister ###########################
#########################################################################################

# Create like_erc20_backend canister without initializing
dfx canister create like_erc20_backend

# The archive controller
export ARCHIVE_CONTROLLER=$(dfx canister id like_erc20_backend)

# canister id of stable coin minter as minting account
export MINTER_ACCOUNT=$(dfx canister id like_erc20_backend)

# canister id of ckusdc_pool as Fee collector
export FEE_COLLECTOR_ACCOUNT=$(dfx canister id like_erc20_backend)

TOKEN_NAME="Dollar"
TOKEN_SYMBOL="USD"
Decimals=6

PRE_MINTED_TOKENS=0

# Fee is 0.01 USDx
TRANSFER_FEE=10_000

TRIGGER_THRESHOLD=2000
NUM_OF_BLOCK_TO_ARCHIVE=1000
CYCLE_FOR_ARCHIVE_CREATION=10_000_000_000_000
FEATURE_FLAGS=true
MAX_MEMO_LENGTH=80

# LOGO_BASE64_ENCODED_SVG=$(echo '<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" /></svg>' | base64 -w 0)

METADATA="vec {
    
  }"

dfx deploy ledger --argument "(variant {Init = 
record {
     decimals = opt ${Decimals};
     token_symbol = \"${TOKEN_SYMBOL}\";
     token_name = \"${TOKEN_NAME}\";
     minting_account = record { owner = principal \"${MINTER_ACCOUNT}\" };
     transfer_fee = ${TRANSFER_FEE};
     feature_flags = opt record{icrc2 = ${FEATURE_FLAGS}};
     initial_balances = vec {};
     maximum_number_of_accounts = null;
    metadata = $METADATA;
     accounts_overflow_trim_quantity = null;
     fee_collector_account = opt record { owner = principal \"${FEE_COLLECTOR_ACCOUNT}\" };
     max_memo_length = opt $MAX_MEMO_LENGTH;
     archive_options = record {
         num_blocks_to_archive = ${NUM_OF_BLOCK_TO_ARCHIVE};
         trigger_threshold = ${TRIGGER_THRESHOLD};
         controller_id = principal \"${ARCHIVE_CONTROLLER}\";
         cycles_for_archive_creation = opt ${CYCLE_FOR_ARCHIVE_CREATION};
         max_transactions_per_response = null;
         more_controller_ids = null;
         max_message_size_bytes = null;
         node_max_memory_size_bytes = null;
     };
 }
})"

######################################################################################
######################################################################################