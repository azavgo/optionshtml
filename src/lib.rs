#[derive(Debug, Copy, Clone)]
pub enum OptionType {
    Put, 
    Call,
}

#[derive(Debug, Copy, Clone)]
pub enum TradeType {
    Bought, 
    Sold,
}

#[derive(Debug)]
pub struct Option {
    pub ticker: String,
    option: OptionType, 
    trade: TradeType,
    strike: f32, 
    days: u32, //days to expiration, when the option was taken 
    price: f32, //price of the option, when the option was taken
    fee: f32, //brokerage fee paid when the option was taken
}

impl Option {
    pub fn new(ticker: String,
               option: OptionType, 
               trade: TradeType, 
               strike: f32, 
               days: u32, 
               price: f32, 
               fee: f32) -> Self {

                Self{
                    ticker: ticker,
                    option: option, 
                    trade: trade,
                    strike: strike, 
                    days: days, 
                    price: price,
                    fee: fee 
                }
    }
  
    pub fn option(self: &Self) -> OptionType {
        self.option
    }

    pub fn trade(self: &Self) -> TradeType {
        self.trade
    }

    pub fn strike(self: &Self) -> f32 {
        self.strike
    }

    pub fn days(self: &Self) -> u32 {
        self.days
    }

    pub fn price(self: &Self) -> f32 {
        self.price
    }

    pub fn fee(self: &Self) -> f32 {
        self.fee
    }

    pub fn paid(self: &Self) -> f32 {
        match self.trade() {
            TradeType::Sold   => self.price() * 100.0 - self.fee(), 
            TradeType::Bought => - self.price() * 100.0 - self.fee(),
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn test_paid() {
        let option = Option::new("RIO".to_string(), OptionType::Put, TradeType::Sold, 100.0, 7, 0.5, 1.05);
        assert_eq!(48.95, option.paid());
    }
}
