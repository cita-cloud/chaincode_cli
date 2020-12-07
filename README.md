# chaincode_cli

A tool for building `executor_chaincode`'s transactions.

## Usage

```rust
let kms_addr = "localhost:50005";
let controller_addr = "localhost:50004";

let channel_id = "cita-cloud";
let org1_mspid = "Org1MSP";
let org1_cert = /* cert file ommitted */;

let org1 = Cli::new(
    kms_addr,
    controller_addr,
    channel_id,
    org1_mspid,
    org1_cert.as_bytes().to_vec(),
).await;

org1.call(
    "CreateAsset", // method name
    &["asset1", "A new asset for Org1MSP"], // args
    &[("asset_properties", "asset1's property")], // transient_map(a key-value map representing the private data)
).await;
```

