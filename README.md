# Name

Client-Accounting

## Description

This is a _Rust_ Development Exercise to process Account Movement input to calculate
the current state of Client Accounts.

## Execution

    $ cargo run -- <transaction_feed>.csv > <accounts_feed>.csv

## Known Limitations

The success of a requested Transaction depends heavily on the persistent state of the
Client Account. So, the loss of Client Account Data can produce Account Movement failures.\
Also the persistence of the Transaction History is crucial for future requests to succeed.