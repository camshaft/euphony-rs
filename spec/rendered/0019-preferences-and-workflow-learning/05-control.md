## Review, control and explanation {#control}

The owner reviews each proposal: its meaning, its scope, its strength, the evidence for and against
it, the behavior it is expected to change and the exact diff of its version. The owner's accept
command approves it, edits and approves it, or rejects it, and the owner can undo that decision. An
assistant can keep permitted evidence, draft a tentative proposal, prepare a preview and make
low-risk, undoable session edits the owner already authorized, each with its reason and its
reversal, and none of these adopts a lasting preference or mechanism. Each assisted change records
the context it was made under, the preference and mechanism versions that applied, typed links to
its effects and the explanation the assistant gave. The Why view shows those records to a viewer
under the viewer's current permissions. A reset withdraws the preferences of selected scopes and can
be undone; a purge removes retained evidence and exports and can't be undone.

### EU-0019-065 {#EU-0019-065}

Before the owner decides a proposal, euphony MUST show its meaning, scope, strength, supporting and contrary evidence, expected affected behavior and exact version diff.

### EU-0019-066 {#EU-0019-066}

The owner's accept command MUST record approve, edit-and-approve or reject.

### EU-0019-067 {#EU-0019-067}

The owner MUST be able to undo a decision of the accept command.

### EU-0019-068 {#EU-0019-068}

A changed proposal MUST need the owner's approval of the changed version.

### EU-0019-069 {#EU-0019-069}

An assistant MUST NOT approve a proposal.

### EU-0019-070 {#EU-0019-070}

The owner's learning store MUST NOT store a collaborator's taste.

### EU-0019-071 {#EU-0019-071}

An assistant MAY make low-risk, undoable session edits that the owner already authorized.

### EU-0019-072 {#EU-0019-072}

Each action an assistant takes without the owner's acceptance MUST record its reason and its reversal.

### EU-0019-073 {#EU-0019-073}

An action an assistant takes without the owner's acceptance MUST NOT adopt a lasting preference or mechanism.

### EU-0019-074 {#EU-0019-074}

Sharing, external export, disclosure to a provider and purge of learning MUST each wait for the owner's approval of the exact effect.

### EU-0019-075 {#EU-0019-075}

A retrospective or another model's review MUST NOT take the place of the owner's approval.

### EU-0019-076 {#EU-0019-076}

Each assisted change MUST record its context manifest, the preference and mechanism versions that applied, typed links to its effects and the explanation the assistant supplied.

### EU-0019-077 {#EU-0019-077}

The Why view MUST join the records of an assisted change under the viewer's current permissions.

### EU-0019-078 {#EU-0019-078}

The Why view MUST mark a missing explanation or unavailable evidence as missing or unavailable.

### EU-0019-079 {#EU-0019-079}

The Why view MUST NOT present an assistant's explanation as proof of its reasoning.

### EU-0019-080 {#EU-0019-080}

The owner MUST be able to inspect the history of each preference and mechanism.

### EU-0019-202 {#EU-0019-202}

Each read of evidence, of a history or of the Why view MUST return one page and the cursor that reads the next page.

### EU-0019-081 {#EU-0019-081}

The owner MUST be able to withdraw an approval.

### EU-0019-082 {#EU-0019-082}

The owner MUST be able to reset the preferences of selected scopes.

### EU-0019-083 {#EU-0019-083}

A reset MUST commit withdrawal records and invalidate each context handle it affects.

### EU-0019-084 {#EU-0019-084}

The owner MUST be able to export the preferences they can read in a documented, versioned format.

### EU-0019-085 {#EU-0019-085}

A purge MUST remove exactly the retained evidence and exports it names.

### EU-0019-086 {#EU-0019-086}

Before a purge, euphony MUST show what the purge can't recover.

### EU-0019-087 {#EU-0019-087}

A purge MUST be a separate operation from a reset.

### EU-0019-088 {#EU-0019-088}

An imported preference or mechanism MUST arrive as a proposal that keeps its source attribution.

### EU-0019-089 {#EU-0019-089}

An import MUST NOT carry an approval or an access grant.
