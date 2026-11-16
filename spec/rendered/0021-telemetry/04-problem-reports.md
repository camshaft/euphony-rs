## Problem reports {#problem-reports}

A problem report is one report of a crash or another problem that the person sends once, with the
parts they choose. It is a choice of the owner apart from the sending choice, and the receiving
service joins it to the finding of the event it describes. Its attachments can hold the person's
music, so each one stays off until the person adds it, and only the maintainers that the project's
configuration lists can open it. RFC 0001 states the owner's approval of sending a report that an
assistant, a script or a workflow drafts (EU-0001-059 and EU-0001-062).

### EU-0021-088 {#EU-0021-088}

A person MUST be able to send one problem report while sending is off.

### EU-0021-089 {#EU-0021-089}

The client MUST offer to draft a problem report at the next start after a crash, from a report command in the native interface, and through the assistant adapter.

### EU-0021-090 {#EU-0021-090}

A drafted report MUST hold the person's own description, the crash or error event they choose, and its trail.

### EU-0021-091 {#EU-0021-091}

Each attachment of a report MUST stay off until the person adds it: the full crash dump, a log excerpt, a reduced copy of the project holding the tracks the person chooses, an audio clip, a screenshot and a reply address.

### EU-0021-092 {#EU-0021-092}

Before the person adds the full crash dump, the client MUST warn that it can hold audio and project content from memory.

### EU-0021-093 {#EU-0021-093}

The preview of a report MUST show each part as the outbox shows a batch.

### EU-0021-095 {#EU-0021-095}

The receiving service MUST answer each report with its identity and join it to its finding.

### EU-0021-096 {#EU-0021-096}

A report's description and attachments MUST NOT be opened by anyone other than the maintainers that the project's configuration lists.

### EU-0021-097 {#EU-0021-097}

A maintainer MUST NOT publish any part of a report without the reporter's consent.

### EU-0021-098 {#EU-0021-098}

The person MUST be able to delete a report with a signed deletion request.
