use statrs::distribution::{Normal, ContinuousCDF};
use timestampepoch::Date;

pub enum Trade {
    Bought, 
    Sold,
}

pub trait Options {
    fn date(&self) -> &Date;
    fn expiration_date(&self) -> &Date;
    
    //Returns time to expiration from the date in years
    fn time_to_expiration(&self) -> f64 {
        let timestamp_date = self.date().timestamp().unwrap(); 
        let timestamp_expiration_date = self.expiration_date().timestamp().unwrap();
        // Number of seconds in a common year = 31536000 
        // Number of seconds in a leap year  = 31622400 
        //TODO: handle the case when option expires before the date
        //TODO: take into account differences between leap year and a normal year
        //TODO: verify whether I have to take into account all days or only 252 trading days
        (timestamp_expiration_date as f64 - timestamp_date as f64)/(31536000f64)
        }
        
    fn option_price(&self) -> f64;

}

pub struct Call {
    name: String, 
    price: f64, 
    strike: f64,
    date: Date, 
    expiration_date: Date,
    sigma: f64,
    rate: f64,
    trade: Trade,
}

impl Call {
    pub fn new(name: String, price: f64, strike: f64, date: Date, expiration_date: Date, sigma: f64, rate: f64, trade: Trade) -> Self {
        Self { name, price, strike, date, expiration_date, sigma, rate, trade }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn strike(&self) -> f64 {
        self.strike
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn trade(&self) -> &Trade {
        &self.trade
    }

    pub fn price_expansion(&self) -> Vec<f64> {
        let number_of_datapoints = 10;
        let min = 0.05 * self.price(); 
        let max = 1.95 * self.price(); 
        let increment = (1f64 / number_of_datapoints as f64) * (max - min);
        
        let mut price_expansion = vec![0f64; number_of_datapoints];
        for i in (1..number_of_datapoints) {
          price_expansion[i] = price_expansion[i-1] + increment;
        }
        price_expansion
    }

    pub fn theoretical_price(&self, date: Date) -> Vec<f64> {
        let name = self.name();
        let price_expansion = self.price_expansion();
        let mut theoretical_price = vec![0f64; price_expansion.len()];
        for i in 0..price_expansion.len() {
            let call = Call::new(name.to_string(), price_expansion[i], self.strike(), date, *self.expiration_date(), self.sigma(), self.rate(), Trade::Bought);
            theoretical_price[i] = call.option_price();
        }
        theoretical_price
    }
   
}


impl Options for Call {
    fn date(&self) -> &Date {
        &self.date
    }

    fn expiration_date(&self) -> &Date {
        &self.expiration_date
    }

    //calculates the price of the CALL option - Black-Sholes model implementation
    fn option_price(&self) -> f64 {
        let t = self.time_to_expiration();
        let d1 = (1.0 / (self.sigma() * t.sqrt())) * ((self.price()/self.strike()).ln() + (self.rate() + 0.5 * self.sigma() * self.sigma()) * t);
        let d2 = d1 - self.sigma() * t.sqrt();
        let pv = self.strike() * (-self.rate()*t).exp();
        //n - standard normal cumulative distribution function
        //TODO: handle Error properly
        let n = Normal::new(0.0, 1.0).unwrap();
        let n1 = n.cdf(d1);
        let n2 = n.cdf(d2); 
        let c = n1 * self.price() - n2 * pv;
        c
    }
}

pub struct Put {
    name: String, 
    price: f64, 
    strike: f64,
    date: Date, 
    expiration_date: Date,
    sigma: f64,
    rate: f64,
    trade: Trade,
}

impl Put {
    pub fn new(name: String, price: f64, strike: f64, date: Date, expiration_date: Date, sigma: f64, rate: f64, trade: Trade) -> Self {
        Self { name, price, strike, date, expiration_date, sigma, rate, trade }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn strike(&self) -> f64 {
        self.strike
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn trade(&self) -> &Trade {
        &self.trade
    }
}

impl Options for Put {
    fn date(&self) -> &Date {
        &self.date
    }

    fn expiration_date(&self) -> &Date {
        &self.expiration_date
    }

    //calculates the price of the CALL option - Black-Sholes model implementation
    fn option_price(&self) -> f64 {
        let t = self.time_to_expiration();
        let d1 = (1.0 / (self.sigma() * t.sqrt())) * ((self.price()/self.strike()).ln() + (self.rate() + 0.5 * self.sigma() * self.sigma()) * t);
        let d2 = d1 - self.sigma() * t.sqrt();
        let pv = self.strike() * (-self.rate()*t).exp();
        //n - standard normal cumulative distribution function
        //TODO: handle Error properly
        let n = Normal::new(0.0, 1.0).unwrap();
        let n1 = n.cdf(-d1);
        let n2 = n.cdf(-d2); 
        let p = -n1 * self.price() + n2 * pv;
        p
    }
}

pub fn call_profit(call: &Call, call_final: &Call) -> f64 {
    let trade = call.trade();
    let trade_final = call_final.trade();
    
    let price = call.option_price(); 
    let price_final = call_final.option_price();
    match (trade, trade_final) {
        (Trade::Sold, Trade::Bought) => price - price_final,
        (Trade::Bought, Trade::Sold) => - price + price_final,
        (Trade::Bought, Trade::Bought) => - price - price_final,
        (Trade::Sold, Trade::Sold) => price + price_final,
    }
}

pub fn put_profit(put: &Put, put_final: &Put) -> f64 {
    let trade = put.trade();
    let trade_final = put_final.trade();
    
    let price = put.option_price(); 
    let price_final = put_final.option_price();
    match (trade, trade_final) {
        (Trade::Sold, Trade::Bought) => price - price_final,
        (Trade::Bought, Trade::Sold) => - price + price_final,
        (Trade::Bought, Trade::Bought) => - price - price_final,
        (Trade::Sold, Trade::Sold) => price + price_final,
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_01() {
        
        assert_eq!(0, 0);
    }
    
    #[test]
    fn test_call_option_price_01() {
        let date = Date::new(25, 9, 2022); 
        let expiration_date = Date::new(16, 12, 2022);
        let call = Call::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Sold);
        assert_eq!(format!("{:.2}", 0.74), format!("{:.2}", call.option_price()));
    }

    #[test]
    fn test_put_option_price_01() {
        let date = Date::new(25, 9, 2022); 
        let expiration_date = Date::new(16, 12, 2022);
        let put = Put::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Sold);
        assert_eq!(format!("{:.2}", 11.15), format!("{:.2}", put.option_price()));
    }
    
}