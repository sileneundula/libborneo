use libsumatracrypt_rs::digest::SumatraBlake3;

pub struct BorneoBLAKE3;

impl BorneoBLAKE3 {
    pub fn new<T: AsRef<[u8]>>(bytes: T) -> String {
        return SumatraBlake3::new(bytes).to_string().to_string()
    }
}