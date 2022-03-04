
use super::super::model::account::AccountFactory;
use super::super::model::transaction::TransactionFactory;


//==============================================================================
// Structure MovementImporter Declaration


#[derive(Debug)]
pub struct MovementImporter {
  _lstaccfact: AccountFactory
  , _lsttxfact: TransactionFactory
  , _bquiet: bool
  , _bdebug: bool
  , _ierr: i32
}





//==============================================================================
// Structure MovementImporter Implementation


impl Default for MovementImporter {
  /*----------------------------------------------------------------------------
   * Default Constructor
   */


    fn default() -> Self {
        MovementImporter::new()
    }
}


#[allow(dead_code)]
impl MovementImporter {
  /*----------------------------------------------------------------------------
   * Constructors
   */


  pub fn new() -> MovementImporter {
    MovementImporter { _lstaccfact: AccountFactory::new(),
    _lsttxfact: TransactionFactory::new(),
     _bquiet: false, _bdebug: false, _ierr: 0 }
  }



  /*----------------------------------------------------------------------------
   *Administration Methods
   */


  pub fn set_quiet(&mut self, bquiet: bool) {
    self._bquiet = bquiet;
  }

  pub fn set_debug(&mut self, bdebug: bool) {
    self._bdebug = bdebug;
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

    //Clear processed Movements
    self._lsttxfact.vmovements.clear();
  }

  pub fn import_movements_bytes(&mut self, vmovements_bytes: &[u8]) -> i32 {

    let itxcount = self._lsttxfact.import_csv_bytes(vmovements_bytes);

    if itxcount == 0 {
        eprintln!("Transactions CSV Import Error: Import Transactions failed");

        self._ierr = 1;
    }



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

  pub fn export_accounts_str(&self) -> String {
    self._lstaccfact.export_csv()
  }

  /*----------------------------------------------------------------------------
   * Consultation Methods
   */


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