use clap::Parser;
// use std::path::PathBuf;

// --- enums
enum AccountTypes {
    Credit(String),
    Debit(String),
}

// --- structs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // module: String,

    #[arg(short, long)]
    acc_credit: String,

    #[arg(short, long)]
    acc_debit: String,

    #[arg(short, long, default_value_t = 1)]
    value: u8,
}

struct Transaction {
    account_set: crate::AccountTypes,
    value: u8,
}

// --- functions
fn greeting(name: String ) -> String {
    format!("Hello, {}!", name)
}

// --- core app
fn main() {
    println!("Hello, world!");
}

// --- TDD
#[test]
fn create_transaction_test() {
    let credit_account: String = String::from("UtiliTieS");
    let debit_account: String = String::from("CasH");
    let xfer_amount: f16 = 42.42;

    let want: Transaction = {Transaction {account_set: {}, value : xfer_amount}};
    
    let result = create_transaction(credit_account, debit_account, xfer_amount);
    assert_eq!(want, result);
}

// ----- end of file -----