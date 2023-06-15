use std::path::Path;

use candid::Principal;
use ic_agent::identity::{BasicIdentity, Secp256k1Identity};
use ic_agent::Identity;

pub enum GenericIdentity {
    Secp256k1Identity(Secp256k1Identity),
    BasicIdentity(BasicIdentity),
}

impl TryFrom<&Path> for GenericIdentity {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Secp256k1Identity::from_pem_file(path)
            .map(GenericIdentity::from)
            .or(BasicIdentity::from_pem_file(path).map(GenericIdentity::from))
            .map_err(|e| anyhow::anyhow!("pem error {e}"))
    }
}

impl Identity for GenericIdentity {
    fn sender(&self) -> std::result::Result<Principal, String> {
        match self {
            Self::BasicIdentity(identity) => identity.sender(),
            Self::Secp256k1Identity(identity) => identity.sender(),
        }
    }

    fn sign(&self, blob: &[u8]) -> std::result::Result<ic_agent::Signature, String> {
        match self {
            Self::BasicIdentity(identity) => identity.sign(blob),
            Self::Secp256k1Identity(identity) => identity.sign(blob),
        }
    }
}

impl From<Secp256k1Identity> for GenericIdentity {
    fn from(value: Secp256k1Identity) -> Self {
        Self::Secp256k1Identity(value)
    }
}

impl From<BasicIdentity> for GenericIdentity {
    fn from(value: BasicIdentity) -> Self {
        Self::BasicIdentity(value)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use std::path::Path;

    #[test]
    fn should_get_identity_from_pem_file() {
        let path = Path::new("./tests/identity/identity.pem");

        assert!(GenericIdentity::try_from(path).is_ok());
        assert!(matches!(
            GenericIdentity::try_from(path).unwrap(),
            GenericIdentity::Secp256k1Identity(_)
        ));
    }

    #[test]
    fn should_get_sender_from_identity() {
        let path = Path::new("./tests/identity/identity.pem");
        let identity = GenericIdentity::try_from(path).unwrap();
        let expected =
            Principal::from_text("zrrb4-gyxmq-nx67d-wmbky-k6xyt-byhmw-tr5ct-vsxu4-nuv2g-6rr65-aae")
                .unwrap();

        let principal = identity.sender().unwrap();

        assert_eq!(expected, principal);
    }

    #[test]
    fn identity_should_sign() {
        let path = Path::new("./tests/identity/identity.pem");
        let identity = GenericIdentity::try_from(path).unwrap();
        let blob = &[0xca, 0xfe, 0xba, 0xbe];

        let signature = identity.sign(blob).unwrap();

        assert!(signature.signature.is_some());
    }
}
