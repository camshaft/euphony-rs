## Supported surface profiles {#profiles}

A surface profile is a kind of device and host, such as a model of hardware controller with the
computer its adapter runs on. A hardware adapter is a helper on an enrolled computer that the
discovery service finds and configures, and translates a controller's own protocol into the surface
commands of this RFC; a network instrument controller keeps the stable device identity that RFC
0016 states. A conformance publisher, set in the project's configuration, records each profile's
identity, the panel, the renderer and the build, the test evidence and the verdict. The project's
configuration also sets the surface categories a release covers: phone, tablet, desktop window or
touch monitor, browser companion and hardware controller. Universal controller compatibility is not
assumed, and later versions add profiles.

### EU-0017-142 {#EU-0017-142}

A hardware adapter MUST translate its controller's protocol into the surface commands of this RFC.

### EU-0017-143 {#EU-0017-143}

Support for hardware controllers MUST be stated for each verified profile.

### EU-0017-144 {#EU-0017-144}

A surface profile without a passing physical conformance record for the running build MUST be unavailable.

### EU-0017-145 {#EU-0017-145}

Simulated evidence MUST NOT stand in for a physical conformance record.

### EU-0017-146 {#EU-0017-146}

The list of supported profiles MUST be derived from the conformance records.

### EU-0017-147 {#EU-0017-147}

A release MUST NOT advertise a profile list that differs from the conformance records.

### EU-0017-148 {#EU-0017-148}

A release MUST hold passing evidence for each surface category the project's configuration requires.

### EU-0017-149 {#EU-0017-149}

A conformance record MUST NOT authorize pairing or control.

### EU-0017-150 {#EU-0017-150}

The surface categories, the mobile platforms of the phone controller and the conformance publisher MUST be set in the project's configuration.

### EU-0017-166 {#EU-0017-166}

A hardware controller that only displays a surface MAY present it without emulating a touch screen.
