use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, error::Error};

pub type AlertsDescriptions = HashMap<String, String>;

#[derive(Deserialize)]
pub struct AlertsYaml {
    alerts: Vec<AlertEntry>,
}

#[derive(Deserialize)]
pub struct AlertEntry {
    #[serde(rename = "type")]
    alert_type: String,
}

pub fn load_alerts_static(yaml: &str) -> Result<AlertsDescriptions, Box<dyn Error>> {
    let data: AlertsYaml = serde_yaml::from_str(yaml)?;
    let mut map = HashMap::new();
    for alert in data.alerts {
        map.insert(alert.alert_type, "Alert".to_string()); // default description
    }
    Ok(map)
}
