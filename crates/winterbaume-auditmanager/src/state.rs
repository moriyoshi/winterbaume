use std::collections::HashMap;

use uuid::Uuid;

use crate::types::{Assessment, Control, Framework};

#[derive(Debug)]
pub struct AuditManagerState {
    pub controls: HashMap<String, Control>,
    pub frameworks: HashMap<String, Framework>,
    pub assessments: HashMap<String, Assessment>,
    pub account_status: String,
}

impl Default for AuditManagerState {
    fn default() -> Self {
        Self {
            controls: HashMap::new(),
            frameworks: HashMap::new(),
            assessments: HashMap::new(),
            account_status: "INACTIVE".to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuditManagerError {
    #[error("Control not found: {id}")]
    ControlNotFound { id: String },

    #[error("Framework not found: {id}")]
    FrameworkNotFound { id: String },

    #[error("Assessment not found: {id}")]
    AssessmentNotFound { id: String },
}

impl AuditManagerState {
    pub fn register_account(&mut self) -> Result<String, AuditManagerError> {
        self.account_status = "ACTIVE".to_string();
        Ok(self.account_status.clone())
    }

    pub fn deregister_account(&mut self) -> Result<String, AuditManagerError> {
        self.account_status = "INACTIVE".to_string();
        Ok(self.account_status.clone())
    }

    pub fn get_account_status(&self) -> &str {
        &self.account_status
    }

    pub fn create_control(
        &mut self,
        name: &str,
        description: Option<String>,
        tags: HashMap<String, String>,
        created_at: f64,
    ) -> Result<Control, AuditManagerError> {
        let id = Uuid::new_v4().to_string();
        let control = Control {
            id: id.clone(),
            name: name.to_string(),
            description,
            control_type: "Custom".to_string(),
            created_at,
            tags,
        };
        self.controls.insert(id, control.clone());
        Ok(control)
    }

    pub fn get_control(&self, id: &str) -> Result<&Control, AuditManagerError> {
        self.controls
            .get(id)
            .ok_or_else(|| AuditManagerError::ControlNotFound { id: id.to_string() })
    }

    pub fn list_controls(&self) -> Vec<&Control> {
        self.controls.values().collect()
    }

    pub fn delete_control(&mut self, id: &str) -> Result<(), AuditManagerError> {
        if self.controls.remove(id).is_none() {
            return Err(AuditManagerError::ControlNotFound { id: id.to_string() });
        }
        Ok(())
    }

    pub fn create_assessment_framework(
        &mut self,
        name: &str,
        description: Option<String>,
        compliance_type: Option<String>,
        tags: HashMap<String, String>,
        created_at: f64,
    ) -> Result<Framework, AuditManagerError> {
        let id = Uuid::new_v4().to_string();
        let framework = Framework {
            id: id.clone(),
            name: name.to_string(),
            description,
            compliance_type,
            framework_type: "Custom".to_string(),
            created_at,
            tags,
        };
        self.frameworks.insert(id, framework.clone());
        Ok(framework)
    }

    pub fn get_assessment_framework(&self, id: &str) -> Result<&Framework, AuditManagerError> {
        self.frameworks
            .get(id)
            .ok_or_else(|| AuditManagerError::FrameworkNotFound { id: id.to_string() })
    }

    pub fn list_assessment_frameworks(&self) -> Vec<&Framework> {
        self.frameworks.values().collect()
    }

    pub fn delete_assessment_framework(&mut self, id: &str) -> Result<(), AuditManagerError> {
        if self.frameworks.remove(id).is_none() {
            return Err(AuditManagerError::FrameworkNotFound { id: id.to_string() });
        }
        Ok(())
    }

    pub fn create_assessment(
        &mut self,
        name: &str,
        description: Option<String>,
        framework_id: &str,
        tags: HashMap<String, String>,
        created_at: f64,
    ) -> Result<Assessment, AuditManagerError> {
        // Verify framework exists
        if !self.frameworks.contains_key(framework_id) {
            return Err(AuditManagerError::FrameworkNotFound {
                id: framework_id.to_string(),
            });
        }
        let id = Uuid::new_v4().to_string();
        let assessment = Assessment {
            id: id.clone(),
            name: name.to_string(),
            description,
            framework_id: framework_id.to_string(),
            status: "ACTIVE".to_string(),
            created_at,
            tags,
        };
        self.assessments.insert(id, assessment.clone());
        Ok(assessment)
    }

    pub fn get_assessment(&self, id: &str) -> Result<&Assessment, AuditManagerError> {
        self.assessments
            .get(id)
            .ok_or_else(|| AuditManagerError::AssessmentNotFound { id: id.to_string() })
    }

    pub fn list_assessments(&self) -> Vec<&Assessment> {
        self.assessments.values().collect()
    }

    pub fn delete_assessment(&mut self, id: &str) -> Result<(), AuditManagerError> {
        if self.assessments.remove(id).is_none() {
            return Err(AuditManagerError::AssessmentNotFound { id: id.to_string() });
        }
        Ok(())
    }
}
