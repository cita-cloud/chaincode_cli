use crate::common;
use crate::msp;
use crate::protos;
use crate::protos::chaincode_message::Type as ChaincodeMsgType;
use crate::protos::ChaincodeMessage;
use prost::Message;
use std::collections::HashMap;

trait MessageDump {
    fn dump(&self) -> Vec<u8>;
}

impl<T: Message> MessageDump for T {
    fn dump(&self) -> Vec<u8> {
        let mut payload = vec![];
        self.encode(&mut payload).unwrap();
        payload
    }
}

fn get_timestamp() -> Option<prost_types::Timestamp> {
    use std::convert::TryFrom;
    use std::time::SystemTime;
    let now = SystemTime::now();
    prost_types::Timestamp::try_from(now).ok()
}

pub struct ChaincodeProposal {
    method: String,
    args: Vec<Vec<u8>>,
    transient_map: HashMap<String, Vec<u8>>,

    mspid: String,
    id_bytes: Vec<u8>,

    tx_id: String,
    channel_id: String,
    nonce: Vec<u8>,
}

impl ChaincodeProposal {
    pub fn dump(&self) -> Vec<u8> {
        let header = {
            let channel_header = common::ChannelHeader {
                r#type: common::HeaderType::EndorserTransaction as i32,
                channel_id: self.channel_id.clone(),
                timestamp: get_timestamp(),
                tx_id: self.tx_id.clone(),
                ..Default::default()
            }
            .dump();
            let creator = msp::SerializedIdentity {
                mspid: self.mspid.clone(),
                id_bytes: self.id_bytes.clone(),
            }
            .dump();
            let signature_header = common::SignatureHeader {
                creator,
                nonce: self.nonce.clone(),
            }
            .dump();
            common::Header {
                channel_header,
                signature_header,
            }
            .dump()
        };
        let input = {
            let args: Vec<Vec<u8>> = [&[self.method.as_bytes().to_vec()], &self.args[..]].concat();
            protos::ChaincodeInput {
                args,
                decorations: HashMap::new(),
                is_init: false,
            }
            .dump()
        };
        let payload = protos::ChaincodeProposalPayload {
            // method and args
            input: input.clone(),
            // private data key-value map
            transient_map: self.transient_map.clone(),
        }
        .dump();
        let proposal = protos::Proposal {
            header,
            payload,
            extension: vec![],
        }
        .dump();
        let signed_proposal = protos::SignedProposal {
            proposal_bytes: proposal,
            // The signature over the proposal_bytes signed by the client.
            // We don't use it.
            signature: vec![],
        };
        ChaincodeMessage {
            r#type: ChaincodeMsgType::Transaction as i32,
            // This payload contains the function name to be called and its args
            payload: input,
            txid: self.tx_id.clone(),
            channel_id: self.channel_id.clone(),
            // signed_proposal is used to provide caller's identity and this call's private data.
            proposal: Some(signed_proposal),
            ..Default::default()
        }
        .dump()
    }
}

pub struct ChaincodeProposalBuilder {
    channel_id: String,
    mspid: String,
    id_bytes: Vec<u8>,
    nonce: u64,
}

impl ChaincodeProposalBuilder {
    pub fn new(channel_id: String, mspid: String, id_bytes: Vec<u8>) -> Self {
        ChaincodeProposalBuilder {
            channel_id,
            mspid,
            id_bytes,
            nonce: 0,
        }
    }

    pub fn build<T: AsRef<str>>(
        &mut self,
        method: T,
        args: &[T],
        transient_map: &[(T, T)],
    ) -> ChaincodeProposal {
        let method = method.as_ref().to_string();
        let args = args
            .iter()
            .map(|arg| arg.as_ref().as_bytes().to_vec())
            .collect();
        let transient_map: HashMap<String, Vec<u8>> = transient_map
            .iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().as_bytes().to_vec()))
            .collect();

        let nonce = self.nonce.to_string();
        self.nonce += 1;
        ChaincodeProposal {
            method,
            args,
            transient_map,
            tx_id: nonce.clone(),
            mspid: self.mspid.clone(),
            id_bytes: self.id_bytes.clone(),
            nonce: nonce.as_bytes().to_vec(),
            channel_id: self.channel_id.clone(),
        }
    }
}
