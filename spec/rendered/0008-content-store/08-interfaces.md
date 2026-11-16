## Interfaces and typed records {#interfaces}

The store offers these operations: publish an item from chunks, with its kind and provenance;
resolve a reference to its chunk list and availability; fetch a frame range with a readiness
deadline; list the holders and receipts of an item; verify a holder's copy; and publish or cancel a
deletion intent. Its errors are absent, pruned with its retention record, corrupt, not ready with
its measured readiness, under-replicated, denied, unsupported codec and unavailable storage.
Placement offers these operations: read and propose the signed policy, which the owner approves;
read the planner's pending actions and the items below their policy; and declare a computer lost.
Its errors are invalid rule, conflicting rules and unknown computer. The catalog offers search, get,
list uses, set metadata, set license, preview and place, keep and release a keep mark, and
subscribe. Its errors are not ready, unavailable, invalid query, unsupported model version and
conflict. Every relation is typed data: references from revisions, uses, chunk lists, receipts,
reservations, intents, barriers, outcomes, provenance and license records. The project's authority
writes references and uses; publishing computers sign chunk lists; holders write receipts and
reservations; each computer writes its own barriers; the decision point writes an intent's outcome;
and the owner writes the policy, keep marks and licenses. The owner's assistants, which act as the
owner, reach the placement, catalog and deletion operations over content they can already read and
place; holders, which are the owner's paired computers, reach the storage operations; and
collaborators reach none of them.

### EU-0008-119 {#EU-0008-119}

Each operation of the store, of placement and of the catalog MUST answer a failure with one error of its typed set.

### EU-0008-120 {#EU-0008-120}

A fetch of a frame range MUST answer not ready, with its measured readiness, where the range cannot arrive before its deadline.

### EU-0008-121 {#EU-0008-121}

Every relation between items, revisions, copies and deletion decisions MUST be held as typed data.

### EU-0008-122 {#EU-0008-122}

Each kind of record of the store MUST be written only by the party that owns that kind: the project authority, a publishing computer, a holder, a computer for its own barriers, the decision point or the owner.

### EU-0008-123 {#EU-0008-123}

An assistant of the owner MUST NOT reach a placement, catalog or deletion operation over content that it may not already read.

### EU-0008-124 {#EU-0008-124}

A collaborator MUST NOT reach any operation of the store, of placement or of the catalog over the content of the owner.
