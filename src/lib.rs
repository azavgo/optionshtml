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

#[derive(Debug, Copy, Clone)]
pub struct Option {
    option: OptionType, 
    trade: TradeType,
    strike: f32, 
    days: u32, //days to expiration, when the option was taken 
    price: f32, //price of the option, when the option was taken
    fee: f32, //brokerage fee paid when the option was taken
}

impl Option {
    pub fn new(option: OptionType, 
                trade: TradeType, 
                strike: f32, 
                days: u32, 
                price: f32, 
                fee: f32) -> Self {
               

                Self{
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

#[derive(Debug, Copy, Clone)]
pub struct IronCondor {
    a: Option, 
    b: Option, 
    c: Option, 
    d: Option, 
}

impl IronCondor {
    pub fn new(a: Option, b: Option, c: Option, d: Option) -> Self {
       Self {
           a: a, 
           b: b, 
           c: c, 
           d: d,
       } 
    }

    pub fn a(self: &Self) -> Option {
        self.a
    }

    pub fn b(self: &Self) -> Option {
        self.b
    }

    pub fn c(self: &Self) -> Option {
        self.c
    }

    pub fn d(self: &Self) -> Option {
        self.d
    }

    pub fn profit(self: &Self) -> f32 {
        self.a().paid() + self.b().paid() + self.c().paid() + self.d().paid()
    }

    pub fn loss(self: &Self) -> f32 {
        (self.a().strike() - self.b().strike()) * 100.0 + self.profit()
    }

    pub fn break_even(self: &Self) -> (f32, f32) {
        let be_1: f32 = self.a().strike() - (self.b().strike() - self.a().strike()) * self.loss() / (self.profit() - self.loss());
        let be_2: f32 = self.c().strike() + (self.d().strike() - self.c().strike()) * self.profit() / (self.profit() - self.loss());
        (be_1, be_2)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BullPutSpread {
    a: Option, 
    b: Option,  
}

impl BullPutSpread {
    pub fn new(a: Option, b: Option) -> Self {
        Self {a: a, b: b}
    }

    pub fn a(self: &Self) -> Option {
        self.a
    }

    pub fn b(self: &Self) -> Option {
        self.b
    }

    pub fn profit(self: &Self) -> f32 {
        self.a().paid() + self.b().paid()
    }

    pub fn loss(self: &Self) -> f32 {
        (self.a().strike() - self.b().strike()) * 100.00 + self.profit()
    }

    pub fn break_even(self: &Self) -> f32 {
        self.a().strike() - (self.b().strike() - self.a().strike()) * self.loss() / (self.profit() - self.loss())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BearPutSpread {
    a: Option, 
    b: Option,  
}

impl BearPutSpread {
    pub fn new(a: Option, b: Option) -> Self {
        Self {a: a, b: b}
    }

    pub fn a(self: &Self) -> Option {
        self.a
    }

    pub fn b(self: &Self) -> Option {
        self.b
    }

    pub fn profit(self: &Self) -> f32 {
        (self.b().strike() - self.a().strike()) * 100.00 + self.loss()
    }

    pub fn loss(self: &Self) -> f32 {
        self.a().paid() + self.b().paid()
    }

    pub fn break_even(self: &Self) -> f32 {
        self.a().strike() + (self.b().strike() - self.a().strike()) * self.profit() / (self.profit() - self.loss())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BullCallSpread {
    a: Option, 
    b: Option,  
}

impl BullCallSpread {
    pub fn new(a: Option, b: Option) -> Self {
        Self {a: a, b: b}
    }

    pub fn a(self: &Self) -> Option {
        self.a
    }

    pub fn b(self: &Self) -> Option {
        self.b
    }

    pub fn profit(self: &Self) -> f32 {
        (self.b().strike() - self.a().strike()) * 100.00 + self.loss()
    }

    pub fn loss(self: &Self) -> f32 {
        self.a().paid() + self.b().paid()
    }

    pub fn break_even(self: &Self) -> f32 {
        self.a().strike() - (self.b().strike() - self.a().strike()) * self.loss() / (self.profit() - self.loss())    
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BearCallSpread {
    a: Option, 
    b: Option,  
}

impl BearCallSpread {
    pub fn new(a: Option, b: Option) -> Self {
        Self {a: a, b: b}
    }

    pub fn a(self: &Self) -> Option {
        self.a
    }

    pub fn b(self: &Self) -> Option {
        self.b
    }

    pub fn profit(self: &Self) -> f32 {
        self.a().paid() + self.b().paid()
    }

    pub fn loss(self: &Self) -> f32 {
        (self.a().strike() - self.b().strike()) * 100.00 + self.profit()
    }

    pub fn break_even(self: &Self) -> f32 {
        self.a().strike() + (self.b().strike() - self.a().strike()) * self.profit() / (self.profit() - self.loss())    
    }
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_bearcallspread_profit_1() {
        let a = Option::new(OptionType::Call, TradeType::Sold, 144.0, 12, 14.60, 0.0);
        let b = Option::new(OptionType::Call, TradeType::Bought, 157.5, 12, 5.55, 0.0);
        let strategy = BearCallSpread::new(a, b); 
        assert_eq!(905.0, strategy.profit());
    }

    #[test]
    fn test_bearcallspread_loss_1() {
        let a = Option::new(OptionType::Call, TradeType::Sold, 144.0, 12, 14.60, 0.0);
        let b = Option::new(OptionType::Call, TradeType::Bought, 157.5, 12, 5.55, 0.0);
        let strategy = BearCallSpread::new(a, b); 
        assert_eq!(-445.0, strategy.loss());
    }

    #[test]
    fn test_bearcallspread_breakeven_1() {
        let a = Option::new(OptionType::Call, TradeType::Sold, 144.0, 12, 14.60, 0.0);
        let b = Option::new(OptionType::Call, TradeType::Bought, 157.5, 12, 5.55, 0.0);
        let strategy = BearCallSpread::new(a, b); 
        assert_eq!(153.05, strategy.break_even());
    }

    #[test]
    fn test_bullcallspread_profit_1() {
        let a = Option::new(OptionType::Call, TradeType::Bought, 157.5, 12, 5.55, 0.0);
        let b = Option::new(OptionType::Call, TradeType::Sold, 170.0, 12, 1.07, 0.0);
        let strategy = BullCallSpread::new(a, b); 
        assert_eq!(802.0, strategy.profit());
    }

    #[test]
    fn test_bullcallspread_loss_1() {
        let a = Option::new(OptionType::Call, TradeType::Bought, 157.5, 12, 5.55, 0.0);
        let b = Option::new(OptionType::Call, TradeType::Sold, 170.0, 12, 1.07, 0.0);
        let strategy = BullCallSpread::new(a, b); 
        assert_eq!(-448.0, strategy.loss());
    }

    #[test]
    fn test_bullcallspread_breakeven_1() {
        let a = Option::new(OptionType::Call, TradeType::Bought, 157.5, 12, 5.55, 0.0);
        let b = Option::new(OptionType::Call, TradeType::Sold, 170.0, 12, 1.07, 0.0);
        let strategy = BullCallSpread::new(a, b); 
        assert_eq!(161.98, strategy.break_even());
    }

    #[test]
    fn test_bearputspread_profit_1() {
        let a = Option::new(OptionType::Put, TradeType::Sold, 145.0, 12, 2.18, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Bought, 155.0, 12, 5.25, 0.0);
        let strategy = BearPutSpread::new(a, b); 
        assert_eq!(693.0, strategy.profit());
    }

    #[test]
    fn test_bearputspread_loss_1() {
        let a = Option::new(OptionType::Put, TradeType::Sold, 145.0, 12, 2.18, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Bought, 155.0, 12, 5.25, 0.0);
        let strategy = BearPutSpread::new(a, b); 
        assert_eq!(-307.0, strategy.loss());
    }

    #[test]
    fn test_bearputspread_break_even_1() {
        let a = Option::new(OptionType::Put, TradeType::Sold, 145.0, 12, 2.18, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Bought, 155.0, 12, 5.25, 0.0);
        let strategy = BearPutSpread::new(a, b); 
        assert_eq!(151.93, strategy.break_even());
    }

    #[test]
    fn test_bullputspread_profit_1() {
        let a = Option::new(OptionType::Put, TradeType::Bought, 144.0, 12, 2.05, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Sold, 157.5, 12, 6.25, 0.0);
        let strategy = BullPutSpread::new(a, b); 
        assert_eq!(420.0, strategy.profit());
    }

    #[test]
    fn test_bullputspread_loss_1() {
        let a = Option::new(OptionType::Put, TradeType::Bought, 144.0, 12, 2.05, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Sold, 157.5, 12, 6.25, 0.0);
        let strategy = BullPutSpread::new(a, b); 
        assert_eq!(-930.0, strategy.loss());
    }

    #[test]
    fn test_bullputspread_breakeven_1() {
        let a = Option::new(OptionType::Put, TradeType::Bought, 144.0, 12, 2.05, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Sold, 157.5, 12, 6.25, 0.0);
        let strategy = BullPutSpread::new(a, b); 
        assert_eq!(153.30, strategy.break_even());
    }

    #[test]
    fn test_option_paid() {
        let option = Option::new(OptionType::Put, TradeType::Sold, 100.0, 7, 0.5, 1.05);
        assert_eq!(48.95, option.paid());
    }

    #[test]
    fn test_ironcondor_profit() {
        let a = Option::new(OptionType::Put, TradeType::Bought, 65.0, 12, 0.26, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Sold, 72.5, 12, 1.72, 0.0);
        let c = Option::new(OptionType::Call, TradeType::Sold, 76.5, 12, 1.77, 0.0);
        let d = Option::new(OptionType::Call, TradeType::Bought, 84.0, 12, 0.42, 0.0);
        let strategy = IronCondor::new(a, b, c, d); 
        assert_eq!(281.0, strategy.profit());
    }

    #[test]
    fn test_ironcondor_loss() {
        let a = Option::new(OptionType::Put, TradeType::Bought, 65.0, 12, 0.26, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Sold, 72.5, 12, 1.72, 0.0);
        let c = Option::new(OptionType::Call, TradeType::Sold, 76.5, 12, 1.77, 0.0);
        let d = Option::new(OptionType::Call, TradeType::Bought, 84.0, 12, 0.42, 0.0);
        let strategy = IronCondor::new(a, b, c, d); 
        assert_eq!(-469.0, strategy.loss());
    }

    #[test]
    fn test_ironcondor_breakeven_1() {
        let a = Option::new(OptionType::Put, TradeType::Bought, 65.0, 12, 0.26, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Sold, 72.5, 12, 1.72, 0.0);
        let c = Option::new(OptionType::Call, TradeType::Sold, 76.5, 12, 1.77, 0.0);
        let d = Option::new(OptionType::Call, TradeType::Bought, 84.0, 12, 0.42, 0.0);
        let strategy = IronCondor::new(a, b, c, d); 
        assert_eq!((69.69, 79.31), strategy.break_even());
    }

    #[test]
    fn test_ironcondor_breakeven_2() {
        let a = Option::new(OptionType::Put, TradeType::Bought, 190.0, 12, 2.95, 0.0);
        let b = Option::new(OptionType::Put, TradeType::Sold, 200.0, 12, 4.65, 0.0);
        let c = Option::new(OptionType::Call, TradeType::Sold, 220.0, 12, 6.30, 0.0);
        let d = Option::new(OptionType::Call, TradeType::Bought, 230.0, 12, 3.75, 0.0);
        let strategy = IronCondor::new(a, b, c, d); 
        assert_eq!((195.75, 224.25), strategy.break_even());
    }

}
