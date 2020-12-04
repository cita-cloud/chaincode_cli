use cita_cloud_proto::common::Hash;

use crate::chaincode_proposal::ChaincodeProposal;
use crate::chaincode_proposal::ChaincodeProposalBuilder;
use crate::proposer::Proposer;
use std::collections::HashMap;

pub struct Cli {
    builder_map: HashMap<String /*mspid*/, ChaincodeProposalBuilder>,
    proposer: Proposer,
    current_org: Option<String>,
}

impl Cli {
    pub async fn new(kms_addr: &str, controller_addr: &str) -> Self {
        let mut cli = Self {
            builder_map: HashMap::new(),
            proposer: Proposer::new(kms_addr, controller_addr).await,
            current_org: None,
        };
        cli.init();
        cli
    }

    fn init(&mut self) {
        let channel_id = "cita-cloud".to_string();
        // certs are from fabric-samples
        let org1_mspid = "Org1MSP".to_string();
        let org1_cert = "-----BEGIN CERTIFICATE-----
MIICJzCCAc6gAwIBAgIQKxBV8QdNKmtS2wu7DExPWzAKBggqhkjOPQQDAjBzMQsw
CQYDVQQGEwJVUzETMBEGA1UECBMKQ2FsaWZvcm5pYTEWMBQGA1UEBxMNU2FuIEZy
YW5jaXNjbzEZMBcGA1UEChMQb3JnMS5leGFtcGxlLmNvbTEcMBoGA1UEAxMTY2Eu
b3JnMS5leGFtcGxlLmNvbTAeFw0yMDEwMTIwODI5MDBaFw0zMDEwMTAwODI5MDBa
MGoxCzAJBgNVBAYTAlVTMRMwEQYDVQQIEwpDYWxpZm9ybmlhMRYwFAYDVQQHEw1T
YW4gRnJhbmNpc2NvMQ0wCwYDVQQLEwRwZWVyMR8wHQYDVQQDExZwZWVyMC5vcmcx
LmV4YW1wbGUuY29tMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEMutVyQ9OX0Ui
29Cn/E4+eq3SZl1LlSlqMNDup5KQqo9lVY2CKcNuWeKeV+YoDijQRPTLW7o2ZDuJ
yn7ZvtOBXaNNMEswDgYDVR0PAQH/BAQDAgeAMAwGA1UdEwEB/wQCMAAwKwYDVR0j
BCQwIoAg6WZDnHPhiJpYBVNBJTwE0YW45ThbtJt7qhk7WivY+AIwCgYIKoZIzj0E
AwIDRwAwRAIgDNvR3C6j+SVncmmr0GvcomW3j3SqbQ4toRRMOiRa56ICIHHcMiAM
S4u7BSot5a2st7igwkukLRk2e5TwFhECcZDA
-----END CERTIFICATE-----";
        self.create_org(
            channel_id.clone(),
            org1_mspid,
            org1_cert.as_bytes().to_vec(),
        );

        let org2_mspid = "Org2MSP".to_string();
        let org2_cert = "-----BEGIN CERTIFICATE-----
MIICJzCCAc6gAwIBAgIQLn1I5xYJ7cb+d5MN8+U+tzAKBggqhkjOPQQDAjBzMQsw
CQYDVQQGEwJVUzETMBEGA1UECBMKQ2FsaWZvcm5pYTEWMBQGA1UEBxMNU2FuIEZy
YW5jaXNjbzEZMBcGA1UEChMQb3JnMi5leGFtcGxlLmNvbTEcMBoGA1UEAxMTY2Eu
b3JnMi5leGFtcGxlLmNvbTAeFw0yMDEwMTIwODI5MDBaFw0zMDEwMTAwODI5MDBa
MGoxCzAJBgNVBAYTAlVTMRMwEQYDVQQIEwpDYWxpZm9ybmlhMRYwFAYDVQQHEw1T
YW4gRnJhbmNpc2NvMQ0wCwYDVQQLEwRwZWVyMR8wHQYDVQQDExZwZWVyMC5vcmcy
LmV4YW1wbGUuY29tMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEPgfrahKAsBxC
mJZSFblj7f2pgiO3sZ2I4I24YB9YKsFFZVXO2USqGnndxhYxHdG2gryZGQ4jDB2B
pgzhSEfUeaNNMEswDgYDVR0PAQH/BAQDAgeAMAwGA1UdEwEB/wQCMAAwKwYDVR0j
BCQwIoAgNIhFkVF64ELH7I2LMF5ozCFDVTDpODp2NUgy9w4tEPQwCgYIKoZIzj0E
AwIDRwAwRAIgXEKPv1tgXjum6aikVT3AJIjig1TF7KCojogDrZqu3lACIGdji2sX
Jfn1p8cfo4BPd3tSllZEIbXE2uCMkKE4LGmo
-----END CERTIFICATE-----";
        self.create_org(channel_id, org2_mspid, org2_cert.as_bytes().to_vec());
    }

    pub async fn call(&mut self, method: &str, args: &[&str], transient_map: &[(&str, &str)]) {
        let builder = self
            .builder_map
            .get_mut(self.current_org.as_ref().unwrap())
            .unwrap();
        let proposal = builder.build(method, args, transient_map);
        self.proposer.propose(proposal.dump()).await;
    }

    pub fn create_org(&mut self, channel_id: String, mspid: String, id_bytes: Vec<u8>) {
        let builder = ChaincodeProposalBuilder::new(channel_id, mspid.clone(), id_bytes);
        self.builder_map.insert(mspid, builder);
    }
}
