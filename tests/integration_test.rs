
//mod app;

//extern crate client_accounting;

use client_accounting::app::importer::MovementImporter;


#[cfg(test)]
mod tests {


  #[test]
  fn create_transaction() {
    //-------------------------------------
    //Read the Application Parameters

    let mut imp = super::MovementImporter::new();

    imp.set_debug(true);



    assert_eq!(imp.import_movements_str(&"deposit,3,3,1.1\n\n", false), 0);



  }
}