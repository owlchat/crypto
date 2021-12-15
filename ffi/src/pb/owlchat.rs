#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyPair {
    #[prost(bytes = "vec", tag = "1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub secret_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub seed: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitKeyPair {
    #[prost(bytes = "vec", tag = "1")]
    pub secret_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreKeyPair {
    #[prost(string, tag = "1")]
    pub paper_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupKeyPair {
    #[prost(bytes = "vec", tag = "1")]
    pub maybe_seed: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateMnemonic {
    #[prost(string, tag = "1")]
    pub phrase: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Encrypt {
    #[prost(bytes = "vec", tag = "1")]
    pub plaintext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decrypt {
    #[prost(bytes = "vec", tag = "1")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sign {
    #[prost(bytes = "vec", tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Verify {
    #[prost(bytes = "vec", tag = "1")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub sig: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffieHellmanKeyExchange {
    #[prost(bytes = "vec", tag = "1")]
    pub their_public_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof = "request::Body", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub body: ::core::option::Option<request::Body>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Body {
        #[prost(message, tag = "1")]
        GenerateKeyPair(()),
        #[prost(message, tag = "2")]
        InitKeyPair(super::InitKeyPair),
        #[prost(message, tag = "3")]
        RestoreKeyPair(super::RestoreKeyPair),
        #[prost(message, tag = "4")]
        BackupKeyPair(super::BackupKeyPair),
        #[prost(message, tag = "5")]
        ValidateMnemonic(super::ValidateMnemonic),
        #[prost(message, tag = "6")]
        Encrypt(super::Encrypt),
        #[prost(message, tag = "7")]
        Decrypt(super::Decrypt),
        #[prost(message, tag = "8")]
        Sign(super::Sign),
        #[prost(message, tag = "9")]
        Verify(super::Verify),
        #[prost(message, tag = "10")]
        DiffieHellmanKeyExchange(super::DiffieHellmanKeyExchange),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof = "response::Body", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub body: ::core::option::Option<response::Body>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        Unknown = 0,
        MissingRequestBody = 1,
        InvalidPublicKey = 2,
        InvalidSecretKey = 3,
        InvalidSignature = 4,
        InvalidSeed = 5,
        InvalidPaperKey = 6,
        NotInitialized = 7,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Body {
        #[prost(enumeration = "Error", tag = "1")]
        Error(i32),
        #[prost(message, tag = "2")]
        KeyPair(super::KeyPair),
        #[prost(string, tag = "3")]
        Mnemonic(::prost::alloc::string::String),
        #[prost(bool, tag = "4")]
        ValidMnemonic(bool),
        #[prost(bytes, tag = "5")]
        EncryptedMessage(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag = "6")]
        DecryptedMessage(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag = "7")]
        Signature(::prost::alloc::vec::Vec<u8>),
        #[prost(bool, tag = "8")]
        ValidSignature(bool),
        #[prost(bytes, tag = "9")]
        SharedSecret(::prost::alloc::vec::Vec<u8>),
    }
}
