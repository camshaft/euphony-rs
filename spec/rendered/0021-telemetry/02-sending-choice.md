## The person's choice to send {#sending-choice}

Sending is the passing of telemetry from the person's computers to the receiving service that the
project runs. The person's sending choice has three parts. The categories are performance, which
holds deadline misses, render timings, resource samples and device latencies; stability, which
holds crashes and errors; and assistant actions, which holds tool calls. Each project is included
or excluded. The sending mode is the first mode, in which each batch waits in the outbox until the
person reviews and sends it, or the second mode, in which batches of the chosen categories go once
built. A batch is an immutable record of submission records that the client builds on the computer
and holds in the outbox until it is sent or deleted. Each change of the sending choice makes a new
consent revision. A send authorization is the record that the person reviewed one batch in the
outbox and chose to send it. The sent log on the computer holds each batch that was sent. Bytes
already sent can't be recalled: the sent log shows them, and deletion at the receiving service
removes what it holds of them. RFC 0001 states the owner's approval of turning sending on or
widening it, and what that approval shows (EU-0001-059, EU-0001-062 and EU-0001-063).

### EU-0021-032 {#EU-0021-032}

Sending MUST be off in a new installation.

### EU-0021-033 {#EU-0021-033}

While sending is off, euphony MUST NOT open a connection to the receiving service, including to read its settings, except to send a problem report that the person chose to send or a deletion request that the person made.

### EU-0021-034 {#EU-0021-034}

A release MUST NOT turn sending on.

### EU-0021-035 {#EU-0021-035}

A category that a later release adds MUST stay off until the person turns it on.

### EU-0021-036 {#EU-0021-036}

The person MUST be able to turn each category on and off apart from the others.

### EU-0021-037 {#EU-0021-037}

The person MUST be able to include or exclude each project.

### EU-0021-038 {#EU-0021-038}

A project that the person holds through another person's edit link MUST be excluded until the person includes it.

### EU-0021-039 {#EU-0021-039}

The sending mode MUST start as the first mode.

### EU-0021-040 {#EU-0021-040}

The person MAY choose the second mode.

### EU-0021-041 {#EU-0021-041}

The client MUST build each batch on the computer, from the submission schema alone, as an immutable record in the outbox.

### EU-0021-042 {#EU-0021-042}

Before a batch goes, the outbox MUST show every field of every record in plain words, the exact bytes, the destination address, the schema version and the submission identity.

### EU-0021-043 {#EU-0021-043}

Each batch and each problem report MUST record the consent it was built under: the consent revision, the destination, the submission schema version and its field set, and the categories and projects it covers.

### EU-0021-044 {#EU-0021-044}

The client MUST send only a batch that the outbox holds.

### EU-0021-045 {#EU-0021-045}

At the first send and at each retry, the client MUST check that the bytes have the digest the outbox showed and that the current consent revision still allows the batch's destination, schema, fields, categories and projects.

### EU-0021-046 {#EU-0021-046}

A batch whose check fails MUST NOT be sent.

### EU-0021-047 {#EU-0021-047}

Where the consent has narrowed or changed while sending is still on, the client MUST delete the batch or rebuild it under the current consent.

### EU-0021-048 {#EU-0021-048}

A rebuilt batch MUST have an identity and a digest of its own.

### EU-0021-049 {#EU-0021-049}

A change of the sending choice MUST apply to every send whose check comes after the change.

### EU-0021-050 {#EU-0021-050}

A release that changes the destination or adds fields to the submission schema MUST pause sending until the person reviews the change and turns sending on again.

### EU-0021-051 {#EU-0021-051}

A batch MUST NOT hold an event of an excluded project, including an event of a crash trail.

### EU-0021-052 {#EU-0021-052}

A batch MUST NOT hold an event that the client can't attribute to included projects alone.

### EU-0021-053 {#EU-0021-053}

In the first mode, the client MUST send a batch only with a send authorization bound to the batch identity, its digest, its destination and the consent revision it was built under.

### EU-0021-054 {#EU-0021-054}

A send authorization MUST come only from the owner's own review of the batch on a paired owner device.

### EU-0021-055 {#EU-0021-055}

An assistant, a script or a workflow MUST NOT create a send authorization.

### EU-0021-056 {#EU-0021-056}

An assistant's, a script's or a workflow's request to send a batch MUST wait for the owner's approval.

### EU-0021-057 {#EU-0021-057}

A retry of an authorized batch MAY reuse its authorization while the consent revision still allows the batch.

### EU-0021-058 {#EU-0021-058}

A change from the second mode to the first MUST apply to every batch not yet sent.

### EU-0021-059 {#EU-0021-059}

A change from the first mode to the second MUST be a change of the sending choice that the owner makes.

### EU-0021-060 {#EU-0021-060}

The sent log MUST keep each sent batch as it was sent.

### EU-0021-061 {#EU-0021-061}

Turning sending off MUST delete every batch still in the outbox.

### EU-0021-062 {#EU-0021-062}

The screen that turns sending off MUST offer the deletion of what was already sent.
