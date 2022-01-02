#[derive(Debug, Copy, Clone)]
pub enum OptionType {
    PutShort, 
    PutLong, 
    CallShort, 
    CallLong,
}

// if option_price > 0 - credit, else debit
#[derive(Debug, Copy, Clone)]
pub struct Option {
    option_type: OptionType, 
    strike: f32, 
    days_to_expiration: u32, 
    option_price: f32, 
}

impl Option {
    pub fn new(option_type: OptionType, 
               strike: f32, 
               days_to_expiration: u32, 
               option_price: f32) -> Self {

                Self{
                    option_type: option_type, 
                    strike: strike, 
                    days_to_expiration: days_to_expiration, 
                    option_price: option_price, 
                }
    }

    pub fn option_type(self: &Self) -> OptionType {
        self.option_type
    }

    pub fn strike(self: &Self) -> f32 {
        self.strike
    }

    pub fn days_to_expiration(self: &Self) -> u32 {
        self.days_to_expiration
    }

    pub fn option_price(self: &Self) -> f32 {
        self.option_price
    }

    pub fn calculate(self: &Self) -> f32 {
        match self.option_type() {
            OptionType::PutShort => self.option_price() * 100.0 / self.strike(), 
            OptionType::PutLong => 0.0,
            OptionType::CallShort => 0.0,
            OptionType::CallLong => 0.0,
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn test_calculate_put_short() {
        let option = Option::new(OptionType::PutShort, 64.0, 16, 0.44);
        assert_eq!(0.6875, Option::calculate(&option));
    }
}
