


use std::io::Read;
use std::fs::File;


pub mod importer;

use super::app::importer::MovementImporter;


//==============================================================================
// Structure RunClientAccounting Declaration


#[derive(Debug)]
pub struct RunClientAccounting {
  _importer: MovementImporter
  , _stxfile: String
  , _vinput: Vec<u8>
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
    let accounting = RunClientAccounting { _importer: MovementImporter::new(),
      _stxfile: String::new(), _vinput: Vec::new(),
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

    self._importer.set_quiet(bquiet);
  }

  pub fn set_debug(&mut self, bdebug: bool) {
    self._bdebug = bdebug;

    self._importer.set_debug(bdebug);
  }

  fn import_from_file(&mut self) {
    if ! self._stxfile.is_empty() {
      let mut oflrs = File::open(self._stxfile.as_str());

      match &mut oflrs {
        Ok(fl) => {
          //let mut buf_reader = BufReader::new(fl);
          let ichunksize = 64; // 32768;
          let mut vchunk = vec![0; ichunksize];
          let mut icnkcnt = 0;
          let mut irdcnt = ichunksize;

          while irdcnt > 0
            && irdcnt == ichunksize {
            match fl.read(&mut vchunk) {
              Ok(icnt) => {
                irdcnt = icnt;

                if self._bdebug
                  && ! self._bquiet {
                  eprintln!("chunk [{}] (sz: '{}'):\n'{:?}'\n", icnkcnt, icnt, &vchunk[..icnt]);
                }

                let ilnend = match find_last(&vchunk[..icnt], &10) {
                  Some(ips) => {
                    if ips == icnt {
                      ips
                    }
                    else {
                      ips + 1
                    }
                  }
                  , None => icnt
                };
                self._vinput.append(&mut vchunk[..ilnend].to_owned());

                if self._bdebug
                  && ! self._bquiet {
                  eprintln!("input [{}] (sz: '{}'):\n'{:?}'\n", icnkcnt, self._vinput.len(), self._vinput);
                }

                if let Some(_) = find_last(&self._vinput, &10) {
                  icnkcnt += 1;

                  let iimprs = match icnkcnt {
                    1 => self._importer.import_movements_bytes(&self._vinput, true)
                    , _ => self._importer.import_movements_bytes(&self._vinput, false)
                  };

                  if iimprs != 0 {
                    self._ierr = iimprs;
                  }

                  self._vinput.clear();
                } //if let Some(_) = find_last(&self._vinput, &10)

                if ilnend < icnt {
                  self._vinput.append(&mut vchunk[ilnend..icnt].to_owned());
                }
              }
              , Err(e) => {
                if ! self._bquiet {
                  eprintln!("Movements CSV Read Error: '{:?}'", e);
                }

                self._ierr = 1;
              }
            } //match fl.read(&mut vchunk)
          } //while irdcnt > 0 && irdcnt == ichunksize

          if self._vinput.len() > 0 {
            if self._bdebug
              && ! self._bquiet {
              eprintln!("input lst (sz: '{}'):\n'{:?}'\n", self._vinput.len(), self._vinput);
            }

            let iimprs = match icnkcnt {
              0 => self._importer.import_movements_bytes(&self._vinput, true)
              , _ => self._importer.import_movements_bytes(&self._vinput, false)
            };

            if iimprs != 0 {
                self._ierr = iimprs;
            }

            self._vinput.clear();
          } //if self._vinput.len() > 0
        }
        , Err(e) => {
          if ! self._bquiet {
            eprintln!("Movements CSV Open Error: '{:?}'", e);
          }

          self._ierr = 1;
        }
      } //match &mut oflrs

    }
    else  // Input File was not given
    {
      if ! self._bquiet {
        eprintln!("Movements CSV Import Error: File is missing.");
      }

      self._ierr = 3;
    } //if ! self._stxfile.is_empty()
  }

  fn export_accounts(&self) -> i32 {
    let data = self._importer.export_accounts_str();

    print!("{}", &data);

    self._ierr
  }

  pub fn do_run(&mut self) -> i32 {

    self.import_from_file();

    self.export_accounts();

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



//==============================================================================
// Auxiliary Functions

fn find_last<T: PartialEq>(vvector: &[T], needle: &T) -> Option<usize> {
  let mut iter = vvector.iter().rev();
  let mut oitem = iter.next();
  let mut iitempos = vvector.len() - 1;
  let mut oipos = None;

  while oitem.is_some() && oipos.is_none() {
    if let Some(item) = oitem {
      if item == needle {
        oipos = Some(iitempos);
      }
      else if iitempos > 0
      {
        iitempos -= 1;
      }

      oitem = iter.next();
    }
  } //while oitem.is_some() && oipos.is_none()

  //eprintln!("find_last rs: '{:?}'\n", oipos);

  oipos
}

