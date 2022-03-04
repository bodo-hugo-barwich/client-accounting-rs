


use serde::Deserialize;

use csv::{ReaderBuilder, Trim};
use std::vec::Vec;
use std::collections::HashMap;



//==============================================================================
// Structure Transaction Declaration


#[derive(Debug, Deserialize)]
pub struct Transaction {
  pub tx_type: String,
  pub client: u16,
  pub tx: u32,
  pub amount: f64
}




//==============================================================================
// Structure Transaction Implementation


impl Clone for Transaction {
  /*----------------------------------------------------------------------------
   * Administration Methods
   */


    fn clone(&self) -> Transaction {
        Transaction{tx_type: String::from(self.tx_type.as_str())
          , client: self.client, tx: self.tx, amount: self.amount}
    }
}



//==============================================================================
// Structure Transaction Declaration


#[derive(Debug, Deserialize)]
pub struct Movement {
  pub tx_type: String,
  pub client: u16,
  pub tx: u32,
  pub amount: Option<f64>,
  pub status: Option<i8>
}



//==============================================================================
// Structure Movement Implementation


impl Movement {
  /*----------------------------------------------------------------------------
   * Administration Methods
   */


  pub fn build_transaction(&self) -> Option<Transaction> {
    match self.tx_type.as_str() {
      "deposit" | "withdrawal"  => {
        match self.amount {
          Some(amnt) => {
            Some(Transaction{tx_type: String::from(self.tx_type.as_str())
              , client: self.client, tx: self.tx, amount: amnt})
          }
          , None => {
            //Invalid Transaction
            eprintln!("Movement Processing Error: Amount is empty. Movement will be omitted.");

            //Do not create an Transaction Object
            None
          }
        }
      }
      , _ => {
        //Do not create an Transaction Object
        None
      }
    }
  }
}



//==============================================================================
// Structure AccountFactory Declaration


#[derive(Debug)]
pub struct TransactionFactory {
  pub vmovements: Vec<Movement>,
  pub lsttransactions: HashMap<u32, Transaction>
}




//==============================================================================
// Structure TransactionFactory Implementation


impl Default for TransactionFactory {
  /*----------------------------------------------------------------------------
   * Default Constructor
   */


    fn default() -> Self {
        TransactionFactory::new()
    }
}

#[allow(dead_code)]
impl TransactionFactory {
  /*----------------------------------------------------------------------------
   * Constructors
   */


  pub fn new() -> TransactionFactory  {
    let factory = TransactionFactory { vmovements: Vec::new(), lsttransactions: HashMap::new() };

    //accounting._init();


    //Return the New AccountFactory Object
    factory
  }

  pub fn from_str(stransactions_csv: &str) -> TransactionFactory {
    let mut factory = TransactionFactory { vmovements: Vec::new(), lsttransactions: HashMap::new() };
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(stransactions_csv.as_bytes());
    let mut iter = rdr.records();

    if let Some(result) = iter.next() {
        match result {
          Ok(r) => {
            eprintln!("{}", &format!("Movement CSV Header: '{:?}'", r));
          }
          , Err(e) => eprintln!("{}", &format!("Movement CSV Parse Error: '{:?}'", e))
        }
    }

    let mut iter = rdr.deserialize();

    while let Some(result) = iter.next() {
        match result {
          Ok(r) => {
            let mvrecord: Movement = r;
            let otx = mvrecord.build_transaction();

            factory.vmovements.push(mvrecord);

            if let Some(txrec) = otx {
              factory.lsttransactions.insert(txrec.tx, txrec);
            }
          }
          , Err(e) => eprintln!("{}", &format!("Movement CSV Parse Error: '{:?}'", e))
        }
    }

    factory
  }



  /*
  #----------------------------------------------------------------------------
  #Administration Methods
  */

  pub fn import_csv_bytes(&mut self, vaccounts_csv: &[u8]) -> u32 {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .flexible(true)
        .from_reader(vaccounts_csv);
    let mut iter = rdr.deserialize();
    let mut icount = 0;

    while let Some(result) = iter.next() {
        match result {
          Ok(r) => {
            let mut mvrecord: Movement = r;
            let otx = mvrecord.build_transaction();


            if mvrecord.status.is_none() {
              mvrecord.status = Some(0);
            }

            self.vmovements.push(mvrecord);

            if let Some(txrec) = otx {
              self.lsttransactions.insert(txrec.tx, txrec);
            }

            icount += 1;
          }
          , Err(e) => eprintln!("{}", &format!("Movement CSV Parse Error: '{:?}'", e))
        }
    } //while let Some(result) = iter.next()

    eprintln!("Transactions CSV Import Count: '{}'", icount);

    icount
  }


  pub fn import_csv_str(&mut self, saccounts_csv: &str) -> u32 {
    self.import_csv_bytes(saccounts_csv.as_bytes())
  }

  pub fn add_transaction(&mut self, transaction: Transaction) -> Option<&mut Transaction> {
    let transaction_id = transaction.tx;

    self.lsttransactions.insert(transaction.tx, transaction);

    self.lsttransactions.get_mut(&transaction_id)
  }

}
