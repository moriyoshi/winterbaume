use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct SimSpaceWeaverState {
    /// Simulations keyed by name.
    pub simulations: HashMap<String, SimulationRecord>,
    /// Apps keyed by (simulation, domain, app_name).
    pub apps: HashMap<(String, String, String), AppRecord>,
    /// Tags keyed by ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct SimulationRecord {
    pub name: String,
    pub arn: String,
    pub execution_id: String,
    pub description: Option<String>,
    pub role_arn: String,
    pub status: String,
    pub target_status: String,
    pub creation_time: f64,
    pub maximum_duration: Option<String>,
    pub clock_status: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AppRecord {
    pub simulation: String,
    pub domain: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub target_status: String,
}

#[derive(Debug, Error)]
pub enum SimSpaceWeaverError {
    #[error("Simulation {name} not found.")]
    SimulationNotFound { name: String },

    #[error("Simulation {name} already exists.")]
    SimulationAlreadyExists { name: String },

    #[error("App {name} in simulation {simulation} domain {domain} not found.")]
    AppNotFound {
        simulation: String,
        domain: String,
        name: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl SimSpaceWeaverState {
    pub fn create_simulation(
        &mut self,
        sim: SimulationRecord,
    ) -> Result<&SimulationRecord, SimSpaceWeaverError> {
        if self.simulations.contains_key(&sim.name) {
            return Err(SimSpaceWeaverError::SimulationAlreadyExists {
                name: sim.name.clone(),
            });
        }
        let name = sim.name.clone();
        self.simulations.insert(name.clone(), sim);
        Ok(self.simulations.get(&name).unwrap())
    }

    pub fn get_simulation(&self, name: &str) -> Result<&SimulationRecord, SimSpaceWeaverError> {
        self.simulations
            .get(name)
            .ok_or(SimSpaceWeaverError::SimulationNotFound {
                name: name.to_string(),
            })
    }

    pub fn delete_simulation(&mut self, name: &str) -> Result<(), SimSpaceWeaverError> {
        self.simulations
            .remove(name)
            .ok_or(SimSpaceWeaverError::SimulationNotFound {
                name: name.to_string(),
            })?;
        // Cascade apps under the simulation.
        self.apps.retain(|(s, _, _), _| s != name);
        Ok(())
    }

    pub fn list_simulations(&self) -> Vec<&SimulationRecord> {
        let mut v: Vec<&SimulationRecord> = self.simulations.values().collect();
        v.sort_by(|a, b| a.name.cmp(&b.name));
        v
    }

    pub fn set_simulation_status(
        &mut self,
        name: &str,
        target_status: &str,
        status: &str,
    ) -> Result<(), SimSpaceWeaverError> {
        let sim =
            self.simulations
                .get_mut(name)
                .ok_or(SimSpaceWeaverError::SimulationNotFound {
                    name: name.to_string(),
                })?;
        sim.target_status = target_status.to_string();
        sim.status = status.to_string();
        Ok(())
    }

    pub fn set_clock_status(
        &mut self,
        name: &str,
        clock_status: &str,
    ) -> Result<(), SimSpaceWeaverError> {
        let sim =
            self.simulations
                .get_mut(name)
                .ok_or(SimSpaceWeaverError::SimulationNotFound {
                    name: name.to_string(),
                })?;
        sim.clock_status = clock_status.to_string();
        Ok(())
    }

    pub fn upsert_app(&mut self, app: AppRecord) -> &AppRecord {
        let key = (app.simulation.clone(), app.domain.clone(), app.name.clone());
        self.apps.insert(key.clone(), app);
        self.apps.get(&key).unwrap()
    }

    pub fn get_app(
        &self,
        simulation: &str,
        domain: &str,
        name: &str,
    ) -> Result<&AppRecord, SimSpaceWeaverError> {
        self.apps
            .get(&(simulation.to_string(), domain.to_string(), name.to_string()))
            .ok_or(SimSpaceWeaverError::AppNotFound {
                simulation: simulation.to_string(),
                domain: domain.to_string(),
                name: name.to_string(),
            })
    }

    pub fn delete_app(
        &mut self,
        simulation: &str,
        domain: &str,
        name: &str,
    ) -> Result<(), SimSpaceWeaverError> {
        self.apps
            .remove(&(simulation.to_string(), domain.to_string(), name.to_string()))
            .ok_or(SimSpaceWeaverError::AppNotFound {
                simulation: simulation.to_string(),
                domain: domain.to_string(),
                name: name.to_string(),
            })?;
        Ok(())
    }

    pub fn set_app_status(
        &mut self,
        simulation: &str,
        domain: &str,
        name: &str,
        target_status: &str,
        status: &str,
    ) -> Result<(), SimSpaceWeaverError> {
        let app = self
            .apps
            .get_mut(&(simulation.to_string(), domain.to_string(), name.to_string()))
            .ok_or(SimSpaceWeaverError::AppNotFound {
                simulation: simulation.to_string(),
                domain: domain.to_string(),
                name: name.to_string(),
            })?;
        app.target_status = target_status.to_string();
        app.status = status.to_string();
        Ok(())
    }

    pub fn list_apps<'a>(&'a self, simulation: &'a str) -> Vec<&'a AppRecord> {
        let mut v: Vec<&AppRecord> = self
            .apps
            .iter()
            .filter(|((s, _, _), _)| s == simulation)
            .map(|(_, v)| v)
            .collect();
        v.sort_by(|a, b| {
            (a.domain.as_str(), a.name.as_str()).cmp(&(b.domain.as_str(), b.name.as_str()))
        });
        v
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
