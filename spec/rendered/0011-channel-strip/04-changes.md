## Changes, undo and assistants {#changes}

A stage's settings change only when the musician, or an assistant the musician connected, asks. The
strip keeps each stage's starting point and the musician's offsets from it apart, so that a change of
taste shows as an offset. A new take, a new analysis result, a new calculation version or a newly
approved preference offers Dial in again and changes nothing until someone asks. The musician's own
change applies at once, as turning a knob does. An assistant acts as the musician, under the trust
model of RFC 0001, through the same commands as every client; while the transport plays or records,
its change to a strip waits as a proposal, a timing rule for sound and not a permission. Every
switch, dial-in and adjustment is one step of the project's history, as RFC 0004 specifies it.

### EU-0011-094 {#EU-0011-094}

A stage's settings MUST NOT change except at the request of the musician or of an assistant the musician connected.

### EU-0011-095 {#EU-0011-095}

A new take, a new analysis result, a new calculation version or a newly approved preference MUST NOT change a stage's settings.

### EU-0011-096 {#EU-0011-096}

Where a new take, analysis result, calculation version or approved preference would change a stage's starting point, the strip MUST offer Dial in again on that stage.

### EU-0011-097 {#EU-0011-097}

Dial in again MUST keep the musician's offsets on top of the new starting point unless the musician clears them.

### EU-0011-098 {#EU-0011-098}

A stage MUST keep its starting point and the musician's offsets from it apart.

### EU-0011-099 {#EU-0011-099}

The musician's own change to a stage MUST apply at once, during playback and recording too.

### EU-0011-100 {#EU-0011-100}

While the transport plays or records, an assistant's change to a strip MUST wait as a proposal.

### EU-0011-101 {#EU-0011-101}

A held proposal MUST apply when the transport stops.

### EU-0011-102 {#EU-0011-102}

A held proposal MUST NOT apply during playback or recording unless the musician applies it from their native client.

### EU-0011-103 {#EU-0011-103}

An apply of a held proposal that an assistant, a script or the command line sends MUST be refused.

### EU-0011-183 {#EU-0011-183}

An apply of a held proposal that repeats an earlier apply of the musician MUST be refused.

### EU-0011-104 {#EU-0011-104}

An assistant MUST read and change a strip through the same commands as the musician's clients.

### EU-0011-105 {#EU-0011-105}

The same strip workflow MUST produce the same project history whether it is made from the interface, the command line or an assistant.

### EU-0011-106 {#EU-0011-106}

Each switch, dial-in and adjustment of a stage MUST be one step of the project's undo history.

### EU-0011-107 {#EU-0011-107}

One undo MUST reverse both a stage's switch and its starting point.

### EU-0011-108 {#EU-0011-108}

Undo, redo, compare and reset MUST restore the exact earlier state of a stage.

### EU-0011-109 {#EU-0011-109}

The musician MUST be able to switch between a stage's current settings and its previous settings, both level-matched.

### EU-0011-110 {#EU-0011-110}

The musician MUST be able to reset a stage to its starting point by clearing its offsets.

### EU-0011-111 {#EU-0011-111}

A collaborator with an edit link MUST dial stages in only on their own branch.

### EU-0011-112 {#EU-0011-112}

A participant with listen or comment access alone MUST NOT change a strip.
