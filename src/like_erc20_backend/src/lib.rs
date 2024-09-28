use candid::{CandidType, Nat, Principal};
use std::collections::HashMap;

// This is like a piggy bank for each person
#[derive(CandidType, Clone)]
struct Account {
    owner: Principal,
    balance: Nat,
    allowances: HashMap<Principal, Nat>,
}

// If we don't have any information, we make an empty piggy bank
impl Default for Account {
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
            balance: Nat::from(0u64),
            allowances: HashMap::new(),
        }
    }
}

// This is like a big box that holds all the piggy banks
#[derive(Default)]
struct Token {
    total_supply: Nat,
    max_supply: Nat,
    accounts: HashMap<Principal, Account>,
}
// We keep our big box of piggy banks in a special place
thread_local! {
    static TOKEN: std::cell::RefCell<Token> = std::cell::RefCell::new(Token::default());
}

// This is how we start our token game
#[ic_cdk::init]
fn init(max_supply: Nat) {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.max_supply = max_supply.clone();
        token.total_supply = Nat::from(0u64);
    });
}

// This tells us the maximum number of tokens that can exist
#[ic_cdk::query]
fn max_supply() -> Nat {
    TOKEN.with(|token| token.borrow().max_supply.clone())
}

// This is how we create new tokens and give them to someone
#[ic_cdk::update]
fn mint(to: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        if token.total_supply.clone() + amount.clone() > token.max_supply {
            return Err("Oops! We can't make more tokens than allowed".to_string());
        }
        let account = token.accounts.entry(to).or_insert_with(|| Account {
            owner: to,
            balance: Nat::from(0u64),
            allowances: HashMap::new(),
        });
        account.balance += amount.clone();
        token.total_supply += amount;
        Ok(())
    })
}

// This is how we remove tokens from someone's piggy bank
#[ic_cdk::update]
fn burn(from: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let account = token
            .accounts
            .get_mut(&from)
            .ok_or("Oops! We can't find this piggy bank")?;
        if account.balance < amount {
            return Err("Oops! Not enough tokens in the piggy bank".to_string());
        }
        account.balance -= amount.clone();
        token.total_supply -= amount;
        Ok(())
    })
}

// This is how we let someone else use our tokens
#[ic_cdk::update]
fn approve(owner: Principal, spender: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let account = token
            .accounts
            .get_mut(&owner)
            .ok_or("Oops! We can't find this piggy bank")?;
        account.allowances.insert(spender, amount);
        Ok(())
    })
}

// This tells us how many tokens someone can use from another person's piggy bank
#[ic_cdk::query]
fn allowance(owner: Principal, spender: Principal) -> Nat {
    TOKEN.with(|token| {
        let token = token.borrow();
        token
            .accounts
            .get(&owner)
            .and_then(|account| account.allowances.get(&spender))
            .cloned()
            .unwrap_or_else(|| Nat::from(0u64))
    })
}

// This is how we move tokens from one piggy bank to another
#[ic_cdk::update]
fn transfer(from: Principal, to: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let from_account = token
            .accounts
            .get_mut(&from)
            .ok_or("Oops! We can't find the sender's piggy bank")?;

        // Check if it's a direct transfer or through allowance
        let caller = ic_cdk::caller();
        if caller != from {
            let allowance = from_account
                .allowances
                .get_mut(&caller)
                .ok_or("Oops! You don't have permission to transfer")?;
            if *allowance < amount {
                return Err(
                    "Oops! You don't have enough allowance to transfer this much".to_string(),
                );
            }
            *allowance -= amount.clone();
        } else if caller != ic_cdk::id() {
            // Allow canister itself to make transfers without restrictions
            return Err("Oops! You don't have permission to transfer".to_string());
        }

        if from_account.balance < amount {
            return Err("Oops! Not enough tokens in the piggy bank".to_string());
        }
        from_account.balance -= amount.clone();
        let to_account = token.accounts.entry(to).or_insert_with(|| Account {
            owner: to,
            balance: Nat::from(0u64),
            allowances: HashMap::new(),
        });
        to_account.balance += amount;
        Ok(())
    })
}

// This tells us how many tokens exist in total
#[ic_cdk::query]
fn total_supply() -> Nat {
    TOKEN.with(|token| token.borrow().total_supply.clone())
}

// This tells us how many tokens are in someone's piggy bank
#[ic_cdk::query]
fn balance_of(account: Principal) -> Nat {
    TOKEN.with(|token| {
        token
            .borrow()
            .accounts
            .get(&account)
            .map(|acc| acc.balance.clone())
            .unwrap_or_else(|| Nat::from(0u64))
    })
}

// This is a special function that helps us create a file to talk with our token system
#[test]
fn generate_candid() {
    candid::export_service!();

    std::fs::write("like_erc20_backend.did", __export_service())
        .expect("Candid interface file likhne mein fail ho gaya");
}
