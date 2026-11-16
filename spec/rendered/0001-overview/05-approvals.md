## Approvals and the command gateway {#approvals}

An approval-held action is one of these: pairing a device; making a share link, and admitting the
person behind an edit link; an export to a destination outside the owner's computers, a backup
destination, or a public publication; pruning unique original audio, and purging retained learning
evidence; sending private preference or project data to an assistant whose processing may leave the
owner's computers, and sharing a preference, a workflow mechanism or a captured model with a
collaborator; marking a network as the owner's, which lets real-time streams run unencrypted on it;
the route, the channels and the level bound of a capture session, whose signals go into physical
gear; and turning on or widening telemetry sending, and sending a problem report with its
attachments. When the owner takes such an action on their own device, taking it is the approval.
Unpairing a device, revoking a share link, and every edit, render, recording, routing, mapping or
layout change are not approval-held. Adopting a learned preference or a workflow mechanism is an
ordinary command that can be undone.

An owner-interaction record is the record a paired owner device makes of the owner's own input
there, such as a press of an approve control, for one exact effect. Before the first pairing, the
local console of the owner's first computer makes such a record for the approval of a pairing alone.
A command's origin is the party whose client session first sent the command.

The command gateway is the one place in the engine that accepts or refuses each command. It refuses
a command with one of these reasons: unpaired, outside link level, link expired or revoked, or
approval required. The commands that pair and unpair devices, make, revoke and list share links,
admit an editor, and request and decide approvals are commands of the same set as every other
command.

### EU-0001-059 {#EU-0001-059}

An approval-held action that an assistant, a script, a workflow mechanism, a collaborator or anything other than the owner's own input on a paired owner device starts MUST wait for the owner's approval of its exact effect.

### EU-0001-060 {#EU-0001-060}

The party that starts an approval-held action MUST NOT be able to approve it.

### EU-0001-061 {#EU-0001-061}

The engine MUST accept an approval only with an owner-interaction record from one of the owner's paired owner devices, or, for the approval of a pairing, from the local console of the owner's first computer.

### EU-0001-062 {#EU-0001-062}

An approval-held action that the owner's own input on a paired owner device starts, with the owner-interaction record of that input, MUST NOT wait for a further approval.

### EU-0001-063 {#EU-0001-063}

The approval of an action's exact effect MUST show its recipient, its destination and the assets it affects.

### EU-0001-064 {#EU-0001-064}

The command gateway MUST check each command against the party that sent it, the level of that party's link and any approval the command needs, before the command takes effect.

### EU-0001-065 {#EU-0001-065}

The engine MUST take a command's origin from the authenticated client session that first sent it, never from a field of the command.

### EU-0001-066 {#EU-0001-066}

A command the command gateway refuses MUST carry exactly one reason from the closed set of unpaired, outside link level, link expired or revoked, and approval required.

### EU-0001-067 {#EU-0001-067}

Pairings, share links and approvals MUST be managed through commands of the same command set that every client uses.

### EU-0001-068 {#EU-0001-068}

Forwarding, queueing or replaying a command MUST NOT change its origin.

### EU-0001-069 {#EU-0001-069}

An owner-interaction record MUST be bound to the exact effect it approves and to the paired owner device or the local console that made it.

### EU-0001-070 {#EU-0001-070}

The engine MUST NOT accept an owner-interaction record whose effect, device or console doesn't match the approval, or that the engine accepted before.

### EU-0001-071 {#EU-0001-071}

The engine MUST accept the approval of a pairing only after the trusted device and the new device have shown the same short code, derived from the pairing attempt and the new device's key.

### EU-0001-072 {#EU-0001-072}

An input on the new device MUST NOT authorize a pairing by itself.

### EU-0001-073 {#EU-0001-073}

A client session on the owner's first computer, such as a command line, a script or an assistant, MUST NOT be able to make an owner-interaction record as that computer's local console.
