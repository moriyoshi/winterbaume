/// Represents a cache cluster.
#[derive(Debug, Clone)]
pub struct CacheCluster {
    pub cache_cluster_id: String,
    pub status: String,
    pub engine: String,
    pub engine_version: String,
    pub cache_node_type: String,
    pub num_cache_nodes: i32,
    pub preferred_availability_zone: String,
    pub cache_subnet_group_name: Option<String>,
    pub replication_group_id: Option<String>,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a replication group.
#[derive(Debug, Clone)]
pub struct ReplicationGroup {
    pub replication_group_id: String,
    pub description: String,
    pub status: String,
    pub member_clusters: Vec<String>,
    pub cache_node_type: String,
    pub engine: String,
    pub automatic_failover: String,
    pub multi_az: String,
    pub arn: String,
    pub tags: Vec<Tag>,
    pub num_cache_clusters: i32,
}

/// Represents a cache subnet group.
#[derive(Debug, Clone)]
pub struct CacheSubnetGroup {
    pub name: String,
    pub description: String,
    pub subnet_ids: Vec<String>,
    pub vpc_id: String,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a cache parameter group.
#[derive(Debug, Clone)]
pub struct CacheParameterGroup {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a cache security group.
#[derive(Debug, Clone)]
pub struct CacheSecurityGroup {
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a snapshot.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub snapshot_name: String,
    pub cache_cluster_id: Option<String>,
    pub replication_group_id: Option<String>,
    pub status: String,
    pub engine: String,
    pub engine_version: String,
    pub cache_node_type: String,
    pub cache_subnet_group_name: Option<String>,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a user.
#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub engine: String,
    pub status: String,
    pub access_string: String,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a tag.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}
