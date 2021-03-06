pub mod account;
pub mod transaction;

use crate::model::account::Account;
use crate::model::transaction::{Movement, Transaction};

//==============================================================================
// Structure Account Implementation

impl Account {
    /*----------------------------------------------------------------------------
     * Administration Methods
     */

    pub fn process_movement(
        &mut self,
        mvrecord: &mut Movement,
        mut otxrecord: Option<&mut Transaction>,
        bdebug: bool,
        bquiet: bool,
    ) {
        if !self.locked {
            match &mut otxrecord {
                Some(txrec) => {
                    //Referencing Transaction must exist and be valid

                    if txrec.client == self.client {
                        match mvrecord.tx_type.as_str() {
                            "deposit" => {
                                //------------------------
                                //Deposit Transaction

                                if bdebug && !bquiet {
                                    eprintln!(
                                        "Movement Processing '{}': + '{} / {}'",
                                        &txrec.tx_type, txrec.amount, self.available
                                    );
                                }

                                self.available += txrec.amount;
                                self.total += txrec.amount;

                                //Mark Transaction as accepted and processed
                                txrec.status = 1;

                                if bdebug && !bquiet {
                                    eprintln!(
                                        "Movement Processing '{}': funds '{} / {}'",
                                        &txrec.tx_type, self.available, self.total
                                    );
                                }
                            }
                            "withdrawal" => {
                                //------------------------
                                //Withdrawal Transaction

                                if bdebug && !bquiet {
                                    eprintln!(
                                        "Movement Processing '{}': - '{} / {}'",
                                        &txrec.tx_type, txrec.amount, self.available
                                    );
                                }

                                if txrec.amount <= self.available {
                                    self.available -= txrec.amount;
                                    self.total -= txrec.amount;

                                    self.available = (self.available * 10000.0).round() / 10000.0;
                                    self.total = (self.total * 10000.0).round() / 10000.0;

                                    //Mark Transaction as accepted and processed
                                    txrec.status = 1;
                                } else
                                //Unsufficiant Funds available
                                {
                                    //Mark Transaction as invalid
                                    txrec.status = -1;

                                    if !bquiet {
                                        eprintln!("Movement Processing Error: Amount '-{} / {}' is not available.", txrec.amount, self.available);
                                    }
                                } //if txrec.amount <= self.available

                                if bdebug && !bquiet {
                                    eprintln!(
                                        "Movement Processing '{}': funds '{} / {}'",
                                        &txrec.tx_type, self.available, self.total
                                    );
                                }
                            }
                            "dispute" => {
                                //------------------------
                                //Dispute Request

                                if txrec.status == 1 {
                                    match txrec.tx_type.as_str() {
                                        "deposit" => {
                                            self.available -= txrec.amount;
                                            self.held += txrec.amount;

                                            self.available =
                                                (self.available * 10000.0).round() / 10000.0;

                                            //Mark Transaction as disputed
                                            txrec.status = -2;
                                        }
                                        "withdrawal" => {
                                            self.held += txrec.amount;
                                            self.total += txrec.amount;

                                            //Mark Transaction as disputed
                                            txrec.status = -2;
                                        }
                                        _ => {
                                            if !bquiet {
                                                eprintln!("Movement Processing Error: Movement Type '{}' on '{}': disallowed ", &mvrecord.tx_type, &txrec.tx_type);
                                            }
                                        }
                                    } //match txrec.tx_type.as_str()
                                } else
                                //Transaction was not processed or rejected
                                {
                                    if !bquiet {
                                        eprintln!("Movement Processing Error: Transaction State '{}': invalid ", &txrec.status);
                                    }
                                } //if txrec.status == 1
                            }
                            "resolve" => {
                                //------------------------
                                //Resolve Request

                                if txrec.status == -2 {
                                    match txrec.tx_type.as_str() {
                                        "deposit" => {
                                            self.held -= txrec.amount;
                                            self.available += txrec.amount;

                                            self.held = (self.held * 10000.0).round() / 10000.0;

                                            //Mark Transaction as accepted and processed
                                            txrec.status = 1;
                                        }
                                        "withdrawal" => {
                                            self.held -= txrec.amount;
                                            self.total -= txrec.amount;

                                            self.held = (self.held * 10000.0).round() / 10000.0;
                                            self.total = (self.total * 10000.0).round() / 10000.0;

                                            //Mark Transaction as accepted and processed
                                            txrec.status = 1;
                                        }
                                        _ => {
                                            if !bquiet {
                                                eprintln!("Movement Processing Error: Movement Type '{}' on '{}': disallowed ", &mvrecord.tx_type, &txrec.tx_type);
                                            }
                                        }
                                    } //match txrec.tx_type.as_str()
                                } else
                                //Transaction was not disputed
                                {
                                    if !bquiet {
                                        eprintln!("Movement Processing Error: Transaction State '{}': not disputed", &txrec.status);
                                    }
                                } //if txrec.status == -2
                            }
                            "chargeback" => {
                                //------------------------
                                //Chargeback Request

                                if txrec.status == -2 {
                                    match txrec.tx_type.as_str() {
                                        "deposit" => {
                                            if txrec.amount <= self.total {
                                                self.held -= txrec.amount;
                                                self.total -= txrec.amount;

                                                self.held = (self.held * 10000.0).round() / 10000.0;
                                                self.total =
                                                    (self.total * 10000.0).round() / 10000.0;

                                                self.locked = true;

                                                //Mark Transaction as invalid
                                                txrec.status = -1;
                                            } else
                                            //Unsufficiant Funds available
                                            {
                                                if !bquiet {
                                                    eprintln!("Movement Processing Error: Amount '-{} / {}' is not available.", txrec.amount, self.total);
                                                }
                                            } //if txrec.amount <= self.total
                                        }
                                        _ => {
                                            if !bquiet {
                                                eprintln!("Movement Processing Error: Movement Type '{}' on '{}': disallowed ", &mvrecord.tx_type, &txrec.tx_type);
                                            }
                                        }
                                    } //match txrec.tx_type.as_str()
                                } else
                                //Transaction was not disputed
                                {
                                    if !bquiet {
                                        eprintln!("Movement Processing Error: Transaction State '{}': not disputed", &txrec.status);
                                    }
                                } //if txrec.status == -2
                            }
                            _ => {
                                if !bquiet {
                                    eprintln!(
                                        "Movement Processing Error: Movement Type '{}': unknown",
                                        &mvrecord.tx_type
                                    );
                                }
                            }
                        } //match mvrecord.tx_type.as_str()
                    } else
                    //Transaction does not belong to this Account
                    {
                        if !bquiet {
                            eprintln!(
                                "Movement Processing Error: Account Missmatch '{} / {} - {}'.",
                                &self.client, &txrec.client, &txrec.tx
                            );
                        }
                    } //if txrec.client != self.client
                }
                None => {
                    if !bquiet {
                        eprintln!("Movement Processing Error: Transaction (id: '{}') does not exist. Movement omitted.", &mvrecord.tx);
                    }
                }
            } //match & otxrecord
        } else {
            //Account is locked and Movements stay pending
            if !bquiet {
                eprintln!(
                    "Movement Processing Error: Account (id: '{}') is locked. Movement refused.",
                    &self.client
                );
            }
        }
    }
}
