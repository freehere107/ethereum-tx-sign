use ethereum_types::{H160, H256, U256};
use rlp::RlpStream;
use tiny_keccak::keccak256;
use secp256k1::key::SecretKey;
use secp256k1::Message;
use secp256k1::Secp256k1;

/// Description of a Transaction, pending or in the chain.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RawTransaction {
    /// Nonce
    pub nonce: U256,
    /// Recipient (None when contract creation)
    pub to: Option<H160>,
    /// Transfered value
    pub value: U256,
    /// Gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount
    pub gas: U256,
    /// Input data
    pub data: Vec<u8>
}

impl RawTransaction {

    pub fn hash(&self, chain_id: u8) -> Vec<u8> {
        let mut hash = RlpStream::new(); 
        hash.begin_unbounded_list();
        self.encode(&mut hash);
        hash.append(&mut vec![chain_id]);
        hash.append(&mut U256::zero());
        hash.append(&mut U256::zero());
        hash.complete_unbounded_list();
        keccak256_hash(&hash.out())
    }

    pub fn encode(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        if let Some(ref t) = self.to {
            s.append(t);
        } else {
            s.append(&vec![]);
        }
        s.append(&self.value);
        s.append(&self.data);
    }
}

fn keccak256_hash(bytes: &[u8]) -> Vec<u8> {
    keccak256(bytes).into_iter().cloned().collect()
}

pub struct EcdsaSig {
    v: Vec<u8>,
    r: Vec<u8>,
    s: Vec<u8>
}

mod test {

    #[test]
    fn test_signs_transaction_eth() {
        use std::io::Read;
        use std::fs::File;
        use ethereum_types::*;
        use raw_transaction::RawTransaction;
        use serde_json;

        #[derive(Deserialize)]
        struct Signing {
            signed: Vec<u8>,
            private_key: H256 
        }

        let mut file = File::open("./test/test_txs.json").unwrap();
        let mut f_string = String::new();
        file.read_to_string(&mut f_string).unwrap();
        let txs: Vec<(RawTransaction, Signing)> = serde_json::from_str(&f_string).unwrap();
        let chain_id = 0;
        for (tx, signed) in txs.into_iter() {
            assert_eq!(signed.signed, tx.sign(&signed.private_key, &chain_id));
        }
    }

    #[test]
    fn test_signs_transaction_ropsten() {
        use std::io::Read;
        use std::fs::File;
        use ethereum_types::*;
        use raw_transaction::RawTransaction;
        use serde_json;

        #[derive(Deserialize)]
        struct Signing {
            signed: Vec<u8>,
            private_key: H256
        } 

        let mut file = File::open("./test/test_txs_ropsten.json").unwrap();
        let mut f_string = String::new();
        file.read_to_string(&mut f_string).unwrap();
        let txs: Vec<(RawTransaction, Signing)> = serde_json::from_str(&f_string).unwrap();
        let chain_id = 3;
        for (tx, signed) in txs.into_iter() {
            assert_eq!(signed.signed, tx.sign(&signed.private_key, &chain_id));
        }
    }
}
