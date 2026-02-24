# AWS SimSpace Weaver

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

SimSpace Weaver (SimSpace Weaver) is a service that you can use to build and run large-scale spatial simulations in the Amazon Web Services Cloud. For example, you can create crowd simulations, large real-world environments, and immersive and interactive experiences. For more information about SimSpace Weaver, see the SimSpace Weaver User Guide . This API reference describes the API operations and data types that you can use to communicate directly with SimSpace Weaver. SimSpace Weaver also provides the SimSpace Weaver app SDK, which you use for app development.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-simspaceweaver/tests/scenario_test.rs`: run a full simulation pipeline through create/start/describe/list/stop/delete style operations.
- Backported from `scenario_test.rs`: tag and untag simulation resources.
- Scenario insight from EC2: add full state-machine walks for AWS SimSpace Weaver resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: model spatial simulation lifecycle, app and simulation metadata, snapshot/clock style state, long-running simulation transitions, and tag-based administration.

## Service Identity and Protocol

- AWS model slug: `simspaceweaver`
- AWS SDK for Rust slug: `simspaceweaver`
- Model version: `2022-10-28`
- Model file: `vendor/api-models-aws/models/simspaceweaver/service/2022-10-28/simspaceweaver-2022-10-28.json`
- SDK ID: `SimSpaceWeaver`
- Endpoint prefix: `simspaceweaver`
- ARN namespace: `simspaceweaver`
- CloudFormation name: `SimSpaceWeaver`
- CloudTrail event source: `simspaceweaver.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (3), `Start` (3), `Stop` (3), `Delete` (2), `Describe` (2), `Create` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateSnapshot`, `DeleteApp`, `DeleteSimulation`, `StartApp`, `StartClock`, `StartSimulation`, `StopApp`, `StopClock`, `StopSimulation`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApp`, `DescribeSimulation`, `ListApps`, `ListSimulations`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartApp`, `StartClock`, `StartSimulation`, `StopApp`, `StopClock`, `StopSimulation`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Simulation` | `SimulationName` | create: `StartSimulation`; read: `DescribeSimulation`; update: `StopSimulation`; delete: `DeleteSimulation`; list: `ListSimulations` | `CreateSnapshot`, `DeleteApp`, `DescribeApp`, `ListApps`, `StartApp`, `StartClock`, `StopApp`, `StopClock` | Represents a simulation |
## Operation Groups

### List

- Operations: `ListApps`, `ListSimulations`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `ResourceArn`, `Simulation`

### Start

- Operations: `StartApp`, `StartClock`, `StartSimulation`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `Domain`, `Name`, `RoleArn`, `Simulation`

### Stop

- Operations: `StopApp`, `StopClock`, `StopSimulation`
- Common required input members in this group: `App`, `Domain`, `Simulation`

### Delete

- Operations: `DeleteApp`, `DeleteSimulation`
- Traits: `idempotent` (2)
- Common required input members in this group: `App`, `Domain`, `Simulation`

### Describe

- Operations: `DescribeApp`, `DescribeSimulation`
- Traits: `readonly` (2)
- Common required input members in this group: `App`, `Domain`, `Simulation`

### Create

- Operations: `CreateSnapshot`
- Common required input members in this group: `Destination`, `Simulation`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateSnapshot` | `POST /createsnapshot` | - | `Destination`, `Simulation` | - | `CreateSnapshotOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates a snapshot of the specified simulation. A snapshot is a file that contains simulation state data at a specific time. |
| `DeleteApp` | `DELETE /deleteapp` | `idempotent` | `App`, `Domain`, `Simulation` | - | `DeleteAppOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the instance of the given custom app. |
| `DeleteSimulation` | `DELETE /deletesimulation` | `idempotent` | `Simulation` | - | `DeleteSimulationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes all SimSpace Weaver resources assigned to the given simulation. Your simulation uses resources in other Amazon Web Services. |
| `DescribeApp` | `GET /describeapp` | `readonly` | `App`, `Domain`, `Simulation` | - | `DescribeAppOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the state of the given custom app. |
| `DescribeSimulation` | `GET /describesimulation` | `readonly` | `Simulation` | - | `DescribeSimulationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the current state of the given simulation. |
| `ListApps` | `GET /listapps` | `readonly`, `paginated` | `Simulation` | - | `ListAppsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all custom apps or service apps for the given simulation and domain. |
| `ListSimulations` | `GET /listsimulations` | `readonly`, `paginated` | - | - | `ListSimulationsOutput` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Lists the SimSpace Weaver simulations in the Amazon Web Services account used to make the API call. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Lists all tags on a SimSpace Weaver resource. |
| `StartApp` | `POST /startapp` | `idempotency-token` | `Domain`, `Name`, `Simulation` | `ClientToken` | `StartAppOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Starts a custom app with the configuration specified in the simulation schema. |
| `StartClock` | `POST /startclock` | - | `Simulation` | - | `StartClockOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Starts the simulation clock. |
| `StartSimulation` | `POST /startsimulation` | `idempotency-token` | `Name`, `RoleArn` | `ClientToken` | `StartSimulationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Starts a simulation with the given name. You must choose to start your simulation from a schema or from a snapshot. |
| `StopApp` | `POST /stopapp` | - | `App`, `Domain`, `Simulation` | - | `StopAppOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Stops the given custom app and shuts down all of its allocated compute resources. |
| `StopClock` | `POST /stopclock` | - | `Simulation` | - | `StopClockOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Stops the simulation clock. |
| `StopSimulation` | `POST /stopsimulation` | - | `Simulation` | - | `StopSimulationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Stops the given simulation. You can't restart a simulation after you stop it. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Adds tags to a SimSpace Weaver resource. For more information about tags, see Tagging Amazon Web Services resources in the Amazon Web Services General Reference . |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `ResourceNotFoundException`, `ValidationException` | Removes tags from a SimSpace Weaver resource. For more information about tags, see Tagging Amazon Web Services resources in the Amazon Web Services General Reference . |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message` | - |
| `AccessDeniedException` | `structure` | `Message` | - |
| `InternalServerException` | `structure` | `Message` | - |
| `ResourceNotFoundException` | `structure` | `Message` | - |
| `ConflictException` | `structure` | `Message` | - |
| `ServiceQuotaExceededException` | `structure` | `Message` | - |
| `CreateSnapshotInput` | `structure` | `Destination`, `Simulation` | - |
| `CreateSnapshotOutput` | `structure` | - | - |
| `DeleteAppInput` | `structure` | `App`, `Domain`, `Simulation` | - |
| `DeleteAppOutput` | `structure` | - | - |
| `DeleteSimulationInput` | `structure` | `Simulation` | - |
| `DeleteSimulationOutput` | `structure` | - | - |
| `DescribeAppInput` | `structure` | `App`, `Domain`, `Simulation` | - |
| `DescribeAppOutput` | `structure` | `Description`, `Domain`, `EndpointInfo`, `LaunchOverrides`, `Name`, `Simulation`, `Status`, `TargetStatus` | - |
| `DescribeSimulationInput` | `structure` | `Simulation` | - |
| `DescribeSimulationOutput` | `structure` | `Arn`, `CreationTime`, `Description`, `ExecutionId`, `LiveSimulationState`, `LoggingConfiguration`, `MaximumDuration`, `Name`, `RoleArn`, `SchemaError`, `SchemaS3Location`, `SnapshotS3Location`, ... (+3) | - |
| `ListAppsInput` | `structure` | `Domain`, `MaxResults`, `NextToken`, `Simulation` | - |
| `ListAppsOutput` | `structure` | `Apps`, `NextToken` | - |
| `ListSimulationsInput` | `structure` | `MaxResults`, `NextToken` | - |
| `ListSimulationsOutput` | `structure` | `NextToken`, `Simulations` | - |
| `ListTagsForResourceInput` | `structure` | `ResourceArn` | - |
| `ListTagsForResourceOutput` | `structure` | `Tags` | - |
| `StartAppInput` | `structure` | `ClientToken`, `Description`, `Domain`, `LaunchOverrides`, `Name`, `Simulation` | - |
| `StartAppOutput` | `structure` | `Domain`, `Name`, `Simulation` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
