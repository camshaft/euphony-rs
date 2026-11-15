## Learning evidence {#evidence}

Learning evidence is an observation about the owner's taste or way of working. Each evidence record
holds a stable identity, a schema version, the subject person, the kind of its author or source,
the project, session and branch revision, the event cursor, the observation time, the command or
suggestion that caused it, the revisions before and after, the assets it refers to, its visibility
policy and the consent it was captured under. Its source is one of: an explicit statement; a
suggestion accepted or rejected; an undo or redo of an assistant's transaction; a later edit by the
person; a choice between two alternatives; and a listening result. Each source carries its own
fields, such as the exact statement and its speaker, or a listening trial's alternatives, revision,
playback quality, level calibration, presentation order, choice and uncertainty. A reversal can be
an experiment, an accident or a timing correction, and accepting a suggestion can reflect a
deadline rather than taste, so evidence is material for a proposal and never a preference by
itself.

### EU-0019-028 {#EU-0019-028}

Each evidence record MUST hold its stable identity, its schema version, its subject person, its author or source kind, its project, session and branch revision, its event cursor, its observation time, its cause, its revisions before and after, its authorized asset references, its visibility policy and its consent provenance.

### EU-0019-029 {#EU-0019-029}

Each evidence record MUST carry the fields of its source kind.

### EU-0019-030 {#EU-0019-030}

An observation time MUST NOT count as a confirmation by the person.

### EU-0019-031 {#EU-0019-031}

Evidence MUST preserve ties, skipped trials, unknown motives and incomplete capture as they were recorded.

### EU-0019-032 {#EU-0019-032}

Euphony MUST derive the evidence of a person's actions in the application from its journal of authorized commands.

### EU-0019-033 {#EU-0019-033}

Submitting evidence MUST carry the expected learning revision and an idempotency key.

### EU-0019-034 {#EU-0019-034}

Euphony MUST derive the author of submitted evidence from the caller's authentication.

### EU-0019-035 {#EU-0019-035}

Euphony MUST refuse submitted evidence whose claimed author or speaker isn't the caller, unless the journal proves that principal's own action or the subject confirmed the exact statement and its attribution.

### EU-0019-036 {#EU-0019-036}

Evidence an assistant reports MUST be stored as agent-reported, with the reporting assistant and its permitted source.

### EU-0019-037 {#EU-0019-037}

A speaker an assistant reports MUST NOT be stored as a confirmed statement of that speaker.

### EU-0019-038 {#EU-0019-038}

Euphony MUST check a confirmation or a journal link for its exact content, subject, scope and current access before it stores the evidence.

### EU-0019-039 {#EU-0019-039}

Submitted evidence whose attribution fails its check MUST be refused as attribution-unverified, without writing any evidence.

### EU-0019-040 {#EU-0019-040}

The owner's capture consent MUST decide which kinds of evidence euphony keeps.

### EU-0019-041 {#EU-0019-041}

By default, euphony MUST NOT record ambient conversation, microphone audio, another application's actions or an assistant's transcript.

### EU-0019-042 {#EU-0019-042}

A person MAY contribute an observation as evidence explicitly.

### EU-0019-197 {#EU-0019-197}

Euphony MUST treat evidence and its text as data, never as an instruction or a command.

### EU-0019-043 {#EU-0019-043}

The learning store MUST NOT infer taste from a count of undos or from the text of evidence.

### EU-0019-044 {#EU-0019-044}

Evidence MUST NOT turn a sealed final evaluation into training material or an approval of a preference.
