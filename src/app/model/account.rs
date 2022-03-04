
use serde::{Serialize, Deserialize};

use csv::{ReaderBuilder, Trim};
use std::collections::HashMap;

//#[macro_use]
//extern crate serde_derive;



//==============================================================================
// Structure Account Declaration


#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool
}


//==============================================================================
// Structure AccountFactory Declaration


#[derive(Debug)]
pub struct AccountFactory {
  pub lstaccounts: HashMap<u16, Account>
}




//==============================================================================
// Structure AccountFactory Implementation


impl Default for AccountFactory {
  /*----------------------------------------------------------------------------
   * Default Constructor
   */


    fn default() -> Self {
        AccountFactory::new()
    }
}

#[allow(dead_code)]
impl AccountFactory {
  /*----------------------------------------------------------------------------
   * Constructors
   */


  pub fn new() -> AccountFactory {
    let factory = AccountFactory { lstaccounts: HashMap::new() };

    //accounting._init();


    //Return the New AccountFactory Object
    factory
  }

  pub fn from_str(saccounts_csv: &str) -> AccountFactory {
    let mut factory = AccountFactory { lstaccounts: HashMap::new() };
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(saccounts_csv.as_bytes());
    let mut iter = rdr.deserialize();


    while let Some(result) = iter.next() {
        match result {
          Ok(r) => {
            let record: Account = r;

            factory.lstaccounts.insert(record.client, record);
          }
          , Err(e) => eprintln!("{}", &format!("Account CSV Parse Error: '{:?}'", e))
        }
    }

    factory
  }

  pub fn import_csv(&mut self, saccounts_csv: &str) -> u32 {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(saccounts_csv.as_bytes());
    let mut iter = rdr.deserialize();
    let mut icount = 0;

    while let Some(result) = iter.next() {
        match result {
          Ok(r) => {
            let record: Account = r;

            self.lstaccounts.insert(record.client, record);

            icount += 1;
          }
          , Err(e) => eprintln!("{}", &format!("Account CSV Parse Error: '{:?}'", e))
        }
    }

    icount
  }

  pub fn create_account(&mut self, client_id: &u16) -> Option<&mut Account> {
    let account = Account{client: *client_id, available: 0.0
      , held: 0.0, total: 0.0, locked: false};

    self.lstaccounts.insert(account.client, account);

    self.lstaccounts.get_mut(client_id)
  }

  pub fn add_account(&mut self, account: Account) -> Option<&mut Account> {
    let client_id = account.client;

    self.lstaccounts.insert(account.client, account);

    self.lstaccounts.get_mut(&client_id)
  }
}