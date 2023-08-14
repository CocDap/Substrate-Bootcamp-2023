## Implement simple `add_number` extrinsic call

### Build the project
```
cargo build
```
Noted: Should be in root directory
### Run the project
```
./target/debug/node-template --dev
```
Noted: Should be in root directory

### Interact with Polkadotjs Explorer

+ Go to `https://polkadot.js.org/apps/#/explorer` -> Choose `Development` -> `Local Node` -> `Switch`

+ Execute Transaction: Go to `Developer` -> `Extrinsics` -> Click `system` -> Switch to `templateModule` -> Choose `addNumber` extrinsics -> Enter your input

+ View on-chain storage: Go to `Developer` -> `Chain state` -> Click `timestamp` -> Switch to `templateModule` -> choose `something()` -> Click `Add Symbol` -> view on-chain storage 

