use timestampepoch::*;

mod options; 
use options::*;

fn main() { 
    let date = Date::new(25, 9, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let put = Put::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Sold);
    
    let date = Date::new(25, 9, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let call = Call::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Sold);

    let price_expansion = call.price_expansion();
    let theoretical_price = call.theoretical_price(Date::new(25, 10, 2022));
    dbg!(theoretical_price);
    dbg!(price_expansion);
}



//pub fn theoretical_profit_loss()