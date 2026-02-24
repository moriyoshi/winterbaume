# AWS Elemental MediaStore Data Plane

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

An AWS Elemental MediaStore asset is an object, similar to an object in the Amazon S3 service. Objects are the fundamental entities that are stored in AWS Elemental MediaStore.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-mediastoredata/tests/scenario_test.rs`: ingest a media object, verify metadata and body retrieval, and remove it.
- Backported from `scenario_test.rs`: keep independent container paths isolated for concurrent media assets.
- From the AWS documentation and model: support object upload, path-based retrieval, listing, deletion, content-range style media access, and container-scoped data-plane isolation.

## Service Identity and Protocol

- AWS model slug: `mediastore-data`
- AWS SDK for Rust slug: `mediastoredata`
- Model version: `2017-09-01`
- Model file: `vendor/api-models-aws/models/mediastore-data/service/2017-09-01/mediastore-data-2017-09-01.json`
- SDK ID: `MediaStore Data`
- Endpoint prefix: `data.mediastore`
- ARN namespace: `mediastore`
- CloudFormation name: `MediaStoreData`
- CloudTrail event source: `mediastoredata.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (1), `Describe` (1), `Get` (1), `List` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteObject`, `PutObject`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeObject`, `GetObject`, `ListItems`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.

## Control-Plane / Data-Plane Coherence

- **Paired with `mediastore`.** This data plane reads and writes objects that live **inside containers** created by the MediaStore control plane ( `winterbaume-mediastore` ) via `CreateContainer`. In real AWS each container has its own data-plane endpoint URL ( `<container-name>.data.mediastore.<region>.amazonaws.com` ); `PutObject` / `GetObject` calls are scoped to a single container by virtue of the endpoint they target.
- **Current Winterbaume status: divergent and the model is wrong.** This crate stores objects in a single global `objects: HashMap<String, MediaStoreObject>` without any container concept, so an object PUT into one container is visible to GET requests targeting any other container, and `DeleteContainer` on the control plane does not affect the data plane's stored objects.
- **What needs to change:** key the `objects` map by `(container_name, path)` ( derived from the request's host header / endpoint ) and observe `winterbaume-mediastore`'s `containers` state so that operations against an unknown or `DELETING` container fail with the right error.

## Operation Groups

### Delete

- Operations: `DeleteObject`
- Common required input members in this group: `Path`

### Describe

- Operations: `DescribeObject`
- Common required input members in this group: `Path`

### Get

- Operations: `GetObject`
- Common required input members in this group: `Path`

### List

- Operations: `ListItems`
- Traits: `paginated` (1)

### Put

- Operations: `PutObject`
- Common required input members in this group: `Body`, `Path`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteObject` | `DELETE /{Path+}` | - | `Path` | - | `DeleteObjectResponse` | `ContainerNotFoundException`, `InternalServerError`, `ObjectNotFoundException` | Deletes an object at the specified path. |
| `DescribeObject` | `HEAD /{Path+}` | - | `Path` | - | `DescribeObjectResponse` | `ContainerNotFoundException`, `InternalServerError`, `ObjectNotFoundException` | Gets the headers for an object at the specified path. |
| `GetObject` | `GET /{Path+}` | - | `Path` | - | `GetObjectResponse` | `ContainerNotFoundException`, `InternalServerError`, `ObjectNotFoundException`, `RequestedRangeNotSatisfiableException` | Downloads the object at the specified path. If the object’s upload availability is set to `streaming`, AWS Elemental MediaStore downloads the object even if it’s still uploading the object. |
| `ListItems` | `GET /` | `paginated` | - | - | `ListItemsResponse` | `ContainerNotFoundException`, `InternalServerError` | Provides a list of metadata entries about folders and objects in the specified folder. |
| `PutObject` | `PUT /{Path+}` | - | `Body`, `Path` | - | `PutObjectResponse` | `ContainerNotFoundException`, `InternalServerError` | Uploads an object to the specified path. Object sizes are limited to 25 MB for standard upload availability and 10 MB for streaming upload availability. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ContainerNotFoundException` | `structure` | `Message` | The specified container was not found for the specified account. |
| `InternalServerError` | `structure` | `Message` | The service is temporarily unavailable. |
| `ObjectNotFoundException` | `structure` | `Message` | Could not perform an operation on an object that does not exist. |
| `DeleteObjectRequest` | `structure` | `Path` | - |
| `DeleteObjectResponse` | `structure` | - | - |
| `DescribeObjectRequest` | `structure` | `Path` | - |
| `DescribeObjectResponse` | `structure` | `CacheControl`, `ContentLength`, `ContentType`, `ETag`, `LastModified` | - |
| `GetObjectRequest` | `structure` | `Path`, `Range` | - |
| `GetObjectResponse` | `structure` | `Body`, `CacheControl`, `ContentLength`, `ContentRange`, `ContentType`, `ETag`, `LastModified`, `StatusCode` | - |
| `RequestedRangeNotSatisfiableException` | `structure` | `Message` | The requested content range is not valid. |
| `ListItemsRequest` | `structure` | `MaxResults`, `NextToken`, `Path` | - |
| `ListItemsResponse` | `structure` | `Items`, `NextToken` | - |
| `PutObjectRequest` | `structure` | `Body`, `CacheControl`, `ContentType`, `Path`, `StorageClass`, `UploadAvailability` | - |
| `PutObjectResponse` | `structure` | `ContentSHA256`, `ETag`, `StorageClass` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
