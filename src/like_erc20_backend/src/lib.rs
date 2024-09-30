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
        // token = RefCell { value: Token { total_supply: 1000, max_supply: 10000, accounts: HashMap { ... } } }
        let mut token = token.borrow_mut();
        // token = Token { total_supply: 1000, max_supply: 10000, accounts: HashMap { ... } }
        
        if token.total_supply.clone() + amount.clone() > token.max_supply {
            // If 1000 + 500 > 10000 is not true, we continue
            return Err("Oops! We can't make more tokens than allowed".to_string());
        }
        let account = token.accounts.entry(to).or_insert_with(|| Account {
            owner: to,
            balance: Nat::from(0u64),
            allowances: HashMap::new(),
        });
        // account = Account { owner: "Alice", balance: 200, allowances: {} }
        
        account.balance += amount.clone();
        // account.balance = 200 + 500 = 700
        
        token.total_supply += amount;
        // token.total_supply = 1000 + 500 = 1500
        
        Ok(())
    })
}

// This is how we remove tokens from someone's piggy bank
#[ic_cdk::update]
fn burn(from: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        // token = RefCell { value: Token { total_supply: 1500, max_supply: 10000, accounts: HashMap { ... } } }
        let mut token = token.borrow_mut();
        // token = Token { total_supply: 1500, max_supply: 10000, accounts: HashMap { ... } }
        
        let account = token
            .accounts
            .get_mut(&from)
            .ok_or("Oops! We can't find this piggy bank")?;
        // account = Account { owner: "Alice", balance: 700, allowances: {} }
        
        if account.balance < amount {
            // If 700 < 300 is not true, we continue
            return Err("Oops! Not enough tokens in the piggy bank".to_string());
        }
        account.balance -= amount.clone();
        // account.balance = 700 - 300 = 400
        
        token.total_supply -= amount;
        // token.total_supply = 1500 - 300 = 1200
        
        Ok(())
    })
}

// This is how we let someone else use our tokens
#[ic_cdk::update]
fn approve(owner: Principal, spender: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        // token = RefCell { value: Token { total_supply: 1200, max_supply: 10000, accounts: HashMap { ... } } }
        let mut token = token.borrow_mut();
        // token = Token { total_supply: 1200, max_supply: 10000, accounts: HashMap { ... } }
        
        let account = token
            .accounts
            .get_mut(&owner)
            .ok_or("Oops! We can't find this piggy bank")?;
        // account = Account { owner: "Alice", balance: 400, allowances: {} }
        
        account.allowances.insert(spender, amount);
        // account.allowances = { "Bob": 100 }
        
        Ok(())
    })
}

// This tells us how many tokens someone can use from another person's piggy bank
#[ic_cdk::query]
fn allowance(owner: Principal, spender: Principal) -> Nat {
    TOKEN.with(|token| {
        // token = RefCell { value: Token { total_supply: 1200, max_supply: 10000, accounts: HashMap { ... } } }
        let token = token.borrow();
        // token = Token { total_supply: 1200, max_supply: 10000, accounts: HashMap { ... } }
        
        token
            .accounts
            .get(&owner)
            .and_then(|account| account.allowances.get(&spender))
            .cloned()
            .unwrap_or_else(|| Nat::from(0u64))
        // If "Alice" gave "Bob" permission to use 100 tokens, this will return 100
    })
}

// This is how we move tokens from one piggy bank to another
#[ic_cdk::update]
fn transfer(from: Principal, to: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        // token = RefCell { value: Token { accounts: HashMap { ... } } }

        let mut token = token.borrow_mut();
        /*
        token = Token { accounts: HashMap {
             "Alice": Account { owner: "Alice", balance: 100, allowances: {} },
             "Bob": Account { owner: "Bob", balance: 50, allowances: {} }
            } 
         }
         */

        let from_account = token
            .accounts
            .get_mut(&from)
            .ok_or("Sender account not found")?;
        // from_account = &mut Account { owner: "Alice", balance: 100, allowances: {} }

        if from_account.balance < amount {
            // 100 < 30 ? False, so we continue
            return Err("Insufficient balance".to_string());
        }

        from_account.balance -= amount.clone();
        // from_account.balance = 100 - 30 = 70

        let to_account = token.accounts.entry(to).or_insert_with(|| Account {
            owner: to,
            balance: Nat::from(0u64),
            allowances: HashMap::new(),
        });
        // to_account = &mut Account { owner: "Bob", balance: 50, allowances: {} }

        to_account.balance += amount;
        // to_account.balance = 50 + 30 = 80

        Ok(())
    })
}

// This tells us how many tokens exist in total
#[ic_cdk::query]
fn total_supply() -> Nat {
    TOKEN.with(|token| {
        // token = RefCell { value: Token { total_supply: 1200, max_supply: 10000, accounts: HashMap { ... } } }
        token.borrow().total_supply.clone()
        // This will return 1200
    })
}

// This tells us how many tokens are in someone's piggy bank
#[ic_cdk::query]
fn balance_of(account: Principal) -> Nat {
    TOKEN.with(|token| {
        // token = RefCell { value: Token { total_supply: 1200, max_supply: 10000, accounts: HashMap { ... } } }
        token
            .borrow()
            .accounts
            .get(&account)
            .map(|acc| acc.balance.clone())
            .unwrap_or_else(|| Nat::from(0u64))
        // If "Alice" has a balance of 400, this will return 400
    })
}

// This is a special function that helps us create a file to talk with our token system
#[test]
fn generate_candid() {
    candid::export_service!();

    std::fs::write("like_erc20_backend.did", __export_service())
        .expect("Failed to write Candid interface file");
}
