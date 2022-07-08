## Solana Multisig


Exampler Multisig to execute groundless Solana transactions.

```mermaid
  Multisig
      Owner-->Multisig: parameter1
      Threshold-->Multisig: parameter2
      Multisig-->Transactions
      Transactions--> execute_transactions: waiting for approval
```