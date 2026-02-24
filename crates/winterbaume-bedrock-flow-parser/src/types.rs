/// The top-level parsed flow definition.
#[derive(Debug, Clone)]
pub struct FlowDefinition {
    pub nodes: Vec<FlowNode>,
    pub connections: Vec<FlowConnection>,
}

/// A single node in the flow graph.
#[derive(Debug, Clone)]
pub struct FlowNode {
    pub name: String,
    pub node_type: FlowNodeType,
    pub configuration: Option<FlowNodeConfiguration>,
    pub inputs: Vec<FlowNodeInput>,
    pub outputs: Vec<FlowNodeOutput>,
}

/// Flow node type — proper enum instead of the wire format's string field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowNodeType {
    Input,
    Output,
    Condition,
    KnowledgeBase,
    Prompt,
    LambdaFunction,
    Lex,
    Agent,
    Storage,
    Retrieval,
    Iterator,
    Collector,
    InlineCode,
    Loop,
    LoopInput,
    LoopController,
}

impl std::fmt::Display for FlowNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Input => "Input",
            Self::Output => "Output",
            Self::Condition => "Condition",
            Self::KnowledgeBase => "KnowledgeBase",
            Self::Prompt => "Prompt",
            Self::LambdaFunction => "LambdaFunction",
            Self::Lex => "Lex",
            Self::Agent => "Agent",
            Self::Storage => "Storage",
            Self::Retrieval => "Retrieval",
            Self::Iterator => "Iterator",
            Self::Collector => "Collector",
            Self::InlineCode => "InlineCode",
            Self::Loop => "Loop",
            Self::LoopInput => "LoopInput",
            Self::LoopController => "LoopController",
        })
    }
}

/// Node configuration — proper enum discriminated by node type.
#[derive(Debug, Clone)]
pub enum FlowNodeConfiguration {
    Input,
    Output,
    Condition(ConditionConfig),
    KnowledgeBase(KnowledgeBaseConfig),
    Prompt(PromptConfig),
    LambdaFunction(LambdaFunctionConfig),
    Lex(LexConfig),
    Agent(AgentConfig),
    Storage(StorageConfig),
    Retrieval(RetrievalConfig),
    Iterator,
    Collector,
    InlineCode(InlineCodeConfig),
    Loop(LoopConfig),
    LoopInput,
    LoopController(LoopControllerConfig),
}

#[derive(Debug, Clone)]
pub struct ConditionConfig {
    pub conditions: Vec<FlowCondition>,
}

#[derive(Debug, Clone)]
pub struct FlowCondition {
    pub name: String,
    /// `None` for the default condition.
    pub expression: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KnowledgeBaseConfig {
    pub knowledge_base_id: String,
    pub model_id: Option<String>,
    /// Pass-through fields not needed by parser consumers.
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct PromptConfig {
    pub source: PromptSource,
    /// Pass-through: guardrail config and other fields.
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum PromptSource {
    Inline {
        model_id: String,
        template_type: String,
        /// Pass-through: templateConfiguration, inferenceConfiguration, etc.
        extra: serde_json::Value,
    },
    Resource {
        prompt_arn: String,
    },
}

#[derive(Debug, Clone)]
pub struct LambdaFunctionConfig {
    pub lambda_arn: String,
}

#[derive(Debug, Clone)]
pub struct LexConfig {
    pub bot_alias_arn: String,
    pub locale_id: String,
}

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub agent_alias_arn: String,
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub bucket_name: String,
}

#[derive(Debug, Clone)]
pub struct RetrievalConfig {
    pub bucket_name: String,
}

#[derive(Debug, Clone)]
pub struct InlineCodeConfig {
    pub code: String,
    pub language: String,
}

#[derive(Debug, Clone)]
pub struct LoopConfig {
    /// Recursive sub-definition for the loop body.
    pub definition: Box<FlowDefinition>,
}

#[derive(Debug, Clone)]
pub struct LoopControllerConfig {
    pub continue_condition: FlowCondition,
    pub max_iterations: Option<i32>,
}

/// Data type for node inputs and outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowDataType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

impl std::fmt::Display for FlowDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::String => "String",
            Self::Number => "Number",
            Self::Boolean => "Boolean",
            Self::Object => "Object",
            Self::Array => "Array",
        })
    }
}

/// A node input port.
#[derive(Debug, Clone)]
pub struct FlowNodeInput {
    pub name: String,
    pub data_type: FlowDataType,
    pub expression: String,
    pub category: Option<String>,
}

/// A node output port.
#[derive(Debug, Clone)]
pub struct FlowNodeOutput {
    pub name: String,
    pub data_type: FlowDataType,
}

/// A connection between two nodes.
#[derive(Debug, Clone)]
pub struct FlowConnection {
    pub name: String,
    pub source: String,
    pub target: String,
    pub connection_type: FlowConnectionType,
}

/// Connection type — proper enum instead of the wire format's struct-of-optionals.
#[derive(Debug, Clone)]
pub enum FlowConnectionType {
    Data {
        source_output: String,
        target_input: String,
    },
    Conditional {
        condition: String,
    },
}
