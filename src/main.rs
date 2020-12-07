mod common {
    tonic::include_proto!("common");
}

mod msp {
    tonic::include_proto!("msp");
}

mod protos {
    tonic::include_proto!("protos");
}

mod kvrwset {
    tonic::include_proto!("kvrwset");
}

mod queryresult {
    tonic::include_proto!("queryresult");
}

mod chaincode_proposal;
mod cli;
mod proposer;

use cli::Cli;

#[tokio::main]
async fn main() {
    let kms_addr = "localhost:50005";
    let controller_addr = "localhost:50004";
    let mut cli = Cli::new(kms_addr, controller_addr).await;
    cli.call(
        "CreateAsset",
        &["asset1", "A new asset for Org1MSP"],
        &[("asset_properties", "asset1's property")],
    )
    .await;
    cli.call("GetAssetPrivateProperties", &["asset1"], &[])
        .await;
    cli.call("ReadAsset", &["asset1"], &[]).await;
    cli.call(
        "ChangePublicDescription",
        &["asset1", "This asset is for sale"],
        &[],
    )
    .await;
    cli.call("ReadAsset", &["asset1"], &[]).await;
    cli.call(
        "ChangePublicDescription",
        &["asset1", "The worst asset"],
        &[],
    )
    .await;
    cli.call("ReadAsset", &["asset1"], &[]).await;
    cli.call(
        "AgreeToSell",
        &["asset1"],
        &[("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":110}")]
    ).await;
    cli.call("GetAssetSalesPrice", &["asset1"], &[]).await;
    cli.call(
        "VerifyAssetProperties",
        &["asset1"],
        &[("asset_properties", "asset1's property")],
    )
    .await;
    cli.call(
        "AgreeToBuy",
        &["asset1"],
        &[("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    cli.call("GetAssetBidPrice", &["asset1"], &[]).await;
    cli.call(
        "TransferAsset",
        &["asset1","Org2MSP"],
        &[("asset_properties", "asset1's property"), ("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    cli.call(
        "AgreeToSell",
        &["asset1"],
        &[("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    cli.call(
        "TransferAsset",
        &["asset1","Org2MSP"],
        &[("asset_properties", "asset1's property"), ("asset_price", "{\"asset_id\":\"asset1\",\"trade_id\":\"109f4b3c50d7b0df729d299bc6f8e9ef9066971f\",\"price\":100}")]
    ).await;
    cli.call("ReadAsset", &["asset1"], &[]).await;
    cli.call("GetAssetPrivateProperties", &["asset1"], &[])
        .await;
    cli.call(
        "ChangePublicDescription",
        &["asset1", "This asset is not for sale"],
        &[],
    )
    .await;
    cli.call("ReadAsset", &["asset1"], &[]).await;
}
