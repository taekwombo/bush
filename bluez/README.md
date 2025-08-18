https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces
https://github.com/bluez/bluez/blob/master/doc/org.bluez.Battery.rst
https://dbus.freedesktop.org/doc/dbus-tutorial.html

```fish
    # List services
    busctl list

    # Print object tree 
    #           <service>
    busctl tree org.bluez

    # Introspection
    #                 <service> <object>
    busctl introspect org.bluez /
    busctl introspect org.bluez /org/bluez/hci0

    # Print introspection of interfaces
    #                                <service>
    gdbus introspect --system --dest org.bluez              --object-path / --recurse
    gdbus introspect --system --dest org.freedesktop.UPower --object-path / --recurse

    # List managed objects
    busctl call org.bluez / org.freedesktop.DBus.ObjectManager GetManagedObjects --json=pretty
```
