use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth};

#[derive(eos::Serialize)]
pub struct UserAge {
    user: eos::Name,
    age: u16,
}

impl UserAge {
    fn insert(&self, payer: &eos::Name) {
        let table_name = eos::name_from_str("usersage");
        let scope = eos::Name::new(0);
        eosio_cdt::table_insert(&scope, &table_name, payer, self.user.value, &self);
    }
}

#[eosio_cdt::action]
pub fn checkage(name: eos::Name, age: u16) {
    require_auth(&name);
    check(
        age >= 18,
        "OOpsss... Looks like you don't have the required age",
    );

    let user_age = UserAge {
        user: name,
        age,
    };
    user_age.insert(contract.get_self());

    print!(
        "Hey ", user_age.user, ", looks like you have ",
        age, ".You are considered adult for our services."
    );
}
