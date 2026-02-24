#[derive(Debug, Clone)]
pub struct DaxCluster {
    pub cluster_name: String,
    pub cluster_arn: String,
    pub node_type: String,
    pub status: String,
    pub description: String,
    pub iam_role_arn: String,
    pub replication_factor: i32,
    pub sse_enabled: bool,
    pub cluster_endpoint_encryption_type: String,
    pub tags: Vec<DaxTag>,
}

#[derive(Debug, Clone)]
pub struct DaxTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct DaxParameterGroup {
    pub parameter_group_name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct DaxSubnetGroup {
    pub subnet_group_name: String,
    pub description: String,
    pub subnet_ids: Vec<String>,
    pub vpc_id: Option<String>,
}
