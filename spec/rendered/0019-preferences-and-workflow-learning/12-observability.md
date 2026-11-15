## Events, metrics and conformance {#observability}

Euphony publishes events for evidence capture and omission, proposal state, context freshness,
activation and refusal, invocation, suspension, retirement, synchronization conflicts and
retrospective completion. It measures the size and time of context reads, queue delay, correction
outcomes and resource cost as demand grows. A learning profile is a supported combination of
preference domains, protocol capabilities and execution profiles, with the conformance, privacy
and rollback results of the exact build that supports it. Conformance fixtures use synthetic
subjects and recordings.

### EU-0019-187 {#EU-0019-187}

Euphony MUST publish an event for each evidence capture or omission, proposal state change, context freshness change, activation or refusal, invocation, suspension, retirement, synchronization conflict and retrospective completion.

### EU-0019-188 {#EU-0019-188}

A learning metric MUST use closed categories of reason, source and status.

### EU-0019-189 {#EU-0019-189}

A learning metric MUST NOT carry personal text or an identity as a label.

### EU-0019-190 {#EU-0019-190}

Euphony MUST NOT set a fixed limit on the number of preferences, collaborators or proposals.

### EU-0019-191 {#EU-0019-191}

Euphony MUST schedule the work of independent learning scopes concurrently.

### EU-0019-192 {#EU-0019-192}

A new preference domain, protocol capability or execution profile MUST have versioned conformance evidence before it is activated.

### EU-0019-193 {#EU-0019-193}

A release MUST refuse a supported learning profile that lacks conformance, privacy and rollback results for its exact build, or whose results fail.

### EU-0019-194 {#EU-0019-194}

A conformance fixture MUST NOT hold private client names, recordings or notes.
