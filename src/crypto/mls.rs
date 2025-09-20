use openmls::{
    prelude::*,
    prelude::tls_codec::*,
};
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;


pub fn ciphersuite() -> Ciphersuite {
    Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519
}

pub fn provider() -> OpenMlsRustCrypto {
    OpenMlsRustCrypto::default()
}

pub fn generate_credential(name: &str) -> (CredentialWithKey, SignatureKeyPair) {
    let provider = provider();
    let credential = BasicCredential::new(name.into());
    let signature_keys =
        SignatureKeyPair::new(ciphersuite().signature_algorithm()).unwrap();

    signature_keys.store(provider.storage()).unwrap();

    (
        CredentialWithKey {
            credential: credential.into(),
            signature_key: signature_keys.public().into(),
        },
        signature_keys,
    )
}

pub fn generate_key_package(
    credential_with_key: CredentialWithKey,
    signer: &SignatureKeyPair,
) -> KeyPackageBundle {
    KeyPackage::builder()
        .build(ciphersuite(), &provider(), signer, credential_with_key)
        .unwrap()
}
