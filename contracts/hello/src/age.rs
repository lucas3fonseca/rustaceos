use eosio_cdt::eos;
use eosio_cdt::{check, print, require_auth, Table, TableIterator};

#[derive(eos::Serialize, eos::Deserialize)]
pub struct UserAge {
    user: eos::Name,
    age: u16,
}

impl Table for UserAge {
    fn new(code: eos::Name, scope: eos::Name) -> TableIterator {
        TableIterator::new(code, scope, eos::name_from_str("usersage"))
    }

    fn pk(&self) -> u64 {
        self.user.value
    }
}

#[eosio_cdt::action]
pub fn signupage(name: eos::Name, age: u16) {
    require_auth(&contract.get_self());
    check_age(age);

    let mut user_age_table = UserAge::new(contract.get_self().clone(), eos::Name::new(0));
    check(
        user_age_table.find(name.value) == user_age_table.end(),
        "user is already signed up",
    );

    let user_age = UserAge { user: name, age };
    user_age_table.insert(&contract.get_self(), &user_age);

    accepted_message(user_age.user, age);
}

#[eosio_cdt::action]
pub fn updateage(name: eos::Name, age: u16) {
    require_auth(&contract.get_self());
    check_age(age);

    let mut user_age_table = UserAge::new(contract.get_self().clone(), eos::Name::new(0));
    let mut user_age: UserAge = user_age_table.get(name.value);
    user_age.age = age;
    user_age_table.update(&contract.get_self(), &user_age);

    accepted_message(name, age);
}

#[eosio_cdt::action]
pub fn signout(name: eos::Name) {
    require_auth(&contract.get_self());

    let mut user_age_table = UserAge::new(contract.get_self().clone(), eos::Name::new(0));
    user_age_table.delete(name.value);
    print!("user ", name, " was removed successfully");
}

#[eosio_cdt::action]
pub fn advanceages() {
    require_auth(&contract.get_self());

    let mut user_age_table = UserAge::new(contract.get_self(), eos::Name::new(0));
    match user_age_table.begin() {
        Some(_) => {
            user_age_table.for_each(|itr| {
                let mut item: UserAge = itr.read();
                item.age += 1;
                itr.update(&contract.get_self(), &item);
                print!(" >>> ", item.user, " now is ", item.age);
            });
        }
        None => print!(" >>> No users are signed up"),
    }
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
