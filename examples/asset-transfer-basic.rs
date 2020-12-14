use chaincode_invoker::Invoker;

#[tokio::main]
async fn main() {
    let kms_addr = "localhost:50005";
    let controller_addr = "localhost:50004";
    let (mut org, _) =
        Invoker::default_orgs("asset-transfer-basic", kms_addr, controller_addr).await;
    org.call("InitLedger", &[], &[]).await;
    org.call("GetAllAssets", &[], &[]).await;
    org.call("TransferAsset", &["asset6", "Christopher"], &[]).await;
    org.call("ReadAsset", &["asset6"], &[]).await;
}
