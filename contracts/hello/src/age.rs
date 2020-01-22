use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth};

#[derive(eos::Serialize)]
pub struct UserAge {
    user: eos::Name,
    age: u16,
}

impl UserAge {
    const NAME: eos::Name = eos::name_from_str("usersage");
    const SCOPE: eos::Name = eos::name_from_str("");

    fn insert(&self, payer: &eos::Name) {
        eosio_cdt::table_insert(&UserAge::SCOPE, &UserAge::NAME, payer, self.user.value, &self);
    }

    pub fn find(id: u64) -> UserAge {
        // eosio_cdt::table_find()
        UserAge { user: eos::Name::new(1), age: 99 }
    }
}

#[eosio_cdt::action]
pub fn signupage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    let user_age = UserAge {
        user: name,
        age,
    };
    user_age.insert(contract.get_self());

    accepted_message(user_age.user, age);
}

#[eosio_cdt::action]
pub fn updateage(name: eos::Name, age: u16) {
    require_auth(contract.get_self());
    check_age(age);

    let user_age = UserAge::find(name.value);
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
        "Hey, it looks like the user ", name, " is ",
        age, " years old. He/she is considered adult for our services."
    );
}
