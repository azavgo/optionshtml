use std::any::{type_name, Any};
use statrs::distribution::{Normal, ContinuousCDF};
use timestampepoch::Date;

#[derive(Debug, Copy, Clone)]
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

    fn price(&self) -> f64;

    fn price_expansion(&self) -> Vec<f64> {
        let number_of_datapoints = 100;
        let min = 0.05 * self.price(); 
        let max = 1.95 * self.price(); 
        let increment = (1f64 / number_of_datapoints as f64) * (max - min);
        
        let mut price_expansion = vec![0f64; number_of_datapoints];
        for i in 1..number_of_datapoints {
          price_expansion[i] = price_expansion[i-1] + increment;
        }
        price_expansion
    }

    fn trade(&self) -> &Trade;

    fn theoretical_price(&self, date: Date) -> Vec<f64>;

    fn theoretical_profit_loss(&self, date: Date) -> Vec<f64> {
        let trade_final = match self.trade() {
            Trade::Bought => Trade::Sold,
            Trade::Sold => Trade::Bought,
        };
        
        let trade = self.trade();
    
        let theoretical_price = self.theoretical_price(date);
        let mut theoretical_profit_loss = vec![0f64; theoretical_price.len()];
    
        for i in 0..theoretical_price.len() {
            theoretical_profit_loss[i] = match (trade, trade_final) {
                (Trade::Sold, Trade::Bought) => self.price() - theoretical_price[i],
                (Trade::Bought, Trade::Sold) => - self.price() + theoretical_price[i],
                (Trade::Bought, Trade::Bought) => - self.price() - theoretical_price[i],
                (Trade::Sold, Trade::Sold) => self.price() + theoretical_price[i],
            }
        }
        theoretical_profit_loss
    }
}

#[derive(Debug)]
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

    pub fn strike(&self) -> f64 {
        self.strike
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }
}

impl Options for Call {
    fn date(&self) -> &Date {
        &self.date
    }

    fn expiration_date(&self) -> &Date {
        &self.expiration_date
    }

    fn price(&self) -> f64 {
        self.price
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

    fn trade(&self) -> &Trade {
        &self.trade
    }

    fn theoretical_price(&self, date: Date) -> Vec<f64> {
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

#[derive(Debug)]
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

    pub fn strike(&self) -> f64 {
        self.strike
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }
}

impl Options for Put {
    fn date(&self) -> &Date {
        &self.date
    }

    fn expiration_date(&self) -> &Date {
        &self.expiration_date
    }

    fn price(&self) -> f64 {
        self.price
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

    fn trade(&self) -> &Trade {
        &self.trade
    }

    fn theoretical_price(&self, date: Date) -> Vec<f64> {
        let name = self.name();
        let price_expansion = self.price_expansion();
        let mut theoretical_price = vec![0f64; price_expansion.len()];
        for i in 0..price_expansion.len() {
            let put = Put::new(name.to_string(), price_expansion[i], self.strike(), date, *self.expiration_date(), self.sigma(), self.rate(), Trade::Bought);
            theoretical_price[i] = put.option_price();
        }
        theoretical_price
    }
}

#[derive(Debug)]
pub enum Option <'a> {
    Call(&'a Call), 
    Put(&'a Put),
}

pub fn strategy_profit_loss(strategy: Vec<Option>, date: Date) -> Vec<f64> {
    let mut theoretical_strategy: Vec<Vec<f64>> = Vec::new();
    for i in 0..strategy.len() {
        match strategy[i] {
            Option::Call(call) => theoretical_strategy.push(call.theoretical_profit_loss(date)),
            Option::Put(put) => theoretical_strategy.push(put.theoretical_profit_loss(date)),
        }
    }
    let mut profit_loss = vec![0f64; theoretical_strategy[0].len()];
    for i in 0..profit_loss.len() {
        for j in 0..theoretical_strategy.len() {
            profit_loss[i] += theoretical_strategy[j][i];
        }
    }
    profit_loss
}

//Returns the type name of T as String. Useful to return a CALL or a PUT type
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

//Writes strategy profit and loss data to file in a format suitable to GnuPlot
pub fn profit_loss_file(price_expansion: Vec<f64>, profit_loss: Vec<f64>) -> Result<(), std::io::Error> {
    let mut profit_loss_string = vec!["".to_string(); profit_loss.len()]; 
    
    for i in 0..profit_loss_string.len() {
        profit_loss_string[i] = format!("{}, {}\n", price_expansion[i], profit_loss[i]);
    }
    
    let file_path = "profit_loss.txt";
    std::fs::write(file_path, profit_loss_string.join(""))?;     
    
    Ok(())
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