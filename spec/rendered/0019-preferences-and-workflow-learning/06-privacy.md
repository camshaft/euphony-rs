## Privacy and disclosure to assistants {#privacy}

The learning store is local to the owner's devices by default, and the credentials it uses stay in
the platform's secure storage. Personal notes are kept apart from project media and from
collaborators' context. Serving private data to an assistant on the same computer can still send
that data to a remote model, since a local connection doesn't prove local inference. A grant of
remote inference over private data classes holds only while a verification record covers the
destination and the version of its retention and training terms; the project's configuration says
who publishes verification records. The verification shows that the consent had its prerequisite;
it doesn't show what the provider does after disclosure. Synchronization to the owner's own paired
devices isn't a disclosure.

### EU-0019-090 {#EU-0019-090}

By default, the learning store and its evidence MUST stay local to the owner's devices.

### EU-0019-091 {#EU-0019-091}

Euphony MUST keep the credentials of the learning store in the platform's secure storage.

### EU-0019-092 {#EU-0019-092}

Personal notes MUST be kept apart from project media and from collaborators' context.

### EU-0019-093 {#EU-0019-093}

The approval that grants an assistant private data MUST show the client and the data classes it allows.

### EU-0019-094 {#EU-0019-094}

The approval that grants an assistant private data MUST warn that a local connection doesn't prove local inference.

### EU-0019-204 {#EU-0019-204}

The approval that grants an assistant private data MUST warn that data euphony releases to an assistant can't be recalled.

### EU-0019-095 {#EU-0019-095}

In private-only operation, private data MUST stay unavailable to an assistant that the owner didn't select as a trusted local assistant whose outbound connections the machine restricts.

### EU-0019-096 {#EU-0019-096}

Remote inference over private data MUST wait for the owner's approval of the exact effect.

### EU-0019-195 {#EU-0019-195}

An assistant's access to private data MUST wait for the owner's approval of the exact effect.

### EU-0019-097 {#EU-0019-097}

Synchronization to the owner's paired devices MUST NOT need the owner's approval of the exact effect.

### EU-0019-098 {#EU-0019-098}

Euphony MUST NOT infer consent from the acceptance of a suggestion.

### EU-0019-099 {#EU-0019-099}

A grant of remote inference over private data classes MUST be refused unless a passing verification record binds its destination, the current version of its retention and training terms, and the grant's subject and project scope.

### EU-0019-199 {#EU-0019-199}

A verification record MUST count toward a grant of remote inference only where the publisher that the project's configuration names wrote it.

### EU-0019-205 {#EU-0019-205}

Euphony MUST NOT accept a verification record that a writer other than the configured publisher wrote.

### EU-0019-100 {#EU-0019-100}

The approval view MUST show the verification record and its scope before the owner consents.

### EU-0019-101 {#EU-0019-101}

Euphony MUST check a grant's verification again at its commit and at each release of private data.

### EU-0019-102 {#EU-0019-102}

A changed or unknown version of the terms MUST suspend the grant until a matching verification and a renewed approval exist.
