use crate::eos;
use crate::eosio_cdt_bindings;

pub trait Print {
    fn print(&self);
}

impl<'a> Print for &'a str {
    fn print(&self) {
        let ptr = self.as_ptr();
        let len = self.len() as u32;
        unsafe { eosio_cdt_bindings::prints_l(ptr, len) };
    }
}

impl Print for eos::Name {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printn(self.value) };
    }
}

impl Print for u8 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printui(*self as u64) };
    }
}

impl Print for u16 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printui(*self as u64) };
    }
}

impl Print for u32 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printui(*self as u64) };
    }
}

impl Print for u64 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printui(*self) };
    }
}

impl Print for i8 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printi(*self as i64) };
    }
}

impl Print for i16 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printi(*self as i64) };
    }
}

impl Print for i32 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printi(*self as i64) };
    }
}

impl Print for i64 {
    fn print(&self) {
        unsafe { eosio_cdt_bindings::printi(*self) };
    }
}

#[macro_export]
macro_rules! print {
    ( $( $x:expr ),+ ) => {
        {
            $(
                $crate::Print::print(&$x);
            )+
        }
    };
}
