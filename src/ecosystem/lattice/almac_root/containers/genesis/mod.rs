pub struct GenesisContainer {
    common_name: String,
    description: String,
    
    container_type: String,
    container_id: String,
}

impl GenesisContainer {
    pub fn new(common_name: String, description: String) -> Self {
        Self {
            common_name,
            description,
        }
    }

    pub fn common_name(&self) -> &str {
        &self.common_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}