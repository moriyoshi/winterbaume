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
- Common required input members in this group: `DirectoryArn`, `SchemaArn`, `Name`, `ObjectReference`

### Get

- Operations: `GetAppliedSchemaVersion`, `GetDirectory`, `GetFacet`, `GetLinkAttributes`, `GetObjectAttributes`, `GetObjectInformation`, `GetSchemaAsJson`, `GetTypedLinkFacetInformation`
- Common required input members in this group: `SchemaArn`, `DirectoryArn`, `Name`, `AttributeNames`, `ObjectReference`

### Create

- Operations: `CreateDirectory`, `CreateFacet`, `CreateIndex`, `CreateObject`, `CreateSchema`, `CreateTypedLinkFacet`
- Common required input members in this group: `Name`, `SchemaArn`, `DirectoryArn`

### Delete

- Operations: `DeleteDirectory`, `DeleteFacet`, `DeleteObject`, `DeleteSchema`, `DeleteTypedLinkFacet`
- Common required input members in this group: `DirectoryArn`, `SchemaArn`, `Name`

### Update

- Operations: `UpdateFacet`, `UpdateLinkAttributes`, `UpdateObjectAttributes`, `UpdateSchema`, `UpdateTypedLinkFacet`
- Common required input members in this group: `SchemaArn`, `Name`, `DirectoryArn`, `AttributeUpdates`

### Attach

- Operations: `AttachObject`, `AttachPolicy`, `AttachToIndex`, `AttachTypedLink`
- Common required input members in this group: `DirectoryArn`

### Detach

- Operations: `DetachFromIndex`, `DetachObject`, `DetachPolicy`, `DetachTypedLink`
- Common required input members in this group: `DirectoryArn`

### Batch

- Operations: `BatchRead`, `BatchWrite`
- Common required input members in this group: `DirectoryArn`, `Operations`

### Upgrade

- Operations: `UpgradeAppliedSchema`, `UpgradePublishedSchema`
- Common required input members in this group: `PublishedSchemaArn`

### Add

- Operations: `AddFacetToObject`
- Common required input members in this group: -

### Apply

- Operations: `ApplySchema`
- Common required input members in this group: -

### Disable

- Operations: `DisableDirectory`
- Common required input members in this group: -

### Enable

- Operations: `EnableDirectory`
- Common required input members in this group: -

### Lookup

- Operations: `LookupPolicy`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Publish

- Operations: `PublishSchema`
- Common required input members in this group: -

### Put

- Operations: `PutSchemaFromJson`
- Common required input members in this group: -

### Remove

- Operations: `RemoveFacetFromObject`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddFacetToObject` | `PUT /amazonclouddirectory/2017-01-11/object/facets` | - | `DirectoryArn`, `SchemaFacet`, `ObjectReference` | - | `AddFacetToObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Adds a new Facet to an object. An object can have more than one facet applied on it. |
| `ApplySchema` | `PUT /amazonclouddirectory/2017-01-11/schema/apply` | - | `PublishedSchemaArn`, `DirectoryArn` | - | `ApplySchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `SchemaAlreadyExistsException`, `ValidationException` | Copies the input published schema, at the specified version, into the Directory with the same name and version as that of the published schema. |
| `AttachObject` | `PUT /amazonclouddirectory/2017-01-11/object/attach` | - | `DirectoryArn`, `ParentReference`, `ChildReference`, `LinkName` | - | `AttachObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Attaches an existing object to another object. An object can be accessed in two ways: Using the path Using ObjectIdentifier |
| `AttachPolicy` | `PUT /amazonclouddirectory/2017-01-11/policy/attach` | - | `DirectoryArn`, `PolicyReference`, `ObjectReference` | - | `AttachPolicyResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotPolicyException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Attaches a policy object to a regular object. An object can have a limited number of attached policies. |
| `AttachToIndex` | `PUT /amazonclouddirectory/2017-01-11/index/attach` | - | `DirectoryArn`, `IndexReference`, `TargetReference` | - | `AttachToIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `IndexedAttributeMissingException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `NotIndexException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Attaches the specified object to the specified index. |
| `AttachTypedLink` | `PUT /amazonclouddirectory/2017-01-11/typedlink/attach` | - | `DirectoryArn`, `SourceObjectReference`, `TargetObjectReference`, `TypedLinkFacet`, `Attributes` | - | `AttachTypedLinkResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Attaches a typed link to a specified source and target object. For more information, see Typed Links . |
| `BatchRead` | `POST /amazonclouddirectory/2017-01-11/batchread` | - | `DirectoryArn`, `Operations` | - | `BatchReadResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Performs all the read operations in a batch. |
| `BatchWrite` | `PUT /amazonclouddirectory/2017-01-11/batchwrite` | - | `DirectoryArn`, `Operations` | - | `BatchWriteResponse` | `AccessDeniedException`, `BatchWriteException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Performs all the write operations in a batch. Either all the operations succeed or none. |
| `CreateDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory/create` | - | `Name`, `SchemaArn` | - | `CreateDirectoryResponse` | `AccessDeniedException`, `DirectoryAlreadyExistsException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Creates a Directory by copying the published schema into the directory. A directory cannot be created without a schema. You can also quickly create a directory using a managed schema, called the QuickStartSchema . Fo ... |
| `CreateFacet` | `PUT /amazonclouddirectory/2017-01-11/facet/create` | - | `SchemaArn`, `Name` | - | `CreateFacetResponse` | `AccessDeniedException`, `FacetAlreadyExistsException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidRuleException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Creates a new Facet in a schema. Facet creation is allowed only in development or applied schemas. |
| `CreateIndex` | `PUT /amazonclouddirectory/2017-01-11/index` | - | `DirectoryArn`, `OrderedIndexedAttributeList`, `IsUnique` | - | `CreateIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `ResourceNotFoundException`, `RetryableConflictException`, `UnsupportedIndexTypeException`, `ValidationException` | Creates an index object. See Indexing and search for more information. |
| `CreateObject` | `PUT /amazonclouddirectory/2017-01-11/object` | - | `DirectoryArn`, `SchemaFacets` | - | `CreateObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `ResourceNotFoundException`, `RetryableConflictException`, `UnsupportedIndexTypeException`, `ValidationException` | Creates an object in a Directory . Additionally attaches the object to a parent, if a parent reference and LinkName is specified. An object is simply a collection of Facet attributes. You can also use this API call t ... |
| `CreateSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/create` | - | `Name` | - | `CreateSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `SchemaAlreadyExistsException`, `ValidationException` | Creates a new schema in a development state. A schema can exist in three phases: Development: This is a mutable phase of the schema. All new schemas are in the development phase. Once the schema is finalized, it can ... |
| `CreateTypedLinkFacet` | `PUT /amazonclouddirectory/2017-01-11/typedlink/facet/create` | - | `SchemaArn`, `Facet` | - | `CreateTypedLinkFacetResponse` | `AccessDeniedException`, `FacetAlreadyExistsException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidRuleException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Creates a TypedLinkFacet . For more information, see Typed Links . |
| `DeleteDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory` | - | `DirectoryArn` | - | `DeleteDirectoryResponse` | `AccessDeniedException`, `DirectoryDeletedException`, `DirectoryNotDisabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Deletes a directory. Only disabled directories can be deleted. A deleted directory cannot be undone. Exercise extreme caution when deleting directories. |
| `DeleteFacet` | `PUT /amazonclouddirectory/2017-01-11/facet/delete` | - | `SchemaArn`, `Name` | - | `DeleteFacetResponse` | `AccessDeniedException`, `FacetInUseException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Deletes a given Facet . All attributes and Rule s that are associated with the facet will be deleted. Only development schema facets are allowed deletion. |
| `DeleteObject` | `PUT /amazonclouddirectory/2017-01-11/object/delete` | - | `DirectoryArn`, `ObjectReference` | - | `DeleteObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ObjectNotDetachedException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Deletes an object and its associated attributes. Only objects with no children and no parents can be deleted. The maximum number of attributes that can be deleted during an object deletion is 30. For more information ... |
| `DeleteSchema` | `PUT /amazonclouddirectory/2017-01-11/schema` | - | `SchemaArn` | - | `DeleteSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `StillContainsLinksException`, `ValidationException` | Deletes a given schema. Schemas in a development and published state can only be deleted. |
| `DeleteTypedLinkFacet` | `PUT /amazonclouddirectory/2017-01-11/typedlink/facet/delete` | - | `SchemaArn`, `Name` | - | `DeleteTypedLinkFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Deletes a TypedLinkFacet . For more information, see Typed Links . |
| `DetachFromIndex` | `PUT /amazonclouddirectory/2017-01-11/index/detach` | - | `DirectoryArn`, `IndexReference`, `TargetReference` | - | `DetachFromIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotIndexException`, `ObjectAlreadyDetachedException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Detaches the specified object from the specified index. |
| `DetachObject` | `PUT /amazonclouddirectory/2017-01-11/object/detach` | - | `DirectoryArn`, `ParentReference`, `LinkName` | - | `DetachObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotNodeException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Detaches a given object from the parent object. The object that is to be detached from the parent is specified by the link name. |
| `DetachPolicy` | `PUT /amazonclouddirectory/2017-01-11/policy/detach` | - | `DirectoryArn`, `PolicyReference`, `ObjectReference` | - | `DetachPolicyResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `NotPolicyException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Detaches a policy from an object. |
| `DetachTypedLink` | `PUT /amazonclouddirectory/2017-01-11/typedlink/detach` | - | `DirectoryArn`, `TypedLinkSpecifier` | - | `Unit` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Detaches a typed link from a specified source and target object. For more information, see Typed Links . |
| `DisableDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory/disable` | - | `DirectoryArn` | - | `DisableDirectoryResponse` | `AccessDeniedException`, `DirectoryDeletedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Disables the specified directory. Disabled directories cannot be read or written to. Only enabled directories can be disabled. Disabled directories may be reenabled. |
| `EnableDirectory` | `PUT /amazonclouddirectory/2017-01-11/directory/enable` | - | `DirectoryArn` | - | `EnableDirectoryResponse` | `AccessDeniedException`, `DirectoryDeletedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Enables the specified directory. Only disabled directories can be enabled. Once enabled, the directory can then be read and written to. |
| `GetAppliedSchemaVersion` | `POST /amazonclouddirectory/2017-01-11/schema/getappliedschema` | - | `SchemaArn` | - | `GetAppliedSchemaVersionResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns current applied schema version ARN, including the minor version in use. |
| `GetDirectory` | `POST /amazonclouddirectory/2017-01-11/directory/get` | - | `DirectoryArn` | - | `GetDirectoryResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Retrieves metadata about a directory. |
| `GetFacet` | `POST /amazonclouddirectory/2017-01-11/facet` | - | `SchemaArn`, `Name` | - | `GetFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Gets details of the Facet , such as facet name, attributes, Rule s, or ObjectType . You can call this on all kinds of schema facets -- published, development, or applied. |
| `GetLinkAttributes` | `POST /amazonclouddirectory/2017-01-11/typedlink/attributes/get` | - | `DirectoryArn`, `TypedLinkSpecifier`, `AttributeNames` | - | `GetLinkAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves attributes that are associated with a typed link. |
| `GetObjectAttributes` | `POST /amazonclouddirectory/2017-01-11/object/attributes/get` | - | `DirectoryArn`, `ObjectReference`, `SchemaFacet`, `AttributeNames` | - | `GetObjectAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves attributes within a facet that are associated with an object. |
| `GetObjectInformation` | `POST /amazonclouddirectory/2017-01-11/object/information` | - | `DirectoryArn`, `ObjectReference` | - | `GetObjectInformationResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves metadata about an object. |
| `GetSchemaAsJson` | `POST /amazonclouddirectory/2017-01-11/schema/json` | - | `SchemaArn` | - | `GetSchemaAsJsonResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves a JSON representation of the schema. See JSON Schema Format for more information. |
| `GetTypedLinkFacetInformation` | `POST /amazonclouddirectory/2017-01-11/typedlink/facet/get` | - | `SchemaArn`, `Name` | - | `GetTypedLinkFacetInformationResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns the identity attribute order for a specific TypedLinkFacet . For more information, see Typed Links . |
| `ListAppliedSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/applied` | `paginated` | `DirectoryArn` | - | `ListAppliedSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists schema major versions applied to a directory. If SchemaArn is provided, lists the minor version. |
| `ListAttachedIndices` | `POST /amazonclouddirectory/2017-01-11/object/indices` | `paginated` | `DirectoryArn`, `TargetReference` | - | `ListAttachedIndicesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists indices attached to the specified object. |
| `ListDevelopmentSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/development` | `paginated` | - | - | `ListDevelopmentSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves each Amazon Resource Name (ARN) of schemas in the development state. |
| `ListDirectories` | `POST /amazonclouddirectory/2017-01-11/directory/list` | `paginated` | - | - | `ListDirectoriesResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Lists directories created within an account. |
| `ListFacetAttributes` | `POST /amazonclouddirectory/2017-01-11/facet/attributes` | `paginated` | `SchemaArn`, `Name` | - | `ListFacetAttributesResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves attributes attached to the facet. |
| `ListFacetNames` | `POST /amazonclouddirectory/2017-01-11/facet/list` | `paginated` | `SchemaArn` | - | `ListFacetNamesResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves the names of facets that exist in a schema. |
| `ListIncomingTypedLinks` | `POST /amazonclouddirectory/2017-01-11/typedlink/incoming` | - | `DirectoryArn`, `ObjectReference` | - | `ListIncomingTypedLinksResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns a paginated list of all the incoming TypedLinkSpecifier information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see Typed Links . |
| `ListIndex` | `POST /amazonclouddirectory/2017-01-11/index/targets` | `paginated` | `DirectoryArn`, `IndexReference` | - | `ListIndexResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `NotIndexException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists objects attached to the specified index. |
| `ListManagedSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/managed` | `paginated` | - | - | `ListManagedSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `ResourceNotFoundException`, `ValidationException` | Lists the major version families of each managed schema. If a major version ARN is provided as SchemaArn, the minor version revisions in that family are listed instead. |
| `ListObjectAttributes` | `POST /amazonclouddirectory/2017-01-11/object/attributes` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists all attributes that are associated with an object. |
| `ListObjectChildren` | `POST /amazonclouddirectory/2017-01-11/object/children` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectChildrenResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `NotNodeException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns a paginated list of child objects that are associated with a given object. |
| `ListObjectParentPaths` | `POST /amazonclouddirectory/2017-01-11/object/parentpaths` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectParentPathsResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Retrieves all available parent paths for any object type such as node, leaf node, policy node, and index node objects. For more information about objects, see Directory Structure . Use this API to evaluate all parent ... |
| `ListObjectParents` | `POST /amazonclouddirectory/2017-01-11/object/parent` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectParentsResponse` | `AccessDeniedException`, `CannotListParentOfRootException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists parent objects that are associated with a given object in pagination fashion. |
| `ListObjectPolicies` | `POST /amazonclouddirectory/2017-01-11/object/policy` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `ListObjectPoliciesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns policies attached to an object in pagination fashion. |
| `ListOutgoingTypedLinks` | `POST /amazonclouddirectory/2017-01-11/typedlink/outgoing` | - | `DirectoryArn`, `ObjectReference` | - | `ListOutgoingTypedLinksResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns a paginated list of all the outgoing TypedLinkSpecifier information for an object. It also supports filtering by typed link facet and identity attributes. For more information, see Typed Links . |
| `ListPolicyAttachments` | `POST /amazonclouddirectory/2017-01-11/policy/attachment` | `paginated` | `DirectoryArn`, `PolicyReference` | - | `ListPolicyAttachmentsResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `NotPolicyException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns all of the ObjectIdentifiers to which a given policy is attached. |
| `ListPublishedSchemaArns` | `POST /amazonclouddirectory/2017-01-11/schema/published` | `paginated` | - | - | `ListPublishedSchemaArnsResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists the major version families of each published schema. If a major version ARN is provided as SchemaArn , the minor version revisions in that family are listed instead. |
| `ListTagsForResource` | `POST /amazonclouddirectory/2017-01-11/tags` | `paginated` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidTaggingRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns tags for a resource. Tagging is currently supported only for directories with a limit of 50 tags per directory. All 50 tags are returned for a given directory with this API call. |
| `ListTypedLinkFacetAttributes` | `POST /amazonclouddirectory/2017-01-11/typedlink/facet/attributes` | `paginated` | `SchemaArn`, `Name` | - | `ListTypedLinkFacetAttributesResponse` | `AccessDeniedException`, `FacetNotFoundException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns a paginated list of all attribute definitions for a particular TypedLinkFacet . For more information, see Typed Links . |
| `ListTypedLinkFacetNames` | `POST /amazonclouddirectory/2017-01-11/typedlink/facet/list` | `paginated` | `SchemaArn` | - | `ListTypedLinkFacetNamesResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Returns a paginated list of TypedLink facet names for a particular schema. For more information, see Typed Links . |
| `LookupPolicy` | `POST /amazonclouddirectory/2017-01-11/policy/lookup` | `paginated` | `DirectoryArn`, `ObjectReference` | - | `LookupPolicyResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `InternalServiceException`, `InvalidArnException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Lists all policies from the root of the Directory to the object specified. If there are no policies present, an empty list is returned. If policies are present, and if some objects don't have the policies attached, i ... |
| `PublishSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/publish` | - | `DevelopmentSchemaArn`, `Version` | - | `PublishSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `SchemaAlreadyPublishedException`, `ValidationException` | Publishes a development schema with a major version and a recommended minor version. |
| `PutSchemaFromJson` | `PUT /amazonclouddirectory/2017-01-11/schema/json` | - | `SchemaArn`, `Document` | - | `PutSchemaFromJsonResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidRuleException`, `InvalidSchemaDocException`, `LimitExceededException`, `RetryableConflictException`, `ValidationException` | Allows a schema to be updated using JSON upload. Only available for development schemas. See JSON Schema Format for more information. |
| `RemoveFacetFromObject` | `PUT /amazonclouddirectory/2017-01-11/object/facets/delete` | - | `DirectoryArn`, `SchemaFacet`, `ObjectReference` | - | `RemoveFacetFromObjectResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Removes the specified facet from the specified object. |
| `TagResource` | `PUT /amazonclouddirectory/2017-01-11/tags/add` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidTaggingRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | An API operation for adding tags to a resource. |
| `UntagResource` | `PUT /amazonclouddirectory/2017-01-11/tags/remove` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `InvalidTaggingRequestException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | An API operation for removing tags from a resource. |
| `UpdateFacet` | `PUT /amazonclouddirectory/2017-01-11/facet` | - | `SchemaArn`, `Name` | - | `UpdateFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidFacetUpdateException`, `InvalidRuleException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Does the following: Adds new Attributes , Rules , or ObjectTypes . Updates existing Attributes , Rules , or ObjectTypes . Deletes existing Attributes , Rules , or ObjectTypes . |
| `UpdateLinkAttributes` | `POST /amazonclouddirectory/2017-01-11/typedlink/attributes/update` | - | `DirectoryArn`, `TypedLinkSpecifier`, `AttributeUpdates` | - | `UpdateLinkAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Updates a given typed link’s attributes. Attributes to be updated must not contribute to the typed link’s identity, as defined by its IdentityAttributeOrder . |
| `UpdateObjectAttributes` | `PUT /amazonclouddirectory/2017-01-11/object/update` | - | `DirectoryArn`, `ObjectReference`, `AttributeUpdates` | - | `UpdateObjectAttributesResponse` | `AccessDeniedException`, `DirectoryNotEnabledException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `LinkNameAlreadyInUseException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Updates a given object's attributes. |
| `UpdateSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/update` | - | `SchemaArn`, `Name` | - | `UpdateSchemaResponse` | `AccessDeniedException`, `InternalServiceException`, `InvalidArnException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Updates the schema name with a new name. Only development schema names can be updated. |
| `UpdateTypedLinkFacet` | `PUT /amazonclouddirectory/2017-01-11/typedlink/facet` | - | `SchemaArn`, `Name`, `AttributeUpdates`, `IdentityAttributeOrder` | - | `UpdateTypedLinkFacetResponse` | `AccessDeniedException`, `FacetNotFoundException`, `FacetValidationException`, `InternalServiceException`, `InvalidArnException`, `InvalidFacetUpdateException`, `InvalidRuleException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Updates a TypedLinkFacet . For more information, see Typed Links . |
| `UpgradeAppliedSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/upgradeapplied` | - | `PublishedSchemaArn`, `DirectoryArn` | - | `UpgradeAppliedSchemaResponse` | `AccessDeniedException`, `IncompatibleSchemaException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `ResourceNotFoundException`, `RetryableConflictException`, `SchemaAlreadyExistsException`, `ValidationException` | Upgrades a single directory in-place using the PublishedSchemaArn with schema updates found in MinorVersion . Backwards-compatible minor version upgrades are instantaneously available for readers on all objects in th ... |
| `UpgradePublishedSchema` | `PUT /amazonclouddirectory/2017-01-11/schema/upgradepublished` | - | `DevelopmentSchemaArn`, `PublishedSchemaArn`, `MinorVersion` | - | `UpgradePublishedSchemaResponse` | `AccessDeniedException`, `IncompatibleSchemaException`, `InternalServiceException`, `InvalidArnException`, `InvalidAttachmentException`, `LimitExceededException`, `ResourceNotFoundException`, `RetryableConflictException`, `ValidationException` | Upgrades a published schema under a new minor version revision using the current contents of DevelopmentSchemaArn . |

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
| `AccessDeniedException` | `structure` | Message | Access denied or directory not found. Either you don't have permissions for this directory or the directory does not exist. Try calling ListDirectories and ... |
| `BatchWriteException` | `structure` | Index, Type, Message | A BatchWrite exception has occurred. |
| `CannotListParentOfRootException` | `structure` | Message | Cannot list the parents of a Directory root. |
| `DirectoryAlreadyExistsException` | `structure` | Message | Indicates that a Directory could not be created due to a naming conflict. Choose a different name and try again. |
| `DirectoryDeletedException` | `structure` | Message | A directory that has been deleted and to which access has been attempted. Note: The requested resource will eventually cease to exist. |
| `DirectoryNotDisabledException` | `structure` | Message | An operation can only operate on a disabled directory. |
| `DirectoryNotEnabledException` | `structure` | Message | Operations are only permitted on enabled directories. |
| `FacetAlreadyExistsException` | `structure` | Message | A facet with the same name already exists. |
| `FacetInUseException` | `structure` | Message | Occurs when deleting a facet that contains an attribute that is a target to an attribute reference in a different facet. |
| `FacetNotFoundException` | `structure` | Message | The specified Facet could not be found. |
| `FacetValidationException` | `structure` | Message | The Facet that you provided was not well formed or could not be validated with the schema. |
| `IncompatibleSchemaException` | `structure` | Message | Indicates a failure occurred while performing a check for backward compatibility between the specified schema and the schema that is currently applied to th ... |
| `IndexedAttributeMissingException` | `structure` | Message | An object has been attempted to be attached to an object that does not have the appropriate attribute value. |
| `InternalServiceException` | `structure` | Message | Indicates a problem that must be resolved by Amazon Web Services. This might be a transient error in which case you can retry your request until it succeeds ... |
| `InvalidArnException` | `structure` | Message | Indicates that the provided ARN value is not valid. |
| `InvalidAttachmentException` | `structure` | Message | Indicates that an attempt to make an attachment was invalid. For example, attaching two nodes with a link type that is not applicable to the nodes or attemp ... |
| `InvalidFacetUpdateException` | `structure` | Message | An attempt to modify a Facet resulted in an invalid schema exception. |
| `InvalidNextTokenException` | `structure` | Message | Indicates that the NextToken value is not valid. |
| `InvalidRuleException` | `structure` | Message | Occurs when any of the rule parameter keys or values are invalid. |
| `InvalidSchemaDocException` | `structure` | Message | Indicates that the provided SchemaDoc value is not valid. |
| `InvalidTaggingRequestException` | `structure` | Message | Can occur for multiple reasons such as when you tag a resource that doesn’t exist or if you specify a higher number of tags for a resource than the allowed ... |
| `LimitExceededException` | `structure` | Message | Indicates that limits are exceeded. See Limits for more information. |
| `LinkNameAlreadyInUseException` | `structure` | Message | Indicates that a link could not be created due to a naming conflict. Choose a different name and then try again. |
| `NotIndexException` | `structure` | Message | Indicates that the requested operation can only operate on index objects. |
| `NotNodeException` | `structure` | Message | Occurs when any invalid operations are performed on an object that is not a node, such as calling ListObjectChildren for a leaf node object. |
| `NotPolicyException` | `structure` | Message | Indicates that the requested operation can only operate on policy objects. |
| `ObjectAlreadyDetachedException` | `structure` | Message | Indicates that the object is not attached to the index. |
| `ObjectNotDetachedException` | `structure` | Message | Indicates that the requested operation cannot be completed because the object has not been detached from the tree. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource could not be found. |
| `RetryableConflictException` | `structure` | Message | Occurs when a conflict with a previous successful write is detected. For example, if a write operation occurs on an object and then an attempt is made to re ... |
| `SchemaAlreadyExistsException` | `structure` | Message | Indicates that a schema could not be created due to a naming conflict. Please select a different name and then try again. |
| `SchemaAlreadyPublishedException` | `structure` | Message | Indicates that a schema is already published. |
| `StillContainsLinksException` | `structure` | Message | The object could not be deleted because links still exist. Remove the links and then try the operation again. |
| `UnsupportedIndexTypeException` | `structure` | Message | Indicates that the requested index type is not supported. |
| `ValidationException` | `structure` | Message | Indicates that your request is malformed in some manner. See the exception message. |
| `AddFacetToObjectRequest` | `structure` | DirectoryArn, SchemaFacet, ObjectAttributeList, ObjectReference | - |
| `AddFacetToObjectResponse` | `structure` | **empty (no members)** | - |
| `ApplySchemaRequest` | `structure` | PublishedSchemaArn, DirectoryArn | - |
| `ApplySchemaResponse` | `structure` | AppliedSchemaArn, DirectoryArn | - |
| `AttachObjectRequest` | `structure` | DirectoryArn, ParentReference, ChildReference, LinkName | - |
| `BatchReadExceptionType` | `enum` | ValidationException, InvalidArnException, ResourceNotFoundException, InvalidNextTokenException, AccessDeniedException, NotNodeException, FacetValidationException, CannotListParentOfRootException, NotIndexException, NotPolicyException, DirectoryNotEnabledException, LimitExceededException, ... (+1) | - |
| `BatchWriteExceptionType` | `enum` | InternalServiceException, ValidationException, InvalidArnException, LinkNameAlreadyInUseException, StillContainsLinksException, FacetValidationException, ObjectNotDetachedException, ResourceNotFoundException, AccessDeniedException, InvalidAttachmentException, NotIndexException, NotNodeException, ... (+6) | - |
| `ConsistencyLevel` | `enum` | SERIALIZABLE, EVENTUAL | - |
| `DirectoryState` | `enum` | ENABLED, DISABLED, DELETED | - |
| `FacetAttributeType` | `enum` | STRING, BINARY, BOOLEAN, NUMBER, DATETIME, VARIANT | - |
| `FacetStyle` | `enum` | STATIC, DYNAMIC | - |
| `ObjectType` | `enum` | NODE, LEAF_NODE, POLICY, INDEX | - |
| `RangeMode` | `enum` | FIRST, LAST, LAST_BEFORE_MISSING_VALUES, INCLUSIVE, EXCLUSIVE | - |
| `RequiredAttributeBehavior` | `enum` | REQUIRED_ALWAYS, NOT_REQUIRED | - |
| `RuleType` | `enum` | BINARY_LENGTH, NUMBER_COMPARISON, STRING_FROM_SET, STRING_LENGTH | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
