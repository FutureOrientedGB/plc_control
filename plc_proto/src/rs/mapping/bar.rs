use bitfield_struct::bitfield;

use strum::{IntoEnumIterator, VariantNames};
use strum_macros::{EnumIter, EnumString, FromRepr};

use crate::plc;

#[bitfield(u16)]
pub struct BarTypePlcStatusModbus {
    #[bits(13)]
    pub status: u16,
    #[bits(1)]
    pub logical_error: bool,
    #[bits(1)]
    pub device_error: bool,
    #[bits(1)]
    pub auto_mode: bool,
}

impl From<plc::BarTypePlcStatus> for BarTypePlcStatusModbus {
    fn from(stat: plc::BarTypePlcStatus) -> Self {
        BarTypePlcStatusModbus::new()
            .with_status(stat.status.unwrap().value as u16)
            .with_logical_error(stat.logical_error)
            .with_device_error(stat.device_error)
            .with_auto_mode(stat.auto_mode)
    }
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum LaneIndicatorStatus {
    Begin,
    AllowForward,
    Forbidden,
    AllowTurn,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum TrafficLightStatus {
    Begin,
    Read,
    Green,
    Yellow,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum BlowerStatus {
    Begin,
    On,
    Off,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum RollingDoorStatus {
    Begin,
    Up,
    Down,
    Off,
    Reset,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum LightStatus {
    Begin,
    Open,
    Close,
    Brighten,
    Dim,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WaterPumpStatus {
    Begin,
    On,
    Off,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WindDirections {
    Begin,
    Forward,
    Reverse,
    End,
}

impl plc::BarTypePlcSchema {
    pub fn new(activated: bool) -> Self {
        Self {
            is_adapter_activated: activated,
            lane_indicator: vec![plc::BarTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<LaneIndicatorStatus>().to_string(),
                    value: LaneIndicatorStatus::Begin as u32,
                    names: LaneIndicatorStatus::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: LaneIndicatorStatus::iter()
                        .filter_map(|v| {
                            if v != LaneIndicatorStatus::Begin && v != LaneIndicatorStatus::End {
                                Some(v as u32)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }),
                ..Default::default()
            }],
            traffic_light: vec![plc::BarTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<TrafficLightStatus>().to_string(),
                    value: TrafficLightStatus::Begin as u32,
                    names: TrafficLightStatus::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: TrafficLightStatus::iter()
                        .filter_map(|v| {
                            if v != TrafficLightStatus::Begin && v != TrafficLightStatus::End {
                                Some(v as u32)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }),
                ..Default::default()
            }],
            blower: vec![plc::BarTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<BlowerStatus>().to_string(),
                    value: BlowerStatus::Begin as u32,
                    names: BlowerStatus::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: BlowerStatus::iter()
                        .filter_map(|v| {
                            if v != BlowerStatus::Begin && v != BlowerStatus::End {
                                Some(v as u32)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }),
                ..Default::default()
            }],
            rolling_door: vec![plc::BarTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<RollingDoorStatus>().to_string(),
                    value: RollingDoorStatus::Begin as u32,
                    names: RollingDoorStatus::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: RollingDoorStatus::iter()
                        .filter_map(|v| {
                            if v != RollingDoorStatus::Begin && v != RollingDoorStatus::End {
                                Some(v as u32)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }),
                ..Default::default()
            }],
            light: vec![plc::BarTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<LightStatus>().to_string(),
                    value: LightStatus::Begin as u32,
                    names: LightStatus::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: LightStatus::iter()
                        .filter_map(|v| {
                            if v != LightStatus::Begin && v != LightStatus::End {
                                Some(v as u32)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }),
                ..Default::default()
            }],
            water_pump: vec![plc::BarTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<WaterPumpStatus>().to_string(),
                    value: WaterPumpStatus::Begin as u32,
                    names: WaterPumpStatus::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: WaterPumpStatus::iter()
                        .filter_map(|v| {
                            if v != WaterPumpStatus::Begin && v != WaterPumpStatus::End {
                                Some(v as u32)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }),
                ..Default::default()
            }],
            wind_direction: vec![plc::BarTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<WindDirections>().to_string(),
                    value: WindDirections::Begin as u32,
                    names: WindDirections::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: WindDirections::iter()
                        .filter_map(|v| {
                            if v != WindDirections::Begin && v != WindDirections::End {
                                Some(v as u32)
                            } else {
                                None
                            }
                        })
                        .collect(),
                }),
                ..Default::default()
            }],
            co: vec![
                plc::Float32Type {
                    address: 0,
                    name: String::from("co"),
                    val: 0.0,
                    max: 10000.0,
                    min: 0.0,
                }
            ],
            vi: vec![
                plc::Float32Type {
                    address: 0,
                    name: String::from("vi"),
                    val: 0.0,
                    max: 10000.0,
                    min: 0.0,
                }
            ],
            no2: vec![
                plc::Float32Type {
                    address: 0,
                    name: String::from("no2"),
                    val: 0.0,
                    max: 10000.0,
                    min: 0.0,
                }
            ],
            wind_speed: vec![
                plc::Float32Type {
                    address: 0,
                    name: String::from("wind_speed"),
                    val: 0.0,
                    max: 10000.0,
                    min: 0.0,
                }
            ],
            illuminance: vec![
                plc::Float32Type {
                    address: 0,
                    name: String::from("illuminance"),
                    val: 0.0,
                    max: 10000.0,
                    min: 0.0,
                }
            ],
            light_intensity: vec![
                plc::Float32Type {
                    address: 0,
                    name: String::from("light_intensity"),
                    val: 0.0,
                    max: 10000.0,
                    min: 0.0,
                }
            ],
            liquid_level: vec![
                plc::Float32Type {
                    address: 0,
                    name: String::from("liquid_level"),
                    val: 0.0,
                    max: 10000.0,
                    min: 0.0,
                }
            ],
        }
    }
}
