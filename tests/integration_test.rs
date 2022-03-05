//mod app;

//extern crate client_accounting;

use client_accounting::app::importer::MovementImporter;

#[cfg(test)]
mod account_tests {

    /// ### Test `create_account_from_deposit()`
    /// This test creates an Account from a "_deposit_" Transaction
    /// So, 1 Account and 1 Transaction must have been created
    /// The Transaction must be marked as processed with `status` (`1`)
    #[test]
    fn create_account_from_deposit() {
        //-------------------------------------
        //Test Create Account from Deposit

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(&"type, client, tx, amount\ndeposit,11,9,1.1\n", true),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        assert_eq!(
            saccounts.as_str(),
            "client,available,held,total,locked\n11,1.1,0.0,1.1,false\n"
        );
        assert_eq!(
            stransactions.as_str(),
            "type,client,tx,amount,status\ndeposit,11,9,1.1,1\n"
        );
    }

    /// ### Test `create_account_from_withdrawal()`
    /// This test creates an Account from a "_withdrawal_" Transaction
    /// So, 1 Account and 1 Transaction must have been created
    /// The Transaction must be marked as invalid with `status` (`-1`)
    #[test]
    fn create_account_from_withdrawal() {
        //-------------------------------------
        //Test Create Account from Withdrawal

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(&"type, client, tx, amount\nwithdrawal,5,3,0.5\n", true),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        assert_eq!(
            saccounts.as_str(),
            "client,available,held,total,locked\n5,0.0,0.0,0.0,false\n"
        );
        assert_eq!(
            stransactions.as_str(),
            "type,client,tx,amount,status\nwithdrawal,5,3,0.5,-1\n"
        );
    }
}

#[cfg(test)]
mod dispute_tests {
    use client_accounting::model::transaction::TransactionFactory;

    /// ### Test `create_dispute_on_deposit()`
    /// This test creates an Account from a "_deposit_" Transaction
    /// So, 1 Account and 1 Transaction must have been created
    /// The Transaction must be marked as disputed with `status` (`-2`)
    /// The Dispute is accepted and Funds are placed on `held`
    #[test]
    fn create_dispute_on_deposit() {
        //-------------------------------------
        //Test Create Dispute on Deposit

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,9,1.1\ndispute,11,9,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        assert_eq!(
            saccounts.as_str(),
            "client,available,held,total,locked\n11,0.0,1.1,1.1,false\n"
        );
        assert_eq!(
            stransactions.as_str(),
            "type,client,tx,amount,status\ndeposit,11,9,1.1,-2\n"
        );
    }

    /// ### Test `create_dispute_on_withdrawal()`
    /// This test creates an Account from a "_withdrawal_" Transaction
    /// So, 1 Account and 2 Transaction must have been created
    /// Transaction (id: '5') must be marked as processed with `status` (`1`)
    /// Transaction (id: '7') must be marked as disputed with `status` (`-2`)
    /// The Dispute is accepted and Funds are placed on `held`
    #[test]
    fn create_dispute_on_withdrawal() {
        //-------------------------------------
        //Test Create Dispute on Withdrawal

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,5,1.1\nwithdrawal,11,7,0.5\ndispute,11,7,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        assert_eq!(
            saccounts.as_str(),
            "client,available,held,total,locked\n11,0.6,0.5,1.1,false\n"
        );

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec5 = txfact.lsttransactions.get(&5);
        let otxrec7 = txfact.lsttransactions.get(&7);

        assert!(otxrec5.is_some());
        assert!(otxrec7.is_some());
        assert_eq!(otxrec5.unwrap().status, 1);
        assert_eq!(otxrec7.unwrap().status, -2);
    }

    /// ### Test `resolve_dispute_on_deposit()`
    /// This test resolves a Dispute on a "_deposit_" Transaction
    /// So, 1 Account and 1 Transaction must have been created
    /// Transaction (id: '5') must be marked as processed with `status` (`1`)
    /// The Dispute is resolved and Funds are placed on `available`
    #[test]
    fn resolve_dispute_on_deposit() {
        //-------------------------------------
        //Test Resolves Dispute on Deposit

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,5,1.1\ndispute,11,5,\nresolve,11,5,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        assert_eq!(
            saccounts.as_str(),
            "client,available,held,total,locked\n11,1.1,0.0,1.1,false\n"
        );

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec5 = txfact.lsttransactions.get(&5);

        assert!(otxrec5.is_some());
        assert_eq!(otxrec5.unwrap().status, 1);
    }

    /// ### Test `resolve_dispute_on_deposit()`
    /// This test resolves a Dispute on a "_withdrawal_" Transaction
    /// So, 1 Account and 2 Transaction must have been created
    /// Transaction (id: '5') must be marked as processed with `status` (`1`)
    /// Transaction (id: '7') must be marked as processed with `status` (`1`)
    /// The Dispute is resolved and Funds are removed from `held` and `total`
    #[test]
    fn resolve_dispute_on_withdrawal() {
        //-------------------------------------
        //Test Resolves Dispute on Withdrawal

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,5,1.1\nwithdrawal,11,7,0.5\ndispute,11,7,\nresolve,11,7,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        assert_eq!(
            saccounts.as_str(),
            "client,available,held,total,locked\n11,0.6,0.0,0.6,false\n"
        );

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec5 = txfact.lsttransactions.get(&5);
        let otxrec7 = txfact.lsttransactions.get(&7);

        assert!(otxrec5.is_some());
        assert!(otxrec7.is_some());
        assert_eq!(otxrec5.unwrap().status, 1);
        assert_eq!(otxrec7.unwrap().status, 1);
    }

    /// ### Test `resolve_dispute_on_deposit()`
    /// This test runs a Chargeback of a "_deposit_" Transaction
    /// So, 1 Account and 1 Transaction must have been created
    /// Transaction (id: '5') must be marked as invalid with `status` (`-1`)
    /// The Funds are removed from `held` and `total`
    /// The Account is `locked`
    #[test]
    fn chargeback_on_deposit() {
        //-------------------------------------
        //Test Chargeback a Deposit

        let mut imp = super::MovementImporter::new();

        imp.set_debug(true);

        assert_eq!(
            imp.import_movements_str(
                &"type, client, tx, amount\ndeposit,11,5,1.1\ndispute,11,5,\nchargeback,11,5,\n",
                true
            ),
            0
        );

        let saccounts = imp.export_accounts_str();
        let stransactions = imp.export_transactions_str();

        println!("{}", saccounts.as_str());
        println!("{}", stransactions.as_str());

        assert_eq!(
            saccounts.as_str(),
            "client,available,held,total,locked\n11,0.0,0.0,0.0,true\n"
        );

        let txfact = TransactionFactory::from_str(stransactions.as_str(), true, true, false);
        let otxrec5 = txfact.lsttransactions.get(&5);

        assert!(otxrec5.is_some());
        assert_eq!(otxrec5.unwrap().status, -1);
    }
}
