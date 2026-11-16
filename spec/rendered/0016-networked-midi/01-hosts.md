## MIDI hosts and automatic appearance {#hosts}

A MIDI host is a networked computer, other than the session host, that the owner provisions to
carry MIDI for euphony. A device is a keyboard, controller, synthesizer or interface plugged into a
MIDI host, and each device has one or more ports, each a source of events, a sink of events, or
both. The session host finds MIDI hosts and their ports through service discovery on the network and
through each host's port change events, and shows each port in its device browser with its
connection state. A host can be found by more than one discovery path, and its ports are shown once
whichever path found them. Pairing a host with euphony belongs to the system overview RFC; this RFC
reads only whether a host is paired. Plugging a device and a network cable into a provisioned host
needs neither a network address nor a manual session connection. A MIDI host captures and forwards
events with their timing, recovers lost events and releases held notes. It doesn't hold the project
or render audio, and it runs a musician's programs only as the section on live transforms states.

### EU-0016-001 {#EU-0016-001}

The session host MUST show each port of each device on a discovered MIDI host in its device browser without manual configuration.

### EU-0016-002 {#EU-0016-002}

The device browser MUST show the connection state of each port it shows.

### EU-0016-003 {#EU-0016-003}

The device browser MUST show a port once, whichever discovery path found its host.

### EU-0016-004 {#EU-0016-004}

The device browser MUST show the arrival and the removal of a port as its host reports them.

### EU-0016-005 {#EU-0016-005}

The session host MUST show each device on an unpaired host as unpaired.

### EU-0016-006 {#EU-0016-006}

A device on an unpaired host MUST NOT send events into a project.

### EU-0016-007 {#EU-0016-007}

A device on an unpaired host MUST NOT receive events or material from a project.

### EU-0016-008 {#EU-0016-008}

Once the owner pairs a host, the session host MUST connect each port that a saved automatic connection rule matches, without further setup.

### EU-0016-009 {#EU-0016-009}

The session host MUST validate each port description a host advertises before it shows or connects the port.

### EU-0016-010 {#EU-0016-010}

The session host MUST NOT infer a host's pairing from its label or its network address.

### EU-0016-011 {#EU-0016-011}

A port that has no verified physical-port description MUST be shown as a session port, not as a device port.

### EU-0016-012 {#EU-0016-012}

A virtual mirror of a remote source and that source MUST NOT both deliver the same events to a project.
