use crate::proposal::ChaincodeProposalBuilder;
use crate::sender::Sender;

pub struct Invoker {
    sender: Sender,
    builder: ChaincodeProposalBuilder,
}

impl Invoker {
    pub async fn new(
        kms_addr: &str,
        controller_addr: &str,
        channel_id: &str,
        mspid: &str,
        id_bytes: Vec<u8>,
    ) -> Self {
        Self {
            sender: Sender::new(kms_addr, controller_addr).await,
            builder: ChaincodeProposalBuilder::new(
                channel_id.to_string(),
                mspid.to_string(),
                id_bytes,
            ),
        }
    }

    pub async fn default_orgs(kms_addr: &str, controller_addr: &str) -> (Self, Self) {
        let channel_id = "cita-cloud";
        // certs are from fabric-samples
        let org1_mspid = "Org1MSP";
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

        let org2_mspid = "Org2MSP";
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

        let org1 = Self::new(
            kms_addr,
            controller_addr,
            channel_id,
            org1_mspid,
            org1_cert.as_bytes().to_vec(),
        )
        .await;

        let org2 = Self::new(
            kms_addr,
            controller_addr,
            channel_id,
            org2_mspid,
            org2_cert.as_bytes().to_vec(),
        )
        .await;
        (org1, org2)
    }

    pub async fn call(&mut self, method: &str, args: &[&str], transient_map: &[(&str, &str)]) {
        let proposal = self.builder.build(method, args, transient_map);
        self.sender.send(proposal.dump()).await;
    }
}
