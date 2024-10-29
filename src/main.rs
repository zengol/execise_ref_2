#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}




fn set_account_id(account: &mut Account, id: u32) {
    account.id = id;
    println!("account id: {}",account.id)

}

 
fn set_account_holder(account:&mut Account, name: String){

    account.holder = name;
    println!("account holder: {}", account.holder)


}

/* One of your fellow engineers has written the following code. 
They claim they had to make the set_account_id and set_account_holder functions return the account 
so some errors around ownership would go away. */
fn main() {
    let mut account = Account::new(1, String::from("me"));
 
    set_account_id(&mut account, 3);

    set_account_holder(&mut account, String::from("you"));
 
   
    println!("{:#?}", account);
}
