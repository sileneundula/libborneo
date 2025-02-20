pub struct ContainerMetadata {
    pivotal: String,
    
    description: String,
    common_name: String,
    author: String,
    version: String,
    license: String,
    tags: Vec<String>,
    dependencies: Vec<String>,
    container_type: String,
    container_id: String,
    container_version: String,
    container_hash: String,
    container_signature: String,
    container_timestamp: String,

}