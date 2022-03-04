
use std::io::Read;
use std::fs::File;

mod model;

use model::account::AccountFactory;
use model::transaction::TransactionFactory;


//==============================================================================
// Structure RunClientAccounting Declaration


#[derive(Debug)]
pub struct RunClientAccounting {
  _lstaccfact: AccountFactory
  , _lsttxfact: TransactionFactory
  , _stxfile: String
  , _bimport: bool
  , _bquiet: bool
  , _bdebug: bool
  , _ierr: i32
}



//==============================================================================
// Structure RunClientAccounting Implementation


impl Default for RunClientAccounting {
  /*----------------------------------------------------------------------------
   * Default Constructor
   */


    fn default() -> Self {
        RunClientAccounting::new()
    }
}


#[allow(dead_code)]
impl RunClientAccounting {
  /*----------------------------------------------------------------------------
   * Constructors
   */


  pub fn new() -> RunClientAccounting {
    let accounting = RunClientAccounting { _lstaccfact: AccountFactory::new(),
    _lsttxfact: TransactionFactory::new(),
      _stxfile: String::new(),
    _bimport: false, _bquiet: false, _bdebug: false, _ierr: 0 };

    //accounting._init();


    //Return the New RunClientAccounting Object
    accounting
  }



/*
  #----------------------------------------------------------------------------
  #Administration Methods
  */


  pub fn set_transaction_filename(&mut self, sfilename: &str) {
    self._stxfile = String::from(sfilename);
  }

  pub fn set_import(&mut self, bimport: bool) {
    self._bimport = bimport;
  }

  pub fn set_quiet(&mut self, bquiet: bool) {
    self._bquiet = bquiet;
  }

  pub fn set_debug(&mut self, bdebug: bool) {
    self._bdebug = bdebug;
  }

  async fn set_error_code(&mut self, ierrorcode: i32) {
    self._ierr = ierrorcode;
  }

  fn process_movements(&mut self) {
    for mut mvrec in &mut self._lsttxfact.vmovements {
      let mut oacc = self._lstaccfact.lstaccounts.get_mut(&mvrec.client);
      let otxrec = self._lsttxfact.lsttransactions.get(&mvrec.tx);

      if oacc.is_none() {
        oacc = self._lstaccfact.create_account(&mvrec.client);
      }

      match &mut oacc {
        Some(acc) => {
          acc.process_movement(&mut mvrec, otxrec);
        }
        None => {
          eprintln!("Movement Processing Error: Add Account failed");
        }
      } //match &mut oacc
    } //for mvrec in &self._lsttxfact.vmovements
  }

  fn import_from_file(&mut self) {
    if ! self._stxfile.is_empty() {
      let mut oflrs = File::open(self._stxfile.as_str());

      match &mut oflrs {
        Ok(fl) => {
          //let mut buf_reader = BufReader::new(fl);
          let mut vchunk = vec![0; 32768];

          match fl.read(&mut vchunk) {
            Ok(icnt) => {
              let itxcount = self._lsttxfact.import_csv_bytes(&vchunk[..icnt]);

              if itxcount == 0 {
                  eprintln!("Transactions CSV Import Error: Import Transactions failed");

                  self._ierr = 1;
              }
            }
            , Err(e) => {
              eprintln!("Movements CSV Read Error: '{:?}'", e);

              self._ierr = 1;
            }
          } //match fl.read(&mut vchunk)
        }
        , Err(e) => {
          eprintln!("Movements CSV Open Error: '{:?}'", e);

          self._ierr = 1;
        }
      } //match &mut oflrs

    }
    else  // Input File was not given
    {
      eprintln!("Movements CSV Import Error: File is missing.");

      self._ierr = 3;
    } //if ! self._stxfile.is_empty()
  }

  pub fn do_run(&mut self) -> i32 {

    self.import_from_file();

/*
    match &mut self._lstaccfact.create_account(&1) {
      Some(acc) => {
        acc.available = 1.5;
        acc.held = 0.0;
        acc.total = 1.5;
      }
      , None => {
        eprintln!("Account CSV Create Error: Add Account failed");

        self._ierr = 1;
      }
    }

    if self._lstaccfact.import_csv(&"2,2.0,0.0,2.0,false\n3,3.1,0.0,3.1,false\n4,4.1,0.0,4.1,false\n") == 0 {
        eprintln!("Account CSV Import Error: Import Accounts failed");

        self._ierr = 1;
    }
*/
    if self._bdebug
      && ! self._bquiet {
      eprintln!("accs fct dmp 1:\n{:?}", self._lstaccfact);
    }

/*
    let itxcount = self._lsttxfact.import_csv_str(&"type,client,tx, amount\ndeposit,1,1,1.0\ndeposit,2,2,2.0\ndeposit,1,3,2.0\nwithdrawal,1,4,1.5\nwithdrawal,2,5,3.0\n");

    if itxcount == 0 {
        eprintln!("Transactions CSV Import Error: Import Transactions failed");

        self._ierr = 1;
    }
*/
    if self._bdebug
      && ! self._bquiet {
      eprintln!("txs fct dmp 1:\n{:?}", self._lsttxfact);
    }

    if self._lsttxfact.vmovements.len() > 0 {
      self.process_movements();
    }

    if self._bdebug
      && ! self._bquiet {
      eprintln!("txs fct dmp 2:\n{:?}", self._lsttxfact);
      eprintln!("accs fct dmp 2:\n{:?}", self._lstaccfact);
    }

    self._ierr
  }



  /*----------------------------------------------------------------------------
   * Consultation Methods
   */


  pub fn is_import(&self) -> bool {
    self._bimport
  }

  pub fn is_quiet(&self) -> bool {
    self._bquiet
  }

  pub fn is_debug(&self) -> bool {
    self._bdebug
  }

  pub fn get_error_code(&self) -> i32 {
    self._ierr
  }
}
