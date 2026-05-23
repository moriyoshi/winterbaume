# Amazon CloudDirectory

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Cloud Directory Amazon Cloud Directory is a component of the AWS Directory Service that simplifies the development and management of cloud-scale web, mobile, and IoT applications. This guide describes the Cloud Directory operations that you can call programmatically and includes detailed information on data types and errors. For information about Cloud Directory features, see AWS Directory Service and the Amazon Cloud Directory Developer Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon CloudDirectory where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon CloudDirectory by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon CloudDirectory workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListAppliedSchemaArns`, `ListAttachedIndices`, `ListDevelopmentSchemaArns`, `ListDirectories`, `GetAppliedSchemaVersion`, `GetDirectory`.

## Service Identity and Protocol

- AWS model slug: `clouddirectory`
- AWS SDK for Rust slug: `clouddirectory`
- Model version: `2017-01-11`
- Model file: `vendor/api-models-aws/models/clouddirectory/service/2017-01-11/clouddirectory-2017-01-11.json`
- SDK ID: `CloudDirectory`
- Endpoint prefix: `clouddirectory`
- ARN namespace: `clouddirectory`
- CloudFormation name: `CloudDirectory`
- CloudTrail event source: `clouddirectory.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (20), `Get` (8), `Create` (6), `Delete` (5), `Update` (5), `Attach` (4), `Detach` (4), `Batch` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddFacetToObject`, `AttachObject`, `AttachPolicy`, `AttachToIndex`, `AttachTypedLink`, `BatchRead`, `BatchWrite`, `CreateDirectory`, `CreateFacet`, `CreateIndex`, `CreateObject`, `CreateSchema`, `CreateTypedLinkFacet`, `DeleteDirectory`, `DeleteFacet`, `DeleteObject`, `DeleteSchema`, `DeleteTypedLinkFacet`, `DetachFromIndex`, `DetachObject`, ... (+13).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAppliedSchemaVersion`, `GetDirectory`, `GetFacet`, `GetLinkAttributes`, `GetObjectAttributes`, `GetObjectInformation`, `GetSchemaAsJson`, `GetTypedLinkFacetInformation`, `ListAppliedSchemaArns`, `ListAttachedIndices`, `ListDevelopmentSchemaArns`, `ListDirectories`, `ListFacetAttributes`, `ListFacetNames`, `ListIncomingTypedLinks`, `ListIndex`, `ListManagedSchemaArns`, `ListObjectAttributes`, `ListObjectChildren`, `ListObjectParentPaths`, ... (+8).
- Pagination is modelled for 19 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 66 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/clouddirectory/latest/developerguide/what_is_cloud_directory.html
- https://docs.aws.amazon.com/clouddirectory/latest/developerguide/key_concepts_schema.html
- https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html

Research outcomes:
- Cloud Directory is a scalable graph-based directory store for hierarchical and relationship-rich data.
- Directories are created from schemas. Schemas contain facets that define attributes and constraints for objects.
- Objects can have one or more facets attached and are connected by links.
- Link types include child links, attachment links, index links, and typed links.
- Typed links represent relationships between objects and can be queried as incoming or outgoing links.
- Cloud Directory validates object attributes against schema/facet definitions.

Parity implications:
- Model schemas, schema versions, facets, attributes, directories, objects, object references, links, typed links, and indexes separately.
- Object creation/update should validate facet requirements and attribute constraints.
- Directory traversal should use graph links rather than assuming a simple tree only.

## Operation Groups

### List

- Operations: `ListAppliedSchemaArns`, `ListAttachedIndices`, `ListDevelopmentSchemaArns`, `ListDirectories`, `ListFacetAttributes`, `ListFacetNames`, `ListIncomingTypedLinks`, `ListIndex`, `ListManagedSchemaArns`, `ListObjectAttributes`, `ListObjectChildren`, `ListObjectParentPaths`, `ListObjectParents`, `ListObjectPolicies`, `ListOutgoingTypedLinks`, `ListPolicyAttachments`, `ListPublishedSchemaArns`, `ListTagsForResource`, `ListTypedLinkFacetAttributes`, `ListTypedLinkFacetNames`
- Traits: `paginated` (18)
- Common required input members in this group: `DirectoryArn`, `IndexReference`, `Name`, `ObjectReference`, `PolicyReference`, `ResourceArn`, `SchemaArn`, `TargetReference`

### Get

- Operations: `GetAppliedSchemaVersion`, `GetDirectory`, `GetFacet`, `GetLinkAttributes`, `GetObjectAttributes`, `GetObjectInformation`, `GetSchemaAsJson`, `GetTypedLinkFacetInformation`
- Common required input members in this group: `AttributeNames`, `DirectoryArn`, `Name`, `ObjectReference`, `SchemaArn`, `SchemaFacet`, `TypedLinkSpecifier`

### Create

- Operations: `CreateDirectory`, `CreateFacet`, `CreateIndex`, `CreateObject`, `CreateSchema`, `CreateTypedLinkFacet`
- Common required input members in this group: `DirectoryArn`, `Facet`, `IsUnique`, `Name`, `OrderedIndexedAttributeList`, `SchemaArn`, `SchemaFacets`

### Delete

- Operations: `DeleteDirectory`, `DeleteFacet`, `DeleteObject`, `DeleteSchema`, `DeleteTypedLinkFacet`
- Common required input members in this group: `DirectoryArn`, `Name`, `ObjectReference`, `SchemaArn`

### Update

- Operations: `UpdateFacet`, `UpdateLinkAttributes`, `UpdateObjectAttributes`, `UpdateSchema`, `UpdateTypedLinkFacet`
- Common required input members in this group: `AttributeUpdates`, `DirectoryArn`, `IdentityAttributeOrder`, `Name`, `ObjectReference`, `SchemaArn`, `TypedLinkSpecifier`

### Attach

- Operations: `AttachObject`, `AttachPolicy`, `AttachToIndex`, `AttachTypedLink`
- Common required input members in this group: `Attributes`, `ChildReference`, `DirectoryArn`, `IndexReference`, `LinkName`, `ObjectReference`, `ParentReference`, `PolicyReference`, `SourceObjectReference`, `TargetObjectReference`, `TargetReference`, `TypedLinkFacet`

### Detach

- Operations: `DetachFromIndex`, `DetachObject`, `DetachPolicy`, `DetachTypedLink`
- Common required input members in this group: `DirectoryArn`, `IndexReference`, `LinkName`, `ObjectReference`, `ParentReference`, `PolicyReference`, `TargetReference`, `TypedLinkSpecifier`

### Batch

- Operations: `BatchRead`, `BatchWrite`
- Common required input members in this group: `DirectoryArn`, `Operations`

### Upgrade

- Operations: `UpgradeAppliedSchema`, `UpgradePublishedSchema`
- Common required input members in this group: `DevelopmentSchemaArn`, `DirectoryArn`, `MinorVersion`, `PublishedSchemaArn`

### Add

- Operations: `AddFacetToObject`
- Common required input members in this group: `DirectoryArn`, `ObjectReference`, `SchemaFacet`

### Apply

- Operations: `ApplySchema`
- Common required input members in this group: `DirectoryArn`, `PublishedSchemaArn`

### Disable

- Operations: `DisableDirectory`
- Common required input members in this group: `DirectoryArn`

### Enable

- Operations: `EnableDirectory`
- Common required input members in this group: `DirectoryArn`

### Lookup

- Operations: `LookupPolicy`
- Traits: `paginated` (1)
- Common required input members in this group: `DirectoryArn`, `ObjectReference`

### Publish

- Operations: `PublishSchema`
- Common required input members in this group: `DevelopmentSchemaArn`, `Version`

### Put

- Operations: `PutSchemaFromJson`
- Common required input members in this group: `Document`, `SchemaArn`

### Remove

- Operations: `RemoveFacetFromObject`
- Common required input members in this group: `DirectoryArn`, `ObjectReference`, `SchemaFacet`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddFacetToObject` | `PUT /amazonclouddirectory/2017-01-11/object/facets` | - | `DirectoryArn`, `ObjectReference`, `SchemaFacet` | - | `AddFacetToObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Adds a new Facet to an object. An object can have more than one facet applied on it. |
| `ApplySchema` | `PUT /amazonclouddirectory/2017-01-11/schema/apply` | - | `DirectoryArn`, `PublishedSchemaArn` | - | `ApplySchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `SchemaAlreadyExistsException`, ... (+1) | Copies the input published schema, at the specified version, into the Directory with the same name and version as that of the published schema. |
| `AttachObject` | `PUT /amazonclouddirectory/2017-01-11/object/attach` | - | `ChildReference`, `DirectoryArn`, `LinkName`, `ParentReference` | - | `AttachObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, ... (+3) | Attaches an existing object to another object. An object can be accessed in two ways: Using the path Using `ObjectIdentifier` |
| `AttachPolicy` | `PUT /amazonclouddirectory/2017-01-11/policy/attach` | - | `DirectoryArn`, `ObjectReference`, `PolicyReference` | - | `AttachPolicyResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotPolicyException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Attaches a policy object to a regular object. An object can have a limited number of attached policies. |
| `AttachToIndex` | `PUT /amazonclouddirectory/2017-01-11/index/attach` | - | `DirectoryArn`, `IndexReference`, `TargetReference` | - | `AttachToIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `IndexedAttributeMissingException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, ... (+4) | Attaches the specified object to the specified index. |
| `AttachTypedLink` | `PUT /amazonclouddirectory/2017-01-11/typedlink/attach` | - | `Attributes`, `DirectoryArn`, `SourceObjectReference`, `TargetObjectReference`, `TypedLinkFacet` | - | `AttachTypedLinkResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `ResourceNotFoundException`, ... (+2) | Attaches a typed link to a specified source and target object. For more information, see Typed Links. |
| `BatchRead` | `POST /amazonclouddirectory/2017-01-11/batchread` | - | `DirectoryArn`, `Operations` | - | `BatchReadResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Performs all the read operations in a batch. |
| `BatchWrite` | `PUT /amazonclouddirectory/2017-01-11/batchwrite` | - | `DirectoryArn`, `Operations` | - | `BatchWriteResponse` | `AccessDeniedException`, `BatchWriteException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Performs all the write operations in a batch. Either all the operations succeed or none. |
| `CreateDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory/create` | - | `Name`, `SchemaArn` | - | `CreateDirectoryResponse` | `AccessDeniedException`, `DirectoryAlreadyExistsException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Creates a Directory by copying the published schema into the directory. A directory cannot be created without a schema. |
| `CreateFacet` | `PUT /amazonclouddirectory/2017-01-11/facet/create` | - | `Name`, `SchemaArn` | - | `CreateFacetResponse` | `AccessDeniedException`, `FacetAlreadyExistsException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidRuleException`, `LimitExceededException`, `ResourceNotFoundException`, ... (+2) | Creates a new Facet in a schema. Facet creation is allowed only in development or applied schemas. |
| `CreateIndex` | `PUT /amazonclouddirectory/2017-01-11/index` | - | `DirectoryArn`, `IsUnique`, `OrderedIndexedAttributeList` | - | `CreateIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `ResourceNotFoundException`, ... (+3) | Creates an index object. See Indexing and search for more information. |
| `CreateObject` | `PUT /amazonclouddirectory/2017-01-11/object` | - | `DirectoryArn`, `SchemaFacets` | - | `CreateObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `ResourceNotFoundException`, ... (+3) | Creates an object in a Directory. Additionally attaches the object to a parent, if a parent reference and `LinkName` is specified. |
| `CreateSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/create` | - | `Name` | - | `CreateSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `SchemaAlreadyExistsException`, `ValidationException` | Creates a new schema in a development state. A schema can exist in three phases: Development: This is a mutable phase of the schema. |
| `CreateTypedLinkFacet` | `PUT /amazonclouddirectory/2017-01-11/typedlink/facet/create` | - | `Facet`, `SchemaArn` | - | `CreateTypedLinkFacetResponse` | `AccessDeniedException`, `FacetAlreadyExistsException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidRuleException`, `LimitExceededException`, `ResourceNotFoundException`, ... (+2) | Creates a TypedLinkFacet. For more information, see Typed Links. |
| `DeleteDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory` | - | `DirectoryArn` | - | `DeleteDirectoryResponse` | `AccessDeniedException`, `DirectoryDeletedException`, `DirectoryNotDisabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Deletes a directory. Only disabled directories can be deleted. |
| `DeleteFacet` | `PUT /amazonclouddirectory/2017-01-11/facet/delete` | - | `Name`, `SchemaArn` | - | `DeleteFacetResponse` | `AccessDeniedException`, `FacetInUseException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Deletes a given Facet. All attributes and Rules that are associated with the facet will be deleted. |
| `DeleteObject` | `PUT /amazonclouddirectory/2017-01-11/object/delete` | - | `DirectoryArn`, `ObjectReference` | - | `DeleteObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ObjectNotDetachedException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted. |
| `DeleteSchema` | `PUT /amazonclouddirectory/2017-01-11/schema` | - | `SchemaArn` | - | `DeleteSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `StillContainsLinksException`, `ValidationException` | Deletes a given schema. Schemas in a development and published state can only be deleted. |
| `DeleteTypedLinkFacet` | `PUT /amazonclouddirectory/2017-01-11/typedlink/facet/delete` | - | `Name`, `SchemaArn` | - | `DeleteTypedLinkFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Deletes a TypedLinkFacet. For more information, see Typed Links. |
| `DetachFromIndex` | `PUT /amazonclouddirectory/2017-01-11/index/detach` | - | `DirectoryArn`, `IndexReference`, `TargetReference` | - | `DetachFromIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotIndexException`, `ObjectAlreadyDetachedException`, `ResourceNotFoundException`, ... (+2) | Detaches the specified object from the specified index. |
| `DetachObject` | `PUT /amazonclouddirectory/2017-01-11/object/detach` | - | `DirectoryArn`, `LinkName`, `ParentReference` | - | `DetachObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotNodeException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name. |
| `DetachPolicy` | `PUT /amazonclouddirectory/2017-01-11/policy/detach` | - | `DirectoryArn`, `ObjectReference`, `PolicyReference` | - | `DetachPolicyResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotPolicyException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Detaches a policy from an object. |
| `DetachTypedLink` | `PUT /amazonclouddirectory/2017-01-11/typedlink/detach` | - | `DirectoryArn`, `TypedLinkSpecifier` | - | `Unit` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Detaches a typed link from a specified source and target object. For more information, see Typed Links. |
| `DisableDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory/disable` | - | `DirectoryArn` | - | `DisableDirectoryResponse` | `AccessDeniedException`, `DirectoryDeletedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Disables the specified directory. Disabled directories cannot be read or written to. |
| `EnableDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory/enable` | - | `DirectoryArn` | - | `EnableDirectoryResponse` | `AccessDeniedException`, `DirectoryDeletedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Enables the specified directory. Only disabled directories can be enabled. |
| `GetAppliedSchemaVersion` | `POST /amazonclouddirectory/2017-01-11/schema/getappliedschema` | - | `SchemaArn` | - | `GetAppliedSchemaVersionResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns current applied schema version ARN, including the minor version in use. |
| `GetDirectory` | `POST /amazonclouddirectory/2017-01-11/directory/get` | - | `DirectoryArn` | - | `GetDirectoryResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Retrieves metadata about a directory. |
| `GetFacet` | `POST /amazonclouddirectory/2017-01-11/facet` | - | `Name`, `SchemaArn` | - | `GetFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Gets details of the Facet, such as facet name, attributes, Rules, or `ObjectType`. You can call this on all kinds of schema facets -- published, development, or applied. |
| `GetLinkAttributes` | `POST /amazonclouddirectory/2017-01-11/typedlink/attributes/get` | - | `AttributeNames`, `DirectoryArn`, `TypedLinkSpecifier` | - | `GetLinkAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Retrieves attributes that are associated with a typed link. |
| `GetObjectAttributes` | `POST /amazonclouddirectory/2017-01-11/object/attributes/get` | - | `AttributeNames`, `DirectoryArn`, `ObjectReference`, `SchemaFacet` | - | `GetObjectAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Retrieves attributes within a facet that are associated with an object. |
| `GetObjectInformation` | `POST /amazonclouddirectory/2017-01-11/object/information` | - | `DirectoryArn`, `ObjectReference` | - | `GetObjectInformationResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves metadata about an object. |
| `GetSchemaAsJson` | `POST /amazonclouddirectory/2017-01-11/schema/json` | - | `SchemaArn` | - | `GetSchemaAsJsonResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves a JSON representation of the schema. See JSON Schema Format for more information. |
| `GetTypedLinkFacetInformation` | `POST /amazonclouddirectory/2017-01-11/typedlink/facet/get` | - | `Name`, `SchemaArn` | - | `GetTypedLinkFacetInformationResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Returns the identity attribute order for a specific TypedLinkFacet. For more information, see Typed Links. |
| `ListAppliedSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/applied` | `paginated` | `DirectoryArn` | - | `ListAppliedSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists schema major versions applied to a directory. If `SchemaArn` is provided, lists the minor version. |
| `ListAttachedIndices` | `POST /amazonclouddirectory/2017-01-11/object/indices` | `paginated` | `DirectoryArn`, `TargetReference` | - | `ListAttachedIndicesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists indices attached to the specified object. |
| `ListDevelopmentSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/development` | `paginated` | - | - | `ListDevelopmentSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves each Amazon Resource Name (ARN) of schemas in the development state. |
| `ListDirectories` | `POST /amazonclouddirectory/2017-01-11/directory/list` | `paginated` | - | - | `ListDirectoriesResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Lists directories created within an account. |
| `ListFacetAttributes` | `POST /amazonclouddirectory/2017-01-11/facet/attributes` | `paginated` | `Name`, `SchemaArn` | - | `ListFacetAttributesResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Retrieves attributes attached to the facet. |
| `ListFacetNames` | `POST /amazonclouddirectory/2017-01-11/facet/list` | `paginated` | `SchemaArn` | - | `ListFacetNamesResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves the names of facets that exist in a schema. |
| `ListIncomingTypedLinks` | `POST /amazonclouddirectory/2017-01-11/typedlink/incoming` | - | `DirectoryArn`, `ObjectReference` | - | `ListIncomingTypedLinksResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, ... (+2) | Returns a paginated list of all the incoming TypedLinkSpecifier information for an object. It also supports filtering by typed link facet and identity attributes. |
| `ListIndex` | `POST /amazonclouddirectory/2017-01-11/index/targets` | `paginated` | `DirectoryArn`, `IndexReference` | - | `ListIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `NotIndexException`, ... (+3) | Lists objects attached to the specified index. |
| `ListManagedSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/managed` | `paginated` | - | - | `ListManagedSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `ResourceNotFoundException`, `ValidationException` | Lists the major version families of each managed schema. If a major version ARN is provided as SchemaArn, the minor version revisions in that family are listed instead. |
| `ListObjectAttributes` | `POST /amazonclouddirectory/2017-01-11/object/attributes` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, ... (+2) | Lists all attributes that are associated with an object. |
| `ListObjectChildren` | `POST /amazonclouddirectory/2017-01-11/object/children` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectChildrenResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `NotNodeException`, `ResourceNotFoundException`, ... (+2) | Returns a paginated list of child objects that are associated with a given object. |
| `ListObjectParentPaths` | `POST /amazonclouddirectory/2017-01-11/object/parentpaths` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectParentPathsResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see Directory Structure. |
| `ListObjectParents` | `POST /amazonclouddirectory/2017-01-11/object/parent` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectParentsResponse` | `AccessDeniedException`, `CannotListParentOfRootException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, ... (+2) | Lists parent objects that are associated with a given object in pagination fashion. |
| `ListObjectPolicies` | `POST /amazonclouddirectory/2017-01-11/object/policy` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectPoliciesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Returns policies attached to an object in pagination fashion. |
| `ListOutgoingTypedLinks` | `POST /amazonclouddirectory/2017-01-11/typedlink/outgoing` | - | `DirectoryArn`, `ObjectReference` | - | `ListOutgoingTypedLinksResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, ... (+2) | Returns a paginated list of all the outgoing TypedLinkSpecifier information for an object. It also supports filtering by typed link facet and identity attributes. |
| `ListPolicyAttachments` | `POST /amazonclouddirectory/2017-01-11/policy/attachment` | `paginated` | `DirectoryArn`, `PolicyReference` | - | `ListPolicyAttachmentsResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `NotPolicyException`, `ResourceNotFoundException`, ... (+2) | Returns all of the `ObjectIdentifiers` to which a given policy is attached. |
| `ListPublishedSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/published` | `paginated` | - | - | `ListPublishedSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists the major version families of each published schema. If a major version ARN is provided as `SchemaArn`, the minor version revisions in that family are listed instead. |
| `ListTagsForResource` | `POST /amazonclouddirectory/2017-01-11/tags` | `paginated` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidTaggingRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. |
| `ListTypedLinkFacetAttributes` | `POST /amazonclouddirectory/2017-01-11/typedlink/facet/attributes` | `paginated` | `Name`, `SchemaArn` | - | `ListTypedLinkFacetAttributesResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Returns a paginated list of all attribute definitions for a particular TypedLinkFacet. For more information, see Typed Links. |
| `ListTypedLinkFacetNames` | `POST /amazonclouddirectory/2017-01-11/typedlink/facet/list` | `paginated` | `SchemaArn` | - | `ListTypedLinkFacetNamesResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns a paginated list of `TypedLink` facet names for a particular schema. For more information, see Typed Links. |
| `LookupPolicy` | `POST /amazonclouddirectory/2017-01-11/policy/lookup` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `LookupPolicyResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Lists all policies from the root of the Directory to the object specified. If there are no policies present, an empty list is returned. |
| `PublishSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/publish` | - | `DevelopmentSchemaArn`, `Version` | - | `PublishSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `SchemaAlreadyPublishedException`, `ValidationException` | Publishes a development schema with a major version and a recommended minor version. |
| `PutSchemaFromJson` | `PUT /amazonclouddirectory/2017-01-11/schema/json` | - | `Document`, `SchemaArn` | - | `PutSchemaFromJsonResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidRuleException`, `InvalidSchemaDocException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Allows a schema to be updated using JSON upload. Only available for development schemas. |
| `RemoveFacetFromObject` | `PUT /amazonclouddirectory/2017-01-11/object/facets/delete` | - | `DirectoryArn`, `ObjectReference`, `SchemaFacet` | - | `RemoveFacetFromObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Removes the specified facet from the specified object. |
| `TagResource` | `PUT /amazonclouddirectory/2017-01-11/tags/add` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidTaggingRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | An API operation for adding tags to a resource. |
| `UntagResource` | `PUT /amazonclouddirectory/2017-01-11/tags/remove` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidTaggingRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | An API operation for removing tags from a resource. |
| `UpdateFacet` | `PUT /amazonclouddirectory/2017-01-11/facet` | - | `Name`, `SchemaArn` | - | `UpdateFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidFacetUpdateException`, `InvalidRuleException`, `LimitExceededException`, ... (+3) | Does the following: Adds new `Attributes`, `Rules`, or `ObjectTypes`. Updates existing `Attributes`, `Rules`, or `ObjectTypes`. |
| `UpdateLinkAttributes` | `POST /amazonclouddirectory/2017-01-11/typedlink/attributes/update` | - | `AttributeUpdates`, `DirectoryArn`, `TypedLinkSpecifier` | - | `UpdateLinkAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Updates a given typed link’s attributes. Attributes to be updated must not contribute to the typed link’s identity, as defined by its `IdentityAttributeOrder`. |
| `UpdateObjectAttributes` | `PUT /amazonclouddirectory/2017-01-11/object/update` | - | `AttributeUpdates`, `DirectoryArn`, `ObjectReference` | - | `UpdateObjectAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `ResourceNotFoundException`, ... (+2) | Updates a given object's attributes. |
| `UpdateSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/update` | - | `Name`, `SchemaArn` | - | `UpdateSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Updates the schema name with a new name. Only development schema names can be updated. |
| `UpdateTypedLinkFacet` | `PUT /amazonclouddirectory/2017-01-11/typedlink/facet` | - | `AttributeUpdates`, `IdentityAttributeOrder`, `Name`, `SchemaArn` | - | `UpdateTypedLinkFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidFacetUpdateException`, `InvalidRuleException`, `LimitExceededException`, ... (+3) | Updates a TypedLinkFacet. For more information, see Typed Links. |
| `UpgradeAppliedSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/upgradeapplied` | - | `DirectoryArn`, `PublishedSchemaArn` | - | `UpgradeAppliedSchemaResponse` | `AccessDeniedException`, `IncompatibleSchemaException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `ResourceNotFoundException`, `RetryableConflictException`, `SchemaAlreadyExistsException`, ... (+1) | Upgrades a single directory in-place using the `PublishedSchemaArn` with schema updates found in `MinorVersion`. Backwards-compatible minor version upgrades are instantaneously available for readers on all objects in the directory. |
| `UpgradePublishedSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/upgradepublished` | - | `DevelopmentSchemaArn`, `MinorVersion`, `PublishedSchemaArn` | - | `UpgradePublishedSchemaResponse` | `AccessDeniedException`, `IncompatibleSchemaException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, ... (+1) | Upgrades a published schema under a new minor version revision using the current contents of `DevelopmentSchemaArn`. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AddFacetToObject` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `ApplySchema` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `AttachObject` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `AttachPolicy` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `AttachToIndex` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `AttachTypedLink` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `BatchRead` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `BatchWrite` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `CreateDirectory` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `CreateFacet` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `CreateIndex` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `CreateObject` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `CreateTypedLinkFacet` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `DeleteDirectory` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `DeleteFacet` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `DeleteObject` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `DeleteSchema` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `DeleteTypedLinkFacet` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `DetachFromIndex` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `DetachObject` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `DetachPolicy` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `DetachTypedLink` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `DisableDirectory` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `EnableDirectory` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `GetDirectory` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `GetFacet` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `GetLinkAttributes` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `GetObjectAttributes` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `GetObjectInformation` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `GetSchemaAsJson` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `GetTypedLinkFacetInformation` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `ListAttachedIndices` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `ListFacetAttributes` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `ListFacetNames` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `ListIncomingTypedLinks` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `ListIndex` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `ListObjectAttributes` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `ListObjectChildren` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `ListObjectParentPaths` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `ListObjectParents` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `ListObjectPolicies` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `ListOutgoingTypedLinks` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `ListPolicyAttachments` | `DirectoryArn -> x-amz-data-partition`, `ConsistencyLevel -> x-amz-consistency-level` | - | - | - |
| `ListTypedLinkFacetAttributes` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `ListTypedLinkFacetNames` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `LookupPolicy` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `PublishSchema` | `DevelopmentSchemaArn -> x-amz-data-partition` | - | - | - |
| `PutSchemaFromJson` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `RemoveFacetFromObject` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `UpdateFacet` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `UpdateLinkAttributes` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `UpdateObjectAttributes` | `DirectoryArn -> x-amz-data-partition` | - | - | - |
| `UpdateSchema` | `SchemaArn -> x-amz-data-partition` | - | - | - |
| `UpdateTypedLinkFacet` | `SchemaArn -> x-amz-data-partition` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | Access denied or directory not found. |
| `InternalServiceException` | `structure` | `Message` | Indicates a problem that must be resolved by Amazon Web Services. |
| `InvalidArnException` | `structure` | `Message` | Indicates that the provided ARN value is not valid. |
| `ValidationException` | `structure` | `Message` | Indicates that your request is malformed in some manner. |
| `RetryableConflictException` | `structure` | `Message` | Occurs when a conflict with a previous successful write is detected. |
| `LimitExceededException` | `structure` | `Message` | Indicates that limits are exceeded. |
| `ResourceNotFoundException` | `structure` | `Message` | The specified resource could not be found. |
| `DirectoryNotEnabledException` | `structure` | `Message` | Operations are only permitted on enabled directories. |
| `InvalidNextTokenException` | `structure` | `Message` | Indicates that the `NextToken` value is not valid. |
| `FacetValidationException` | `structure` | `Message` | The Facet that you provided was not well formed or could not be validated with the schema. |
| `FacetNotFoundException` | `structure` | `Message` | The specified Facet could not be found. |
| `InvalidAttachmentException` | `structure` | `Message` | Indicates that an attempt to make an attachment was invalid. |
| `LinkNameAlreadyInUseException` | `structure` | `Message` | Indicates that a link could not be created due to a naming conflict. |
| `InvalidRuleException` | `structure` | `Message` | Occurs when any of the rule parameter keys or values are invalid. |
| `SchemaAlreadyExistsException` | `structure` | `Message` | Indicates that a schema could not be created due to a naming conflict. |
| `NotPolicyException` | `structure` | `Message` | Indicates that the requested operation can only operate on policy objects. |
| `NotIndexException` | `structure` | `Message` | Indicates that the requested operation can only operate on index objects. |
| `DirectoryDeletedException` | `structure` | `Message` | A directory that has been deleted and to which access has been attempted. |
| `InvalidTaggingRequestException` | `structure` | `Message` | Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed limit. |
| `FacetAlreadyExistsException` | `structure` | `Message` | A facet with the same name already exists. |
| `UnsupportedIndexTypeException` | `structure` | `Message` | Indicates that the requested index type is not supported. |
| `NotNodeException` | `structure` | `Message` | Occurs when any invalid operations are performed on an object that is not a node, such as calling `ListObjectChildren` for a leaf node object. |
| `InvalidFacetUpdateException` | `structure` | `Message` | An attempt to modify a Facet resulted in an invalid schema exception. |
| `IncompatibleSchemaException` | `structure` | `Message` | Indicates a failure occurred while performing a check for backward compatibility between the specified schema and the schema that is currently applied to the directory. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
