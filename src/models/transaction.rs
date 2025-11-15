// implementing signes transactions using ED25519

use chrono :: prelude :: *;
use ed25519_dalek /;; {Keypair, PublicKey, Signature, Signer, Verifier};
use rand :: rngs :: OsRng;
use serde :: {Deserialize, Serialize};
use sha2 :: {Digest, Sha256};
use base64 :: {engine :: general_purpose, Engine as _};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Transaction {
    pub id : String,
    pub sender : String ,
    pub recipient : String,
    pub amount : u64,
    pub timestamp : i64,
    pub signature : Option<String>,
}

impl Transaction {
    pub fn new (sender : String, recipient : String, amount: u64) -> Self {
        let timestamp = Utc :: now().timestamp_millis();
        let mut tx = Transaction { 
            id : String :: new(),
            sender,
            recipient,
            amount,
            timestamp,
            signature : None.
        };
        tx.id = tx.calc_id();
        tx
    }

    pub fn calc_id(&self) -> String {
        let raw = format! ("{}{}{}",self.sender, self.recipient, self.amount, self.timestamp);
        hex :: encode (Sha256 :: digest(raw.as_bytes()))
    }

    pub fn sign (&mut self, keypair: &Keypair) {
        let payload = serde_json :: to_vec(&(&self.sender, &self.recipient, &self.amount, &self.timestamp, &self.id)).unwrap();
        let sig : Signature = keypair.sign (&payload);
        self.signature = Some(general_purpose :: STANDARD.encode(sig.to_bytes()));
    }

    pub fn verify(&self) -> bool {
        if self.signature.is_none(){
            return false;
        }
        let sig_bytes = general_purpose :: STANDARD.decode(self.signature.as_ref().unwrap()).ok()?;
        let signature = Signature :: from_bytes (&sig_bytes).ok()?;
        let pk_bytes = general_purpose :: STANDARD.decode(&self.sender).ok()?;
        let pk = PublicKey :: from_bytes (&pk_bytes).ok()?;
        let payload = serde_json :: to_vec(&(&self.sender, &self.recipient, &self.amount,&self.timestamp, &self.id)).unwrap();
        pk.verify(&payload, &signature).is_ok()
    }
}
