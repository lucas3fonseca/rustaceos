use eosio_cdt::eos;

#[derive(eos::Serialize, eos::Deserialize)]
pub struct Account {
    name: eos::Name,
    likes: i32,
}

impl Account {
    pub fn new(name: eos::Name) -> Account {
        Account { name, likes: 0 }
    }
}

pub fn create_account(name: eos::Name) {
    let account = get_account(name.value);
    if (account.is_some()) {
        panic!("account already exists")
    }

    let new_account = Account::new(name);
    insert_account(new_account);
}

extern "C" {
    fn get_account(name: u64) -> Option<Account>;
    fn insert_account(account: Account);
}
