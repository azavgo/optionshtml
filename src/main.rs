use timestampepoch::*;

//use std::fs::*;

mod options; 
use options::*;

fn main() { 
    let date = Date::new(25, 9, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let put = Put::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Sold);
    
    let date = Date::new(25, 9, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let call = Call::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Sold);

    let date = Date::new(25, 9, 2022); 
    let expiration_date = Date::new(16, 12, 2022);
    let put_2 = Put::new("FTNT".to_string(), 49.19, 60.0, date, expiration_date, 0.385, 0.03, Trade::Sold);

    let mut strategy: Vec<Option> = Vec::new(); 
    //strategy.push(Option::Call(&call)); 
    strategy.push(Option::Put(&put));
    //strategy.push(Option::Put(&put_2)); 

    let price_expansion = put.price_expansion();
    let date = Date::new(25, 10, 2022);
    let profit_loss = strategy_profit_loss(strategy, date);
    
    profit_loss_file(price_expansion, profit_loss).unwrap();
    
}


//TODO: this macro does not work yet. Problem with recognising the correct arguments
#[macro_export]
macro_rules! strategy_builder {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            let call = "optionshtml::options::Call".to_string();
            let put = "optionshtml::options::Put".to_string();
            $(
                match type_of($x) {
                    call => temp_vec.push(Option::Call($x)),
                    put => temp_vec.push(Option::Put($x)),   
                }
                //temp_vec.push($x);
            )*
            temp_vec
        }
    };
}