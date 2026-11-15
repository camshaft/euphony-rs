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

The command gateway is the one place in the engine that accepts or refuses each command. It refuses
a command with one of these reasons: unpaired, outside link level, link expired or revoked, or
approval required. The commands that pair and unpair devices, make, revoke and list share links,
admit an editor, and request and decide approvals are commands of the same set as every other
command.

### EU-0001-059 {#EU-0001-059}

An approval-held action that anyone other than the owner starts MUST wait for the owner's approval of its exact effect.

### EU-0001-060 {#EU-0001-060}

The party that starts an approval-held action MUST NOT be able to approve it.

### EU-0001-061 {#EU-0001-061}

The engine MUST accept an approval only with a record of the owner's own interaction on one of their paired owner devices.

### EU-0001-062 {#EU-0001-062}

An action the owner takes on their own device MUST NOT wait for an approval.

### EU-0001-063 {#EU-0001-063}

The approval of an action's exact effect MUST show its recipient, its destination and the assets it affects.

### EU-0001-064 {#EU-0001-064}

The command gateway MUST check each command against the party that sent it, the level of that party's link and any approval the command needs, before the command takes effect.

### EU-0001-065 {#EU-0001-065}

The engine MUST take a command's origin from the client session that carries it, never from a field of the command.

### EU-0001-066 {#EU-0001-066}

A refused command MUST carry exactly one reason from the closed set of unpaired, outside link level, link expired or revoked, and approval required.

### EU-0001-067 {#EU-0001-067}

Pairings, share links and approvals MUST be managed through commands of the same command set that every client uses.
