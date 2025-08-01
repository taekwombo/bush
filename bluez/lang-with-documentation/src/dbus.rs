fn main() {
    let message = dbus::message::Message::new_method_call(
        "org.bluez",
        "/org/bluez/hci0",
        "org.freedesktop.DBus.Introspectable",
        "Introspect",
    ).unwrap();
    let session = dbus::blocking::Connection::new_system().unwrap();
    let timeout = std::time::Duration::from_secs(10);
    // println!("{}", session.channel().is_connected());
    // println!("{:?}", session.channel().pop_message());
    // println!("{:?}", session.channel().pop_message());

    println!("{:?}", session.channel().send_with_reply_and_block(message, timeout));
}
