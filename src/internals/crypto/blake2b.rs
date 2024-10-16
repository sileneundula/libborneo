use libsumatracrypt_rs::digest::SumatraBlake2b;

pub struct BorneoBLAKE2B;

impl BorneoBLAKE2B {
    pub fn new_with_key<T: AsRef<[u8]>>(bytes: T, key: T, digest_size: usize) -> String {
        return SumatraBlake2b::new(bytes, key, digest_size).to_string().to_string();
    }
    pub fn new<T: AsRef<[u8]>>(bytes: T, digest_size: usize) -> String {
        let empty_vec = vec![];
        return SumatraBlake2b::new(bytes.as_ref(), &empty_vec, digest_size).to_string().to_string()
    }
}