
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
        , "q" => application.set_quiet(true)
        , "d" | "v" => application.set_debug(true)
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
  let mut accounting = RunClientAccounting::new();

  parse_parameters(&mut accounting);

  if accounting.is_debug()
    && ! accounting.is_quiet() {
    eprintln!("app dmp 1:\n{:?}", accounting);
  }

  accounting.do_run();

  accounting.get_error_code()
}

fn main() {

  let ierr = run_app();

  match ierr {
    0 => {}
    _ => {
      eprintln!("Application failed with [{}]", ierr);

      exit(ierr);
    }
  }
}
