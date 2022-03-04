


use serde::{Deserialize, Serialize};
use csv::{WriterBuilder, ReaderBuilder, Trim};
use std::vec::Vec;
use std::collections::HashMap;



//==============================================================================
// Structure Transaction Declaration


#[derive(Debug, Deserialize, Serialize)]
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
// Structure Movement Declaration


#[derive(Debug, Deserialize)]
pub struct Movement {
  #[serde(rename = "type")]
  pub tx_type: String,
  pub client: u16,
  pub tx: u32,
  pub amount: Option<f64>
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
              , client: self.client, tx: self.tx, amount: (amnt * 10000.0).round() / 10000.0})
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
// Structure MovementImportError Declaration


#[derive(Debug)]
pub struct MovementImportError {
  pub ok_count: u32,
  pub lines: Vec<u32>,
  pub message: String,
  pub code: i8
}




//==============================================================================
// Structure TransactionFactory Implementation


impl MovementImportError {
  /*----------------------------------------------------------------------------
   * Constructors
   */

  pub fn new(ok_count: u32, error_lines: Vec<u32>, error_message: String, error_code: i8) -> MovementImportError {
    MovementImportError { ok_count: ok_count, lines: error_lines, message: error_message, code: error_code }
  }

}


//==============================================================================
// Structure TransactionFactory Declaration


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

    //Return the New TransactionFactory Object
    factory
  }

  pub fn from_str(smovements_csv: &str, bheaders: bool, bdebug: bool, bquiet: bool) -> TransactionFactory {
    let mut factory = TransactionFactory { vmovements: Vec::new(), lsttransactions: HashMap::new() };

    match factory.import_csv_str(smovements_csv, bheaders, bdebug, bquiet) {
      Ok(_) => {}
      , Err(e) => {
        if !bquiet {
          eprintln!("Movement CSV Import Error: Import Movements failed");
          eprintln!("Movement Error: '{:?}'", e);
        }
      }
    } //match factory.import_csv_str(smovements_csv, bdebug, bquiet)

    factory
  }



  /*
  #----------------------------------------------------------------------------
  #Administration Methods
  */

  pub fn import_csv_bytes(&mut self, vmovements_csv: &[u8]
    , bheaders: bool, bdebug: bool, bquiet: bool) -> Result<u32, MovementImportError> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(bheaders)
        .trim(Trim::All)
        .from_reader(vmovements_csv);
    let mut iter = rdr.deserialize();
    let mut serr = String::new();
    let mut verrlines: Vec<u32> = Vec::new();
    let mut icsvline = 1;
    let mut icount = 0;
    let mut ierr = 0;

    if bheaders {
      //Header will be skipped
      icsvline += 1;
    }

    while let Some(result) = iter.next() {
        match result {
          Ok(r) => {
            let mvrecord: Movement = r;
            let otx = mvrecord.build_transaction();

            self.vmovements.push(mvrecord);

            if let Some(txrec) = otx {
              self.lsttransactions.insert(txrec.tx, txrec);
            }

            icount += 1;
          }
          , Err(e) => {
            serr.push_str(&format!("Parse Error: '{:?}'; ", e));
            verrlines.push(icsvline);
            ierr = 1;
          }
        } //match result

        //Keep track of the Input Lines
        icsvline += 1;
    } //while let Some(result) = iter.next()

    if bdebug
      && ! bquiet {
      eprintln!("Transactions CSV Import Count: '{}'", icount);
    }

    if ierr == 0 {
      Ok(icount)
    }
    else
    {
      Err(MovementImportError::new(icount, verrlines, serr, ierr))
    }
  }

  pub fn import_csv_str(&mut self, smovements_csv: &str
    , bheaders: bool, bdebug: bool, bquiet: bool) -> Result<u32, MovementImportError> {
    self.import_csv_bytes(smovements_csv.as_bytes(), bheaders, bdebug, bquiet)
  }

  pub fn add_transaction(&mut self, transaction: Transaction) -> Option<&mut Transaction> {
    let transaction_id = transaction.tx;

    self.lsttransactions.insert(transaction.tx, transaction);

    self.lsttransactions.get_mut(&transaction_id)
  }

  #[allow(unused_variables)]
  pub fn export_transactions_csv(&self, bdebug: bool, bquiet: bool) -> String {
    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    for txrec in self.lsttransactions.iter() {
      match wtr.serialize(txrec.1) {
        Ok(_) => {}
        Err(e) => {
          if ! bquiet {
            eprintln!("Transaction CSV Export Error: '{:?}'", e)
          }
        }
      } //match wtr.serialize(txrec.1)
    } //for txrec in self.lsttransactions.iter()

    let data = match wtr.into_inner() {
      Ok(iwtr) => {
        match String::from_utf8(iwtr) {
          Ok(s) => s
          , Err(e) => {
            if ! bquiet {
              eprintln!("Transactions CSV Export Error: '{:?}'", e);
            }

            //Return empty String
            String::new()
          }
        } //match String::from_utf8(iwtr)
      }
      , Err(e) => {
        if ! bquiet {
          eprintln!("Transactions CSV Export Error: '{:?}'", e);
        }

        //Return empty String
        String::new()
      }
    };  //match wtr.into_inner()

    data
  }

}
