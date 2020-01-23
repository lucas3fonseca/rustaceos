use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth};

#[derive(eos::Serialize, eos::Deserialize)]
pub struct UserAge {
    user: eos::Name,
    age: u16,
}

pub struct UserAgeTable<'a> {
    code: &'a eos::Name,
    scope: eos::Name,
    name: eos::Name,
    itr: Option<i32>,
    data: Option<&'a UserAge>,
}

impl<'a> UserAgeTable<'a> {
    pub fn new(code: &'a eos::Name, scope: eos::Name) -> Self {
        UserAgeTable {
            code,
            scope,
            name: eos::name_from_str("usersage"),
            itr: None,
            data: None,
        }
    }

    fn insert(&self, payer: &eos::Name) {
        check(self.data.is_some(), "the data of the table row is empty");
        let data = &self.data.unwrap();
        eosio_cdt::table_insert(&self.scope, &self.name, payer, data.user.value, data);
    }

    pub fn find(&mut self, id: u64) -> i32 {
        let itr = eosio_cdt::table_find(&self.code, &self.scope, &self.name, id);
        self.itr = Some(itr);
        itr
    }

    pub fn end(&self) -> i32 {
        eosio_cdt::table_end(&self.code, &self.scope, &self.name)
    }

    pub fn get(&mut self, id: u64) -> UserAge {
        let itr = self.find(id);
        check(itr != self.end(), "invalid record");
        eosio_cdt::table_get(itr)
    }

    pub fn remove(&mut self) {
        let itr = self.itr.expect("invalid iterator");
        eosio_cdt::table_remove(itr);
    }

    pub fn update(&mut self, payer: &eos::Name) {
        check(self.data.is_some(), "the data of the table row is empty");
        check(self.itr.is_some(), "the table iterator is invalid");
        eosio_cdt::table_update(payer, self.itr.unwrap(), self.data.unwrap());
    }
}

#[eosio_cdt::action]
pub fn signupage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    let mut user_age_table = UserAgeTable::new(contract.get_self(), eos::Name::new(0));
    check(
        user_age_table.find(name.value) == user_age_table.end(),
        "user already is already signed up",
    );

    let user_age = UserAge { user: name, age };
    user_age_table.data = Some(&user_age);
    user_age_table.insert(contract.get_self());

    accepted_message(user_age.user, age);
}

#[eosio_cdt::action]
pub fn updateage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    let mut user_age_table = UserAgeTable::new(contract.get_self(), eos::Name::new(0));
    let mut user_age = user_age_table.get(name.value);
    user_age.age = age;
    user_age_table.data = Some(&user_age);
    user_age_table.update(contract.get_self());

    accepted_message(name, age);
}

#[eosio_cdt::action]
pub fn signout(name: eos::Name) {
    require_auth(contract.get_self());

    let mut user_age_table = UserAgeTable::new(contract.get_self(), eos::Name::new(0));
    user_age_table.find(name.value);
    user_age_table.remove();
    print!("user ", name, " was removed successfully");
}

fn check_age(age: u16) {
    check(
        age >= 18,
        "OOpsss... Looks like the user does not have the required age",
    );
}

fn accepted_message(name: eos::Name, age: u16) {
    print!(
        "Hey, it looks like the user ",
        name, " is ", age, " years old. He/she is considered adult for our services."
    );
}
