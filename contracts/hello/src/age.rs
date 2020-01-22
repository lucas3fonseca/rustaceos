use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth};

#[derive(eos::Serialize)]
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

    pub fn find(&self, id: u64) {
        let itr = Some(eosio_cdt::table_find(
            &self.code,
            &self.scope,
            &self.name,
            id,
        ));
    }

    pub fn end(&self) -> i32 {
        eosio_cdt::table_end(&self.code, &self.scope, &self.name)
    }
}

#[eosio_cdt::action]
pub fn signupage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    let user_age = UserAge { user: name, age };
    let mut user_age_table = UserAgeTable::new(contract.get_self(), eos::Name::new(0));
    user_age_table.data = Some(&user_age);
    user_age_table.insert(contract.get_self());

    accepted_message(user_age.user, age);
}

#[eosio_cdt::action]
pub fn updateage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    // let user_age = UserAge::find(name.value);
    // user_age.age = age;
    // user_age.update(contract.get_self());

    accepted_message(name, age);
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
