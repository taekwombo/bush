use std::env::args;
use bluez_async::*;

const USAGE: &str = "
    USAGE:
        blebat <device>
";

async fn read_dactyl_battery_levels(session: BluetoothSession, device_name: String) {
    const BAT_SERVICE: uuid::Uuid = make_uuid(0x180F);
    const BAT_LEVEL: uuid::Uuid   = make_uuid(0x2a19);

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
        if service.uuid != BAT_SERVICE {
            continue;
        }

        for characteristic in session.get_characteristics(&service.id).await.unwrap() {
            if !characteristic.flags.contains(CharacteristicFlags::READ) {
                continue;
            }

            if characteristic.uuid != BAT_LEVEL {
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
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
        _ = handle => {
            eprintln!("Bluetooth session handle closed");
        },
        _ = read_dactyl_battery_levels(session, device_name) => (),
    };
}
