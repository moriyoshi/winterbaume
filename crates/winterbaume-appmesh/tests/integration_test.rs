use aws_sdk_appmesh::config::BehaviorVersion;
use winterbaume_appmesh::AppMeshService;
use winterbaume_core::MockAws;

async fn make_appmesh_client() -> aws_sdk_appmesh::Client {
    let mock = MockAws::builder()
        .with_service(AppMeshService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appmesh::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_appmesh::Client::new(&config)
}

#[tokio::test]
async fn test_create_mesh() {
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("test-mesh")
        .send()
        .await
        .expect("create_mesh should succeed");

    let mesh = resp.mesh().expect("should have mesh");
    assert_eq!(mesh.mesh_name(), "test-mesh");
    assert!(mesh.metadata().is_some());
    let metadata = mesh.metadata().unwrap();
    assert!(metadata.arn().contains("test-mesh"));
    assert_eq!(
        mesh.status().and_then(|s| s.status()),
        Some(&aws_sdk_appmesh::types::MeshStatusCode::Active)
    );
}

#[tokio::test]
async fn test_describe_mesh() {
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("desc-mesh")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_mesh()
        .mesh_name("desc-mesh")
        .send()
        .await
        .expect("describe_mesh should succeed");

    let mesh = resp.mesh().unwrap();
    assert_eq!(mesh.mesh_name(), "desc-mesh");
    assert_eq!(
        mesh.status().and_then(|s| s.status()),
        Some(&aws_sdk_appmesh::types::MeshStatusCode::Active)
    );
}

#[tokio::test]
async fn test_list_meshes() {
    let client = make_appmesh_client().await;

    for name in ["mesh-a", "mesh-b", "mesh-c"] {
        client.create_mesh().mesh_name(name).send().await.unwrap();
    }

    let resp = client
        .list_meshes()
        .send()
        .await
        .expect("list_meshes should succeed");

    assert_eq!(resp.meshes().len(), 3);
}

#[tokio::test]
async fn test_delete_mesh() {
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("del-mesh")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_mesh()
        .mesh_name("del-mesh")
        .send()
        .await
        .expect("delete_mesh should succeed");

    let mesh = resp.mesh().unwrap();
    assert_eq!(
        mesh.status().and_then(|s| s.status()),
        Some(&aws_sdk_appmesh::types::MeshStatusCode::Deleted)
    );

    // After deletion, describe should fail
    let result = client.describe_mesh().mesh_name("del-mesh").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_duplicate_mesh_fails() {
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("dup-mesh")
        .send()
        .await
        .unwrap();

    let result = client.create_mesh().mesh_name("dup-mesh").send().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_mesh() {
    let client = make_appmesh_client().await;

    let result = client.describe_mesh().mesh_name("nonexistent").send().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_mesh_with_status_active() {
    // Port of moto test_create_list_update_describe_delete_mesh - verify ACTIVE status and metadata
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("mesh1")
        .send()
        .await
        .expect("create_mesh should succeed");

    let mesh = resp.mesh().unwrap();
    assert_eq!(mesh.mesh_name(), "mesh1");
    assert_eq!(
        mesh.status().and_then(|s| s.status()),
        Some(&aws_sdk_appmesh::types::MeshStatusCode::Active)
    );
    let metadata = mesh.metadata().unwrap();
    assert_eq!(metadata.version(), 1);
    assert!(!metadata.arn().is_empty());
    assert!(!metadata.mesh_owner().is_empty());
    assert!(!metadata.resource_owner().is_empty());
}

#[tokio::test]
async fn test_list_meshes_contains_created() {
    // Port of moto test - verify created meshes appear in list with correct names
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("list-mesh-a")
        .send()
        .await
        .unwrap();
    client
        .create_mesh()
        .mesh_name("list-mesh-b")
        .send()
        .await
        .unwrap();

    let resp = client.list_meshes().send().await.unwrap();
    let meshes = resp.meshes();
    assert_eq!(meshes.len(), 2);

    let names: Vec<&str> = meshes.iter().map(|m| m.mesh_name()).collect();
    assert!(names.contains(&"list-mesh-a"));
    assert!(names.contains(&"list-mesh-b"));
}

#[tokio::test]
async fn test_delete_mesh_then_list_empty() {
    // Port of moto - after deleting mesh, list should have fewer entries
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("to-delete-mesh")
        .send()
        .await
        .unwrap();
    client
        .create_mesh()
        .mesh_name("to-keep-mesh")
        .send()
        .await
        .unwrap();

    client
        .delete_mesh()
        .mesh_name("to-delete-mesh")
        .send()
        .await
        .expect("delete_mesh should succeed");

    let resp = client.list_meshes().send().await.unwrap();
    assert_eq!(resp.meshes().len(), 1);
    assert_eq!(resp.meshes()[0].mesh_name(), "to-keep-mesh");
}

#[tokio::test]
async fn test_mesh_metadata_fields() {
    // Port of moto - verify mesh metadata fields: arn, meshOwner, resourceOwner
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("meta-mesh")
        .send()
        .await
        .unwrap();

    let mesh = resp.mesh().unwrap();
    let metadata = mesh.metadata().unwrap();
    assert!(metadata.arn().contains("meta-mesh"));
    assert!(metadata.arn().starts_with("arn:aws:appmesh:"));
    // mesh_owner and resource_owner should be non-empty account IDs
    assert!(!metadata.mesh_owner().is_empty());
    assert!(!metadata.resource_owner().is_empty());
}

// ============================================================================
// Tests derived from AWS documentation: AWS App Mesh
// ============================================================================

// Test 1: CreateMesh with no spec should return DROP_ALL as default egress filter
#[tokio::test]
async fn test_create_mesh_default_egress_filter() {
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("default-egress-mesh")
        .send()
        .await
        .expect("create_mesh without spec should succeed");

    let mesh = resp.mesh().expect("should have mesh");
    let egress_type = mesh
        .spec()
        .and_then(|s| s.egress_filter())
        .map(|e| e.r#type());
    assert_eq!(
        egress_type,
        Some(&aws_sdk_appmesh::types::EgressFilterType::DropAll),
        "Default egress filter type should be DROP_ALL"
    );
}

// Test 2: CreateMesh with explicit DROP_ALL egress filter
#[tokio::test]
async fn test_create_mesh_egress_filter_drop_all() {
    let client = make_appmesh_client().await;

    let spec = aws_sdk_appmesh::types::MeshSpec::builder()
        .egress_filter(
            aws_sdk_appmesh::types::EgressFilter::builder()
                .r#type(aws_sdk_appmesh::types::EgressFilterType::DropAll)
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .create_mesh()
        .mesh_name("drop-all-mesh")
        .spec(spec)
        .send()
        .await
        .expect("create_mesh with DROP_ALL spec should succeed");

    let mesh = resp.mesh().expect("should have mesh");
    let egress_type = mesh
        .spec()
        .and_then(|s| s.egress_filter())
        .map(|e| e.r#type());
    assert_eq!(
        egress_type,
        Some(&aws_sdk_appmesh::types::EgressFilterType::DropAll),
        "Egress filter type should be DROP_ALL"
    );
}

// Test 3: Verify uid field is non-empty in metadata
#[tokio::test]
async fn test_metadata_uid_nonempty() {
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("uid-mesh")
        .send()
        .await
        .expect("create_mesh should succeed");

    let metadata = resp
        .mesh()
        .unwrap()
        .metadata()
        .expect("should have metadata");
    let uid = metadata.uid();
    assert!(!uid.is_empty(), "uid should be non-empty, got: '{uid}'");
}

// Test 4: Verify createdAt and lastUpdatedAt timestamps are present
#[tokio::test]
async fn test_metadata_timestamps() {
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("ts-mesh")
        .send()
        .await
        .expect("create_mesh should succeed");

    let metadata = resp
        .mesh()
        .unwrap()
        .metadata()
        .expect("should have metadata");
    // created_at and last_updated_at are non-optional DateTime fields; verify they are non-zero
    assert!(
        metadata.created_at().secs() > 0,
        "createdAt should be a non-zero timestamp"
    );
    assert!(
        metadata.last_updated_at().secs() > 0,
        "lastUpdatedAt should be a non-zero timestamp"
    );
}

// Test 5: ListMeshes mesh refs should include arn, meshOwner, resourceOwner, version
#[tokio::test]
async fn test_list_meshes_ref_fields() {
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("ref-fields-mesh")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_meshes()
        .send()
        .await
        .expect("list_meshes should succeed");

    let mesh_ref = resp
        .meshes()
        .iter()
        .find(|m| m.mesh_name() == "ref-fields-mesh")
        .expect("should find ref-fields-mesh in list");

    assert!(!mesh_ref.arn().is_empty(), "arn should be non-empty");
    assert!(
        mesh_ref.arn().contains("ref-fields-mesh"),
        "arn should contain mesh name"
    );
    assert!(
        !mesh_ref.mesh_owner().is_empty(),
        "meshOwner should be non-empty"
    );
    assert!(
        !mesh_ref.resource_owner().is_empty(),
        "resourceOwner should be non-empty"
    );
    assert!(mesh_ref.version() > 0, "version should be greater than 0");
}

// Test 6: DeleteMesh response should contain the correct mesh name
#[tokio::test]
async fn test_delete_mesh_returns_correct_name() {
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("name-check-del-mesh")
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_mesh()
        .mesh_name("name-check-del-mesh")
        .send()
        .await
        .expect("delete_mesh should succeed");

    let mesh = resp.mesh().expect("should have mesh in delete response");
    assert_eq!(
        mesh.mesh_name(),
        "name-check-del-mesh",
        "delete response should contain correct mesh name"
    );
}

// Test 7: Creating a duplicate mesh should return ConflictException error type
#[tokio::test]
async fn test_create_mesh_conflict_error_type() {
    let client = make_appmesh_client().await;

    client
        .create_mesh()
        .mesh_name("conflict-err-mesh")
        .send()
        .await
        .unwrap();

    let err = client
        .create_mesh()
        .mesh_name("conflict-err-mesh")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

// Test 8: DescribeMesh on nonexistent name should return NotFoundException error type
#[tokio::test]
async fn test_describe_mesh_not_found_error_type() {
    let client = make_appmesh_client().await;

    let err = client
        .describe_mesh()
        .mesh_name("does-not-exist-mesh")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// Test 9: DeleteMesh on nonexistent name should return NotFoundException error type
#[tokio::test]
async fn test_delete_mesh_not_found_error_type() {
    let client = make_appmesh_client().await;

    let err = client
        .delete_mesh()
        .mesh_name("ghost-mesh")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_meshes_empty() {
    let client = make_appmesh_client().await;

    let resp = client
        .list_meshes()
        .send()
        .await
        .expect("list_meshes on empty store should succeed");

    assert_eq!(resp.meshes().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_mesh() {
    let client = make_appmesh_client().await;

    let result = client
        .delete_mesh()
        .mesh_name("nonexistent-mesh")
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete nonexistent mesh should return error"
    );
}

#[tokio::test]
async fn test_create_mesh_with_egress_filter_spec() {
    let client = make_appmesh_client().await;

    let spec = aws_sdk_appmesh::types::MeshSpec::builder()
        .egress_filter(
            aws_sdk_appmesh::types::EgressFilter::builder()
                .r#type(aws_sdk_appmesh::types::EgressFilterType::AllowAll)
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .create_mesh()
        .mesh_name("spec-mesh")
        .spec(spec)
        .send()
        .await
        .expect("create_mesh with spec should succeed");

    let mesh = resp.mesh().expect("should have mesh");
    assert_eq!(mesh.mesh_name(), "spec-mesh");
    let egress_type = mesh
        .spec()
        .and_then(|s| s.egress_filter())
        .map(|e| e.r#type());
    assert_eq!(
        egress_type,
        Some(&aws_sdk_appmesh::types::EgressFilterType::AllowAll)
    );
}

// ============================================================================
// VirtualRouter tests
// ============================================================================

#[tokio::test]
async fn test_create_virtual_router() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vr-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualRouterSpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualRouterListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::PortMapping::builder()
                        .port(8080)
                        .protocol(aws_sdk_appmesh::types::PortProtocol::Http)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    let resp = client
        .create_virtual_router()
        .mesh_name("vr-mesh")
        .virtual_router_name("my-vr")
        .spec(spec)
        .send()
        .await
        .expect("create_virtual_router should succeed");

    let vr = resp.virtual_router().expect("should have virtual_router");
    assert_eq!(vr.virtual_router_name(), "my-vr");
    assert_eq!(vr.mesh_name(), "vr-mesh");
    assert!(vr.metadata().is_some());
    assert!(vr.metadata().unwrap().arn().contains("my-vr"));
}

#[tokio::test]
async fn test_describe_virtual_router() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vr-mesh2")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualRouterSpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualRouterListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::PortMapping::builder()
                        .port(9090)
                        .protocol(aws_sdk_appmesh::types::PortProtocol::Http2)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    client
        .create_virtual_router()
        .mesh_name("vr-mesh2")
        .virtual_router_name("desc-vr")
        .spec(spec)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_virtual_router()
        .mesh_name("vr-mesh2")
        .virtual_router_name("desc-vr")
        .send()
        .await
        .expect("describe_virtual_router should succeed");

    let vr = resp.virtual_router().unwrap();
    assert_eq!(vr.virtual_router_name(), "desc-vr");
    assert_eq!(vr.mesh_name(), "vr-mesh2");
}

#[tokio::test]
async fn test_list_virtual_routers() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vr-list-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualRouterSpec::builder().build();
    for name in ["vr-a", "vr-b", "vr-c"] {
        client
            .create_virtual_router()
            .mesh_name("vr-list-mesh")
            .virtual_router_name(name)
            .spec(spec.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_virtual_routers()
        .mesh_name("vr-list-mesh")
        .send()
        .await
        .expect("list_virtual_routers should succeed");

    assert_eq!(resp.virtual_routers().len(), 3);
}

#[tokio::test]
async fn test_delete_virtual_router() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vr-del-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualRouterSpec::builder().build();
    client
        .create_virtual_router()
        .mesh_name("vr-del-mesh")
        .virtual_router_name("del-vr")
        .spec(spec)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_virtual_router()
        .mesh_name("vr-del-mesh")
        .virtual_router_name("del-vr")
        .send()
        .await
        .expect("delete_virtual_router should succeed");

    let vr = resp.virtual_router().unwrap();
    assert_eq!(vr.virtual_router_name(), "del-vr");

    let result = client
        .describe_virtual_router()
        .mesh_name("vr-del-mesh")
        .virtual_router_name("del-vr")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_update_virtual_router() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vr-upd-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualRouterSpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualRouterListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::PortMapping::builder()
                        .port(8080)
                        .protocol(aws_sdk_appmesh::types::PortProtocol::Http)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();
    client
        .create_virtual_router()
        .mesh_name("vr-upd-mesh")
        .virtual_router_name("upd-vr")
        .spec(spec)
        .send()
        .await
        .unwrap();

    let new_spec = aws_sdk_appmesh::types::VirtualRouterSpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualRouterListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::PortMapping::builder()
                        .port(9090)
                        .protocol(aws_sdk_appmesh::types::PortProtocol::Http2)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    let resp = client
        .update_virtual_router()
        .mesh_name("vr-upd-mesh")
        .virtual_router_name("upd-vr")
        .spec(new_spec)
        .send()
        .await
        .expect("update_virtual_router should succeed");

    let vr = resp.virtual_router().unwrap();
    assert_eq!(vr.virtual_router_name(), "upd-vr");
    assert!(vr.metadata().unwrap().version() >= 2);
}

#[tokio::test]
async fn test_virtual_router_not_found() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vr-nf-mesh")
        .send()
        .await
        .unwrap();

    let err = client
        .describe_virtual_router()
        .mesh_name("vr-nf-mesh")
        .virtual_router_name("ghost-vr")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NotFoundException"), "got: {err_str}");
}

// ============================================================================
// VirtualNode tests
// ============================================================================

#[tokio::test]
async fn test_create_virtual_node() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vn-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualNodeSpec::builder().build();

    let resp = client
        .create_virtual_node()
        .mesh_name("vn-mesh")
        .virtual_node_name("my-vn")
        .spec(spec)
        .send()
        .await
        .expect("create_virtual_node should succeed");

    let vn = resp.virtual_node().expect("should have virtual_node");
    assert_eq!(vn.virtual_node_name(), "my-vn");
    assert_eq!(vn.mesh_name(), "vn-mesh");
    assert!(vn.metadata().is_some());
}

#[tokio::test]
async fn test_list_virtual_nodes() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vn-list-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualNodeSpec::builder().build();
    for name in ["vn-x", "vn-y"] {
        client
            .create_virtual_node()
            .mesh_name("vn-list-mesh")
            .virtual_node_name(name)
            .spec(spec.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_virtual_nodes()
        .mesh_name("vn-list-mesh")
        .send()
        .await
        .expect("list_virtual_nodes should succeed");
    assert_eq!(resp.virtual_nodes().len(), 2);
}

#[tokio::test]
async fn test_delete_virtual_node() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vn-del-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualNodeSpec::builder().build();
    client
        .create_virtual_node()
        .mesh_name("vn-del-mesh")
        .virtual_node_name("del-vn")
        .spec(spec)
        .send()
        .await
        .unwrap();

    client
        .delete_virtual_node()
        .mesh_name("vn-del-mesh")
        .virtual_node_name("del-vn")
        .send()
        .await
        .expect("delete_virtual_node should succeed");

    let result = client
        .describe_virtual_node()
        .mesh_name("vn-del-mesh")
        .virtual_node_name("del-vn")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_virtual_node() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vn-upd-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualNodeSpec::builder().build();
    client
        .create_virtual_node()
        .mesh_name("vn-upd-mesh")
        .virtual_node_name("upd-vn")
        .spec(spec.clone())
        .send()
        .await
        .unwrap();

    let resp = client
        .update_virtual_node()
        .mesh_name("vn-upd-mesh")
        .virtual_node_name("upd-vn")
        .spec(spec)
        .send()
        .await
        .expect("update_virtual_node should succeed");

    let vn = resp.virtual_node().unwrap();
    assert_eq!(vn.virtual_node_name(), "upd-vn");
    assert!(vn.metadata().unwrap().version() >= 2);
}

// ============================================================================
// VirtualGateway tests
// ============================================================================

#[tokio::test]
async fn test_create_virtual_gateway() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vg-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualGatewaySpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualGatewayListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::VirtualGatewayPortMapping::builder()
                        .port(8443)
                        .protocol(aws_sdk_appmesh::types::VirtualGatewayPortProtocol::Http)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build()
        .unwrap();

    let resp = client
        .create_virtual_gateway()
        .mesh_name("vg-mesh")
        .virtual_gateway_name("my-vg")
        .spec(spec)
        .send()
        .await
        .expect("create_virtual_gateway should succeed");

    let vg = resp.virtual_gateway().expect("should have virtual_gateway");
    assert_eq!(vg.virtual_gateway_name(), "my-vg");
    assert_eq!(vg.mesh_name(), "vg-mesh");
    assert!(vg.metadata().is_some());
}

#[tokio::test]
async fn test_list_virtual_gateways() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vg-list-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualGatewaySpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualGatewayListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::VirtualGatewayPortMapping::builder()
                        .port(8080)
                        .protocol(aws_sdk_appmesh::types::VirtualGatewayPortProtocol::Http)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build()
        .unwrap();

    for name in ["vg-a", "vg-b"] {
        client
            .create_virtual_gateway()
            .mesh_name("vg-list-mesh")
            .virtual_gateway_name(name)
            .spec(spec.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_virtual_gateways()
        .mesh_name("vg-list-mesh")
        .send()
        .await
        .expect("list_virtual_gateways should succeed");
    assert_eq!(resp.virtual_gateways().len(), 2);
}

#[tokio::test]
async fn test_delete_virtual_gateway() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vg-del-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualGatewaySpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualGatewayListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::VirtualGatewayPortMapping::builder()
                        .port(8080)
                        .protocol(aws_sdk_appmesh::types::VirtualGatewayPortProtocol::Http)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build()
        .unwrap();

    client
        .create_virtual_gateway()
        .mesh_name("vg-del-mesh")
        .virtual_gateway_name("del-vg")
        .spec(spec)
        .send()
        .await
        .unwrap();

    client
        .delete_virtual_gateway()
        .mesh_name("vg-del-mesh")
        .virtual_gateway_name("del-vg")
        .send()
        .await
        .expect("delete_virtual_gateway should succeed");

    let result = client
        .describe_virtual_gateway()
        .mesh_name("vg-del-mesh")
        .virtual_gateway_name("del-vg")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// VirtualService tests
// ============================================================================

#[tokio::test]
async fn test_create_virtual_service() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vs-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualServiceSpec::builder().build();

    let resp = client
        .create_virtual_service()
        .mesh_name("vs-mesh")
        .virtual_service_name("my-svc.local")
        .spec(spec)
        .send()
        .await
        .expect("create_virtual_service should succeed");

    let vs = resp.virtual_service().expect("should have virtual_service");
    assert_eq!(vs.virtual_service_name(), "my-svc.local");
    assert_eq!(vs.mesh_name(), "vs-mesh");
    assert!(vs.metadata().is_some());
}

#[tokio::test]
async fn test_list_virtual_services() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vs-list-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualServiceSpec::builder().build();
    for name in ["svc-a.local", "svc-b.local", "svc-c.local"] {
        client
            .create_virtual_service()
            .mesh_name("vs-list-mesh")
            .virtual_service_name(name)
            .spec(spec.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_virtual_services()
        .mesh_name("vs-list-mesh")
        .send()
        .await
        .expect("list_virtual_services should succeed");
    assert_eq!(resp.virtual_services().len(), 3);
}

#[tokio::test]
async fn test_delete_virtual_service() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vs-del-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualServiceSpec::builder().build();
    client
        .create_virtual_service()
        .mesh_name("vs-del-mesh")
        .virtual_service_name("del-svc.local")
        .spec(spec)
        .send()
        .await
        .unwrap();

    client
        .delete_virtual_service()
        .mesh_name("vs-del-mesh")
        .virtual_service_name("del-svc.local")
        .send()
        .await
        .expect("delete_virtual_service should succeed");

    let result = client
        .describe_virtual_service()
        .mesh_name("vs-del-mesh")
        .virtual_service_name("del-svc.local")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_virtual_service() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("vs-upd-mesh")
        .send()
        .await
        .unwrap();

    let spec = aws_sdk_appmesh::types::VirtualServiceSpec::builder().build();
    client
        .create_virtual_service()
        .mesh_name("vs-upd-mesh")
        .virtual_service_name("upd-svc.local")
        .spec(spec.clone())
        .send()
        .await
        .unwrap();

    let resp = client
        .update_virtual_service()
        .mesh_name("vs-upd-mesh")
        .virtual_service_name("upd-svc.local")
        .spec(spec)
        .send()
        .await
        .expect("update_virtual_service should succeed");

    let vs = resp.virtual_service().unwrap();
    assert_eq!(vs.virtual_service_name(), "upd-svc.local");
    assert!(vs.metadata().unwrap().version() >= 2);
}

// ============================================================================
// Route tests
// ============================================================================

async fn make_appmesh_client_with_vr(mesh_name: &str, vr_name: &str) -> aws_sdk_appmesh::Client {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name(mesh_name)
        .send()
        .await
        .unwrap();
    let spec = aws_sdk_appmesh::types::VirtualRouterSpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualRouterListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::PortMapping::builder()
                        .port(8080)
                        .protocol(aws_sdk_appmesh::types::PortProtocol::Http)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();
    client
        .create_virtual_router()
        .mesh_name(mesh_name)
        .virtual_router_name(vr_name)
        .spec(spec)
        .send()
        .await
        .unwrap();
    client
}

#[tokio::test]
async fn test_create_route() {
    let client = make_appmesh_client_with_vr("rt-mesh", "rt-vr").await;

    let route_spec = aws_sdk_appmesh::types::RouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpRouteMatch::builder()
                        .prefix("/")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpRouteAction::builder()
                        .weighted_targets(
                            aws_sdk_appmesh::types::WeightedTarget::builder()
                                .virtual_node("rt-vn")
                                .weight(100)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    let resp = client
        .create_route()
        .mesh_name("rt-mesh")
        .virtual_router_name("rt-vr")
        .route_name("my-route")
        .spec(route_spec)
        .send()
        .await
        .expect("create_route should succeed");

    let route = resp.route().expect("should have route");
    assert_eq!(route.route_name(), "my-route");
    assert_eq!(route.mesh_name(), "rt-mesh");
    assert_eq!(route.virtual_router_name(), "rt-vr");
    assert!(route.metadata().is_some());
}

#[tokio::test]
async fn test_describe_route() {
    let client = make_appmesh_client_with_vr("rt-desc-mesh", "rt-desc-vr").await;

    let route_spec = aws_sdk_appmesh::types::RouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpRouteMatch::builder()
                        .prefix("/api")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpRouteAction::builder()
                        .weighted_targets(
                            aws_sdk_appmesh::types::WeightedTarget::builder()
                                .virtual_node("some-vn")
                                .weight(100)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    client
        .create_route()
        .mesh_name("rt-desc-mesh")
        .virtual_router_name("rt-desc-vr")
        .route_name("desc-route")
        .spec(route_spec)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_route()
        .mesh_name("rt-desc-mesh")
        .virtual_router_name("rt-desc-vr")
        .route_name("desc-route")
        .send()
        .await
        .expect("describe_route should succeed");

    let route = resp.route().unwrap();
    assert_eq!(route.route_name(), "desc-route");
    assert_eq!(route.virtual_router_name(), "rt-desc-vr");
}

#[tokio::test]
async fn test_list_routes() {
    let client = make_appmesh_client_with_vr("rt-list-mesh", "rt-list-vr").await;

    let route_spec = aws_sdk_appmesh::types::RouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpRouteMatch::builder()
                        .prefix("/")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpRouteAction::builder()
                        .weighted_targets(
                            aws_sdk_appmesh::types::WeightedTarget::builder()
                                .virtual_node("vn")
                                .weight(100)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    for name in ["route-1", "route-2"] {
        client
            .create_route()
            .mesh_name("rt-list-mesh")
            .virtual_router_name("rt-list-vr")
            .route_name(name)
            .spec(route_spec.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_routes()
        .mesh_name("rt-list-mesh")
        .virtual_router_name("rt-list-vr")
        .send()
        .await
        .expect("list_routes should succeed");
    assert_eq!(resp.routes().len(), 2);
}

#[tokio::test]
async fn test_delete_route() {
    let client = make_appmesh_client_with_vr("rt-del-mesh", "rt-del-vr").await;

    let route_spec = aws_sdk_appmesh::types::RouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpRouteMatch::builder()
                        .prefix("/")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpRouteAction::builder()
                        .weighted_targets(
                            aws_sdk_appmesh::types::WeightedTarget::builder()
                                .virtual_node("vn")
                                .weight(100)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    client
        .create_route()
        .mesh_name("rt-del-mesh")
        .virtual_router_name("rt-del-vr")
        .route_name("del-route")
        .spec(route_spec)
        .send()
        .await
        .unwrap();

    client
        .delete_route()
        .mesh_name("rt-del-mesh")
        .virtual_router_name("rt-del-vr")
        .route_name("del-route")
        .send()
        .await
        .expect("delete_route should succeed");

    let result = client
        .describe_route()
        .mesh_name("rt-del-mesh")
        .virtual_router_name("rt-del-vr")
        .route_name("del-route")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_route() {
    let client = make_appmesh_client_with_vr("rt-upd-mesh", "rt-upd-vr").await;

    let route_spec = aws_sdk_appmesh::types::RouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpRouteMatch::builder()
                        .prefix("/")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpRouteAction::builder()
                        .weighted_targets(
                            aws_sdk_appmesh::types::WeightedTarget::builder()
                                .virtual_node("vn-old")
                                .weight(100)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    client
        .create_route()
        .mesh_name("rt-upd-mesh")
        .virtual_router_name("rt-upd-vr")
        .route_name("upd-route")
        .spec(route_spec)
        .send()
        .await
        .unwrap();

    let new_spec = aws_sdk_appmesh::types::RouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpRouteMatch::builder()
                        .prefix("/v2")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpRouteAction::builder()
                        .weighted_targets(
                            aws_sdk_appmesh::types::WeightedTarget::builder()
                                .virtual_node("vn-new")
                                .weight(100)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build();

    let resp = client
        .update_route()
        .mesh_name("rt-upd-mesh")
        .virtual_router_name("rt-upd-vr")
        .route_name("upd-route")
        .spec(new_spec)
        .send()
        .await
        .expect("update_route should succeed");

    let route = resp.route().unwrap();
    assert_eq!(route.route_name(), "upd-route");
    assert!(route.metadata().unwrap().version() >= 2);
}

// ============================================================================
// GatewayRoute tests
// ============================================================================

async fn make_appmesh_client_with_vg(mesh_name: &str, vg_name: &str) -> aws_sdk_appmesh::Client {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name(mesh_name)
        .send()
        .await
        .unwrap();
    let spec = aws_sdk_appmesh::types::VirtualGatewaySpec::builder()
        .listeners(
            aws_sdk_appmesh::types::VirtualGatewayListener::builder()
                .port_mapping(
                    aws_sdk_appmesh::types::VirtualGatewayPortMapping::builder()
                        .port(8080)
                        .protocol(aws_sdk_appmesh::types::VirtualGatewayPortProtocol::Http)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .build()
        .unwrap();
    client
        .create_virtual_gateway()
        .mesh_name(mesh_name)
        .virtual_gateway_name(vg_name)
        .spec(spec)
        .send()
        .await
        .unwrap();
    client
}

#[tokio::test]
async fn test_create_gateway_route() {
    let client = make_appmesh_client_with_vg("gr-mesh", "gr-vg").await;

    let gr_spec = aws_sdk_appmesh::types::GatewayRouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpGatewayRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpGatewayRouteMatch::builder()
                        .prefix("/")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpGatewayRouteAction::builder()
                        .target(
                            aws_sdk_appmesh::types::GatewayRouteTarget::builder()
                                .virtual_service(
                                    aws_sdk_appmesh::types::GatewayRouteVirtualService::builder()
                                        .virtual_service_name("my-svc.local")
                                        .build()
                                        .unwrap(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    let resp = client
        .create_gateway_route()
        .mesh_name("gr-mesh")
        .virtual_gateway_name("gr-vg")
        .gateway_route_name("my-gr")
        .spec(gr_spec)
        .send()
        .await
        .expect("create_gateway_route should succeed");

    let gr = resp.gateway_route().expect("should have gateway_route");
    assert_eq!(gr.gateway_route_name(), "my-gr");
    assert_eq!(gr.mesh_name(), "gr-mesh");
    assert_eq!(gr.virtual_gateway_name(), "gr-vg");
    assert!(gr.metadata().is_some());
}

#[tokio::test]
async fn test_list_gateway_routes() {
    let client = make_appmesh_client_with_vg("gr-list-mesh", "gr-list-vg").await;

    let gr_spec = aws_sdk_appmesh::types::GatewayRouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpGatewayRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpGatewayRouteMatch::builder()
                        .prefix("/")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpGatewayRouteAction::builder()
                        .target(
                            aws_sdk_appmesh::types::GatewayRouteTarget::builder()
                                .virtual_service(
                                    aws_sdk_appmesh::types::GatewayRouteVirtualService::builder()
                                        .virtual_service_name("svc.local")
                                        .build()
                                        .unwrap(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    for name in ["gr-1", "gr-2", "gr-3"] {
        client
            .create_gateway_route()
            .mesh_name("gr-list-mesh")
            .virtual_gateway_name("gr-list-vg")
            .gateway_route_name(name)
            .spec(gr_spec.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_gateway_routes()
        .mesh_name("gr-list-mesh")
        .virtual_gateway_name("gr-list-vg")
        .send()
        .await
        .expect("list_gateway_routes should succeed");
    assert_eq!(resp.gateway_routes().len(), 3);
}

#[tokio::test]
async fn test_delete_gateway_route() {
    let client = make_appmesh_client_with_vg("gr-del-mesh", "gr-del-vg").await;

    let gr_spec = aws_sdk_appmesh::types::GatewayRouteSpec::builder()
        .http_route(
            aws_sdk_appmesh::types::HttpGatewayRoute::builder()
                .r#match(
                    aws_sdk_appmesh::types::HttpGatewayRouteMatch::builder()
                        .prefix("/")
                        .build(),
                )
                .action(
                    aws_sdk_appmesh::types::HttpGatewayRouteAction::builder()
                        .target(
                            aws_sdk_appmesh::types::GatewayRouteTarget::builder()
                                .virtual_service(
                                    aws_sdk_appmesh::types::GatewayRouteVirtualService::builder()
                                        .virtual_service_name("svc.local")
                                        .build()
                                        .unwrap(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    client
        .create_gateway_route()
        .mesh_name("gr-del-mesh")
        .virtual_gateway_name("gr-del-vg")
        .gateway_route_name("del-gr")
        .spec(gr_spec)
        .send()
        .await
        .unwrap();

    client
        .delete_gateway_route()
        .mesh_name("gr-del-mesh")
        .virtual_gateway_name("gr-del-vg")
        .gateway_route_name("del-gr")
        .send()
        .await
        .expect("delete_gateway_route should succeed");

    let result = client
        .describe_gateway_route()
        .mesh_name("gr-del-mesh")
        .virtual_gateway_name("gr-del-vg")
        .gateway_route_name("del-gr")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// UpdateMesh test
// ============================================================================

#[tokio::test]
async fn test_update_mesh() {
    let client = make_appmesh_client().await;
    client
        .create_mesh()
        .mesh_name("upd-mesh")
        .send()
        .await
        .unwrap();

    let new_spec = aws_sdk_appmesh::types::MeshSpec::builder()
        .egress_filter(
            aws_sdk_appmesh::types::EgressFilter::builder()
                .r#type(aws_sdk_appmesh::types::EgressFilterType::AllowAll)
                .build()
                .unwrap(),
        )
        .build();

    let resp = client
        .update_mesh()
        .mesh_name("upd-mesh")
        .spec(new_spec)
        .send()
        .await
        .expect("update_mesh should succeed");

    let mesh = resp.mesh().unwrap();
    assert_eq!(mesh.mesh_name(), "upd-mesh");
    assert!(mesh.metadata().unwrap().version() >= 2);
    let egress_type = mesh
        .spec()
        .and_then(|s| s.egress_filter())
        .map(|e| e.r#type());
    assert_eq!(
        egress_type,
        Some(&aws_sdk_appmesh::types::EgressFilterType::AllowAll)
    );
}

// ============================================================================
// Tag operations tests
// ============================================================================

#[tokio::test]
async fn test_tag_resource_and_list_tags() {
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("tag-mesh")
        .send()
        .await
        .unwrap();
    let arn = resp.mesh().unwrap().metadata().unwrap().arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_appmesh::types::TagRef::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_appmesh::types::TagRef::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 2);
    let env_tag = tags
        .iter()
        .find(|t| t.key() == "env")
        .expect("env tag should exist");
    assert_eq!(env_tag.value(), "prod");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_appmesh_client().await;

    let resp = client
        .create_mesh()
        .mesh_name("untag-mesh")
        .send()
        .await
        .unwrap();
    let arn = resp.mesh().unwrap().metadata().unwrap().arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_appmesh::types::TagRef::builder()
                .key("k1")
                .value("v1")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_appmesh::types::TagRef::builder()
                .key("k2")
                .value("v2")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("k1")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "k2");
    assert_eq!(tags[0].value(), "v2");
}

// ============================================================================
// State views tests
// ============================================================================

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    let svc = winterbaume_appmesh::AppMeshService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use winterbaume_core::StatefulService;

    let svc = winterbaume_appmesh::AppMeshService::new();
    let svc2 = winterbaume_appmesh::AppMeshService::new();

    // Seed via restore with a known view (empty — just initializes state)
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();

    // Snapshot the empty state
    let view = svc.snapshot("123456789012", "us-east-1").await;

    // Restore into svc2 and snapshot again — should produce same shape
    svc2.restore("123456789012", "us-east-1", view.clone())
        .await
        .unwrap();
    let view2 = svc2.snapshot("123456789012", "us-east-1").await;

    // Both should have same number of meshes (zero in this case)
    assert_eq!(view.meshes.len(), view2.meshes.len());
}

#[tokio::test]
async fn test_merge_does_not_remove_existing() {
    use winterbaume_appmesh::AppMeshStateView;
    use winterbaume_appmesh::views::MeshView;
    use winterbaume_core::StatefulService;

    let svc = winterbaume_appmesh::AppMeshService::new();

    // Build a view with mesh-a already present
    let mut view1 = AppMeshStateView::default();
    view1.meshes.insert(
        "mesh-a".to_string(),
        MeshView {
            mesh_name: "mesh-a".to_string(),
            arn: "arn:aws:appmesh:us-east-1:123456789012:mesh/mesh-a".to_string(),
            uid: "uid-a".to_string(),
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            last_updated_at: "2024-01-01T00:00:00Z".to_string(),
            mesh_owner: "123456789012".to_string(),
            resource_owner: "123456789012".to_string(),
            egress_filter_type: "DROP_ALL".to_string(),
            tags: vec![],
        },
    );
    svc.restore("123456789012", "us-east-1", view1)
        .await
        .unwrap();

    // Merge a view containing mesh-b
    let mut view2 = AppMeshStateView::default();
    view2.meshes.insert(
        "mesh-b".to_string(),
        MeshView {
            mesh_name: "mesh-b".to_string(),
            arn: "arn:aws:appmesh:us-east-1:123456789012:mesh/mesh-b".to_string(),
            uid: "uid-b".to_string(),
            status: "ACTIVE".to_string(),
            version: 1,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            last_updated_at: "2024-01-01T00:00:00Z".to_string(),
            mesh_owner: "123456789012".to_string(),
            resource_owner: "123456789012".to_string(),
            egress_filter_type: "DROP_ALL".to_string(),
            tags: vec![],
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let final_view = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        final_view.meshes.contains_key("mesh-a"),
        "mesh-a should still exist after merge"
    );
    assert!(
        final_view.meshes.contains_key("mesh-b"),
        "mesh-b should exist after merge"
    );
}

#[tokio::test]
async fn test_mesh_lifecycle_full() {
    let client = make_appmesh_client().await;

    // Create
    client
        .create_mesh()
        .mesh_name("lifecycle-mesh")
        .send()
        .await
        .expect("create should succeed");

    // Describe
    let desc = client
        .describe_mesh()
        .mesh_name("lifecycle-mesh")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.mesh().unwrap().mesh_name(), "lifecycle-mesh");

    // Delete
    client
        .delete_mesh()
        .mesh_name("lifecycle-mesh")
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .describe_mesh()
        .mesh_name("lifecycle-mesh")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}
