pub mod account;
pub mod transaction;


use crate::model::account::Account;
use crate::model::transaction::{Movement, Transaction};




//==============================================================================
// Structure Account Implementation


impl Account {
  /*----------------------------------------------------------------------------
   * Administration Methods
   */


  pub fn process_movement(&mut self, mvrecord: &mut Movement, otxrecord: Option<&Transaction>
    , bdebug: bool, bquiet: bool) {
    if !self.locked {
      match & otxrecord {
        Some(txrec) => {
          //Referencing Transaction must exist and be valid

          if txrec.client == self.client {
            match mvrecord.tx_type.as_str() {
              "deposit" => {
                if bdebug
                  && ! bquiet {
                  eprintln!("Movement Processing '{}': + '{} / {}'", &txrec.tx_type, txrec.amount, self.available);
                }

                self.available += txrec.amount;
                self.total += txrec.amount;

                if bdebug
                  && ! bquiet {
                  eprintln!("Movement Processing '{}': funds '{} / {}'", &txrec.tx_type, self.available, self.total);
                }
              }
              , "withdrawal" => {
                if bdebug
                  && ! bquiet {
                  eprintln!("Movement Processing '{}': - '{} / {}'", &txrec.tx_type, txrec.amount, self.available);
                }

                if txrec.amount as f64 <= self.available as f64 {
                  self.available -= txrec.amount;
                  self.total -= txrec.amount;
                }
                else  //Unsufficiant Funds available
                {
                  if ! bquiet {
                    eprintln!("Movement Processing Error: Amount '-{} / {}' is not available.", txrec.amount, self.available);
                  }
                } //if txrec.amount as f64 <= self.available as f64

                if bdebug
                  && ! bquiet {
                  eprintln!("Movement Processing '{}': funds '{} / {}'", &txrec.tx_type, self.available, self.total);
                }
              }
              , "dispute" => {
                match txrec.tx_type.as_str() {
                  "deposit" => {
                    self.available -= txrec.amount;
                    self.held += txrec.amount;
                  }
                  , "withdrawal" => {
                    self.held += txrec.amount;
                    self.total += txrec.amount;
                  }
                  , _ => {
                    if ! bquiet {
                      eprintln!("Movement Processing Error: Movement Type '{}' on '{}': disallowed ", &mvrecord.tx_type, &txrec.tx_type);
                    }
                  }
                } //match txrec.tx_type.as_str()
              }
              , "resolve" => {
                self.held -= txrec.amount;
                self.available += txrec.amount;
              }
              , "chargeback" => {
                self.held += txrec.amount;
                self.total += txrec.amount;
                self.locked = true;
              }
              , _ => {
                if ! bquiet {
                  eprintln!("Movement Processing Error: Movement Type '{}': unknown", &mvrecord.tx_type);
                }
              }
            } //match mvrecord.tx_type.as_str()
          }
          else  //Transaction does not belong to this Account
          {
            if ! bquiet {
              eprintln!("Movement Processing Error: Account Missmatch '{} / {} - {}'."
                , &self.client, &txrec.client, &txrec.tx);
            }
          } //if txrec.client != self.client
        }
        , None => {
          if ! bquiet {
            eprintln!("Movement Processing Error: Transaction (id: '{}') does not exist. Movement omitted.", &mvrecord.tx);
          }
        }
      } //match & otxrecord
    }
    else {  //Account is locked and Movements stay pending
      if ! bquiet {
        eprintln!("Movement Processing Error: Account (id: '{}') is locked. Movement refused.", &self.client);
      }
    }
  }


}
