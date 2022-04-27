# optionshtml
## A library to estimate the outcome of an option trade

### Features: 
1. Calculates how much each option costs;
1. Calculates maximum profit and maximum loss of an Iron Condor strategy; 
1. Calculates break even points of an Iron Condor strategy;
1. Includes brokerage fees

### How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    optionshtml = {git = "https://github.com/azavgo/optionshtml", branch = "main"}
```
2. Generate an Iron Condor and calculate its parameters at expiration:  
```Rust
use optionshtml::*; 

fn main() {
    //Parameters of an option: Put or Call, Sold or Bought, Strike, Days to Expiration, Option price, Brokerage fee
    let a = Option::new(OptionType::Put, TradeType::Bought, 190.0, 12, 2.95, 1.05);
    let b = Option::new(OptionType::Put, TradeType::Sold, 200.0, 12, 4.65, 1.05);
    let c = Option::new(OptionType::Call, TradeType::Sold, 220.0, 12, 6.30, 1.05);
    let d = Option::new(OptionType::Call, TradeType::Bought, 230.0, 12, 3.75, 1.05);
    
    let strategy = IronCondor::new(a, b, c, d);  

    println!("Breakeven points: {:?}", strategy.break_even());
    println!("Reserve capital: {:?}", strategy.loss());
    println!("Maximum profit: {:?}", strategy.profit());
}  
```
