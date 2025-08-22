use std::time::Duration;
use std::process::ExitCode;
use std::collections::HashMap;

use dbus::arg::PropMap;
use dbus::Path;
use dbus::blocking::Connection;

mod dapi;

use dapi::root::*;

const fn make_uuid(assigned: u32) -> uuid::Uuid {
    // https://www.bluetooth.com/specifications/assigned-numbers/
    // Battery Service: 0000180f-0000-1000-8000-00805f9b34fb
    // Battery Level:   00002a19-0000-1000-8000-00805f9b34fb
    uuid::Uuid::from_fields(
        assigned,
        0x0000,
        0x1000,
        &[0x80, 0x0, 0x0, 0x80, 0x5f, 0x9b, 0x34, 0xfb],
    )
}

mod name {
    pub const I_DEVICE: &str = "org.bluez.Device1";
    pub const I_GATTS: &str = "org.bluez.GattService1";
    pub const I_GATTC: &str = "org.bluez.GattCharacteristic1";
    pub const P_NAME: &str = "Name";
    pub const P_VALUE: &str = "Value";
    pub const P_UUID: &str = "UUID";
    pub const P_CONNECTED: &str = "Connected";

    pub const S_BLUEZ: &str = "org.bluez";
    pub const ROOT: &str = "/";
}

fn find_device(objects: &HashMap<Path<'static>, HashMap<String, PropMap>>, arg_device_name: String) -> Option<Path<'static>> {
    for (path, interfaces) in objects.iter() {
        let Some(dev) = interfaces.get(name::I_DEVICE) else {
            continue;
        };

        let name = dev.get(name::P_NAME)
            .expect("Device1.Name exists")
            .0
            .as_str()
            .expect("Device1.Name is String");

        if name != arg_device_name {
            continue;
        }

        let connected = dev.get(name::P_CONNECTED)
            .expect("Device1.Connected exists")
            .0
            .as_u64()
            .expect("Device1.Connected is Boolean");

        if connected == 0 {
            eprintln!("Device {} not connected", arg_device_name);
            return None;
        }

        return Some(path.clone());
    }

    eprintln!("Device {} not found", arg_device_name);
    None
}

fn get_battery_levels(objects: &HashMap<Path<'static>, HashMap<String, PropMap>>, device: Path<'static>) {
    let dev_name = String::from_utf8_lossy(device.as_bytes());

    const BAT_SERVICE: uuid::Uuid = make_uuid(0x180F);
    const BAT_LEVEL: uuid::Uuid   = make_uuid(0x2a19);

    let services = objects.iter().filter_map(|(path, interfaces)| {
        let name = String::from_utf8_lossy(path.as_bytes());

        let Some(name) = name.strip_prefix(&*dev_name) else {
            return None;
        };
        let Some(name) = name.strip_prefix("/service") else {
            return None;
        };

        if name.contains("/") {
            return None;
        }

        let Some(gs) = interfaces.get(name::I_GATTS) else {
            return None;
        };


        let id = gs
            .get(name::P_UUID)
            .expect("GattService1.UUID exists")
            .0
            .as_str()
            .expect("GattService1.UUID is String");

        if uuid::Uuid::try_parse(id).ok().map(|v| v != BAT_SERVICE).unwrap_or(true) {
            return None;
        }

        Some(path)
    }).collect::<Vec<_>>();

    for (path, interfaces) in objects.iter() {
        let name = String::from_utf8_lossy(path.as_bytes());

        if !services.iter().any(|v| name.starts_with(&*String::from_utf8_lossy(v.as_bytes()))) {
            continue;
        }

        let Some(gc) = interfaces.get(name::I_GATTC) else {
            continue;
        };

        let id = gc
            .get(name::P_UUID)
            .expect("GattCharacteristic1.UUID exists")
            .0
            .as_str()
            .expect("GattCharacteristic1.UUID is String");

        if uuid::Uuid::try_parse(id).ok().map(|v| v != BAT_LEVEL).unwrap_or(true) {
            continue;
        }

        let Some(v) = gc.get(name::P_VALUE) else {
            continue;
        };
        

        if let Some(mut v) = v.0.as_iter() {
            if let Some(l) = v.next() {
                println!("{}", l.as_u64().unwrap());
            }
        }
    }
}

fn main() -> ExitCode {
    let Some(arg_device_name) = std::env::args().nth(1) else {
        eprintln!("Missing <device> argument.");
        return ExitCode::FAILURE;
    };

    let timeout = Duration::from_secs(5);
    let session = Connection::new_system().unwrap();

    let bluez_proxy = session.with_proxy(name::S_BLUEZ, name::ROOT, timeout);
    let Ok(objects) = bluez_proxy.get_managed_objects() else {
        eprintln!("DBus Failure");
        return ExitCode::FAILURE;
    };

    let Some(device) = find_device(&objects, arg_device_name) else {
        return ExitCode::FAILURE;
    };

    get_battery_levels(&objects, device);

    ExitCode::SUCCESS
}
