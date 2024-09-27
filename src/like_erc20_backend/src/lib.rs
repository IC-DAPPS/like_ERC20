use candid::{CandidType, Nat, Principal};
use std::collections::HashMap;

#[derive(CandidType, Clone)]
struct Account {
    owner: Principal,
    balance: Nat,
    allowances: HashMap<Principal, Nat>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
            balance: Nat::from(0u64),
            allowances: HashMap::new(),
        }
    }
}
#[derive(Default)]
struct Token {
    total_supply: Nat,
    max_supply: Nat,
    accounts: HashMap<Principal, Account>,
}

thread_local! {
    static TOKEN: std::cell::RefCell<Token> = std::cell::RefCell::new(Token::default());
}

#[ic_cdk::init]
fn init(max_supply: Nat) {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        token.max_supply = max_supply;
    });
}

#[ic_cdk::update]
fn mint(to: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        if token.total_supply.clone() + amount.clone() > token.max_supply {
            return Err("Exceeds max supply".to_string());
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

#[ic_cdk::update]
fn burn(from: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let account = token.accounts.get_mut(&from).ok_or("Account not found")?;
        if account.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        account.balance -= amount.clone();
        token.total_supply -= amount;
        Ok(())
    })
}

#[ic_cdk::update]
fn approve(owner: Principal, spender: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let account = token.accounts.get_mut(&owner).ok_or("Account not found")?;
        account.allowances.insert(spender, amount);
        Ok(())
    })
}

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

#[ic_cdk::update]
fn transfer(from: Principal, to: Principal, amount: Nat) -> Result<(), String> {
    TOKEN.with(|token| {
        let mut token = token.borrow_mut();
        let from_account = token
            .accounts
            .get_mut(&from)
            .ok_or("Sender account not found")?;
        if from_account.balance < amount {
            return Err("Insufficient balance".to_string());
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

#[ic_cdk::query]
fn total_supply() -> Nat {
    TOKEN.with(|token| token.borrow().total_supply.clone())
}

#[ic_cdk::query]
fn max_supply() -> Nat {
    TOKEN.with(|token| token.borrow().max_supply.clone())
}

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

#[test]
fn generate_candid() {
    candid::export_service!();

    std::fs::write("like_erc20_backend.did", __export_service())
        .expect("Candid interface file likhne mein fail ho gaya");
}
