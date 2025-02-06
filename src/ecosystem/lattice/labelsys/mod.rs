pub trait TxContainer {
    const label: &str = "LABEL_TXCONTAINER";
    fn serialize(&self) -> String;
}