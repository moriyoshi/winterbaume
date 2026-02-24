use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::OutpostsService;
use crate::state::OutpostsState;
use crate::types::{Outpost, Site};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutpostsStateView {
    #[serde(default)]
    pub outposts: HashMap<String, OutpostView>,
    #[serde(default)]
    pub sites: HashMap<String, SiteView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutpostView {
    pub outpost_id: String,
    pub outpost_arn: String,
    pub owner_id: String,
    pub name: String,
    pub description: Option<String>,
    pub site_id: String,
    pub site_arn: String,
    pub availability_zone: Option<String>,
    pub availability_zone_id: Option<String>,
    pub life_cycle_status: String,
    pub supported_hardware_type: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteView {
    pub site_id: String,
    pub site_arn: String,
    pub account_id: String,
    pub name: String,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub operating_address_country_code: Option<String>,
    pub operating_address_state_or_region: Option<String>,
    pub operating_address_city: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&OutpostsState> for OutpostsStateView {
    fn from(state: &OutpostsState) -> Self {
        OutpostsStateView {
            outposts: state
                .outposts
                .iter()
                .map(|(k, v)| (k.clone(), OutpostView::from(v)))
                .collect(),
            sites: state
                .sites
                .iter()
                .map(|(k, v)| (k.clone(), SiteView::from(v)))
                .collect(),
        }
    }
}

impl From<&Outpost> for OutpostView {
    fn from(o: &Outpost) -> Self {
        OutpostView {
            outpost_id: o.outpost_id.clone(),
            outpost_arn: o.outpost_arn.clone(),
            owner_id: o.owner_id.clone(),
            name: o.name.clone(),
            description: o.description.clone(),
            site_id: o.site_id.clone(),
            site_arn: o.site_arn.clone(),
            availability_zone: o.availability_zone.clone(),
            availability_zone_id: o.availability_zone_id.clone(),
            life_cycle_status: o.life_cycle_status.clone(),
            supported_hardware_type: o.supported_hardware_type.clone(),
            tags: o.tags.clone(),
        }
    }
}

impl From<&Site> for SiteView {
    fn from(s: &Site) -> Self {
        SiteView {
            site_id: s.site_id.clone(),
            site_arn: s.site_arn.clone(),
            account_id: s.account_id.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            notes: s.notes.clone(),
            operating_address_country_code: s.operating_address_country_code.clone(),
            operating_address_state_or_region: s.operating_address_state_or_region.clone(),
            operating_address_city: s.operating_address_city.clone(),
            tags: s.tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<OutpostsStateView> for OutpostsState {
    fn from(view: OutpostsStateView) -> Self {
        OutpostsState {
            outposts: view
                .outposts
                .into_iter()
                .map(|(k, v)| (k, Outpost::from(v)))
                .collect(),
            sites: view
                .sites
                .into_iter()
                .map(|(k, v)| (k, Site::from(v)))
                .collect(),
        }
    }
}

impl From<OutpostView> for Outpost {
    fn from(v: OutpostView) -> Self {
        Outpost {
            outpost_id: v.outpost_id,
            outpost_arn: v.outpost_arn,
            owner_id: v.owner_id,
            name: v.name,
            description: v.description,
            site_id: v.site_id,
            site_arn: v.site_arn,
            availability_zone: v.availability_zone,
            availability_zone_id: v.availability_zone_id,
            life_cycle_status: v.life_cycle_status,
            supported_hardware_type: v.supported_hardware_type,
            tags: v.tags,
        }
    }
}

impl From<SiteView> for Site {
    fn from(v: SiteView) -> Self {
        Site {
            site_id: v.site_id,
            site_arn: v.site_arn,
            account_id: v.account_id,
            name: v.name,
            description: v.description,
            notes: v.notes,
            operating_address_country_code: v.operating_address_country_code,
            operating_address_state_or_region: v.operating_address_state_or_region,
            operating_address_city: v.operating_address_city,
            tags: v.tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for OutpostsService {
    type StateView = OutpostsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        OutpostsStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = OutpostsState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.sites {
                guard.sites.insert(k, Site::from(v));
            }
            for (k, v) in view.outposts {
                guard.outposts.insert(k, Outpost::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
