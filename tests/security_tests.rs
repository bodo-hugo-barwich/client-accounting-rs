use client_accounting::app::importer::MovementImporter;

#[cfg(test)]
mod dispute_fails_tests {
    use client_accounting::model::account::AccountFactory;
    use client_accounting::model::transaction::TransactionFactory;

    /// ### Test `dispute_on_foreign_deposit()`
    /// This test creates an Account from a "_deposit_" Transaction
    /// and another one from a "_withdrawal_" Transaction
    /// So, 2 Account and 2 Transaction must have been created
    /// Account (id: '5') disputes Transaction of Account (id: '11') and fails
    /// Account (id: '11') must have `available` Funds (`1.1`)
    /// Account (id: '5') must have `available` Funds (`0.0`)
    /// Transaction (id: '9') must be marked as processed with `status` (`1`)
    /// Transaction (id: '3') must be marked as invalid with `status` (`-1`)
    #[test]
    fn dispute_on_foreign_deposit() {
        //-------------------------------------
        //Test Dispute on foreign Deposit fails

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,9,1.1\nwithdrawal,5,3,0.5\ndispute,5,9,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        let accfact = AccountFactory::from_str(saccounts.as_str(), true, true, false);
        let oaccrec11 = accfact.lstaccounts.get(&11);
        let oaccrec5 = accfact.lstaccounts.get(&5);

        assert!(oaccrec11.is_some());
        assert!(oaccrec5.is_some());
        assert_eq!(oaccrec11.unwrap().available, 1.1);
        assert_eq!(oaccrec5.unwrap().available, 0.0);

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec3 = txfact.lsttransactions.get(&3);
        let otxrec9 = txfact.lsttransactions.get(&9);

        assert!(otxrec3.is_some());
        assert!(otxrec9.is_some());
        assert_eq!(otxrec3.unwrap().status, -1);
        assert_eq!(otxrec9.unwrap().status, 1);
    }

    /// ### Test `dispute_on_failed_transaction()`
    /// This test creates an Account from a "_deposit_" Transaction
    /// and produces a "_withdrawal_" Transaction
    /// So, 1 Account and 2 Transaction must have been created
    /// Transaction (id: '5') fails
    /// Transaction (id: '5') is disputed and fails
    /// Account (id: '11') must have `available` Funds (`1.1`)
    /// Transaction (id: '9') must be marked as processed with `status` (`1`)
    /// Transaction (id: '3') must be marked as invalid with `status` (`-1`)
    #[test]
    fn dispute_on_failed_transaction() {
        //-------------------------------------
        //Test Dispute on foreign Deposit fails

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,9,1.1\nwithdrawal,11,3,2.2\ndispute,11,3,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        let accfact = AccountFactory::from_str(saccounts.as_str(), true, true, false);
        let oaccrec11 = accfact.lstaccounts.get(&11);

        assert!(oaccrec11.is_some());
        assert_eq!(oaccrec11.unwrap().available, 1.1);

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec3 = txfact.lsttransactions.get(&3);
        let otxrec9 = txfact.lsttransactions.get(&9);

        assert!(otxrec3.is_some());
        assert!(otxrec9.is_some());
        assert_eq!(otxrec3.unwrap().status, -1);
        assert_eq!(otxrec9.unwrap().status, 1);
    }

    /// ### Test `chargeback_non_disputed_transaction()`
    /// This test creates an Account from a "_deposit_" Transaction
    /// and produces a "_withdrawal_" Transaction
    /// So, 1 Account and 2 Transaction must have been created
    /// Transaction (id: '9') is disputed
    /// Transaction (id: '5') requests a Cargeback
    /// Transaction (id: '9') must be marked as disputed with `status` (`-2`)
    /// Transaction (id: '3') must be marked as processed with `status` (`1`)
    #[test]
    fn chargeback_non_disputed_transaction() {
        //-------------------------------------
        //Test Chargeback on non-disputed Transaction fails

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,9,1.1\nwithdrawal,11,3,0.5\ndispute,11,9,\nchargeback,11,3,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        let accfact = AccountFactory::from_str(saccounts.as_str(), true, true, false);
        let oaccrec11 = accfact.lstaccounts.get(&11);

        assert!(oaccrec11.is_some());
        assert_eq!(oaccrec11.unwrap().available, -0.5);

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec3 = txfact.lsttransactions.get(&3);
        let otxrec9 = txfact.lsttransactions.get(&9);

        assert!(otxrec3.is_some());
        assert!(otxrec9.is_some());
        assert_eq!(otxrec3.unwrap().status, 1);
        assert_eq!(otxrec9.unwrap().status, -2);
    }

    /// ### Test `chargeback_insufficient_funds()`
    /// This test creates an Account from a "_deposit_" Transaction
    /// and produces a "_withdrawal_" Transaction
    /// So, 1 Account and 2 Transaction must have been created
    /// Transaction (id: '9') is disputed
    /// Transaction (id: '9') requested Cargeback fails on insufficient funds
    /// Transaction (id: '9') must be marked as disputed with `status` (`-2`)
    /// Transaction (id: '3') must be marked as processed with `status` (`1`)
    #[test]
    fn chargeback_insufficient_funds() {
        //-------------------------------------
        //Test Chargeback on non-disputed Transaction fails

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,9,1.1\nwithdrawal,11,3,0.5\ndispute,11,9,\nchargeback,11,9,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        let accfact = AccountFactory::from_str(saccounts.as_str(), true, true, false);
        let oaccrec11 = accfact.lstaccounts.get(&11);

        assert!(oaccrec11.is_some());
        assert_eq!(oaccrec11.unwrap().available, -0.5);

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec3 = txfact.lsttransactions.get(&3);
        let otxrec9 = txfact.lsttransactions.get(&9);

        assert!(otxrec3.is_some());
        assert!(otxrec9.is_some());
        assert_eq!(otxrec3.unwrap().status, 1);
        assert_eq!(otxrec9.unwrap().status, -2);
    }
}
