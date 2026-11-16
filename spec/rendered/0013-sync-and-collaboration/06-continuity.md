## Continuing a session between rooms {#continuity}

An active session is separate from any project revision. Its record holds the selected branch and
revision, the transport position and mode, the armed tracks, the registered live sources, the output
assignments, a control epoch and each person's view state. Computers paired with the owner's
subscribe to the session's changes and prefetch the selected manifest, recent edits and media. The
readiness of a computer is the state of its synchronized project data, its prepared audio, its
connected live inputs and its clock lock. The computer that captures a microphone keeps it and keeps
the raw recording, and its inputs reach another room as network routes with stable source identities
and qualified clocks. A person selects Continue session on the destination computer, which attaches
to the existing session instead of opening an unrelated branch. Moving the session authority or its
audio process to another computer is a separate checkpoint and recovery operation, which a room
switch during a take never performs.

### EU-0013-093 {#EU-0013-093}

A person MUST be able to continue a recording session on another paired computer with the current project, controls and live inputs ready.

### EU-0013-094 {#EU-0013-094}

An active session MUST be a record separate from every project revision.

### EU-0013-220 {#EU-0013-220}

Computers paired with the owner's MUST subscribe to the changes of a session and prefetch its selected manifest, recent edits and media.

### EU-0013-095 {#EU-0013-095}

Prefetch of a session MUST go only to computers paired with the owner's.

### EU-0013-096 {#EU-0013-096}

The readiness view MUST show synchronized project data, prepared audio, connected live inputs and clock lock separately.

### EU-0013-097 {#EU-0013-097}

Presence on the local network MUST NOT count as readiness.

### EU-0013-098 {#EU-0013-098}

Continuing a session on another computer MUST restore its selection, editors, transport and monitoring assignments.

### EU-0013-099 {#EU-0013-099}

Moving the control of a session MUST NOT move physical inputs off the computer that captures them.

### EU-0013-100 {#EU-0013-100}

Changing rooms MUST NOT stop a take.

### EU-0013-101 {#EU-0013-101}

Exclusive control of transport and recording MUST move by a conditional change of the control epoch.

### EU-0013-102 {#EU-0013-102}

A stale or repeated control message MUST NOT start a take.

### EU-0013-103 {#EU-0013-103}

After control moves, the previous controller MUST be read-only for transport and recording.

### EU-0013-104 {#EU-0013-104}

Views of a session MAY stay attached on several computers at once.

### EU-0013-105 {#EU-0013-105}

Before it switches a computer's monitor output, the session MUST check its device format, prepared buffers, live subscriptions and clock quality.

### EU-0013-106 {#EU-0013-106}

Where readiness fails, the session MUST keep the previous output and recording active and show what is missing.

### EU-0013-107 {#EU-0013-107}

A supported audio transition MUST happen at a shared sample boundary with latency compensation.

### EU-0013-108 {#EU-0013-108}

An audio transition MUST NOT stop the outputs of other listeners.

### EU-0013-109 {#EU-0013-109}

The session MUST NOT claim an uninterrupted transfer where the hardware or the network cannot support one.

### EU-0013-110 {#EU-0013-110}

A room switch MUST NOT move the session authority during a take.

### EU-0013-111 {#EU-0013-111}

A reopened replica MUST reconcile the completion of capture and establish a new control epoch before recording resumes.
