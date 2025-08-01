use std::env::args;
use bluez_async::*;

const USAGE: &str = "
    USAGE:
        blebat <device>
";

async fn read_dactyl_battery_levels(session: BluetoothSession, device_name: String, bat_service: uuid::Uuid, bat_characteristic: uuid::Uuid) {
    let device  = session.get_devices().await.unwrap().into_iter().find(|ref d| match d.name.as_ref() {
        Some(n) => n == &device_name,
        None => false,
    });

    if device.is_none() {
        eprintln!("Device {} not found", device_name);
        return;
    }

    if !device.as_ref().map(|d| d.connected).unwrap_or(false) {
        eprintln!("Device {} not connected", device_name);
        return;
    }

    let device = device.unwrap();

    for service in session.get_services(&device.id).await.unwrap() {
        if service.uuid != bat_service {
            continue;
        }

        for characteristic in session.get_characteristics(&service.id).await.unwrap() {
            if !characteristic.flags.contains(CharacteristicFlags::READ) {
                continue;
            }

            if characteristic.uuid != bat_characteristic {
                continue;
            }

            let level = session.read_characteristic_value(&characteristic.id)
                   .await
                   .ok()
                   .and_then(|mut vec| vec.pop());

            if let Some(level) = level {
                println!("{}", level);
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let bat_service = uuid::Uuid::parse_str("0000180f-0000-1000-8000-00805f9b34fb").unwrap();
    let bat_characteristic = uuid::Uuid::parse_str("00002a19-0000-1000-8000-00805f9b34fb").unwrap();

    let device_name = match args().nth(1) {
        Some(d) => d,
        None => {
            eprintln!("Missing <device> argument.");
            eprintln!("{}", USAGE);
            std::process::exit(1);
        }
    };

    let (handle, session) = BluetoothSession::new().await.unwrap();

    tokio::select! {
        _ = handle => (),
        _ = read_dactyl_battery_levels(session, device_name, bat_service, bat_characteristic) => (),
    };
}
