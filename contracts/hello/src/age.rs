use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth};

#[derive(eos::Serialize, eos::Deserialize)]
pub struct UserAge {
    user: eos::Name,
    age: u16,
}

pub struct UserAgeTable {
    code: eos::Name,
    scope: eos::Name,
    name: eos::Name,
    itr: Option<i32>,
}

impl UserAgeTable {
    pub fn new(code: eos::Name, scope: eos::Name) -> Self {
        UserAgeTable {
            code,
            scope,
            name: eos::name_from_str("usersage"),
            itr: None,
        }
    }

    pub fn insert(&self, payer: &eos::Name, data: &UserAge) {
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

    pub fn update(&mut self, payer: &eos::Name, data: &UserAge) {
        let itr = eosio_cdt::expect(self.itr, "iterator is not valid to update");
        eosio_cdt::table_update(payer, itr, data);
    }

    pub fn delete(&mut self, id: u64) {
        let itr = self.find(id);
        check(itr != self.end(), "invalid record to delete");
        self.erase();
    }

    pub fn erase(&mut self) {
        let itr = eosio_cdt::expect(self.itr, "iterator is not valid to delete");
        eosio_cdt::table_remove(itr);
        self.itr = Some(itr + 1);
    }
}

#[eosio_cdt::action]
pub fn signupage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    let mut user_age_table = UserAgeTable::new(contract.get_self().clone(), eos::Name::new(0));
    check(
        user_age_table.find(name.value) == user_age_table.end(),
        "user is already signed up",
    );

    let user_age = UserAge { user: name, age };
    user_age_table.insert(contract.get_self(), &user_age);

    accepted_message(user_age.user, age);
}

#[eosio_cdt::action]
pub fn updateage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    let mut user_age_table = UserAgeTable::new(contract.get_self().clone(), eos::Name::new(0));
    let mut user_age = user_age_table.get(name.value);
    user_age.age = age;
    user_age_table.update(contract.get_self(), &user_age);

    accepted_message(name, age);
}

#[eosio_cdt::action]
pub fn signout(name: eos::Name) {
    require_auth(contract.get_self());

    let mut user_age_table = UserAgeTable::new(contract.get_self().clone(), eos::Name::new(0));
    user_age_table.delete(name.value);
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
