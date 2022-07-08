## Solana Multisig


Exampler Multisig to execute groundless Solana transactions.

```mermaid
  Multisig;
      Owner-->Multisig;
      Threshold-->Multisig;
      Multisig-->Transactions;
      Transactions--> execute_transactions;
```