use clap::Parser;

use plc_log::open_log_file;
use plc_proto::{plc, rs};

pub mod conf;
pub mod version;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse conf first from file, then from command lines
    let mut conf = conf::Conf::parse();
    conf.update(&version::APP_NAME, &version::APP_VERSION);

    // init log
    open_log_file(&conf.app_name, &conf.app_version, conf.service_port);

    // test service api

    // test query_plc_types
    let mut service_client =
        rs::rpc_client::PlcServiceRpcClient::new(&conf.service_host, conf.service_port);
    let resp_query_plc_types = service_client
        .query_plc_types(plc::QueryPlcTypesRequest {
            version: plc::QueryPlcTypesVersion::QueryPlcTypes20240927.into(),
        })
        .await
        .unwrap();
    tracing::info!(plc_types = serde_json::to_string(&resp_query_plc_types).unwrap());

    // test query_plc_schema
    for plc_type in resp_query_plc_types.plc_types {
        let resp_query_plc_schema = service_client
            .query_plc_schema(plc::QueryPlcSchemaRequest {
                version: plc::QueryPlcSchemaVersion::QueryPlcSchema20240927.into(),
                plc_type: Some(plc_type),
            })
            .await
            .unwrap();
        tracing::info!(plc_types = serde_json::to_string(&resp_query_plc_schema).unwrap());
    }

    // test upsert_plc_device
    let resp_upsert_plc_device = service_client
        .upsert_plc_device(plc::UpsertPlcDeviceRequest {
            version: plc::UpsertPlcVersion::UpsertPlc20240928.into(),
            info: Some(plc::PlcDeviceInfo {
                id: String::from("234b77e6-848e-11ef-ab9e-00155dfc822e"),
                r#type: Some(plc::DeviceType {
                    id: plc::PlcDeviceTypeId::Foo.into(),
                    name: plc::PlcDeviceTypeId::Foo.as_str_name().to_string(),
                }),
                address: Some(plc::plc_device_info::Address::TcpAddress(String::from(
                    "localhost:11502",
                ))),
                schema: Some(plc::plc_device_info::Schema::Foo(plc::FooTypePlcSchema {
                    is_adapter_activated: false,
                    lane_indicator: vec![
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7001],
                                name: String::from("lane indicator 1"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7002],
                                name: String::from("lane indicator 2"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                    ],
                    traffic_light: vec![
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7003],
                                name: String::from("traffic light 1"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7004],
                                name: String::from("traffic light 2"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                    ],
                    blower: vec![
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7005],
                                name: String::from("blower 1"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7006],
                                name: String::from("blower 2"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                    ],
                    rolling_door: vec![
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7007],
                                name: String::from("rolling door 1"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7008],
                                name: String::from("rolling door 2"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                    ],
                    light: vec![
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x7009],
                                name: String::from("light 1"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x700a],
                                name: String::from("light 2"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                    ],
                    water_pump: vec![
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x700b],
                                name: String::from("water pump 1"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x700c],
                                name: String::from("water pump 2"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                    ],
                    wind_direction: vec![
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x700d],
                                name: String::from("wind direction 1"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                        plc::FooTypePlcStatus {
                            status: Some(plc::EnumType {
                                address: vec![0x700e],
                                name: String::from("wind direction 2"),
                                value: 0,
                                names: vec![],
                                values: vec![],
                            }),
                            auto_mode: false,
                            device_error: false,
                            logical_error: false,
                        },
                    ],
                    co: vec![
                        plc::Uint32Type {
                            address: vec![0x8001],
                            name: String::from("CO sensor 1"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                        plc::Uint32Type {
                            address: vec![0x8005],
                            name: String::from("CO sensor 2"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                    ],
                    vi: vec![
                        plc::Uint32Type {
                            address: vec![0x8009],
                            name: String::from("VI sensor 1"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                        plc::Uint32Type {
                            address: vec![0x800d],
                            name: String::from("VI sensor 2"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                    ],
                    no2: vec![
                        plc::Uint32Type {
                            address: vec![0x8011],
                            name: String::from("NO2 sensor 1"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                        plc::Uint32Type {
                            address: vec![0x8015],
                            name: String::from("NO2 sensor 2"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                    ],
                    wind_speed: vec![
                        plc::Uint32Type {
                            address: vec![0x8019],
                            name: String::from("wind speed sensor 1"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                        plc::Uint32Type {
                            address: vec![0x801d],
                            name: String::from("wind speed sensor 2"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                    ],
                    illuminance: vec![
                        plc::Uint32Type {
                            address: vec![0x8021],
                            name: String::from("illuminance sensor 1"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                        plc::Uint32Type {
                            address: vec![0x8025],
                            name: String::from("illuminance sensor 2"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                    ],
                    light_intensity: vec![
                        plc::Uint32Type {
                            address: vec![0x8029],
                            name: String::from("light intensity sensor 1"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                        plc::Uint32Type {
                            address: vec![0x802d],
                            name: String::from("light intensity sensor 2"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                    ],
                    liquid_level: vec![
                        plc::Uint32Type {
                            address: vec![0x8031],
                            name: String::from("liquid level sensor 1"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                        plc::Uint32Type {
                            address: vec![0x8035],
                            name: String::from("liquid level sensor 2"),
                            val: 0,
                            max: 10000,
                            min: 0,
                        },
                    ],
                })),
            }),
        })
        .await
        .unwrap();
    tracing::info!(plc_types = serde_json::to_string(&resp_upsert_plc_device).unwrap());

    // test query_plc_devices
    let resp_query_plc_devices = service_client
        .query_plc_devices(plc::QueryPlcDevicesRequest {
            version: plc::QueryPlcDevicesVersion::QueryPlcDevices20240927.into(),
        })
        .await
        .unwrap();
    for plc_device in resp_query_plc_devices.plc_devices {
        tracing::info!(plc_device = serde_json::to_string(&plc_device).unwrap());
    }

    Ok(())
}
