use timestampepoch::*;

mod options; 
use options::{Call, Put, Trade, Options};


fn main() { 
    let date = Date::new(25, 9, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let put = Put::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Bought);
    
    let date = Date::new(25, 9, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let call = Call::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Bought);
    
    println!("CALL option price = ${:.2}", call.option_price());
    println!("PUT option price = ${:.2}", put.option_price());

    let date = Date::new(25, 10, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let put_1 = Put::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Bought);
    
    let date = Date::new(25, 10, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let call_1 = Call::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Bought);
    
    println!("CALL_1 option price = ${:.2}", call_1.option_price());
    println!("PUT_1 option price = ${:.2}", put_1.option_price());

    println!("CALL profit: {:.2}", call_profit(&call, &call_1));
    println!("PUT profit: {:.2}", put_profit(&put, &put_1));

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
