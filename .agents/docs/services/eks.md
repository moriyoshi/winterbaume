# Amazon Elastic Kubernetes Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elastic Kubernetes Service (Amazon EKS) is a managed service that makes it easy for you to run Kubernetes on Amazon Web Services without needing to setup or maintain your own Kubernetes control plane. Kubernetes is an open-source system for automating the deployment, scaling, and management of containerized applications. Amazon EKS runs up-to-date versions of the open-source Kubernetes software, so you can use all the existing plugins and tooling from the Kubernetes community. Applications running on Amazon EKS are fully compatible with applications running on any standard Kubernetes environment, whether running in on-premises data centers or public clouds. This means that you can easily migrate any standard Kubernetes application to Amazon EKS without any code modification required.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Elastic Kubernetes Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Elastic Kubernetes Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Elastic Kubernetes Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Elastic Kubernetes Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Update`, `Create`, `Delete` operation families, including `DescribeAccessEntry`, `DescribeAddon`, `DescribeAddonConfiguration`, `DescribeAddonVersions`, `ListAccessEntries`, `ListAccessPolicies`.

## Service Identity and Protocol

- AWS model slug: `eks`
- AWS SDK for Rust slug: `eks`
- Model version: `2017-11-01`
- Model file: `vendor/api-models-aws/models/eks/service/2017-11-01/eks-2017-11-01.json`
- SDK ID: `EKS`
- Endpoint prefix: `eks`
- ARN namespace: `eks`
- CloudFormation name: `EKS`
- CloudTrail event source: `eks.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (15), `List` (14), `Update` (9), `Create` (8), `Delete` (8), `Associate` (3), `Disassociate` (2), `Deregister` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAccessPolicy`, `AssociateEncryptionConfig`, `AssociateIdentityProviderConfig`, `CreateAccessEntry`, `CreateAddon`, `CreateCapability`, `CreateCluster`, `CreateEksAnywhereSubscription`, `CreateFargateProfile`, `CreateNodegroup`, `CreatePodIdentityAssociation`, `DeleteAccessEntry`, `DeleteAddon`, `DeleteCapability`, `DeleteCluster`, `DeleteEksAnywhereSubscription`, `DeleteFargateProfile`, `DeleteNodegroup`, `DeletePodIdentityAssociation`, `DeregisterCluster`, ... (+15).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccessEntry`, `DescribeAddon`, `DescribeAddonConfiguration`, `DescribeAddonVersions`, `DescribeCapability`, `DescribeCluster`, `DescribeClusterVersions`, `DescribeEksAnywhereSubscription`, `DescribeFargateProfile`, `DescribeIdentityProviderConfig`, `DescribeInsight`, `DescribeInsightsRefresh`, `DescribeNodegroup`, `DescribePodIdentityAssociation`, `DescribeUpdate`, `ListAccessEntries`, `ListAccessPolicies`, `ListAddons`, `ListAssociatedAccessPolicies`, `ListCapabilities`, ... (+9).
- Pagination is modelled for 15 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 21 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartInsightsRefresh`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 64 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `EC2/VPC`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/eks/latest/userguide/eks-architecture.html
- https://docs.aws.amazon.com/eks/latest/userguide/update-cluster.html
- https://docs.aws.amazon.com/eks/latest/userguide/cluster-endpoint.html
- https://docs.aws.amazon.com/eks/latest/userguide/managed-node-groups.html

Research outcomes:
- Each EKS cluster has a separate Kubernetes control plane with API server and etcd components distributed across Availability Zones.
- EKS compute can be provided by Auto Mode, Fargate, Karpenter, managed node groups, self-managed nodes, or hybrid nodes.
- Cluster upgrades proceed by updating the control plane to the next minor version, then nodes, components, add-ons, and clients. EKS does not support Kubernetes version downgrade.
- Cluster update APIs return update records with statuses such as InProgress and Successful, rather than being simple synchronous mutations.
- Endpoint access can be public, private, or both. Public access CIDRs affect only the public endpoint, while cluster security groups affect the private endpoint and kubelet API connections.
- Private endpoint access creates an EKS-managed Route 53 private hosted zone that is not visible as a normal account Route 53 resource.
- Managed node groups use EKS-managed Auto Scaling groups in the customer account and can eventually reflect external scaling changes made to those groups.
- Managed node group updates drain nodes through Kubernetes; Pod disruption budgets are respected for updates, but not for some termination and AZ rebalance paths.

Parity implications:
- Model clusters, control-plane endpoint settings, add-ons, access entries, node groups, Fargate profiles, and update records separately.
- Cluster and node group mutations should be asynchronous and expose update status and error information.
- Preserve endpoint mode validation, public CIDR handling, private DNS side effects, managed Auto Scaling group reconciliation, and node draining semantics.

## Current Network Resource Stub Semantics

EKS currently treats cluster networking values as cluster metadata.

- Cluster VPC configuration fields such as subnet IDs, security group IDs, cluster security group ID, endpoint access flags, and public access CIDRs are stored or returned with the cluster record when handled.
- Node group and Fargate profile subnet references are service-local placement fields.
- The implementation does not allocate ENIs, reconcile control-plane security groups, or enforce subnet/VPC consistency.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccessEntry`, `DescribeAddon`, `DescribeAddonConfiguration`, `DescribeAddonVersions`, `DescribeCapability`, `DescribeCluster`, `DescribeClusterVersions`, `DescribeEksAnywhereSubscription`, `DescribeFargateProfile`, `DescribeIdentityProviderConfig`, `DescribeInsight`, `DescribeInsightsRefresh`, `DescribeNodegroup`, `DescribePodIdentityAssociation`, `DescribeUpdate`
- Traits: `paginated` (2)
- Common required input members in this group: `clusterName`, `addonName`, `name`, `id`

### List

- Operations: `ListAccessEntries`, `ListAccessPolicies`, `ListAddons`, `ListAssociatedAccessPolicies`, `ListCapabilities`, `ListClusters`, `ListEksAnywhereSubscriptions`, `ListFargateProfiles`, `ListIdentityProviderConfigs`, `ListInsights`, `ListNodegroups`, `ListPodIdentityAssociations`, `ListTagsForResource`, `ListUpdates`
- Traits: `paginated` (13)
- Common required input members in this group: `clusterName`

### Update

- Operations: `UpdateAccessEntry`, `UpdateAddon`, `UpdateCapability`, `UpdateClusterConfig`, `UpdateClusterVersion`, `UpdateEksAnywhereSubscription`, `UpdateNodegroupConfig`, `UpdateNodegroupVersion`, `UpdatePodIdentityAssociation`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `clusterName`, `name`, `nodegroupName`

### Create

- Operations: `CreateAccessEntry`, `CreateAddon`, `CreateCapability`, `CreateCluster`, `CreateEksAnywhereSubscription`, `CreateFargateProfile`, `CreateNodegroup`, `CreatePodIdentityAssociation`
- Traits: `idempotency-token` (8)
- Common required input members in this group: `clusterName`, `roleArn`, `name`

### Delete

- Operations: `DeleteAccessEntry`, `DeleteAddon`, `DeleteCapability`, `DeleteCluster`, `DeleteEksAnywhereSubscription`, `DeleteFargateProfile`, `DeleteNodegroup`, `DeletePodIdentityAssociation`
- Common required input members in this group: `clusterName`

### Associate

- Operations: `AssociateAccessPolicy`, `AssociateEncryptionConfig`, `AssociateIdentityProviderConfig`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `clusterName`

### Disassociate

- Operations: `DisassociateAccessPolicy`, `DisassociateIdentityProviderConfig`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `clusterName`

### Deregister

- Operations: `DeregisterCluster`
- Common required input members in this group: -

### Register

- Operations: `RegisterCluster`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Start

- Operations: `StartInsightsRefresh`
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
| `AssociateAccessPolicy` | `POST /clusters/{clusterName}/access-entries/{principalArn}/access-policies` | - | `clusterName`, `principalArn`, `policyArn`, `accessScope` | - | `AssociateAccessPolicyResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Associates an access policy and its scope to an access entry. For more information about associating access policies, see Associating and disassociating access policies to and from access entries in the Amazon EKS Us ... |
| `AssociateEncryptionConfig` | `POST /clusters/{clusterName}/encryption-config/associate` | `idempotency-token` | `clusterName`, `encryptionConfig` | `clientRequestToken` | `AssociateEncryptionConfigResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ThrottlingException` | Associates an encryption configuration to an existing cluster. Use this API to enable encryption on existing clusters that don't already have encryption enabled. This allows you to implement a defense-in-depth securi ... |
| `AssociateIdentityProviderConfig` | `POST /clusters/{clusterName}/identity-provider-configs/associate` | `idempotency-token` | `clusterName`, `oidc` | `clientRequestToken` | `AssociateIdentityProviderConfigResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ThrottlingException` | Associates an identity provider configuration to a cluster. If you want to authenticate identities using an identity provider, you can create an identity provider configuration and associate it to your cluster. After ... |
| `CreateAccessEntry` | `POST /clusters/{clusterName}/access-entries` | `idempotency-token` | `clusterName`, `principalArn` | `clientRequestToken` | `CreateAccessEntryResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerException` | Creates an access entry. An access entry allows an IAM principal to access your cluster. Access entries can replace the need to maintain entries in the aws-auth ConfigMap for authentication. You have the following op ... |
| `CreateAddon` | `POST /clusters/{clusterName}/addons` | `idempotency-token` | `clusterName`, `addonName` | `clientRequestToken` | `CreateAddonResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException` | Creates an Amazon EKS add-on. Amazon EKS add-ons help to automate the provisioning and lifecycle management of common operational software for Amazon EKS clusters. For more information, see Amazon EKS add-ons in the ... |
| `CreateCapability` | `POST /clusters/{clusterName}/capabilities` | `idempotency-token` | `capabilityName`, `clusterName`, `type`, `roleArn`, `deletePropagationPolicy` | `clientRequestToken` | `CreateCapabilityResponse` | `AccessDeniedException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ServerException`, `ThrottlingException` | Creates a managed capability resource for an Amazon EKS cluster. Capabilities provide fully managed capabilities to build and scale with Kubernetes. When you create a capability, Amazon EKSprovisions and manages the ... |
| `CreateCluster` | `POST /clusters` | `idempotency-token` | `name`, `roleArn`, `resourcesVpcConfig` | `clientRequestToken` | `CreateClusterResponse` | `ClientException`, `InvalidParameterException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ServerException`, `ServiceUnavailableException`, `UnsupportedAvailabilityZoneException` | Creates an Amazon EKS control plane. The Amazon EKS control plane consists of control plane instances that run the Kubernetes software, such as etcd and the API server. The control plane runs in an account managed by ... |
| `CreateEksAnywhereSubscription` | `POST /eks-anywhere-subscriptions` | `idempotency-token` | `name`, `term` | `clientRequestToken` | `CreateEksAnywhereSubscriptionResponse` | `ClientException`, `InvalidParameterException`, `ResourceLimitExceededException`, `ServerException`, `ServiceUnavailableException` | Creates an EKS Anywhere subscription. When a subscription is created, it is a contract agreement for the length of the term specified in the request. Licenses that are used to validate support are provisioned in Amaz ... |
| `CreateFargateProfile` | `POST /clusters/{clusterName}/fargate-profiles` | `idempotency-token` | `fargateProfileName`, `clusterName`, `podExecutionRoleArn` | `clientRequestToken` | `CreateFargateProfileResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceLimitExceededException`, `ServerException`, `UnsupportedAvailabilityZoneException` | Creates an Fargate profile for your Amazon EKS cluster. You must have at least one Fargate profile in a cluster to be able to run pods on Fargate. The Fargate profile allows an administrator to declare which pods run ... |
| `CreateNodegroup` | `POST /clusters/{clusterName}/node-groups` | `idempotency-token` | `clusterName`, `nodegroupName`, `subnets`, `nodeRole` | `clientRequestToken` | `CreateNodegroupResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ServerException`, `ServiceUnavailableException` | Creates a managed node group for an Amazon EKS cluster. You can only create a node group for your cluster that is equal to the current Kubernetes version for the cluster. All node groups are created with the latest A ... |
| `CreatePodIdentityAssociation` | `POST /clusters/{clusterName}/pod-identity-associations` | `idempotency-token` | `clusterName`, `namespace`, `serviceAccount`, `roleArn` | `clientRequestToken` | `CreatePodIdentityAssociationResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServerException` | Creates an EKS Pod Identity association between a service account in an Amazon EKS cluster and an IAM role with EKS Pod Identity . Use EKS Pod Identity to give temporary IAM credentials to Pods and the credentials ar ... |
| `DeleteAccessEntry` | `DELETE /clusters/{clusterName}/access-entries/{principalArn}` | - | `clusterName`, `principalArn` | - | `DeleteAccessEntryResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Deletes an access entry. Deleting an access entry of a type other than Standard can cause your cluster to function improperly. If you delete an access entry in error, you can recreate it. |
| `DeleteAddon` | `DELETE /clusters/{clusterName}/addons/{addonName}` | - | `clusterName`, `addonName` | - | `DeleteAddonResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Deletes an Amazon EKS add-on. When you remove an add-on, it's deleted from the cluster. You can always manually start an add-on on the cluster using the Kubernetes API. |
| `DeleteCapability` | `DELETE /clusters/{clusterName}/capabilities/{capabilityName}` | - | `clusterName`, `capabilityName` | - | `DeleteCapabilityResponse` | `AccessDeniedException`, `InvalidParameterException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException` | Deletes a managed capability from your Amazon EKS cluster. When you delete a capability, Amazon EKS removes the capability infrastructure but retains all resources that were managed by the capability. Before deleting ... |
| `DeleteCluster` | `DELETE /clusters/{name}` | - | `name` | - | `DeleteClusterResponse` | `ClientException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Deletes an Amazon EKS cluster control plane. If you have active services and ingress resources in your cluster that are associated with a load balancer, you must delete those services before deleting the cluster so t ... |
| `DeleteEksAnywhereSubscription` | `DELETE /eks-anywhere-subscriptions/{id}` | - | `id` | - | `DeleteEksAnywhereSubscriptionResponse` | `ClientException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Deletes an expired or inactive subscription. Deleting inactive subscriptions removes them from the Amazon Web Services Management Console view and from list/describe API responses. Subscriptions can only be cancelled ... |
| `DeleteFargateProfile` | `DELETE /clusters/{clusterName}/fargate-profiles/{fargateProfileName}` | - | `clusterName`, `fargateProfileName` | - | `DeleteFargateProfileResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Deletes an Fargate profile. When you delete a Fargate profile, any Pod running on Fargate that was created with the profile is deleted. If the Pod matches another Fargate profile, then it is scheduled on Fargate with ... |
| `DeleteNodegroup` | `DELETE /clusters/{clusterName}/node-groups/{nodegroupName}` | - | `clusterName`, `nodegroupName` | - | `DeleteNodegroupResponse` | `ClientException`, `InvalidParameterException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Deletes a managed node group. |
| `DeletePodIdentityAssociation` | `DELETE /clusters/{clusterName}/pod-identity-associations/{associationId}` | - | `clusterName`, `associationId` | - | `DeletePodIdentityAssociationResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Deletes a EKS Pod Identity association. The temporary Amazon Web Services credentials from the previous IAM role session might still be valid until the session expiry. If you need to immediately revoke the temporary ... |
| `DeregisterCluster` | `DELETE /cluster-registrations/{name}` | - | `name` | - | `DeregisterClusterResponse` | `AccessDeniedException`, `ClientException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Deregisters a connected cluster to remove it from the Amazon EKS control plane. A connected cluster is a Kubernetes cluster that you've connected to your control plane using the Amazon EKS Connector . |
| `DescribeAccessEntry` | `GET /clusters/{clusterName}/access-entries/{principalArn}` | - | `clusterName`, `principalArn` | - | `DescribeAccessEntryResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Describes an access entry. |
| `DescribeAddon` | `GET /clusters/{clusterName}/addons/{addonName}` | - | `clusterName`, `addonName` | - | `DescribeAddonResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Describes an Amazon EKS add-on. |
| `DescribeAddonConfiguration` | `GET /addons/configuration-schemas` | - | `addonName`, `addonVersion` | - | `DescribeAddonConfigurationResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Returns configuration options. |
| `DescribeAddonVersions` | `GET /addons/supported-versions` | `paginated` | - | - | `DescribeAddonVersionsResponse` | `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Describes the versions for an add-on. Information such as the Kubernetes versions that you can use the add-on with, the owner , publisher , and the type of the add-on are returned. |
| `DescribeCapability` | `GET /clusters/{clusterName}/capabilities/{capabilityName}` | - | `clusterName`, `capabilityName` | - | `DescribeCapabilityResponse` | `AccessDeniedException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Returns detailed information about a specific managed capability in your Amazon EKS cluster, including its current status, configuration, health information, and any issues that may be affecting its operation. |
| `DescribeCluster` | `GET /clusters/{name}` | - | `name` | - | `DescribeClusterResponse` | `ClientException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Describes an Amazon EKS cluster. The API server endpoint and certificate authority data returned by this operation are required for kubelet and kubectl to communicate with your Kubernetes API server. For more informa ... |
| `DescribeClusterVersions` | `GET /cluster-versions` | `paginated` | - | - | `DescribeClusterVersionsResponse` | `InvalidParameterException`, `InvalidRequestException`, `ServerException` | Lists available Kubernetes versions for Amazon EKS clusters. |
| `DescribeEksAnywhereSubscription` | `GET /eks-anywhere-subscriptions/{id}` | - | `id` | - | `DescribeEksAnywhereSubscriptionResponse` | `ClientException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Returns descriptive information about a subscription. |
| `DescribeFargateProfile` | `GET /clusters/{clusterName}/fargate-profiles/{fargateProfileName}` | - | `clusterName`, `fargateProfileName` | - | `DescribeFargateProfileResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Describes an Fargate profile. |
| `DescribeIdentityProviderConfig` | `POST /clusters/{clusterName}/identity-provider-configs/describe` | - | `clusterName`, `identityProviderConfig` | - | `DescribeIdentityProviderConfigResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Describes an identity provider configuration. |
| `DescribeInsight` | `GET /clusters/{clusterName}/insights/{id}` | - | `clusterName`, `id` | - | `DescribeInsightResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Returns details about an insight that you specify using its ID. |
| `DescribeInsightsRefresh` | `GET /clusters/{clusterName}/insights-refresh` | - | `clusterName` | - | `DescribeInsightsRefreshResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Returns the status of the latest on-demand cluster insights refresh operation. |
| `DescribeNodegroup` | `GET /clusters/{clusterName}/node-groups/{nodegroupName}` | - | `clusterName`, `nodegroupName` | - | `DescribeNodegroupResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Describes a managed node group. |
| `DescribePodIdentityAssociation` | `GET /clusters/{clusterName}/pod-identity-associations/{associationId}` | - | `clusterName`, `associationId` | - | `DescribePodIdentityAssociationResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Returns descriptive information about an EKS Pod Identity association. This action requires the ID of the association. You can get the ID from the response to the CreatePodIdentityAssocation for newly created associa ... |
| `DescribeUpdate` | `GET /clusters/{name}/updates/{updateId}` | - | `name`, `updateId` | - | `DescribeUpdateResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Describes an update to an Amazon EKS resource. When the status of the update is Successful , the update is complete. If an update fails, the status is Failed , and an error detail explains the reason for the failure. |
| `DisassociateAccessPolicy` | `DELETE /clusters/{clusterName}/access-entries/{principalArn}/access-policies/{policyArn}` | - | `clusterName`, `principalArn`, `policyArn` | - | `DisassociateAccessPolicyResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Disassociates an access policy from an access entry. |
| `DisassociateIdentityProviderConfig` | `POST /clusters/{clusterName}/identity-provider-configs/disassociate` | `idempotency-token` | `clusterName`, `identityProviderConfig` | `clientRequestToken` | `DisassociateIdentityProviderConfigResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ThrottlingException` | Disassociates an identity provider configuration from a cluster. If you disassociate an identity provider from your cluster, users included in the provider can no longer access the cluster. However, you can still acc ... |
| `ListAccessEntries` | `GET /clusters/{clusterName}/access-entries` | `paginated` | `clusterName` | - | `ListAccessEntriesResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Lists the access entries for your cluster. |
| `ListAccessPolicies` | `GET /access-policies` | `paginated` | - | - | `ListAccessPoliciesResponse` | `ServerException` | Lists the available access policies. |
| `ListAddons` | `GET /clusters/{clusterName}/addons` | `paginated` | `clusterName` | - | `ListAddonsResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Lists the installed add-ons. |
| `ListAssociatedAccessPolicies` | `GET /clusters/{clusterName}/access-entries/{principalArn}/access-policies` | `paginated` | `clusterName`, `principalArn` | - | `ListAssociatedAccessPoliciesResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Lists the access policies associated with an access entry. |
| `ListCapabilities` | `GET /clusters/{clusterName}/capabilities` | `paginated` | `clusterName` | - | `ListCapabilitiesResponse` | `InvalidParameterException`, `ServerException` | Lists all managed capabilities in your Amazon EKS cluster. You can use this operation to get an overview of all capabilities and their current status. |
| `ListClusters` | `GET /clusters` | `paginated` | - | - | `ListClustersResponse` | `ClientException`, `InvalidParameterException`, `ServerException`, `ServiceUnavailableException` | Lists the Amazon EKS clusters in your Amazon Web Services account in the specified Amazon Web Services Region. |
| `ListEksAnywhereSubscriptions` | `GET /eks-anywhere-subscriptions` | `paginated` | - | - | `ListEksAnywhereSubscriptionsResponse` | `ClientException`, `InvalidParameterException`, `ServerException`, `ServiceUnavailableException` | Displays the full description of the subscription. |
| `ListFargateProfiles` | `GET /clusters/{clusterName}/fargate-profiles` | `paginated` | `clusterName` | - | `ListFargateProfilesResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Lists the Fargate profiles associated with the specified cluster in your Amazon Web Services account in the specified Amazon Web Services Region. |
| `ListIdentityProviderConfigs` | `GET /clusters/{clusterName}/identity-provider-configs` | `paginated` | `clusterName` | - | `ListIdentityProviderConfigsResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Lists the identity provider configurations for your cluster. |
| `ListInsights` | `POST /clusters/{clusterName}/insights` | `paginated` | `clusterName` | - | `ListInsightsResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Returns a list of all insights checked for against the specified cluster. You can filter which insights are returned by category, associated Kubernetes version, and status. The default filter lists all categories and ... |
| `ListNodegroups` | `GET /clusters/{clusterName}/node-groups` | `paginated` | `clusterName` | - | `ListNodegroupsResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException`, `ServiceUnavailableException` | Lists the managed node groups associated with the specified cluster in your Amazon Web Services account in the specified Amazon Web Services Region. Self-managed node groups aren't listed. |
| `ListPodIdentityAssociations` | `GET /clusters/{clusterName}/pod-identity-associations` | `paginated` | `clusterName` | - | `ListPodIdentityAssociationsResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | List the EKS Pod Identity associations in a cluster. You can filter the list by the namespace that the association is in or the service account that the association uses. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `NotFoundException` | List the tags for an Amazon EKS resource. |
| `ListUpdates` | `GET /clusters/{name}/updates` | `paginated` | `name` | - | `ListUpdatesResponse` | `ClientException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Lists the updates associated with an Amazon EKS resource in your Amazon Web Services account, in the specified Amazon Web Services Region. |
| `RegisterCluster` | `POST /cluster-registrations` | `idempotency-token` | `name`, `connectorConfig` | `clientRequestToken` | `RegisterClusterResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ResourceInUseException`, `ResourceLimitExceededException`, `ResourcePropagationDelayException`, `ServerException`, `ServiceUnavailableException` | Connects a Kubernetes cluster to the Amazon EKS control plane. Any Kubernetes cluster can be connected to the Amazon EKS control plane to view current information about the cluster and its nodes. Cluster connection r ... |
| `StartInsightsRefresh` | `POST /clusters/{clusterName}/insights-refresh` | - | `clusterName` | - | `StartInsightsRefreshResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Initiates an on-demand refresh operation for cluster insights, getting the latest analysis outside of the standard refresh schedule. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `NotFoundException` | Associates the specified tags to an Amazon EKS resource with the specified resourceArn . If existing tags on a resource are not specified in the request parameters, they aren't changed. When a resource is deleted, th ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `NotFoundException` | Deletes specified tags from an Amazon EKS resource. |
| `UpdateAccessEntry` | `POST /clusters/{clusterName}/access-entries/{principalArn}` | `idempotency-token` | `clusterName`, `principalArn` | `clientRequestToken` | `UpdateAccessEntryResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Updates an access entry. |
| `UpdateAddon` | `POST /clusters/{clusterName}/addons/{addonName}/update` | `idempotency-token` | `clusterName`, `addonName` | `clientRequestToken` | `UpdateAddonResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException` | Updates an Amazon EKS add-on. |
| `UpdateCapability` | `POST /clusters/{clusterName}/capabilities/{capabilityName}` | `idempotency-token` | `clusterName`, `capabilityName` | `clientRequestToken` | `UpdateCapabilityResponse` | `AccessDeniedException`, `InvalidParameterException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException` | Updates the configuration of a managed capability in your Amazon EKS cluster. You can update the IAM role, configuration settings, and delete propagation policy for a capability. When you update a capability, Amazon ... |
| `UpdateClusterConfig` | `POST /clusters/{name}/update-config` | `idempotency-token` | `name` | `clientRequestToken` | `UpdateClusterConfigResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ThrottlingException` | Updates an Amazon EKS cluster configuration. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster update with DescribeUp ... |
| `UpdateClusterVersion` | `POST /clusters/{name}/updates` | `idempotency-token` | `name`, `version` | `clientRequestToken` | `UpdateClusterVersionResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `InvalidStateException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException`, `ThrottlingException` | Updates an Amazon EKS cluster to the specified Kubernetes version. Your cluster continues to function during the update. The response output includes an update ID that you can use to track the status of your cluster ... |
| `UpdateEksAnywhereSubscription` | `POST /eks-anywhere-subscriptions/{id}` | `idempotency-token` | `id`, `autoRenew` | `clientRequestToken` | `UpdateEksAnywhereSubscriptionResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Update an EKS Anywhere Subscription. Only auto renewal and tags can be updated after subscription creation. |
| `UpdateNodegroupConfig` | `POST /clusters/{clusterName}/node-groups/{nodegroupName}/update-config` | `idempotency-token` | `clusterName`, `nodegroupName` | `clientRequestToken` | `UpdateNodegroupConfigResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException` | Updates an Amazon EKS managed node group configuration. Your node group continues to function during the update. The response output includes an update ID that you can use to track the status of your node group updat ... |
| `UpdateNodegroupVersion` | `POST /clusters/{clusterName}/node-groups/{nodegroupName}/update-version` | `idempotency-token` | `clusterName`, `nodegroupName` | `clientRequestToken` | `UpdateNodegroupVersionResponse` | `ClientException`, `InvalidParameterException`, `InvalidRequestException`, `ResourceInUseException`, `ResourceNotFoundException`, `ServerException` | Updates the Kubernetes version or AMI version of an Amazon EKS managed node group. You can update a node group using a launch template only if the node group was originally deployed with a launch template. Additional ... |
| `UpdatePodIdentityAssociation` | `POST /clusters/{clusterName}/pod-identity-associations/{associationId}` | `idempotency-token` | `clusterName`, `associationId` | `clientRequestToken` | `UpdatePodIdentityAssociationResponse` | `InvalidParameterException`, `InvalidRequestException`, `ResourceNotFoundException`, `ServerException` | Updates a EKS Pod Identity association. In an update, you can change the IAM role, the target IAM role, or disableSessionTags . You must change at least one of these in an update. An association can't be moved betwee ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteAddon` | - | `preserve -> preserve` | - | - |
| `DescribeAddonConfiguration` | - | `addonName -> addonName`, `addonVersion -> addonVersion` | - | - |
| `DescribeAddonVersions` | - | `kubernetesVersion -> kubernetesVersion`, `maxResults -> maxResults`, `nextToken -> nextToken`, `addonName -> addonName`, `types -> types`, `publishers -> publishers`, `owners -> owners` | - | - |
| `DescribeClusterVersions` | - | `clusterType -> clusterType`, `maxResults -> maxResults`, `nextToken -> nextToken`, `defaultOnly -> defaultOnly`, `includeAll -> includeAll`, `clusterVersions -> clusterVersions`, `status -> status`, `versionStatus -> versionStatus` | - | - |
| `DescribeUpdate` | - | `nodegroupName -> nodegroupName`, `addonName -> addonName`, `capabilityName -> capabilityName` | - | - |
| `ListAccessEntries` | - | `associatedPolicyArn -> associatedPolicyArn`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListAccessPolicies` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListAddons` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListAssociatedAccessPolicies` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListCapabilities` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListClusters` | - | `maxResults -> maxResults`, `nextToken -> nextToken`, `include -> include` | - | - |
| `ListEksAnywhereSubscriptions` | - | `maxResults -> maxResults`, `nextToken -> nextToken`, `includeStatus -> includeStatus` | - | - |
| `ListFargateProfiles` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListIdentityProviderConfigs` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListNodegroups` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListPodIdentityAssociations` | - | `namespace -> namespace`, `serviceAccount -> serviceAccount`, `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListUpdates` | - | `nodegroupName -> nodegroupName`, `addonName -> addonName`, `capabilityName -> capabilityName`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have permissions to perform the requested operation. The IAM principal making the request must have at least one IAM permissions policy attached t ... |
| `BadRequestException` | `structure` | message | This exception is thrown if the request contains a semantic error. The precise meaning will depend on the API, and will be documented in the error message. |
| `ClientException` | `structure` | clusterName, nodegroupName, addonName, subscriptionId, message | These errors are usually caused by a client action. Actions can include using an action or resource on behalf of an IAM principal that doesn't have permissi ... |
| `InvalidParameterException` | `structure` | clusterName, nodegroupName, fargateProfileName, addonName, subscriptionId, message | The specified parameter is invalid. Review the available parameters for the API request. |
| `InvalidRequestException` | `structure` | clusterName, nodegroupName, addonName, subscriptionId, message | The request is invalid given the state of the cluster. Check the state of the cluster and the associated operations. |
| `InvalidStateException` | `structure` | clusterName, message | Amazon EKS detected upgrade readiness issues. Call the ListInsights API to view detected upgrade blocking issues. Pass the force flag when updating to overr ... |
| `NotFoundException` | `structure` | message | A service resource associated with the request could not be found. Clients should not retry such requests. |
| `ResourceInUseException` | `structure` | clusterName, nodegroupName, addonName, message | The specified resource is in use. |
| `ResourceLimitExceededException` | `structure` | clusterName, nodegroupName, subscriptionId, message | You have encountered a service limit on the specified resource. |
| `ResourceNotFoundException` | `structure` | clusterName, nodegroupName, fargateProfileName, addonName, subscriptionId, message | The specified resource could not be found. You can view your available clusters with ListClusters . You can view your available managed node groups with Lis ... |
| `ResourcePropagationDelayException` | `structure` | message | Required resources (such as service-linked roles) were created and are still propagating. Retry later. |
| `ServerException` | `structure` | clusterName, nodegroupName, addonName, subscriptionId, message | These errors are usually caused by a server-side issue. |
| `ServiceUnavailableException` | `structure` | message | The service is unavailable. Back off and retry the operation. |
| `ThrottlingException` | `structure` | clusterName, message | The request or operation couldn't be performed because a service is throttling requests. |
| `UnsupportedAvailabilityZoneException` | `structure` | message, clusterName, nodegroupName, validZones | At least one of your specified cluster subnets is in an Availability Zone that does not support Amazon EKS. The exception output specifies the supported Ava ... |
| `AssociateAccessPolicyRequest` | `structure` | clusterName, principalArn, policyArn, accessScope | - |
| `AssociateAccessPolicyResponse` | `structure` | clusterName, principalArn, associatedAccessPolicy | - |
| `AssociateEncryptionConfigRequest` | `structure` | clusterName, encryptionConfig, clientRequestToken | - |
| `AssociateEncryptionConfigResponse` | `structure` | update | - |
| `AssociateIdentityProviderConfigRequest` | `structure` | clusterName, oidc, tags, clientRequestToken | - |
| `AssociateIdentityProviderConfigResponse` | `structure` | update, tags | - |
| `CreateAccessEntryRequest` | `structure` | clusterName, principalArn, kubernetesGroups, tags, clientRequestToken, username, type | - |
| `CreateAccessEntryResponse` | `structure` | accessEntry | - |
| `CreateAddonRequest` | `structure` | clusterName, addonName, addonVersion, serviceAccountRoleArn, resolveConflicts, clientRequestToken, tags, configurationValues, podIdentityAssociations, namespaceConfig | - |
| `CreateAddonResponse` | `structure` | addon | - |
| `CreateCapabilityRequest` | `structure` | capabilityName, clusterName, clientRequestToken, type, roleArn, configuration, tags, deletePropagationPolicy | - |
| `CreateCapabilityResponse` | `structure` | capability | - |
| `CreateClusterRequest` | `structure` | name, version, roleArn, resourcesVpcConfig, kubernetesNetworkConfig, logging, clientRequestToken, tags, encryptionConfig, outpostConfig, accessConfig, bootstrapSelfManagedAddons, ... (+7) | - |
| `CreateClusterResponse` | `structure` | cluster | - |
| `CreateEksAnywhereSubscriptionRequest` | `structure` | name, term, licenseQuantity, licenseType, autoRenew, clientRequestToken, tags | - |
| `CreateEksAnywhereSubscriptionResponse` | `structure` | subscription | - |
| `CreateFargateProfileRequest` | `structure` | fargateProfileName, clusterName, podExecutionRoleArn, subnets, selectors, clientRequestToken, tags | - |
| `CreateFargateProfileResponse` | `structure` | fargateProfile | - |
| `CreateNodegroupRequest` | `structure` | clusterName, nodegroupName, scalingConfig, diskSize, subnets, instanceTypes, amiType, remoteAccess, nodeRole, labels, taints, tags, ... (+8) | - |
| `CreateNodegroupResponse` | `structure` | nodegroup | - |
| `CreatePodIdentityAssociationRequest` | `structure` | clusterName, namespace, serviceAccount, roleArn, clientRequestToken, tags, disableSessionTags, targetRoleArn, policy | - |
| `CreatePodIdentityAssociationResponse` | `structure` | association | - |
| `DeleteAccessEntryRequest` | `structure` | clusterName, principalArn | - |
| `DeleteAccessEntryResponse` | `structure` | **empty (no members)** | - |
| `DeleteAddonRequest` | `structure` | clusterName, addonName, preserve | - |
| `AMITypes` | `enum` | AL2_x86_64, AL2_x86_64_GPU, AL2_ARM_64, CUSTOM, BOTTLEROCKET_ARM_64, BOTTLEROCKET_x86_64, BOTTLEROCKET_ARM_64_FIPS, BOTTLEROCKET_x86_64_FIPS, BOTTLEROCKET_ARM_64_NVIDIA, BOTTLEROCKET_x86_64_NVIDIA, BOTTLEROCKET_ARM_64_NVIDIA_FIPS, BOTTLEROCKET_x86_64_NVIDIA_FIPS, ... (+11) | - |
| `AccessScopeType` | `enum` | cluster, namespace | - |
| `AddonIssueCode` | `enum` | ACCESS_DENIED, INTERNAL_FAILURE, CLUSTER_UNREACHABLE, INSUFFICIENT_NUMBER_OF_REPLICAS, CONFIGURATION_CONFLICT, ADMISSION_REQUEST_DENIED, UNSUPPORTED_ADDON_MODIFICATION, K8S_RESOURCE_NOT_FOUND, ADDON_SUBSCRIPTION_NEEDED, ADDON_PERMISSION_FAILURE | - |
| `AddonStatus` | `enum` | CREATING, ACTIVE, CREATE_FAILED, UPDATING, DELETING, DELETE_FAILED, DEGRADED, UPDATE_FAILED | - |
| `ArgoCdRole` | `enum` | ADMIN, EDITOR, VIEWER | - |
| `AuthenticationMode` | `enum` | API, API_AND_CONFIG_MAP, CONFIG_MAP | - |
| `CapabilityDeletePropagationPolicy` | `enum` | RETAIN | - |
| `CapabilityIssueCode` | `enum` | ACCESS_DENIED, CLUSTER_UNREACHABLE | - |
| `CapabilityStatus` | `enum` | CREATING, CREATE_FAILED, UPDATING, DELETING, DELETE_FAILED, ACTIVE, DEGRADED | - |
| `CapabilityType` | `enum` | ACK, KRO, ARGOCD | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
