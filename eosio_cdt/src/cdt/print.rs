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
