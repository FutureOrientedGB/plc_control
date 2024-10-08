use bitfield_struct::bitfield;

use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, FromRepr};

use crate::plc;

#[bitfield(u16)]
pub struct FooTypePlcStatusModbus {
    #[bits(13)]
    pub status: u16,
    #[bits(1)]
    pub logical_error: bool,
    #[bits(1)]
    pub device_error: bool,
    #[bits(1)]
    pub auto_mode: bool,
}

impl From<plc::FooTypePlcStatus> for FooTypePlcStatusModbus {
    fn from(stat: plc::FooTypePlcStatus) -> Self {
        FooTypePlcStatusModbus::new()
            .with_status(stat.status.unwrap().value as u16)
            .with_logical_error(stat.logical_error)
            .with_device_error(stat.device_error)
            .with_auto_mode(stat.auto_mode)
    }
}

#[derive(EnumIter, AsRefStr, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum LaneIndicatorStatus {
    Begin,
    AllowForward,
    Forbidden,
    AllowTurn,
    End,
}

#[derive(EnumIter, AsRefStr, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum TrafficLightStatus {
    Begin,
    Read,
    Green,
    Yellow,
    AllowTurnRight,
    End,
}

#[derive(EnumIter, AsRefStr, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum BlowerStatus {
    Begin,
    On,
    Off,
    End,
}

#[derive(EnumIter, AsRefStr, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum RollingDoorStatus {
    Begin,
    Up,
    Down,
    Off,
    Reset,
    End,
}

#[derive(EnumIter, AsRefStr, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum LightStatus {
    Begin,
    Open,
    Close,
    End,
}

#[derive(EnumIter, AsRefStr, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WaterPumpStatus {
    Begin,
    On,
    Off,
    End,
}

#[derive(EnumIter, AsRefStr, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WindDirections {
    Begin,
    Forward,
    Reverse,
    End,
}

impl plc::FooTypePlcSchema {
    pub fn new(activated: bool) -> Self {
        Self {
            is_adapter_activated: activated,
            lane_indicator: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: vec![],
                    name: std::any::type_name::<LaneIndicatorStatus>().to_string(),
                    value: LaneIndicatorStatus::Begin as u32,
                    names: LaneIndicatorStatus::iter()
                        .filter_map(|v| {
                            if v != LaneIndicatorStatus::Begin && v != LaneIndicatorStatus::End {
                                Some(v.as_ref().to_string())
                            } else {
                                None
                            }
                        })
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
            traffic_light: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: vec![],
                    name: std::any::type_name::<TrafficLightStatus>().to_string(),
                    value: TrafficLightStatus::Begin as u32,
                    names: TrafficLightStatus::iter()
                        .filter_map(|v| {
                            if v != TrafficLightStatus::Begin && v != TrafficLightStatus::End {
                                Some(v.as_ref().to_string())
                            } else {
                                None
                            }
                        })
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
            blower: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: vec![],
                    name: std::any::type_name::<BlowerStatus>().to_string(),
                    value: BlowerStatus::Begin as u32,
                    names: BlowerStatus::iter()
                        .filter_map(|v| {
                            if v != BlowerStatus::Begin && v != BlowerStatus::End {
                                Some(v.as_ref().to_string())
                            } else {
                                None
                            }
                        })
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
            rolling_door: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: vec![],
                    name: std::any::type_name::<RollingDoorStatus>().to_string(),
                    value: RollingDoorStatus::Begin as u32,
                    names: RollingDoorStatus::iter()
                        .filter_map(|v| {
                            if v != RollingDoorStatus::Begin && v != RollingDoorStatus::End {
                                Some(v.as_ref().to_string())
                            } else {
                                None
                            }
                        })
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
            light: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: vec![],
                    name: std::any::type_name::<LightStatus>().to_string(),
                    value: LightStatus::Begin as u32,
                    names: LightStatus::iter()
                        .filter_map(|v| {
                            if v != LightStatus::Begin && v != LightStatus::End {
                                Some(v.as_ref().to_string())
                            } else {
                                None
                            }
                        })
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
            water_pump: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: vec![],
                    name: std::any::type_name::<WaterPumpStatus>().to_string(),
                    value: WaterPumpStatus::Begin as u32,
                    names: WaterPumpStatus::iter()
                        .filter_map(|v| {
                            if v != WaterPumpStatus::Begin && v != WaterPumpStatus::End {
                                Some(v.as_ref().to_string())
                            } else {
                                None
                            }
                        })
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
            wind_direction: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: vec![],
                    name: std::any::type_name::<WindDirections>().to_string(),
                    value: WindDirections::Begin as u32,
                    names: WindDirections::iter()
                        .filter_map(|v| {
                            if v != WindDirections::Begin && v != WindDirections::End {
                                Some(v.as_ref().to_string())
                            } else {
                                None
                            }
                        })
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
            co: vec![plc::Uint32Type {
                address: vec![],
                name: String::from("co"),
                val: 0,
                max: 10000,
                min: 0,
            }],
            vi: vec![plc::Uint32Type {
                address: vec![],
                name: String::from("vi"),
                val: 0,
                max: 10000,
                min: 0,
            }],
            no2: vec![plc::Uint32Type {
                address: vec![],
                name: String::from("no2"),
                val: 0,
                max: 10000,
                min: 0,
            }],
            wind_speed: vec![plc::Uint32Type {
                address: vec![],
                name: String::from("wind_speed"),
                val: 0,
                max: 10000,
                min: 0,
            }],
            illuminance: vec![plc::Uint32Type {
                address: vec![],
                name: String::from("illuminance"),
                val: 0,
                max: 10000,
                min: 0,
            }],
            light_intensity: vec![plc::Uint32Type {
                address: vec![],
                name: String::from("light_intensity"),
                val: 0,
                max: 10000,
                min: 0,
            }],
            liquid_level: vec![plc::Uint32Type {
                address: vec![],
                name: String::from("liquid_level"),
                val: 0,
                max: 10000,
                min: 0,
            }],
        }
    }
}
