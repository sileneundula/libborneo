pub struct BorneoDigest(String);

impl BorneoDigest {
    pub fn new<T: AsRef<str>>(hash: T) -> Self {
        return Self(hash.as_ref().to_string())
    }
}