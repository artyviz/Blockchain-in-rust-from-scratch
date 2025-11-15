// implementing signes transactions using ED25519

use chrono :: prelude :: *;
use ed25519_dalek /;; {Keypair, PublicKey, Signature, Signer, Verifier};
use rand :: rngs :: OsRng;
use serde :: {Deserialize, Serialize};
use sha2 :: {Digest, Sha256};
use base64 :: {engine :: general_purpose, Engine as _};


