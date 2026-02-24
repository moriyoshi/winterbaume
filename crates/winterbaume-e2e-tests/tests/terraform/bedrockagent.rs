use crate::harness::*;

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_agent_basic() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_agent" "bedrockagent_agent_basic" {
  agent_name              = "test-bedrock-agent"
  agent_resource_role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForAgents"
  foundation_model        = "anthropic.claude-v2"
  idle_session_ttl_in_seconds = 600
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-bedrock-agent"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_agent_with_description() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_agent" "bedrockagent_agent_desc" {
  agent_name              = "described-bedrock-agent"
  agent_resource_role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForAgents"
  foundation_model        = "anthropic.claude-v2"
  description             = "A test agent with a description"
  instruction             = "You are a helpful assistant designed for E2E testing."
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("described-bedrock-agent"));
    assert!(result.state.contains("A test agent with a description"));
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_knowledge_base
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_knowledge_base_basic() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_knowledge_base" "bedrockagent_kb_basic" {
  name     = "test-knowledge-base"
  role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForKB"

  knowledge_base_configuration {
    type = "VECTOR"
    vector_knowledge_base_configuration {
      embedding_model_arn = "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1"
    }
  }

  storage_configuration {
    type = "OPENSEARCH_SERVERLESS"
    opensearch_serverless_configuration {
      collection_arn    = "arn:aws:aoss:us-east-1:123456789012:collection/test-collection"
      vector_index_name = "bedrock-knowledge-base-default-index"
      field_mapping {
        vector_field   = "bedrock-knowledge-base-default-vector"
        text_field     = "AMAZON_BEDROCK_TEXT_CHUNK"
        metadata_field = "AMAZON_BEDROCK_METADATA"
      }
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-knowledge-base"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_knowledge_base_with_description() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_knowledge_base" "bedrockagent_kb_desc" {
  name        = "described-knowledge-base"
  description = "A knowledge base for E2E testing"
  role_arn    = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForKB"

  knowledge_base_configuration {
    type = "VECTOR"
    vector_knowledge_base_configuration {
      embedding_model_arn = "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1"
    }
  }

  storage_configuration {
    type = "OPENSEARCH_SERVERLESS"
    opensearch_serverless_configuration {
      collection_arn    = "arn:aws:aoss:us-east-1:123456789012:collection/desc-collection"
      vector_index_name = "bedrock-knowledge-base-default-index"
      field_mapping {
        vector_field   = "bedrock-knowledge-base-default-vector"
        text_field     = "AMAZON_BEDROCK_TEXT_CHUNK"
        metadata_field = "AMAZON_BEDROCK_METADATA"
      }
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("described-knowledge-base"));
    assert!(result.state.contains("A knowledge base for E2E testing"));
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_data_source
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_data_source() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_knowledge_base" "bedrockagent_kb_for_ds" {
  name     = "kb-for-data-source"
  role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForKB"

  knowledge_base_configuration {
    type = "VECTOR"
    vector_knowledge_base_configuration {
      embedding_model_arn = "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1"
    }
  }

  storage_configuration {
    type = "OPENSEARCH_SERVERLESS"
    opensearch_serverless_configuration {
      collection_arn    = "arn:aws:aoss:us-east-1:123456789012:collection/ds-collection"
      vector_index_name = "bedrock-knowledge-base-default-index"
      field_mapping {
        vector_field   = "bedrock-knowledge-base-default-vector"
        text_field     = "AMAZON_BEDROCK_TEXT_CHUNK"
        metadata_field = "AMAZON_BEDROCK_METADATA"
      }
    }
  }
}

resource "aws_bedrockagent_data_source" "bedrockagent_ds" {
  name              = "test-data-source"
  knowledge_base_id = aws_bedrockagent_knowledge_base.bedrockagent_kb_for_ds.id

  data_source_configuration {
    type = "S3"
    s3_configuration {
      bucket_arn = "arn:aws:s3:::my-test-data-bucket"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-data-source"));
    assert!(result.state.contains("kb-for-data-source"));
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent_alias
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_agent_alias() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_agent" "bedrockagent_agent_for_alias" {
  agent_name              = "agent-for-alias-test"
  agent_resource_role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForAgents"
  foundation_model        = "anthropic.claude-v2"
}

resource "aws_bedrockagent_agent_alias" "bedrockagent_alias" {
  agent_alias_name = "test-alias"
  agent_id         = aws_bedrockagent_agent.bedrockagent_agent_for_alias.id
  description      = "A test agent alias"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-alias"));
    assert!(result.state.contains("agent-for-alias-test"));
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent_action_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_agent_action_group() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_agent" "bedrockagent_agent_for_ag" {
  agent_name              = "agent-for-action-group"
  agent_resource_role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForAgents"
  foundation_model        = "anthropic.claude-v2"
}

resource "aws_bedrockagent_agent_action_group" "bedrockagent_ag" {
  action_group_name          = "test-action-group"
  agent_id                   = aws_bedrockagent_agent.bedrockagent_agent_for_ag.id
  agent_version              = "DRAFT"
  action_group_executor {
    lambda = "arn:aws:lambda:us-east-1:123456789012:function:test-function"
  }
  description                = "A test action group"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("test-action-group"));
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent_knowledge_base_association
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_agent_knowledge_base_association() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_agent" "bedrockagent_agent_for_kb_assoc" {
  agent_name              = "agent-for-kb-association"
  agent_resource_role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForAgents"
  foundation_model        = "anthropic.claude-v2"
}

resource "aws_bedrockagent_knowledge_base" "bedrockagent_kb_for_assoc" {
  name     = "kb-for-association"
  role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForKB"

  knowledge_base_configuration {
    type = "VECTOR"
    vector_knowledge_base_configuration {
      embedding_model_arn = "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1"
    }
  }

  storage_configuration {
    type = "OPENSEARCH_SERVERLESS"
    opensearch_serverless_configuration {
      collection_arn    = "arn:aws:aoss:us-east-1:123456789012:collection/assoc-collection"
      vector_index_name = "bedrock-knowledge-base-default-index"
      field_mapping {
        vector_field   = "bedrock-knowledge-base-default-vector"
        text_field     = "AMAZON_BEDROCK_TEXT_CHUNK"
        metadata_field = "AMAZON_BEDROCK_METADATA"
      }
    }
  }
}

resource "aws_bedrockagent_agent_knowledge_base_association" "bedrockagent_kb_assoc" {
  agent_id             = aws_bedrockagent_agent.bedrockagent_agent_for_kb_assoc.id
  description          = "Test KB association"
  knowledge_base_id    = aws_bedrockagent_knowledge_base.bedrockagent_kb_for_assoc.id
  knowledge_base_state = "ENABLED"
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("agent-for-kb-association"));
    assert!(result.state.contains("kb-for-association"));
}

// ---------------------------------------------------------------------------
// Full stack: agent + knowledge base + data source + alias
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_bedrockagent_full_stack() {
    let result = batch_apply(
        r#"
resource "aws_bedrockagent_agent" "bedrockagent_full" {
  agent_name              = "full-stack-bedrock-agent"
  agent_resource_role_arn = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForAgents"
  foundation_model        = "anthropic.claude-v2"
  description             = "Full stack E2E test agent"
  instruction             = "You are a helpful test assistant for E2E testing."
  idle_session_ttl_in_seconds = 300
}

resource "aws_bedrockagent_agent_alias" "bedrockagent_full_alias" {
  agent_alias_name = "full-stack-alias"
  agent_id         = aws_bedrockagent_agent.bedrockagent_full.id
  description      = "Full stack test alias"
}

resource "aws_bedrockagent_knowledge_base" "bedrockagent_full_kb" {
  name        = "full-stack-knowledge-base"
  description = "Full stack E2E test KB"
  role_arn    = "arn:aws:iam::123456789012:role/AmazonBedrockExecutionRoleForKB"

  knowledge_base_configuration {
    type = "VECTOR"
    vector_knowledge_base_configuration {
      embedding_model_arn = "arn:aws:bedrock:us-east-1::foundation-model/amazon.titan-embed-text-v1"
    }
  }

  storage_configuration {
    type = "OPENSEARCH_SERVERLESS"
    opensearch_serverless_configuration {
      collection_arn    = "arn:aws:aoss:us-east-1:123456789012:collection/full-stack-collection"
      vector_index_name = "bedrock-knowledge-base-default-index"
      field_mapping {
        vector_field   = "bedrock-knowledge-base-default-vector"
        text_field     = "AMAZON_BEDROCK_TEXT_CHUNK"
        metadata_field = "AMAZON_BEDROCK_METADATA"
      }
    }
  }
}

resource "aws_bedrockagent_data_source" "bedrockagent_full_ds" {
  name              = "full-stack-data-source"
  knowledge_base_id = aws_bedrockagent_knowledge_base.bedrockagent_full_kb.id

  data_source_configuration {
    type = "S3"
    s3_configuration {
      bucket_arn = "arn:aws:s3:::full-stack-data-bucket"
    }
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("full-stack-bedrock-agent"));
    assert!(result.state.contains("full-stack-alias"));
    assert!(result.state.contains("full-stack-knowledge-base"));
    assert!(result.state.contains("full-stack-data-source"));
}
