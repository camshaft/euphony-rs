## Requirements every component shares {#shared-requirements}

A session action is any action on a session or on its project that the owner can take, such as an
edit, a transport change, a recording or a render. A project action is a session action that changes
the project. An interface is any client that shows or controls a session: the desktop interface, a
command line, a script, a browser companion, a phone, a control surface or the adapter through which
an assistant works. A release is one published version of a component. The engine is the program on
the owner's computers that runs the session, which RFC 0002 states. Each component's RFC refines
these requirements for that component.

### EU-0001-004 {#EU-0001-004}

An assistant the owner connects MUST be able to take every project action the owner can take.

### EU-0001-005 {#EU-0001-005}

A project action that an assistant takes MUST pass the same validation as the same action taken by the owner.

### EU-0001-006 {#EU-0001-006}

Each change to a project MUST be recorded in a history the owner can inspect, with the party that made it.

### EU-0001-007 {#EU-0001-007}

A session action MUST return the same result whether it is taken from the desktop interface, a command line, a script or an assistant.

### EU-0001-008 {#EU-0001-008}

A session action MUST return the same result whether it is taken on the computer that runs the session or on another of the owner's computers.

### EU-0001-009 {#EU-0001-009}

An interface that a later release adds MUST return the same results for the same session actions as the interfaces before it.

### EU-0001-010 {#EU-0001-010}

Closing, restarting or losing an interface MUST NOT stop or interrupt playback or recording.

### EU-0001-011 {#EU-0001-011}

Closing, restarting or losing an interface MUST NOT interrupt another interface.

### EU-0001-012 {#EU-0001-012}

An interface that reattaches to a session MUST show the session as it stands.

### EU-0001-013 {#EU-0001-013}

An interface that reattaches to a session MUST NOT repeat input it sent before it reattached.

### EU-0001-014 {#EU-0001-014}

A crash of an interface during a recording MUST NOT lose a recorded sample.

### EU-0001-015 {#EU-0001-015}

A crash of an interface MUST NOT change any output of the session other than that interface's own display.

### EU-0001-016 {#EU-0001-016}

On one computer, the desktop interface MUST respond to faders, the transport, meters, waveforms and scrolling within the difference the owner accepts from the same interface and engine built into one program, after seeing that difference measured on their computer.

### EU-0001-017 {#EU-0001-017}

Where an interface and the engine it attaches to are from releases that cannot work together, the interface MUST say which release it needs.

### EU-0001-018 {#EU-0001-018}

An interface and an engine from different releases MUST NOT misread each other's state or commands.

### EU-0001-019 {#EU-0001-019}

Each behavior this set specifies MUST hold when a computer the session uses fails or returns outdated audio.
