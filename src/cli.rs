use cita_cloud_proto::common::Hash;

use crate::chaincode_tx::ChaincodeProposal;
use crate::chaincode_tx::ChaincodeProposalBuilder;
use crate::proposer::Proposer;
use std::collections::HashMap;

pub struct Cli {
    builder_map: HashMap<String /*org*/, ChaincodeProposalBuilder>,
    proposer: Proposer,
    current_org: Option<String>,
}

impl Cli {
    pub async fn new(kms_addr: &str, controller_addr: &str) -> Self {
        Self {
            builder_map: HashMap::new(),
            proposer: Proposer::new(kms_addr, controller_addr).await,
            current_org: None,
        }
    }

    pub async fn call(&mut self, method: &str, args: &[&str], transient_map:&[(&str, &str)]) {
        let builder = self.builder_map
            .get_mut(self.current_org.as_ref().unwrap())
            .unwrap();
        let proposal = builder.build(method, args, transient_map);
        self.proposer.propose(proposal.dump()).await;
    }

    pub fn create_org(&mut self, org: String, channel_id: String, mspid: String, id_bytes: Vec<u8>) {
        let builder = ChaincodeProposalBuilder::new(channel_id, mspid, id_bytes);
        self.builder_map.insert(org, builder);
    }

}
