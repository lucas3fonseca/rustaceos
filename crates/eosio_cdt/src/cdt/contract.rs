use crate::eos;

pub struct Contract {
    pub contract: eos::Name,
    pub code: eos::Name,
    pub data_stream: Option<Vec<u8>>,
}

impl Contract {
    pub fn new(contract: eos::Name, code: eos::Name) -> Self {
        Contract {
            contract,
            code,
            data_stream: None,
        }
    }

    pub fn set_data_stream(&mut self, data_stream: Vec<u8>) {
        self.data_stream = Some(data_stream);
    }

    pub fn is_contract_code(&self) -> bool {
        self.contract.value == self.code.value
    }

    pub fn get_self(&self) -> eos::Name {
        self.contract.clone()
    }

    pub fn get_code(&self) -> eos::Name {
        self.code.clone()
    }
}
