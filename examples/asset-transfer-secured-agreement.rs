use chaincode_invoker::Invoker;

#[tokio::main]
async fn main() {
    let kms_addr = "localhost:50005";
    let controller_addr = "localhost:50004";
    let (mut org1, mut org2) = Invoker::default_orgs(kms_addr, controller_addr).await;
    org1.call(
        "CreateAsset",
        &["asset1", "A new asset for Org1MSP"],
        &[("asset_properties", "asset1's property")],
    )
    .await;
    org1.call("GetAssetPrivateProperties", &["asset1"], &[])
        .await;
    org1.call("ReadAsset", &["asset1"], &[]).await;
    org1.call(
        "ChangePublicDescription",
        &["asset1", "This asset is for sale"],
        &[],
    )
    .await;
    org1.call("ReadAsset", &["asset1"], &[]).await;
    org2.call(
        "ChangePublicDescription",
        &["asset1", "The worst asset"],
        &[],
    )
    .await;
    org1.call("ReadAsset", &["asset1"], &[]).await;
    org1.call(
        "AgreeToSell",
        &["asset1"],
        &[("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":110}")]
    ).await;
    org1.call("GetAssetSalesPrice", &["asset1"], &[]).await;
    org2.call(
        "VerifyAssetProperties",
        &["asset1"],
        &[("asset_properties", "asset1's property")],
    )
    .await;
    org2.call(
        "AgreeToBuy",
        &["asset1"],
        &[("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    org2.call("GetAssetBidPrice", &["asset1"], &[]).await;
    org1.call(
        "TransferAsset",
        &["asset1","Org2MSP"],
        &[("asset_properties", "asset1's property"), ("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    org1.call(
        "AgreeToSell",
        &["asset1"],
        &[("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    org1.call(
        "TransferAsset",
        &["asset1","Org2MSP"],
        &[("asset_properties", "asset1's property"), ("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    org2.call("ReadAsset", &["asset1"], &[]).await;
    org2.call("GetAssetPrivateProperties", &["asset1"], &[])
        .await;
    org2.call(
        "ChangePublicDescription",
        &["asset1", "This asset is not for sale"],
        &[],
    )
    .await;
    org2.call("ReadAsset", &["asset1"], &[]).await;
}
