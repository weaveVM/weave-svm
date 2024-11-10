<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About
This repository is a customized fork of Agave's SVM [PayTube](https://github.com/anza-xyz/agave/tree/master/svm/examples/paytube) state channel (payment channel) that integrates [WeaveVM](https://wvm.dev) data settlement alongside Solana's. ***This repository is for educational purposes only***

## Build & Run

Before running the [native_sol](./tests/native_sol.rs) transfer test, ensure you have provided a WeaveVM-funded EOA in the `.env` file according to the `.env.example` format. You can obtain tWVM from the [WeaveVM faucet](https://wvm.dev/faucet).

```bash
git clone https://github.com/WeaveVM/wvm-svm.git

cd wvm-svm

cargo build && cargo test -- --nocapture
```

## PayTube-WeaveVM Architecture

```mermaid

sequenceDiagram
   participant PayTube as PayTube-WeaveVM VM
   participant Bob
   participant Alice 
   participant Will
   participant Solana
   participant DataProcessor as Data Processor
   participant WeaveVM

   rect rgb(200, 200, 255)
       Note over PayTube: Channel 1
       Bob->>Alice: Multiple transactions
       Alice->>Bob: Multiple transactions
   end

   rect rgb(200, 200, 255)
       Note over PayTube: Channel 2 
       Bob->>Alice: Multiple transactions
       Alice->>Will: Multiple transactions
   end

   PayTube->>Solana: Submit final ledger
   Note right of Solana: Final State<br/>Alice: x<br/>Bob: x<br/>Will: x

   PayTube->>DataProcessor: Submit transaction data
   Note right of DataProcessor: Process Data<br/>1. Encode in Borsh<br/>2. Compress with Brotli

   DataProcessor->>WeaveVM: Store processed data
   Note right of WeaveVM: Permanent Storage

   Note over Solana,WeaveVM: Settlement Complete<br/>- Solana: Final Balances<br/>- WeaveVM: Transaction History & Final Balances

```
## License
This repository is licensed under the [MIT License](./LICENSE)
