use bitfield_struct::bitfield;

use strum::{IntoEnumIterator, VariantNames};
use strum_macros::{EnumIter, EnumString, FromRepr};

use crate::plc;

#[bitfield(u16)]
struct FooTypePlcStatusModbus {
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

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum LaneIndicatorStatus {
    Begin,
    AllowForward,
    Forbidden,
    AllowTurn,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum TrafficLightStatus {
    Begin,
    Read,
    Green,
    Yellow,
    AllowTurnRight,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum BlowerStatus {
    Begin,
    On,
    Off,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum RollingDoorStatus {
    Begin,
    Up,
    Down,
    Off,
    Reset,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum LedStatus {
    Begin,
    Open,
    Close,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum WaterPumpStatus {
    Begin,
    On,
    Off,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum WindDirections {
    Begin,
    Forward,
    Reverse,
    End,
}

#[derive(EnumIter, EnumString, FromRepr, strum_macros::VariantNames, Debug, PartialEq, Eq)]
enum FooTypePlcEnums {
    LaneIndicatorStatus,
    TrafficLightStatus,
    BlowerStatus,
    RollingDoorStatus,
    LedStatus,
    WaterPumpStatus,
    WindDirections,
}

impl plc::FooTypePlcSchema {
    pub fn new() -> Self {
        Self {
            lane_indicator: vec![plc::FooTypePlcStatus {
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
            traffic_light: vec![plc::FooTypePlcStatus {
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
            blower: vec![plc::FooTypePlcStatus {
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
            rolling_door: vec![plc::FooTypePlcStatus {
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
            led: vec![plc::FooTypePlcStatus {
                status: Some(plc::EnumType {
                    address: 0,
                    name: std::any::type_name::<LedStatus>().to_string(),
                    value: LedStatus::Begin as u32,
                    names: LedStatus::VARIANTS
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    values: LedStatus::iter()
                        .filter_map(|v| {
                            if v != LedStatus::Begin && v != LedStatus::End {
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
            wind_direction: vec![plc::FooTypePlcStatus {
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
                plc::Uint32Type {
                    address: 0,
                    name: String::from("co"),
                    val: 0,
                    max: 10000,
                    min: 0,
                }
            ],
            vi: vec![
                plc::Uint32Type {
                    address: 0,
                    name: String::from("vi"),
                    val: 0,
                    max: 10000,
                    min: 0,
                }
            ],
            no2: vec![
                plc::Uint32Type {
                    address: 0,
                    name: String::from("no2"),
                    val: 0,
                    max: 10000,
                    min: 0,
                }
            ],
            wind_speed: vec![
                plc::Uint32Type {
                    address: 0,
                    name: String::from("wind_speed"),
                    val: 0,
                    max: 10000,
                    min: 0,
                }
            ],
            illuminance: vec![
                plc::Uint32Type {
                    address: 0,
                    name: String::from("illuminance"),
                    val: 0,
                    max: 10000,
                    min: 0,
                }
            ],
            light_intensity: vec![
                plc::Uint32Type {
                    address: 0,
                    name: String::from("light_intensity"),
                    val: 0,
                    max: 10000,
                    min: 0,
                }
            ],
            liquid_level: vec![
                plc::Uint32Type {
                    address: 0,
                    name: String::from("liquid_level"),
                    val: 0,
                    max: 10000,
                    min: 0,
                }
            ],
        }
    }
}
