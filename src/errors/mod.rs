use thiserror::Error;

#[derive(Error,Debug)]
pub enum BorneoErrors {
    // [0x0000] Encoding Errors
    #[error("Encoding Error Found In {Context:?} Using The Encoding {Encoding:?} With The Following Message: {Message}")]
    EncodingError {
        Encoding: EncodingTypesErrors,
        Context: ContextTypeErrors,
        Message: String, 
    },

    // [0x0001] Serialization Errors
    #[error("Serialization Error In {Serde:?} For Context {Context:?} With Message: {Message}")]
    SerializationError {
        Serde: SerializationTypeErrors,
        Context: ContextTypeErrors,
        Message: String,
    }
}

#[derive(Debug)]
pub enum EncodingTypesErrors {
    HEX,
    BASE32z,
    BASE58,
    BASE64,
}

#[derive(Debug)]
pub enum ContextTypeErrors {
    BlockID,
    BorneoAccount,
}

#[derive(Debug)]
pub enum SerializationTypeErrors {
    JSON,
    YAML,
    BINCODE,
}