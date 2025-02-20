pub trait BorneoContainer {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &str) -> Self;
    
    fn verify(&self) -> bool;
    fn get_id(&self) -> String;
    
    fn get_tx_type(&self) -> String;
    fn get_tx_action(&self) -> String;
    fn get_tx_hash(&self) -> String;
    fn get_tx_timestamp(&self) -> String;
    fn get_tx_signature(&self) -> String;
    fn get_tx_data(&self) -> String;
}