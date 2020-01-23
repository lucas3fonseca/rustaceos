use eosio_cdt;
use eosio_cdt::eos;

mod age;
mod hello;

use age::*;
use hello::*;

eosio_cdt::abi!(hi, hello, checkage);
