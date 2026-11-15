## Retrospectives {#retrospectives}

The end of a session, the completion of a project or an explicit request creates retrospective
work over a closed interval of events. The assistant that picks up the work reads that interval
and records what worked, what needed correction, the problems still open, the gaps in evidence,
and its proposals of preferences and mechanisms. The owner sees the proposed changes together as
one diff and decides each of them on its own.

### EU-0019-154 {#EU-0019-154}

The end of a session, the completion of a project or an explicit request MUST create retrospective work over a closed interval of events.

### EU-0019-155 {#EU-0019-155}

Creating retrospective work MUST be idempotent for the same interval.

### EU-0019-156 {#EU-0019-156}

A retrospective MUST record what worked, what needed correction, unresolved problems, evidence gaps and its proposals.

### EU-0019-157 {#EU-0019-157}

An interrupted retrospective MUST resume from its cursor.

### EU-0019-158 {#EU-0019-158}

Evidence that arrives after a retrospective's interval MUST create a linked amendment instead of duplicate proposals.

### EU-0019-159 {#EU-0019-159}

The event that creates retrospective work MUST make the work available at once and at the next connection of an assistant.

### EU-0019-160 {#EU-0019-160}

Retrospective work MUST NOT depend on polling or on a calendar wake.

### EU-0019-210 {#EU-0019-210}

Euphony MUST show the owner the changes a retrospective proposes together as one diff.

### EU-0019-161 {#EU-0019-161}

The owner MUST be able to accept or reject each change a retrospective proposes on its own.

### EU-0019-162 {#EU-0019-162}

A retrospective that found nothing MUST record its coverage and the versions it examined.

### EU-0019-163 {#EU-0019-163}

Euphony MUST NOT consider learning complete because an assistant stopped receiving corrections.
