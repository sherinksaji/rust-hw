use crate::location::{Country, Continent};
use chrono::NaiveDate;
use std::str::FromStr;
use std::error::Error;


#[derive(Debug)]
pub struct Transaction {
    pub transaction_id: u32,
    pub client_id: u32,
    pub asset_name: String,
    pub country: Country,
    pub continent: Continent,
    pub amount: f64,
    pub days_under_management: i64,
}


impl Transaction {
  pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
      // Step a: Split the line by comma and collect into a Vec of string slices
      let fields: Vec<&str> = line.split(',').collect();

      // Step b: Check if the length of fields is equal to 7
      if fields.len() != 7 {
          return Err(String::from("Invalid number of fields in CSV line"));
      }

      // Step c: Parse transaction_id
      let transaction_id: u32 = fields[0].parse().unwrap();

      // Step d: Parse client_id
      let client_id: u32 = fields[1].parse().unwrap();

      // Step e: Get asset_name in uppercase
      let asset_name = fields[2].to_uppercase();

       // Step f: Parse transaction_start_date
       let transaction_start_date_str = fields[3];
       let transaction_start_date = NaiveDate::parse_from_str(transaction_start_date_str, "%Y-%m-%d")
           .unwrap();

       // Step g: Parse transaction_end_date
       let transaction_end_date_str = fields[4];
       let transaction_end_date = NaiveDate::parse_from_str(transaction_end_date_str, "%Y-%m-%d")
           .unwrap();

       // Step h: Parse country
       let country_str = fields[5];
       let country = Country::from_str(country_str)?;

       // Step i: Parse amount
       let amount: f64 = fields[6].parse().unwrap();


       // Step j: Calculate days_under_management
       let days_under_management = transaction_end_date.signed_duration_since(transaction_start_date)
       .num_days();

      // Step k: Get continent based on the country
      let continent = country.country_to_continent();

      // Step l: Create a Transaction instance
      let transaction = Transaction {
          transaction_id,
          client_id,
          asset_name,
          country,
          continent,
          amount,
          days_under_management,
      };

      // Step m: Return an Ok with the transaction variable
      Ok(transaction)

      
     
  }
}