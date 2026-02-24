//! End-to-end tests that run real `terraform` commands against an in-process
//! winterbaume HTTP server.
//!
//! These tests are `#[ignore]` by default because they require:
//! - `terraform` CLI installed
//! - Network access (terraform init downloads the AWS provider on first run)
//!
//! Run all terraform E2E tests:
//!   cargo test -p winterbaume-e2e-tests --test terraform -- --ignored
//!
//! Run tests for a specific service:
//!   cargo test -p winterbaume-e2e-tests --test terraform s3 -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform sqs -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform iam -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform kms -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform secretsmanager -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform ssm -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform ecr -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform events -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform cloudwatch -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform lambda -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform stepfunctions -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform firehose -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform kinesis -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform cognitoidp -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform ecs -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform route53 -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform efs -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform acm -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform cloudfront -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform wafv2 -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform elbv2 -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform eks -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform organizations -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform appconfig -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform bedrockagent -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform codecommit -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform transfer -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform workspaces -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform networkmanager -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform config -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform autoscaling -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform vpclattice -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform apigatewayv2 -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform scheduler -- --ignored
//!   cargo test -p winterbaume-e2e-tests --test terraform multi_service -- --ignored

mod harness;

mod acm;
mod apigatewayv2;
mod appconfig;
mod appfabric;
mod appflow;
mod applicationcostprofiler;
mod autoscaling;
mod batch;
mod bedrockagent;
mod cloudfront;
mod cloudwatch;
mod codecommit;
mod cognitoidp;
mod config;
mod directconnect;
mod dynamodb;
mod ebs;
mod ec2;
mod ecr;
mod ecs;
mod efs;
mod eks;
mod elbv2;
mod events;
mod firehose;
mod guardduty;
mod iam;
mod inspector2;
mod iot;
mod keyspaces;
mod kinesis;
mod kms;
mod lambda;
mod logs;
mod medialive;
mod mediapackage;
mod mediastore;
mod multi_service;
mod neptune;
mod networkmanager;
mod opensearchserverless;
mod organizations;
mod pinpoint;
mod quicksight;
mod rds;
mod redshift;
mod route53;
mod s3;
mod scheduler;
mod secretsmanager;
mod sns;
mod sqs;
mod ssm;
mod stepfunctions;
mod transfer;
mod vpclattice;
mod wafv2;
mod workspaces;
