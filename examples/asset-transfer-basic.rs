use chaincode_invoker::Invoker;

#[tokio::main]
async fn main() {
    let kms_addr = "localhost:50005";
    let controller_addr = "localhost:50004";
    let (mut org, _) =
        Invoker::default_orgs("asset-transfer-basic", kms_addr, controller_addr).await;
    org.call("InitLedger", &["asset8", "blue", "16", "Kelly", "750"], &[])
        .await;
    org.call("ReadAsset", &["asset8"], &[]).await;
    org.call("GetAllAssets", &[], &[]).await;
    org.call("TransferAsset", &["asset1", "Alice"], &[]).await;
    org.call("ReadAsset", &["asset1"], &[]).await;
}
