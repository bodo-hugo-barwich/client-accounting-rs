pub mod account;
pub mod transaction;


use crate::app::model::account::Account;
use crate::app::model::transaction::Movement;
use crate::app::model::transaction::Transaction;




//==============================================================================
// Structure Account Implementation


impl Account {
  /*----------------------------------------------------------------------------
   * Administration Methods
   */


  pub fn process_movement(&mut self, mvrecord: &mut Movement, otxrecord: Option<&Transaction>) {
    let mut istatus = match mvrecord.status {
        Some(s) => s
        , None => {
          mvrecord.status = Some(0);
          0
        }
      };

    if !self.locked && istatus == 0 {
      match & otxrecord {
        Some(txrec) => {
          //Referencing Transaction must exist and be valid

          if txrec.client != self.client {
            eprintln!("Movement Processing Error: Account Missmatch '{} / {} - {}'."
              , &self.client, &txrec.client, &txrec.tx);

            //Mark Movement as invalid
            istatus = -1;
          } //if txrec.client != self.client

          if istatus == 0 {
            match mvrecord.tx_type.as_str() {
              "deposit" => {
                eprintln!("Movement Processing '{}': + '{} / {}'", &txrec.tx_type, txrec.amount, self.available);

                self.available += txrec.amount;
                self.total += txrec.amount;

                //Mark Movement as accepted and processed
                istatus = 1;

                eprintln!("Movement Processing '{}': funds '{} / {}'", &txrec.tx_type, self.available, self.total);
              }
              , "withdrawal" => {
                eprintln!("Movement Processing '{}': - '{} / {}'", &txrec.tx_type, txrec.amount, self.available);

                if txrec.amount as f64 <= self.available as f64 {
                  self.available -= txrec.amount;
                  self.total -= txrec.amount;

                  //Mark Movement as accepted and processed
                  istatus = 1;
                }
                else  //Unsufficiant Funds available
                {
                  eprintln!("Movement Processing Error: Amount '-{} / {}' is not available.", txrec.amount, self.available);

                  //Mark Movement as invalid
                  istatus = -1;
                } //if txrec.amount as f64 <= self.available as f64

                eprintln!("Movement Processing '{}': funds '{} / {}'", &txrec.tx_type, self.available, self.total);
              }
              , "dispute" => {
                match txrec.tx_type.as_str() {
                  "deposit" => {
                    self.available -= txrec.amount;
                    self.held += txrec.amount;

                    //Mark Movement as accepted and processed
                    istatus = 1;
                  }
                  , "withdrawal" => {
                    self.held += txrec.amount;
                    self.total += txrec.amount;

                    //Mark Movement as accepted and processed
                    istatus = 1;
                  }
                  , _ => {
                    eprintln!("Movement Processing Error: Movement Type '{}' on '{}': disallowed ", &mvrecord.tx_type, &txrec.tx_type);

                    //Mark Movement as invalid
                    istatus = -1;
                  }
                } //match txrec.tx_type.as_str()
              }
              , "resolve" => {
                self.held -= txrec.amount;
                self.available += txrec.amount;

                //Mark Movement as accepted and processed
                istatus = 1;
              }
              , "chargeback" => {
                self.held += txrec.amount;
                self.total += txrec.amount;
                self.locked = true;

                //Mark Movement as accepted and processed
                istatus = 1;
              }
              , _ => {
                eprintln!("Movement Processing Error: Movement Type '{}': unknown", &mvrecord.tx_type);

                //Mark Movement as invalid
                istatus = -1;
              }
            } //match mvrecord.tx_type.as_str()
          } //if istatus == 0
        }
        , None => {
          eprintln!("Movement Processing Error: Transaction (id: '{}') does not exist. Movement omitted.", &mvrecord.tx);

          //Mark Movement as invalid
          istatus = -1;
        }
      } //match & otxrecord

      //Store Evaluation Result
      mvrecord.status = Some(istatus);
    }
    else {  //Account is locked and Movements stay pending
      eprintln!("Movement Processing Error: Account (id: '{}') is locked. Movement refused.", &self.client);
    }
  }

  pub fn process_transaction(&mut self, txrecord: &Transaction) {
    match txrecord.tx_type.as_str() {
      "deposit" => {

      }
      , "withdrawal" => {

      }
      , _ => {
        eprintln!("Transaction Processing Error: Transaction Type '{}': unknown", &txrecord.tx_type);
      }
    }
  }


}
