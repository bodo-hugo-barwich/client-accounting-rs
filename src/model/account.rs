use serde::{Deserialize, Serialize};

use csv::{ReaderBuilder, Trim, WriterBuilder};
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
    pub locked: bool,
}

//==============================================================================
// Structure AccountFactory Declaration

#[derive(Debug)]
pub struct AccountFactory {
    pub lstaccounts: HashMap<u16, Account>,
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
        let factory = AccountFactory {
            lstaccounts: HashMap::new(),
        };

        //Return the New AccountFactory Object
        factory
    }

    pub fn from_str(
        saccounts_csv: &str,
        bheaders: bool,
        bdebug: bool,
        bquiet: bool,
    ) -> AccountFactory {
        let mut factory = AccountFactory {
            lstaccounts: HashMap::new(),
        };
        factory.import_csv(saccounts_csv, bheaders, bdebug, bquiet);

        factory
    }

    #[allow(unused_variables)]
    pub fn import_csv(
        &mut self,
        saccounts_csv: &str,
        bheaders: bool,
        bdebug: bool,
        bquiet: bool,
    ) -> u32 {
        let mut rdr = ReaderBuilder::new()
            .has_headers(bheaders)
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
                Err(e) => {
                    if !bquiet {
                        eprintln!("Account CSV Parse Error: '{:?}'", e);
                    }
                }
            } //match result
        } //while let Some(result) = iter.next()

        icount
    }

    #[allow(unused_variables)]
    pub fn export_csv(&self, bdebug: bool, bquiet: bool) -> String {
        let mut wtr = WriterBuilder::new().from_writer(vec![]);

        for acc in self.lstaccounts.iter() {
            match wtr.serialize(acc.1) {
                Ok(_) => {}
                Err(e) => {
                    if !bquiet {
                        eprintln!("Account CSV Export Error: '{:?}'", e)
                    }
                }
            } //match wtr.serialize(acc.1)
        } //for acc in self.lstaccounts.iter()

        let data = match wtr.into_inner() {
            Ok(iwtr) => {
                match String::from_utf8(iwtr) {
                    Ok(s) => s,
                    Err(e) => {
                        if !bquiet {
                            eprintln!("Account CSV Export Error: '{:?}'", e);
                        }

                        //Return empty String
                        String::new()
                    }
                }
            }
            Err(e) => {
                if !bquiet {
                    eprintln!("Account CSV Export Error: '{:?}'", e);
                }

                //Return empty String
                String::new()
            }
        }; //match wtr.into_inner()

        data
    }

    pub fn create_account(&mut self, client_id: &u16) -> Option<&mut Account> {
        let account = Account {
            client: *client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };

        self.lstaccounts.insert(account.client, account);

        self.lstaccounts.get_mut(client_id)
    }

    pub fn add_account(&mut self, account: Account) -> Option<&mut Account> {
        let client_id = account.client;

        self.lstaccounts.insert(account.client, account);

        self.lstaccounts.get_mut(&client_id)
    }
}
