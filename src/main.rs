
mod app;
mod model;

use app::RunClientAccounting;

use std::process::exit;



//==============================================================================
// Auxiliary Functions


fn parse_parameters(application: &mut RunClientAccounting) {
  //-------------------------------------
  //Read the Script Parameters

  let mut sarg;
  let mut iargidx = 0;


  //eprintln!("args: ");

  // Prints each argument on a separate line
  for argument in std::env::args() {
    //eprintln!("[{}] '{}'", iargidx, argument.as_str());

    if argument.starts_with("--") {  //Parameter with Double Dash
      sarg = argument.split_at(2).1;
      sarg.to_lowercase();
    }
    else if argument.starts_with('-') {  //Parameter with Single Dash
      sarg = argument.split_at(1).1;
      sarg.to_lowercase();

      match sarg {
        "i" => application.set_import(true)
        , "v" => application.set_quiet(false)
        , "d" => {
          //Reenable Notices
          application.set_quiet(false);
          //Enable Debug Output
          application.set_debug(true);
        }
        , _ => {}
      } //match sarg
    }
    else if iargidx > 0 {  //Any Not First Parameter
      application.set_transaction_filename(&argument);
    } //if argument.starts_with("--")

    iargidx += 1;
  } //for argument in std::env::args()

  //eprintln!("args end.");
}


fn run_app() -> i32 {
  //-------------------------------------
  //Create the Application Object

  let mut accounting = RunClientAccounting::new();

  //Suppress Notices by default
  accounting.set_quiet(true);

  parse_parameters(&mut accounting);

  if accounting.is_debug()
    && ! accounting.is_quiet() {
    eprintln!("app dmp 1:\n{:?}", accounting);
  }

  //Execute the Application
  accounting.do_run();

  let ierr = accounting.get_error_code();


  if ! accounting.is_quiet() {
    if ierr == 0 {
      eprintln!("Application finished with [{}]", ierr);
    }
    else
    {
      eprintln!("Application failed with [{}]", ierr);
    }
  }

  ierr
}

fn main() {

  let ierr = run_app();

  match ierr {
    0 => {}
    _ => {
      exit(ierr);
    }
  }
}
