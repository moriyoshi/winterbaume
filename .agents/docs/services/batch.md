# AWS Batch

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Batch Using Batch, you can run batch computing workloads on the Amazon Web Services Cloud. Batch computing is a common means for developers, scientists, and engineers to access large amounts of compute resources. Batch uses the advantages of the batch computing to remove the undifferentiated heavy lifting of configuring and managing required infrastructure. At the same time, it also adopts a familiar batch computing software approach. You can use Batch to efficiently provision resources, and work toward eliminating capacity constraints, reducing your overall compute costs, and delivering results more quickly.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Batch resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: manage compute environments, job queues, job definitions, scheduling policies, consumable resources, and submitted jobs.
- From the operation surface: model batch-processing pipelines, job dependency chains, array jobs, fair-share scheduling, retry/termination behaviour, and tag-based workload inventory.

## Service Identity and Protocol

- AWS model slug: `batch`
- AWS SDK for Rust slug: `batch`
- Model version: `2016-08-10`
- Model file: `vendor/api-models-aws/models/batch/service/2016-08-10/batch-2016-08-10.json`
- SDK ID: `Batch`
- Endpoint prefix: `batch`
- ARN namespace: `batch`
- CloudFormation name: `Batch`
- CloudTrail event source: `batch.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (8), `List` (6), `Create` (5), `Delete` (5), `Update` (5), `Submit` (2), `Terminate` (2), `Cancel` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelJob`, `CreateComputeEnvironment`, `CreateConsumableResource`, `CreateJobQueue`, `CreateSchedulingPolicy`, `CreateServiceEnvironment`, `DeleteComputeEnvironment`, `DeleteConsumableResource`, `DeleteJobQueue`, `DeleteSchedulingPolicy`, `DeleteServiceEnvironment`, `DeregisterJobDefinition`, `RegisterJobDefinition`, `SubmitJob`, `SubmitServiceJob`, `TagResource`, `TerminateJob`, `TerminateServiceJob`, `UntagResource`, `UpdateComputeEnvironment`, ... (+4).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeComputeEnvironments`, `DescribeConsumableResource`, `DescribeJobDefinitions`, `DescribeJobQueues`, `DescribeJobs`, `DescribeSchedulingPolicies`, `DescribeServiceEnvironments`, `DescribeServiceJob`, `GetJobQueueSnapshot`, `ListConsumableResources`, `ListJobs`, `ListJobsByConsumableResource`, `ListSchedulingPolicies`, `ListServiceJobs`, `ListTagsForResource`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelJob`, `CreateJobQueue`, `DeleteJobQueue`, `DeregisterJobDefinition`, `DescribeJobDefinitions`, `DescribeJobQueues`, `DescribeJobs`, `DescribeServiceJob`, `GetJobQueueSnapshot`, `ListJobs`, `ListJobsByConsumableResource`, `ListServiceJobs`, `RegisterJobDefinition`, `SubmitJob`, `SubmitServiceJob`, `TerminateJob`, `TerminateServiceJob`, `UpdateJobQueue`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 39 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SQS`, `EC2/VPC`, `ECR`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/batch/latest/userguide/job_states.html
- https://docs.aws.amazon.com/batch/latest/userguide/job_queues.html
- https://docs.aws.amazon.com/batch/latest/userguide/service-job-retries.html

Research outcomes:
- Batch jobs progress through SUBMITTED, PENDING, RUNNABLE, STARTING, RUNNING, and terminal SUCCEEDED or FAILED states.
- SUBMITTED jobs are evaluated for dependencies. Jobs with unmet dependencies move to PENDING; jobs without dependencies move to RUNNABLE.
- RUNNABLE jobs can remain runnable indefinitely when mapped compute environments lack sufficient resources.
- STARTING covers container image pulls, EKS init containers, and ECS container dependencies. STARTING duration is not counted against job timeouts; timeout duration starts at RUNNING.
- RUNNING success or failure is determined by container exit code. Exit code 0 succeeds; non-zero fails unless retry attempts remain.
- SUCCEEDED and FAILED job states are retained for at least seven days.
- Job queues have priorities that determine which queues the scheduler evaluates first, and can be mapped to different compute environments.
- Retry attempts count total RUNNABLE placements, including the initial attempt. Conditional retry strategies can match status reasons with wildcards and choose RETRY or EXIT.

Parity implications:
- Model compute environments, job queues, job definitions, jobs, dependencies, attempts, array jobs, logs, retry strategies, scheduling state, and terminal retention.
- Job state transitions should distinguish dependency evaluation, resource scheduling, container start, RUNNING timeout start, retry backoff, and terminal persistence.
- Queue priority and compute-environment resource availability should influence scheduling outcomes instead of every submitted job immediately running.

## Operation Groups

### Describe

- Operations: `DescribeComputeEnvironments`, `DescribeConsumableResource`, `DescribeJobDefinitions`, `DescribeJobQueues`, `DescribeJobs`, `DescribeSchedulingPolicies`, `DescribeServiceEnvironments`, `DescribeServiceJob`
- Traits: `paginated` (4)
- Common required input members in this group: `arns`, `consumableResource`, `jobId`, `jobs`

### List

- Operations: `ListConsumableResources`, `ListJobs`, `ListJobsByConsumableResource`, `ListSchedulingPolicies`, `ListServiceJobs`, `ListTagsForResource`
- Traits: `paginated` (5)
- Common required input members in this group: `consumableResource`, `resourceArn`

### Create

- Operations: `CreateComputeEnvironment`, `CreateConsumableResource`, `CreateJobQueue`, `CreateSchedulingPolicy`, `CreateServiceEnvironment`
- Common required input members in this group: `capacityLimits`, `computeEnvironmentName`, `consumableResourceName`, `jobQueueName`, `name`, `priority`, `serviceEnvironmentName`, `serviceEnvironmentType`, `type`

### Delete

- Operations: `DeleteComputeEnvironment`, `DeleteConsumableResource`, `DeleteJobQueue`, `DeleteSchedulingPolicy`, `DeleteServiceEnvironment`
- Common required input members in this group: `arn`, `computeEnvironment`, `consumableResource`, `jobQueue`, `serviceEnvironment`

### Update

- Operations: `UpdateComputeEnvironment`, `UpdateConsumableResource`, `UpdateJobQueue`, `UpdateSchedulingPolicy`, `UpdateServiceEnvironment`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `arn`, `computeEnvironment`, `consumableResource`, `jobQueue`, `serviceEnvironment`

### Submit

- Operations: `SubmitJob`, `SubmitServiceJob`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `jobDefinition`, `jobName`, `jobQueue`, `serviceJobType`, `serviceRequestPayload`

### Terminate

- Operations: `TerminateJob`, `TerminateServiceJob`
- Common required input members in this group: `jobId`, `reason`

### Cancel

- Operations: `CancelJob`
- Common required input members in this group: `jobId`, `reason`

### Deregister

- Operations: `DeregisterJobDefinition`
- Common required input members in this group: `jobDefinition`

### Get

- Operations: `GetJobQueueSnapshot`
- Common required input members in this group: `jobQueue`

### Register

- Operations: `RegisterJobDefinition`
- Common required input members in this group: `jobDefinitionName`, `type`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelJob` | `POST /v1/canceljob` | - | `jobId`, `reason` | - | `CancelJobResponse` | `ClientException`, `ServerException` | Cancels a job in an Batch job queue. Jobs that are in a `SUBMITTED`, `PENDING`, or `RUNNABLE` state are cancelled and the job status is updated to `FAILED`. |
| `CreateComputeEnvironment` | `POST /v1/createcomputeenvironment` | - | `computeEnvironmentName`, `type` | - | `CreateComputeEnvironmentResponse` | `ClientException`, `ServerException` | Creates an Batch compute environment. You can create `MANAGED` or `UNMANAGED` compute environments. |
| `CreateConsumableResource` | `POST /v1/createconsumableresource` | - | `consumableResourceName` | - | `CreateConsumableResourceResponse` | `ClientException`, `ServerException` | Creates an Batch consumable resource. |
| `CreateJobQueue` | `POST /v1/createjobqueue` | - | `jobQueueName`, `priority` | - | `CreateJobQueueResponse` | `ClientException`, `ServerException` | Creates an Batch job queue. When you create a job queue, you associate one or more compute environments to the queue and assign an order of preference for the compute environments. |
| `CreateSchedulingPolicy` | `POST /v1/createschedulingpolicy` | - | `name` | - | `CreateSchedulingPolicyResponse` | `ClientException`, `ServerException` | Creates an Batch scheduling policy. |
| `CreateServiceEnvironment` | `POST /v1/createserviceenvironment` | - | `capacityLimits`, `serviceEnvironmentName`, `serviceEnvironmentType` | - | `CreateServiceEnvironmentResponse` | `ClientException`, `ServerException` | Creates a service environment for running service jobs. Service environments define capacity limits for specific service types such as SageMaker Training jobs. |
| `DeleteComputeEnvironment` | `POST /v1/deletecomputeenvironment` | - | `computeEnvironment` | - | `DeleteComputeEnvironmentResponse` | `ClientException`, `ServerException` | Deletes an Batch compute environment. Before you can delete a compute environment, you must set its state to `DISABLED` with the UpdateComputeEnvironment API operation and disassociate it from any job queues with the UpdateJobQueue API operation. |
| `DeleteConsumableResource` | `POST /v1/deleteconsumableresource` | - | `consumableResource` | - | `DeleteConsumableResourceResponse` | `ClientException`, `ServerException` | Deletes the specified consumable resource. |
| `DeleteJobQueue` | `POST /v1/deletejobqueue` | - | `jobQueue` | - | `DeleteJobQueueResponse` | `ClientException`, `ServerException` | Deletes the specified job queue. You must first disable submissions for a queue with the UpdateJobQueue operation. |
| `DeleteSchedulingPolicy` | `POST /v1/deleteschedulingpolicy` | - | `arn` | - | `DeleteSchedulingPolicyResponse` | `ClientException`, `ServerException` | Deletes the specified scheduling policy. You can't delete a scheduling policy that's used in any job queues. |
| `DeleteServiceEnvironment` | `POST /v1/deleteserviceenvironment` | - | `serviceEnvironment` | - | `DeleteServiceEnvironmentResponse` | `ClientException`, `ServerException` | Deletes a Service environment. Before you can delete a service environment, you must first set its state to `DISABLED` with the `UpdateServiceEnvironment` API operation and disassociate it from any job queues with the `UpdateJobQueue` API operation. |
| `DeregisterJobDefinition` | `POST /v1/deregisterjobdefinition` | - | `jobDefinition` | - | `DeregisterJobDefinitionResponse` | `ClientException`, `ServerException` | Deregisters an Batch job definition. Job definitions are permanently deleted after 180 days. |
| `DescribeComputeEnvironments` | `POST /v1/describecomputeenvironments` | `paginated` | - | - | `DescribeComputeEnvironmentsResponse` | `ClientException`, `ServerException` | Describes one or more of your compute environments. If you're using an unmanaged compute environment, you can use the `DescribeComputeEnvironment` operation to determine the `ecsClusterArn` that you launch your Amazon ECS container instances into. |
| `DescribeConsumableResource` | `POST /v1/describeconsumableresource` | - | `consumableResource` | - | `DescribeConsumableResourceResponse` | `ClientException`, `ServerException` | Returns a description of the specified consumable resource. |
| `DescribeJobDefinitions` | `POST /v1/describejobdefinitions` | `paginated` | - | - | `DescribeJobDefinitionsResponse` | `ClientException`, `ServerException` | Describes a list of job definitions. You can specify a `status` (such as `ACTIVE`) to only return job definitions that match that status. |
| `DescribeJobQueues` | `POST /v1/describejobqueues` | `paginated` | - | - | `DescribeJobQueuesResponse` | `ClientException`, `ServerException` | Describes one or more of your job queues. |
| `DescribeJobs` | `POST /v1/describejobs` | - | `jobs` | - | `DescribeJobsResponse` | `ClientException`, `ServerException` | Describes a list of Batch jobs. |
| `DescribeSchedulingPolicies` | `POST /v1/describeschedulingpolicies` | - | `arns` | - | `DescribeSchedulingPoliciesResponse` | `ClientException`, `ServerException` | Describes one or more of your scheduling policies. |
| `DescribeServiceEnvironments` | `POST /v1/describeserviceenvironments` | `paginated` | - | - | `DescribeServiceEnvironmentsResponse` | `ClientException`, `ServerException` | Describes one or more of your service environments. |
| `DescribeServiceJob` | `POST /v1/describeservicejob` | - | `jobId` | - | `DescribeServiceJobResponse` | `ClientException`, `ServerException` | The details of a service job. |
| `GetJobQueueSnapshot` | `POST /v1/getjobqueuesnapshot` | - | `jobQueue` | - | `GetJobQueueSnapshotResponse` | `ClientException`, `ServerException` | Provides a list of the first 100 `RUNNABLE` jobs associated to a single job queue and includes capacity utilization, including total usage and breakdown by share for fairshare scheduling job queues. |
| `ListConsumableResources` | `POST /v1/listconsumableresources` | `paginated` | - | - | `ListConsumableResourcesResponse` | `ClientException`, `ServerException` | Returns a list of Batch consumable resources. |
| `ListJobs` | `POST /v1/listjobs` | `paginated` | - | - | `ListJobsResponse` | `ClientException`, `ServerException` | Returns a list of Batch jobs. You must specify only one of the following items: A job queue ID to return a list of jobs in that job queue A multi-node parallel job ID to return a list of nodes for that job An array job ID to return a list of the children for... |
| `ListJobsByConsumableResource` | `POST /v1/listjobsbyconsumableresource` | `paginated` | `consumableResource` | - | `ListJobsByConsumableResourceResponse` | `ClientException`, `ServerException` | Returns a list of Batch jobs that require a specific consumable resource. |
| `ListSchedulingPolicies` | `POST /v1/listschedulingpolicies` | `paginated` | - | - | `ListSchedulingPoliciesResponse` | `ClientException`, `ServerException` | Returns a list of Batch scheduling policies. |
| `ListServiceJobs` | `POST /v1/listservicejobs` | `paginated` | - | - | `ListServiceJobsResponse` | `ClientException`, `ServerException` | Returns a list of service jobs for a specified job queue. |
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `ClientException`, `ServerException` | Lists the tags for an Batch resource. Batch resources that support tags are compute environments, jobs, job definitions, job queues, and scheduling policies. |
| `RegisterJobDefinition` | `POST /v1/registerjobdefinition` | - | `jobDefinitionName`, `type` | - | `RegisterJobDefinitionResponse` | `ClientException`, `ServerException` | Registers an Batch job definition. |
| `SubmitJob` | `POST /v1/submitjob` | - | `jobDefinition`, `jobName`, `jobQueue` | - | `SubmitJobResponse` | `ClientException`, `ServerException` | Submits an Batch job from a job definition. Parameters that are specified during SubmitJob override parameters defined in the job definition. |
| `SubmitServiceJob` | `POST /v1/submitservicejob` | `idempotency-token` | `jobName`, `jobQueue`, `serviceJobType`, `serviceRequestPayload` | `clientToken` | `SubmitServiceJobResponse` | `ClientException`, `ServerException` | Submits a service job to a specified job queue to run on SageMaker AI. A service job is a unit of work that you submit to Batch for execution on SageMaker AI. |
| `TagResource` | `POST /v1/tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ClientException`, `ServerException` | Associates the specified tags to a resource with the specified `resourceArn`. If existing tags on a resource aren't specified in the request parameters, they aren't changed. |
| `TerminateJob` | `POST /v1/terminatejob` | - | `jobId`, `reason` | - | `TerminateJobResponse` | `ClientException`, `ServerException` | Terminates a job in a job queue. Jobs that are in the `STARTING` or `RUNNING` state are terminated, which causes them to transition to `FAILED`. |
| `TerminateServiceJob` | `POST /v1/terminateservicejob` | - | `jobId`, `reason` | - | `TerminateServiceJobResponse` | `ClientException`, `ServerException` | Terminates a service job in a job queue. |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ClientException`, `ServerException` | Deletes specified tags from an Batch resource. |
| `UpdateComputeEnvironment` | `POST /v1/updatecomputeenvironment` | - | `computeEnvironment` | - | `UpdateComputeEnvironmentResponse` | `ClientException`, `ServerException` | Updates an Batch compute environment. |
| `UpdateConsumableResource` | `POST /v1/updateconsumableresource` | `idempotency-token` | `consumableResource` | `clientToken` | `UpdateConsumableResourceResponse` | `ClientException`, `ServerException` | Updates a consumable resource. |
| `UpdateJobQueue` | `POST /v1/updatejobqueue` | - | `jobQueue` | - | `UpdateJobQueueResponse` | `ClientException`, `ServerException` | Updates a job queue. |
| `UpdateSchedulingPolicy` | `POST /v1/updateschedulingpolicy` | - | `arn` | - | `UpdateSchedulingPolicyResponse` | `ClientException`, `ServerException` | Updates a scheduling policy. |
| `UpdateServiceEnvironment` | `POST /v1/updateserviceenvironment` | - | `serviceEnvironment` | - | `UpdateServiceEnvironmentResponse` | `ClientException`, `ServerException` | Updates a service environment. You can update the state of a service environment from `ENABLED` to `DISABLED` to prevent new service jobs from being placed in the service environment. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClientException` | `structure` | `message` | These errors are usually caused by a client action. |
| `ServerException` | `structure` | `message` | These errors are usually caused by a server issue. |
| `CancelJobRequest` | `structure` | `jobId`, `reason` | Contains the parameters for `CancelJob`. |
| `CancelJobResponse` | `structure` | - | - |
| `CreateComputeEnvironmentRequest` | `structure` | `computeEnvironmentName`, `computeResources`, `context`, `eksConfiguration`, `serviceRole`, `state`, `tags`, `type`, `unmanagedvCpus` | Contains the parameters for `CreateComputeEnvironment`. |
| `CreateComputeEnvironmentResponse` | `structure` | `computeEnvironmentArn`, `computeEnvironmentName` | - |
| `CreateConsumableResourceRequest` | `structure` | `consumableResourceName`, `resourceType`, `tags`, `totalQuantity` | - |
| `CreateConsumableResourceResponse` | `structure` | `consumableResourceArn`, `consumableResourceName` | - |
| `CreateJobQueueRequest` | `structure` | `computeEnvironmentOrder`, `jobQueueName`, `jobQueueType`, `jobStateTimeLimitActions`, `priority`, `schedulingPolicyArn`, `serviceEnvironmentOrder`, `state`, `tags` | Contains the parameters for `CreateJobQueue`. |
| `CreateJobQueueResponse` | `structure` | `jobQueueArn`, `jobQueueName` | - |
| `CreateSchedulingPolicyRequest` | `structure` | `fairsharePolicy`, `name`, `tags` | Contains the parameters for `CreateSchedulingPolicy`. |
| `CreateSchedulingPolicyResponse` | `structure` | `arn`, `name` | - |
| `CreateServiceEnvironmentRequest` | `structure` | `capacityLimits`, `serviceEnvironmentName`, `serviceEnvironmentType`, `state`, `tags` | - |
| `CreateServiceEnvironmentResponse` | `structure` | `serviceEnvironmentArn`, `serviceEnvironmentName` | - |
| `DeleteComputeEnvironmentRequest` | `structure` | `computeEnvironment` | Contains the parameters for `DeleteComputeEnvironment`. |
| `DeleteComputeEnvironmentResponse` | `structure` | - | - |
| `DeleteConsumableResourceRequest` | `structure` | `consumableResource` | - |
| `DeleteConsumableResourceResponse` | `structure` | - | - |
| `DeleteJobQueueRequest` | `structure` | `jobQueue` | Contains the parameters for `DeleteJobQueue`. |
| `DeleteJobQueueResponse` | `structure` | - | - |
| `DeleteSchedulingPolicyRequest` | `structure` | `arn` | Contains the parameters for `DeleteSchedulingPolicy`. |
| `DeleteSchedulingPolicyResponse` | `structure` | - | - |
| `DeleteServiceEnvironmentRequest` | `structure` | `serviceEnvironment` | - |
| `DeleteServiceEnvironmentResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
