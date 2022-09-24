use statrs::distribution::{Normal, ContinuousCDF};
use chrono::{Utc, TimeZone};

pub fn option_price(s: f64, k: f64, t: f64, sigma: f64, r: f64) -> f64 {
    let d1 = (1.0 / (sigma * t.sqrt())) * ((s/k).log10() + (r + 0.5 * sigma * sigma) * t);
    let d2 = d1 - sigma * t.sqrt();
    let pv = k * (-r*t).exp();
    //N - standard normal cumulative distribution function
    let n = Normal::new(0.0, 1.0).unwrap();
    let n1 = n.cdf(d1);
    let n2 = n.cdf(d2); 
    
    let c = n1 * s - n2 * pv;
    c
}

//returns time to expration from the date in years
pub fn time_to_expiration(date: String, expiration_date: String) -> f64 {
    let dt = Utc.ymd(2022, 9, 24).and_hms_milli(0, 0, 0, 0);
    let date = dt.timestamp();
    unimplemented!();
}

fn main() {
    
    let s = 100.0;
    let k = 95.0; 
    let t = 0.25; 
    let sigma = 0.5; 
    let r = 0.01; 
    println!("Option price = ${:.2}", option_price(s, k, t, sigma, r));
    println!("The correct answer is $12.5279");

    
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_01() {
        assert_eq!(905.0, 905.0);
    }


}
