# AWS DataSync

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

DataSync DataSync is an online data movement service that simplifies data migration and helps you quickly, easily, and securely transfer your file or object data to, from, and between Amazon Web Services storage services. This API interface reference includes documentation for using DataSync programmatically. For complete information, see the DataSync User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS DataSync resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS DataSync workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Update`, `Create`, `List`, `Delete` operation families, including `DescribeAgent`, `DescribeLocationAzureBlob`, `DescribeLocationEfs`, `DescribeLocationFsxLustre`, `UpdateAgent`, `UpdateLocationAzureBlob`.

## Service Identity and Protocol

- AWS model slug: `datasync`
- AWS SDK for Rust slug: `datasync`
- Model version: `2018-11-09`
- Model file: `vendor/api-models-aws/models/datasync/service/2018-11-09/datasync-2018-11-09.json`
- SDK ID: `DataSync`
- Endpoint prefix: `datasync`
- ARN namespace: `datasync`
- CloudFormation name: `DataSync`
- CloudTrail event source: `datasync.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (14), `Update` (14), `Create` (13), `List` (5), `Delete` (3), `Cancel` (1), `Start` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelTaskExecution`, `CreateAgent`, `CreateLocationAzureBlob`, `CreateLocationEfs`, `CreateLocationFsxLustre`, `CreateLocationFsxOntap`, `CreateLocationFsxOpenZfs`, `CreateLocationFsxWindows`, `CreateLocationHdfs`, `CreateLocationNfs`, `CreateLocationObjectStorage`, `CreateLocationS3`, `CreateLocationSmb`, `CreateTask`, `DeleteAgent`, `DeleteLocation`, `DeleteTask`, `StartTaskExecution`, `TagResource`, `UntagResource`, ... (+14).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAgent`, `DescribeLocationAzureBlob`, `DescribeLocationEfs`, `DescribeLocationFsxLustre`, `DescribeLocationFsxOntap`, `DescribeLocationFsxOpenZfs`, `DescribeLocationFsxWindows`, `DescribeLocationHdfs`, `DescribeLocationNfs`, `DescribeLocationObjectStorage`, `DescribeLocationS3`, `DescribeLocationSmb`, `DescribeTask`, `DescribeTaskExecution`, `ListAgents`, `ListLocations`, `ListTagsForResource`, `ListTaskExecutions`, `ListTasks`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelTaskExecution`, `CreateTask`, `DeleteTask`, `DescribeTask`, `DescribeTaskExecution`, `ListTaskExecutions`, `ListTasks`, `StartTaskExecution`, `UpdateTask`, `UpdateTaskExecution`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 53 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `EC2/VPC`, `ECS`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/datasync/latest/userguide/how-datasync-transfer-works.html
- https://docs.aws.amazon.com/datasync/latest/userguide/run-task.html
- https://docs.aws.amazon.com/datasync/latest/userguide/task-options.html

Research outcomes:
- DataSync transfers data between supported source and destination locations through tasks. A task identifies the two locations and transfer options.
- A task execution is one run of a task and moves through phases: queueing, launching, launched, preparing, transferring, verifying, success, cancelling, or error.
- An agent is a VM appliance used to read from or write to certain storage environments. Transfers between AWS storage services in the same account and partition can be agentless.
- Each agent can run a single task at a time. Executions using the same agent or additional executions of the same task can be queued first-in-first-out.
- During preparation, DataSync scans source and destination metadata to decide what to transfer unless configured to transfer all data.
- Basic mode prepares, transfers, and verifies sequentially. Enhanced mode prepares objects as they are found and can transfer and verify in parallel.
- DataSync transfers data and selected metadata according to options including filters, manifests, overwrite behaviour, deleted-file handling, permissions, and bandwidth limits.
- DataSync always performs integrity checks during transfer. End-of-transfer verification can compare checksums and metadata for transferred data or the full dataset, depending on configuration and task mode.
- Verification failures can report checksum failures, metadata failures, files added, or files removed.
- Open files can be transferred, but files written during transfer can produce verification inconsistency and require a rerun. Locked files that cannot be opened are skipped and logged.
- `StartTaskExecution` can override selected task options for one execution and returns a task execution ARN.

Parity implications:
- Model agents, locations, tasks, task executions, options, filters, manifests, reports, schedules, queued executions, bandwidth limits, and verification state separately.
- StartTaskExecution should create an execution record, enforce agent/task queueing, and apply per-run option overrides.
- Execution progress should move through preparation, transfer, verification, success/error/cancel states with counters and error records.

## Operation Groups

### Describe

- Operations: `DescribeAgent`, `DescribeLocationAzureBlob`, `DescribeLocationEfs`, `DescribeLocationFsxLustre`, `DescribeLocationFsxOntap`, `DescribeLocationFsxOpenZfs`, `DescribeLocationFsxWindows`, `DescribeLocationHdfs`, `DescribeLocationNfs`, `DescribeLocationObjectStorage`, `DescribeLocationS3`, `DescribeLocationSmb`, `DescribeTask`, `DescribeTaskExecution`
- Common required input members in this group: `LocationArn`

### Update

- Operations: `UpdateAgent`, `UpdateLocationAzureBlob`, `UpdateLocationEfs`, `UpdateLocationFsxLustre`, `UpdateLocationFsxOntap`, `UpdateLocationFsxOpenZfs`, `UpdateLocationFsxWindows`, `UpdateLocationHdfs`, `UpdateLocationNfs`, `UpdateLocationObjectStorage`, `UpdateLocationS3`, `UpdateLocationSmb`, `UpdateTask`, `UpdateTaskExecution`
- Common required input members in this group: `LocationArn`

### Create

- Operations: `CreateAgent`, `CreateLocationAzureBlob`, `CreateLocationEfs`, `CreateLocationFsxLustre`, `CreateLocationFsxOntap`, `CreateLocationFsxOpenZfs`, `CreateLocationFsxWindows`, `CreateLocationHdfs`, `CreateLocationNfs`, `CreateLocationObjectStorage`, `CreateLocationS3`, `CreateLocationSmb`, `CreateTask`
- Common required input members in this group: `AuthenticationType`, `FsxFilesystemArn`, `SecurityGroupArns`, `Protocol`, `AgentArns`, `Subdirectory`, `ServerHostname`

### List

- Operations: `ListAgents`, `ListLocations`, `ListTagsForResource`, `ListTaskExecutions`, `ListTasks`
- Traits: `paginated` (5)
- Common required input members in this group: -

### Delete

- Operations: `DeleteAgent`, `DeleteLocation`, `DeleteTask`
- Common required input members in this group: -

### Cancel

- Operations: `CancelTaskExecution`
- Common required input members in this group: -

### Start

- Operations: `StartTaskExecution`
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
| `CancelTaskExecution` | `-` | - | `TaskExecutionArn` | - | `CancelTaskExecutionResponse` | `InternalException`, `InvalidRequestException` | Stops an DataSync task execution that's in progress. The transfer of some files are abruptly interrupted. File contents that're transferred to the destination might be incomplete or inconsistent with the source files ... |
| `CreateAgent` | `-` | - | `ActivationKey` | - | `CreateAgentResponse` | `InternalException`, `InvalidRequestException` | Activates an DataSync agent that you deploy in your storage environment. The activation process associates the agent with your Amazon Web Services account. If you haven't deployed an agent yet, see Do I need a DataSy ... |
| `CreateLocationAzureBlob` | `-` | - | `ContainerUrl`, `AuthenticationType` | - | `CreateLocationAzureBlobResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for a Microsoft Azure Blob Storage container. DataSync can use this location as a transfer source or destination. You can make transfers with or without a DataSync agent that connects to y ... |
| `CreateLocationEfs` | `-` | - | `EfsFilesystemArn`, `Ec2Config` | - | `CreateLocationEfsResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for an Amazon EFS file system. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how DataSync accesses Amazon ... |
| `CreateLocationFsxLustre` | `-` | - | `FsxFilesystemArn`, `SecurityGroupArns` | - | `CreateLocationFsxLustreResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for an Amazon FSx for Lustre file system. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how DataSync acce ... |
| `CreateLocationFsxOntap` | `-` | - | `Protocol`, `SecurityGroupArns`, `StorageVirtualMachineArn` | - | `CreateLocationFsxOntapResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for an Amazon FSx for NetApp ONTAP file system. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how DataSyn ... |
| `CreateLocationFsxOpenZfs` | `-` | - | `FsxFilesystemArn`, `Protocol`, `SecurityGroupArns` | - | `CreateLocationFsxOpenZfsResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for an Amazon FSx for OpenZFS file system. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how DataSync acc ... |
| `CreateLocationFsxWindows` | `-` | - | `FsxFilesystemArn`, `SecurityGroupArns`, `User` | - | `CreateLocationFsxWindowsResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for an Amazon FSx for Windows File Server file system. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how ... |
| `CreateLocationHdfs` | `-` | - | `NameNodes`, `AuthenticationType`, `AgentArns` | - | `CreateLocationHdfsResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for a Hadoop Distributed File System (HDFS). DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how DataSync a ... |
| `CreateLocationNfs` | `-` | - | `Subdirectory`, `ServerHostname`, `OnPremConfig` | - | `CreateLocationNfsResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for a Network File System (NFS) file server. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how DataSync a ... |
| `CreateLocationObjectStorage` | `-` | - | `ServerHostname`, `BucketName` | - | `CreateLocationObjectStorageResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for an object storage system. DataSync can use this location as a source or destination for transferring data. You can make transfers with or without a DataSync agent . Before you begin, m ... |
| `CreateLocationS3` | `-` | - | `S3BucketArn`, `S3Config` | - | `CreateLocationS3Response` | `InternalException`, `InvalidRequestException` | Creates a transfer location for an Amazon S3 bucket. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you read the following topics: Storage class cons ... |
| `CreateLocationSmb` | `-` | - | `Subdirectory`, `ServerHostname`, `AgentArns` | - | `CreateLocationSmbResponse` | `InternalException`, `InvalidRequestException` | Creates a transfer location for a Server Message Block (SMB) file server. DataSync can use this location as a source or destination for transferring data. Before you begin, make sure that you understand how DataSync ... |
| `CreateTask` | `-` | - | `SourceLocationArn`, `DestinationLocationArn` | - | `CreateTaskResponse` | `InternalException`, `InvalidRequestException` | Configures a task , which defines where and how DataSync transfers your data. A task includes a source location, destination location, and transfer options (such as bandwidth limits, scheduling, and more). If you're ... |
| `DeleteAgent` | `-` | - | `AgentArn` | - | `DeleteAgentResponse` | `InternalException`, `InvalidRequestException` | Removes an DataSync agent resource from your Amazon Web Services account. Keep in mind that this operation (which can't be undone) doesn't remove the agent's virtual machine (VM) or Amazon EC2 instance from your stor ... |
| `DeleteLocation` | `-` | - | `LocationArn` | - | `DeleteLocationResponse` | `InternalException`, `InvalidRequestException` | Deletes a transfer location resource from DataSync. |
| `DeleteTask` | `-` | - | `TaskArn` | - | `DeleteTaskResponse` | `InternalException`, `InvalidRequestException` | Deletes a transfer task resource from DataSync. |
| `DescribeAgent` | `-` | - | `AgentArn` | - | `DescribeAgentResponse` | `InternalException`, `InvalidRequestException` | Returns information about an DataSync agent, such as its name, service endpoint type, and status. |
| `DescribeLocationAzureBlob` | `-` | - | `LocationArn` | - | `DescribeLocationAzureBlobResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for Microsoft Azure Blob Storage is configured. |
| `DescribeLocationEfs` | `-` | - | `LocationArn` | - | `DescribeLocationEfsResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for an Amazon EFS file system is configured. |
| `DescribeLocationFsxLustre` | `-` | - | `LocationArn` | - | `DescribeLocationFsxLustreResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for an Amazon FSx for Lustre file system is configured. |
| `DescribeLocationFsxOntap` | `-` | - | `LocationArn` | - | `DescribeLocationFsxOntapResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for an Amazon FSx for NetApp ONTAP file system is configured. If your location uses SMB, the DescribeLocationFsxOntap operation doesn't actually return a Passw ... |
| `DescribeLocationFsxOpenZfs` | `-` | - | `LocationArn` | - | `DescribeLocationFsxOpenZfsResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for an Amazon FSx for OpenZFS file system is configured. Response elements related to SMB aren't supported with the DescribeLocationFsxOpenZfs operation. |
| `DescribeLocationFsxWindows` | `-` | - | `LocationArn` | - | `DescribeLocationFsxWindowsResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for an Amazon FSx for Windows File Server file system is configured. |
| `DescribeLocationHdfs` | `-` | - | `LocationArn` | - | `DescribeLocationHdfsResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for a Hadoop Distributed File System (HDFS) is configured. |
| `DescribeLocationNfs` | `-` | - | `LocationArn` | - | `DescribeLocationNfsResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for a Network File System (NFS) file server is configured. |
| `DescribeLocationObjectStorage` | `-` | - | `LocationArn` | - | `DescribeLocationObjectStorageResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for an object storage system is configured. |
| `DescribeLocationS3` | `-` | - | `LocationArn` | - | `DescribeLocationS3Response` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for an S3 bucket is configured. |
| `DescribeLocationSmb` | `-` | - | `LocationArn` | - | `DescribeLocationSmbResponse` | `InternalException`, `InvalidRequestException` | Provides details about how an DataSync transfer location for a Server Message Block (SMB) file server is configured. |
| `DescribeTask` | `-` | - | `TaskArn` | - | `DescribeTaskResponse` | `InternalException`, `InvalidRequestException` | Provides information about a task , which defines where and how DataSync transfers your data. |
| `DescribeTaskExecution` | `-` | - | `TaskExecutionArn` | - | `DescribeTaskExecutionResponse` | `InternalException`, `InvalidRequestException` | Provides information about an execution of your DataSync task. You can use this operation to help monitor the progress of an ongoing data transfer or check the results of the transfer. Some DescribeTaskExecution resp ... |
| `ListAgents` | `-` | `paginated` | - | - | `ListAgentsResponse` | `InternalException`, `InvalidRequestException` | Returns a list of DataSync agents that belong to an Amazon Web Services account in the Amazon Web Services Region specified in the request. With pagination, you can reduce the number of agents returned in a response. ... |
| `ListLocations` | `-` | `paginated` | - | - | `ListLocationsResponse` | `InternalException`, `InvalidRequestException` | Returns a list of source and destination locations. If you have more locations than are returned in a response (that is, the response returns only a truncated list of your agents), the response contains a token that ... |
| `ListTagsForResource` | `-` | `paginated` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalException`, `InvalidRequestException` | Returns all the tags associated with an Amazon Web Services resource. |
| `ListTaskExecutions` | `-` | `paginated` | - | - | `ListTaskExecutionsResponse` | `InternalException`, `InvalidRequestException` | Returns a list of executions for an DataSync transfer task. |
| `ListTasks` | `-` | `paginated` | - | - | `ListTasksResponse` | `InternalException`, `InvalidRequestException` | Returns a list of the DataSync tasks you created. |
| `StartTaskExecution` | `-` | - | `TaskArn` | - | `StartTaskExecutionResponse` | `InternalException`, `InvalidRequestException` | Starts an DataSync transfer task. For each task, you can only run one task execution at a time. There are several steps to a task execution. For more information, see Task execution statuses . If you're planning to t ... |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalException`, `InvalidRequestException` | Applies a tag to an Amazon Web Services resource. Tags are key-value pairs that can help you manage, filter, and search for your resources. These include DataSync resources, such as locations, tasks, and task executions. |
| `UntagResource` | `-` | - | `ResourceArn`, `Keys` | - | `UntagResourceResponse` | `InternalException`, `InvalidRequestException` | Removes tags from an Amazon Web Services resource. |
| `UpdateAgent` | `-` | - | `AgentArn` | - | `UpdateAgentResponse` | `InternalException`, `InvalidRequestException` | Updates the name of an DataSync agent. |
| `UpdateLocationAzureBlob` | `-` | - | `LocationArn` | - | `UpdateLocationAzureBlobResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configurations of the Microsoft Azure Blob Storage transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with Azure Blob Storage . |
| `UpdateLocationEfs` | `-` | - | `LocationArn` | - | `UpdateLocationEfsResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Amazon EFS transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with Amazon EFS . |
| `UpdateLocationFsxLustre` | `-` | - | `LocationArn` | - | `UpdateLocationFsxLustreResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Amazon FSx for Lustre transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with FSx for Lustre . |
| `UpdateLocationFsxOntap` | `-` | - | `LocationArn` | - | `UpdateLocationFsxOntapResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Amazon FSx for NetApp ONTAP transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with FSx for ONTAP . |
| `UpdateLocationFsxOpenZfs` | `-` | - | `LocationArn` | - | `UpdateLocationFsxOpenZfsResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Amazon FSx for OpenZFS transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with FSx for OpenZFS . Request ... |
| `UpdateLocationFsxWindows` | `-` | - | `LocationArn` | - | `UpdateLocationFsxWindowsResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Amazon FSx for Windows File Server transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with FSx for Windo ... |
| `UpdateLocationHdfs` | `-` | - | `LocationArn` | - | `UpdateLocationHdfsResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Hadoop Distributed File System (HDFS) transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with an HDFS cl ... |
| `UpdateLocationNfs` | `-` | - | `LocationArn` | - | `UpdateLocationNfsResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Network File System (NFS) transfer location that you're using with DataSync. For more information, see Configuring transfers with an NFS file server . |
| `UpdateLocationObjectStorage` | `-` | - | `LocationArn` | - | `UpdateLocationObjectStorageResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the object storage transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with an object storage system . |
| `UpdateLocationS3` | `-` | - | `LocationArn` | - | `UpdateLocationS3Response` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Amazon S3 transfer location that you're using with DataSync. Before you begin, make sure that you read the following topics: Storage class considerations with Am ... |
| `UpdateLocationSmb` | `-` | - | `LocationArn` | - | `UpdateLocationSmbResponse` | `InternalException`, `InvalidRequestException` | Modifies the following configuration parameters of the Server Message Block (SMB) transfer location that you're using with DataSync. For more information, see Configuring DataSync transfers with an SMB file server . |
| `UpdateTask` | `-` | - | `TaskArn` | - | `UpdateTaskResponse` | `InternalException`, `InvalidRequestException` | Updates the configuration of a task , which defines where and how DataSync transfers your data. |
| `UpdateTaskExecution` | `-` | - | `TaskExecutionArn`, `Options` | - | `UpdateTaskExecutionResponse` | `InternalException`, `InvalidRequestException` | Updates the configuration of a running DataSync task execution. Currently, the only Option that you can modify with UpdateTaskExecution is BytesPerSecond , which throttles bandwidth for a running or queued task execu ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalException` | `structure` | message, errorCode | This exception is thrown when an error occurs in the DataSync service. |
| `InvalidRequestException` | `structure` | message, errorCode, datasyncErrorCode | This exception is thrown when the client submits a malformed request. |
| `CancelTaskExecutionRequest` | `structure` | TaskExecutionArn | CancelTaskExecutionRequest |
| `CancelTaskExecutionResponse` | `structure` | **empty (no members)** | - |
| `CreateAgentRequest` | `structure` | ActivationKey, AgentName, Tags, VpcEndpointId, SubnetArns, SecurityGroupArns | CreateAgentRequest |
| `CreateAgentResponse` | `structure` | AgentArn | CreateAgentResponse |
| `CreateLocationAzureBlobRequest` | `structure` | ContainerUrl, AuthenticationType, SasConfiguration, BlobType, AccessTier, Subdirectory, AgentArns, Tags, CmkSecretConfig, CustomSecretConfig | - |
| `CreateLocationAzureBlobResponse` | `structure` | LocationArn | - |
| `CreateLocationEfsRequest` | `structure` | Subdirectory, EfsFilesystemArn, Ec2Config, Tags, AccessPointArn, FileSystemAccessRoleArn, InTransitEncryption | CreateLocationEfsRequest |
| `CreateLocationEfsResponse` | `structure` | LocationArn | CreateLocationEfs |
| `CreateLocationFsxLustreRequest` | `structure` | FsxFilesystemArn, SecurityGroupArns, Subdirectory, Tags | - |
| `CreateLocationFsxLustreResponse` | `structure` | LocationArn | - |
| `CreateLocationFsxOntapRequest` | `structure` | Protocol, SecurityGroupArns, StorageVirtualMachineArn, Subdirectory, Tags | - |
| `CreateLocationFsxOntapResponse` | `structure` | LocationArn | - |
| `CreateLocationFsxOpenZfsRequest` | `structure` | FsxFilesystemArn, Protocol, SecurityGroupArns, Subdirectory, Tags | - |
| `CreateLocationFsxOpenZfsResponse` | `structure` | LocationArn | - |
| `CreateLocationFsxWindowsRequest` | `structure` | Subdirectory, FsxFilesystemArn, SecurityGroupArns, Tags, User, Domain, Password, CmkSecretConfig, CustomSecretConfig | - |
| `CreateLocationFsxWindowsResponse` | `structure` | LocationArn | - |
| `CreateLocationHdfsRequest` | `structure` | Subdirectory, NameNodes, BlockSize, ReplicationFactor, KmsKeyProviderUri, QopConfiguration, AuthenticationType, SimpleUser, KerberosPrincipal, KerberosKeytab, KerberosKrb5Conf, AgentArns, ... (+3) | - |
| `CreateLocationHdfsResponse` | `structure` | LocationArn | - |
| `CreateLocationNfsRequest` | `structure` | Subdirectory, ServerHostname, OnPremConfig, MountOptions, Tags | CreateLocationNfsRequest |
| `CreateLocationNfsResponse` | `structure` | LocationArn | CreateLocationNfsResponse |
| `CreateLocationObjectStorageRequest` | `structure` | ServerHostname, ServerPort, ServerProtocol, Subdirectory, BucketName, AccessKey, SecretKey, AgentArns, Tags, ServerCertificate, CmkSecretConfig, CustomSecretConfig | CreateLocationObjectStorageRequest |
| `CreateLocationObjectStorageResponse` | `structure` | LocationArn | CreateLocationObjectStorageResponse |
| `CreateLocationS3Request` | `structure` | Subdirectory, S3BucketArn, S3StorageClass, S3Config, AgentArns, Tags | CreateLocationS3Request |
| `CreateLocationS3Response` | `structure` | LocationArn | CreateLocationS3Response |
| `CreateLocationSmbRequest` | `structure` | Subdirectory, ServerHostname, User, Domain, Password, CmkSecretConfig, CustomSecretConfig, AgentArns, MountOptions, Tags, AuthenticationType, DnsIpAddresses, ... (+3) | CreateLocationSmbRequest |
| `CreateLocationSmbResponse` | `structure` | LocationArn | CreateLocationSmbResponse |
| `CreateTaskRequest` | `structure` | SourceLocationArn, DestinationLocationArn, CloudWatchLogGroupArn, Name, Options, Excludes, Schedule, Tags, Includes, ManifestConfig, TaskReportConfig, TaskMode | CreateTaskRequest |
| `CreateTaskResponse` | `structure` | TaskArn | CreateTaskResponse |
| `DeleteAgentRequest` | `structure` | AgentArn | DeleteAgentRequest |
| `DeleteAgentResponse` | `structure` | **empty (no members)** | - |
| `DeleteLocationRequest` | `structure` | LocationArn | DeleteLocation |
| `DeleteLocationResponse` | `structure` | **empty (no members)** | - |
| `DeleteTaskRequest` | `structure` | TaskArn | DeleteTask |
| `DeleteTaskResponse` | `structure` | **empty (no members)** | - |
| `DescribeAgentRequest` | `structure` | AgentArn | DescribeAgent |
| `DescribeAgentResponse` | `structure` | AgentArn, Name, Status, LastConnectionTime, CreationTime, EndpointType, PrivateLinkConfig, Platform | DescribeAgentResponse |
| `DescribeLocationAzureBlobRequest` | `structure` | LocationArn | - |
| `DescribeLocationAzureBlobResponse` | `structure` | LocationArn, LocationUri, AuthenticationType, BlobType, AccessTier, AgentArns, CreationTime, ManagedSecretConfig, CmkSecretConfig, CustomSecretConfig | - |
| `AgentStatus` | `enum` | ONLINE, OFFLINE | - |
| `Atime` | `enum` | NONE, BEST_EFFORT | - |
| `AzureAccessTier` | `enum` | HOT, COOL, ARCHIVE | - |
| `AzureBlobAuthenticationType` | `enum` | SAS, NONE | - |
| `AzureBlobType` | `enum` | BLOCK | - |
| `EfsInTransitEncryption` | `enum` | NONE, TLS1_2 | - |
| `EndpointType` | `enum` | PUBLIC, PRIVATE_LINK, FIPS, FIPS_PRIVATE_LINK | - |
| `FilterType` | `enum` | SIMPLE_PATTERN | - |
| `Gid` | `enum` | NONE, INT_VALUE, NAME, BOTH | - |
| `HdfsAuthenticationType` | `enum` | SIMPLE, KERBEROS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
