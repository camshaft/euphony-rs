## Surfaces as views of one session {#surfaces}

A control surface is a device or a window that a person uses to see and control a running session:
a phone, a tablet, a second desktop window or screen, a touch monitor, the browser companion or a
hardware controller. A surface attaches to one running session as a view and an input endpoint,
and holds its own layout over that session's existing controls. It is not another authority over
the project, a render worker or an audio source. A second window or a touch monitor wired to a
computer is a local surface of that computer, while a phone, a tablet, the browser companion or a
hardware controller reached through an adapter on an enrolled computer joins over the network.
Surfaces attached to one session share its branch and its transport; collaborators who work apart
use branches of their own, and the session's exclusive control epoch, which decides who controls
transport and recording, is stated by RFC 0013. A listener station keeps the binding to its own
listener mix that RFC 0015 states.

### EU-0017-001 {#EU-0017-001}

A surface MUST attach to exactly one running session at a time.

### EU-0017-002 {#EU-0017-002}

A surface MUST NOT hold its own copy of the project.

### EU-0017-003 {#EU-0017-003}

A surface MUST NOT act as an authority over the session it controls.

### EU-0017-004 {#EU-0017-004}

Attaching a surface MUST NOT make it a render worker or an audio input of the session.

### EU-0017-005 {#EU-0017-005}

The system MUST let phones, tablets, desktop windows, touch monitors, the browser companion and hardware controllers attach to one running session at the same time.

### EU-0017-006 {#EU-0017-006}

A surface MUST act through the same commands, access rules and state as every other client of the session.

### EU-0017-007 {#EU-0017-007}

Surfaces attached to one session MUST share its selected branch and its transport.

### EU-0017-008 {#EU-0017-008}

Attaching a surface MUST NOT transfer the control of the session's transport and recording to it.

### EU-0017-009 {#EU-0017-009}

A surface's transport and recording actions MUST act on behalf of the session's current controller.

### EU-0017-010 {#EU-0017-010}

The session host MUST deny a surface's request that lacks the session's current control epoch.

### EU-0017-011 {#EU-0017-011}

The surface protocol MUST NOT let a listener station change the main mix or the recording.
