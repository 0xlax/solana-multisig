## Solana Multisig


Exampler Multisig to execute groundless Solana transactions.


```mermaid
graph TD;
    Program-->Owner_parameter;
    Program-->Threshold_paramater;
    Owner_parameter-->Multisig;
    Threshold_parameter-->Multisig;
    Multisig-->approval_await;
    approval_await-->execute-transaction;
```