use std::collections::{HashMap, VecDeque};

use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct Transaction {
    pub resource_arn: String,
    pub secret_arn: String,
}

#[derive(Debug, Default)]
pub struct RdsDataState {
    pub result_queue: VecDeque<QueryResults>,
    pub transactions: HashMap<String, Transaction>,
}

#[derive(Debug, Error)]
pub enum RdsDataError {
    #[error("resourceArn is required")]
    ResourceArnRequired,

    #[error("secretArn is required")]
    SecretArnRequired,

    #[error("sql is required")]
    SqlRequired,

    #[error("transactionId is required")]
    TransactionIdRequired,

    #[error("Transaction {transaction_id} not found")]
    TransactionNotFound { transaction_id: String },

    #[error("dbClusterOrInstanceArn is required")]
    DbClusterOrInstanceArnRequired,

    #[error("awsSecretStoreArn is required")]
    AwsSecretStoreArnRequired,

    #[error("sqlStatements is required")]
    SqlStatementsRequired,
}

impl RdsDataState {
    /// Pre-populate expected query results. Each call to execute_statement
    /// dequeues the next result.
    pub fn enqueue_result(&mut self, result: QueryResults) {
        self.result_queue.push_back(result);
    }

    pub fn execute_statement(
        &mut self,
        resource_arn: &str,
        secret_arn: &str,
        sql: &str,
    ) -> Result<QueryResults, RdsDataError> {
        if resource_arn.is_empty() {
            return Err(RdsDataError::ResourceArnRequired);
        }
        if secret_arn.is_empty() {
            return Err(RdsDataError::SecretArnRequired);
        }
        if sql.is_empty() {
            return Err(RdsDataError::SqlRequired);
        }

        // Dequeue next result, or return empty
        Ok(self
            .result_queue
            .pop_front()
            .unwrap_or_else(|| QueryResults {
                records: vec![],
                column_metadata: vec![],
                number_of_records_updated: 0,
            }))
    }

    pub fn begin_transaction(
        &mut self,
        resource_arn: &str,
        secret_arn: &str,
    ) -> Result<String, RdsDataError> {
        if resource_arn.is_empty() {
            return Err(RdsDataError::ResourceArnRequired);
        }
        if secret_arn.is_empty() {
            return Err(RdsDataError::SecretArnRequired);
        }
        let transaction_id = Uuid::new_v4().to_string();
        self.transactions.insert(
            transaction_id.clone(),
            Transaction {
                resource_arn: resource_arn.to_string(),
                secret_arn: secret_arn.to_string(),
            },
        );
        Ok(transaction_id)
    }

    pub fn commit_transaction(
        &mut self,
        resource_arn: &str,
        secret_arn: &str,
        transaction_id: &str,
    ) -> Result<(), RdsDataError> {
        if resource_arn.is_empty() {
            return Err(RdsDataError::ResourceArnRequired);
        }
        if secret_arn.is_empty() {
            return Err(RdsDataError::SecretArnRequired);
        }
        if transaction_id.is_empty() {
            return Err(RdsDataError::TransactionIdRequired);
        }
        if self.transactions.remove(transaction_id).is_none() {
            return Err(RdsDataError::TransactionNotFound {
                transaction_id: transaction_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn rollback_transaction(
        &mut self,
        resource_arn: &str,
        secret_arn: &str,
        transaction_id: &str,
    ) -> Result<(), RdsDataError> {
        if resource_arn.is_empty() {
            return Err(RdsDataError::ResourceArnRequired);
        }
        if secret_arn.is_empty() {
            return Err(RdsDataError::SecretArnRequired);
        }
        if transaction_id.is_empty() {
            return Err(RdsDataError::TransactionIdRequired);
        }
        if self.transactions.remove(transaction_id).is_none() {
            return Err(RdsDataError::TransactionNotFound {
                transaction_id: transaction_id.to_string(),
            });
        }
        Ok(())
    }
}
