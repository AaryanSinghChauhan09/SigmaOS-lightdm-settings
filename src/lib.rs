// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-lightdm-settings: lightdm settings component model.
//!
//! Real implementation: settings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setting {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LightdmSettingsError {
    EmptySettingName,
    DuplicateSetting(String),
    EmptyEventSubject,
}

impl fmt::Display for LightdmSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySettingName => write!(f, "setting name must not be empty"),
            Self::DuplicateSetting(id) => write!(f, "duplicate setting {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LightdmSettingsError {}

pub struct LightdmSettings {
    pub name: String,
    settings: BTreeMap<String, Setting>,
    events: BTreeMap<String, Event>,
}

impl LightdmSettings {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            settings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_setting(&mut self, item: Setting) -> Result<(), LightdmSettingsError> {
        if item.name.trim().is_empty() {
            return Err(LightdmSettingsError::EmptySettingName);
        }
        let id = item.id.clone();
        if self.settings.contains_key(&id) {
            return Err(LightdmSettingsError::DuplicateSetting(id.clone()));
        }
        self.settings.insert(id, item);
        Ok(())
    }

    pub fn setting_count(&self) -> usize {
        self.settings.len()
    }

    pub fn get_setting(&self, id: &str) -> Option<&Setting> {
        self.settings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LightdmSettingsError> {
        if event.subject.trim().is_empty() {
            return Err(LightdmSettingsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_setting(id: &str, name: &str) -> Setting {
        Setting {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LightdmSettings::new("LightdmSettings");
        assert_eq!(m.name, "LightdmSettings");
        assert_eq!(m.setting_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_setting() {
        let mut m = LightdmSettings::new("LightdmSettings");
        m.add_setting(sample_setting("a1", "Alpha")).unwrap();
        assert_eq!(m.setting_count(), 1);
        assert!(m.get_setting("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LightdmSettings::new("LightdmSettings");
        assert!(m.add_setting(sample_setting("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LightdmSettings::new("LightdmSettings");
        m.add_setting(sample_setting("x", "One")).unwrap();
        assert!(m.add_setting(sample_setting("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LightdmSettings::new("LightdmSettings");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
