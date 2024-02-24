use clang::Availability;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    pub x: u32,
    pub y: Option<u32>,
    pub z: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformAvailability {
    pub platform: String,
    pub unavailable: bool,
    pub introduced: Option<Version>,
    pub deprecated: Option<Version>,
    pub obsoleted: Option<Version>,
    pub message: Option<String>,
}

impl Version {
    pub fn from(version: clang::Version) -> Self {
        Self {
            x: version.x,
            y: version.y,
            z: version.z,
        }
    }
}

impl PlatformAvailability {
    pub fn from(availability: &clang::PlatformAvailability) -> Self {
        Self {
            platform: availability.platform.clone(),
            unavailable: availability.unavailable,
            introduced: availability.introduced.map(|v| Version::from(v)),
            deprecated: availability.deprecated.map(|v| Version::from(v)),
            obsoleted: availability.obsoleted.map(|v| Version::from(v)),
            message: availability.message.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "Availability")]
pub enum AvailabilityDef {
    Available = 0,
    Deprecated = 1,
    Inaccessible = 3,
    Unavailable = 2,
}

pub fn get_platform_availability(entity: &clang::Entity) -> Option<Vec<PlatformAvailability>> {
    entity
        .get_platform_availability()
        .map(|v| v.iter().map(|a| PlatformAvailability::from(a)).collect())
}
