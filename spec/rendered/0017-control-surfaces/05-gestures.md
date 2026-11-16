## Commands, gestures and ownership {#gestures}

A gesture is a continuous input on one or more targets, such as a fader drag, a knob turn, an XY
pad move or a macro. Beginning a gesture names the surface, the session, the control epoch, the
map revision, the target set with each target's expected version, and the gesture's identity; the
session host reserves the target set in one stable order and returns a gesture token, or busy,
stale, denied or unsupported. Each update carries the token, a sequence number, values in the
target's declared domain and its capture time with the clock's quality. Ending a gesture commits
its last accepted value; cancelling it, or losing its surface, closes it. The first accepted begin
on a target owns it until release, cancellation, explicit takeover or detected loss of the
connection. A relative encoder turn adds to a value, while an absolute fader sets it. Knob gestures
follow the parameter's mapping, with fine adjustment as an explicit mode. Touch-down and release
are recorded for automation, and the project's automation recording mode decides what a lane does
at release. Live control sources follow the converters, rates and cache identities of RFC 0012,
and creating or changing a modulation route uses its preview and atomic apply.

### EU-0017-069 {#EU-0017-069}

Every command a surface sends MUST carry its expected state, the control epoch and an idempotency identity.

### EU-0017-070 {#EU-0017-070}

The session host MUST acknowledge each surface command with the state that actually resulted.

### EU-0017-071 {#EU-0017-071}

Beginning a gesture MUST reserve its whole target set atomically or return busy, stale, denied or unsupported.

### EU-0017-072 {#EU-0017-072}

The session host MUST validate the control epoch, the map and the pairing at every update of a gesture.

### EU-0017-073 {#EU-0017-073}

A repeated gesture update MUST receive the same result as its first delivery.

### EU-0017-074 {#EU-0017-074}

A gesture update with an older sequence number MUST NOT overwrite a newer one.

### EU-0017-075 {#EU-0017-075}

The session host MUST journal each accepted control event with its gesture's undo grouping.

### EU-0017-076 {#EU-0017-076}

Recovery after a crash MUST NOT restore a reservation held by a connection that has ended.

### EU-0017-077 {#EU-0017-077}

Cancelling a gesture or losing its surface MUST NOT set its targets to a value other than its last accepted value or the loss behavior its control states.

### EU-0017-157 {#EU-0017-157}

Ending a gesture MUST commit its last accepted value.

### EU-0017-158 {#EU-0017-158}

Ending a gesture MUST release the reservation of its target set.

### EU-0017-078 {#EU-0017-078}

Where two surfaces move absolute controls on the same target, the first accepted begin MUST own the gesture until it ends, is cancelled, is taken over or its connection is lost.

### EU-0017-079 {#EU-0017-079}

A surface that doesn't own the gesture on a target MUST show who owns it and the acknowledged value.

### EU-0017-080 {#EU-0017-080}

A takeover MUST invalidate the earlier owner's gesture token.

### EU-0017-159 {#EU-0017-159}

The session host MUST report a takeover to the surface of the earlier owner and to the surface of the new owner.

### EU-0017-081 {#EU-0017-081}

A gesture MUST NOT hold a lock on any parameter or session outside its target set.

### EU-0017-082 {#EU-0017-082}

The session host MUST revalidate only the versions of a gesture's own targets.

### EU-0017-083 {#EU-0017-083}

Relative encoder turns MUST add up in the session host's order, each counted once.

### EU-0017-084 {#EU-0017-084}

A relative turn on a target that an absolute gesture owns MUST receive busy.

### EU-0017-085 {#EU-0017-085}

A nonmotorized absolute fader MUST pick up at the acknowledged value before it moves its target.

### EU-0017-086 {#EU-0017-086}

Feedback sent to a hardware controller MUST NOT come back as a new command from a person.

### EU-0017-087 {#EU-0017-087}

Motor output to a controller MUST respect its touch detection and its verified device profile.

### EU-0017-088 {#EU-0017-088}

A surface MUST track each touch contact separately.

### EU-0017-089 {#EU-0017-089}

A touch contact MUST NOT take another contact's target by crossing widgets.

### EU-0017-090 {#EU-0017-090}

An XY pad MUST send one paired update for both of its axes.

### EU-0017-091 {#EU-0017-091}

A two-contact gesture such as zoom or pan MUST act on the view unless the layout maps it to sound.

### EU-0017-092 {#EU-0017-092}

Pointer cancellation, suspending the app, locking the device, replacing the map and changing the orientation MUST end each gesture they affect.

### EU-0017-093 {#EU-0017-093}

A layout edit that affects an active target MUST wait for the gesture's release before it commits.

### EU-0017-160 {#EU-0017-160}

A surface MUST show a layout edit that waits for a gesture's release as waiting.

### EU-0017-094 {#EU-0017-094}

Touch-down and release on an automated parameter MUST be recorded as distinct events.

### EU-0017-095 {#EU-0017-095}

The project's automation recording mode MUST decide whether a lane returns to its existing automation or keeps the accepted value at release.

### EU-0017-096 {#EU-0017-096}

Moving a finger over a widget MUST NOT arm automation.

### EU-0017-097 {#EU-0017-097}

Cancelling or losing a gesture during automation recording MUST record a forced end at the last accepted sample, with the reason it ended.

### EU-0017-098 {#EU-0017-098}

An ordinary manual value MUST hold its last accepted state when its gesture ends by loss.

### EU-0017-099 {#EU-0017-099}

A momentary control MUST release when its gesture ends by loss.

### EU-0017-100 {#EU-0017-100}

On the loss of its surface, an external control source MUST fall to its neutral contribution unless its stored route chooses to hold its last value.

### EU-0017-101 {#EU-0017-101}

The system MUST persist the loss transition of a control source with its captured performance.

### EU-0017-102 {#EU-0017-102}

Accepted performance records, converter state and their timing MUST be inputs to later bounces.
