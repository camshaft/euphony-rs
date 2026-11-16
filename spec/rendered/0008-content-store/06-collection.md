## Collection and deletion that keep referenced content {#collection}

Collection runs across the owner's computers. Its roots are every retained revision of every project
on any of the owner's computers, pins, working copies, exports, active transfers, recordings in
progress, catalog items the owner marked to keep, and items that a placement in progress refers to.
An item that none of these reach is a candidate for deletion. The last copies of an item are deleted
through a deletion intent for that item, which one computer, the decision point, issues and settles.
The intent names the item, the chunks that only this item holds, and the owner's computers as they
were paired when it was issued. Each computer in that set checks its roots for the item and answers
referenced, which cancels the intent, or unreferenced with a durable barrier. While a computer holds
a barrier, a new reference to the item commits only once the decision point records that the intent
is cancelled; where the deletion is recorded, the reference is refused as unavailable with the
deletion record; and where the decision point can't be reached, the command waits in the project's
queue with its state, where the owner sees it. The decision point records exactly one outcome for
each intent, cancelled or deleted, and records deleted only once every computer in the set has
answered unreferenced and the pairing is unchanged; a pairing or a computer declared lost since the
intent was issued cancels it. Holders remove chunks only on a recorded deleted outcome, and each
computer releases its barrier once the outcome reaches it. After a crash, each party resumes from
its durable intents, barriers and states. Where the owner declares the decision point's computer
lost, recovery settles the intent on its holders under a new epoch, so that a delayed delete from
the lost computer removes nothing, and an intent that recovery can't settle stays unresolved, with
its barriers held, where the owner sees it. A computer that hasn't answered holds back the deletion
of the last copies, and doesn't hold back the release of copies above the count the policy sets. A
rendered result whose inputs are retained can be evicted under storage pressure without asking the
owner. Deleting a source original or a recording takes the owner's approval of a preview that shows
the items, the bytes freed, every revision and project that loses them, the copies that remain
anywhere and whether anything can recreate them. Deleting a project removes its roots, and an item
that another project, an export or a keep mark refers to stays. A collaborator's copies stay theirs,
and the owner's deletion never reaches them.

### EU-0008-094 {#EU-0008-094}

Collection MUST treat as roots every retained revision of every project on any computer of the owner, pins, working copies, exports, active transfers, recordings in progress, keep marks and placements in progress.

### EU-0008-095 {#EU-0008-095}

Collection MUST NOT delete an item that a root on any computer of the owner refers to.

### EU-0008-096 {#EU-0008-096}

The last copies of an item MUST be deleted only through a deletion intent that one decision point settles.

### EU-0008-097 {#EU-0008-097}

Each computer of the owner that was paired when an intent was issued MUST answer the intent as referenced, or as unreferenced with a durable barrier.

### EU-0008-098 {#EU-0008-098}

While a computer holds a barrier for an intent, a new reference to its item MUST NOT commit before the decision point records the intent as cancelled.

### EU-0008-099 {#EU-0008-099}

A new reference to an item whose deletion is recorded MUST be refused as unavailable, with the deletion record.

### EU-0008-100 {#EU-0008-100}

A new reference that waits for a decision point that cannot be reached MUST wait in the queue of its project with its state shown to the owner.

### EU-0008-101 {#EU-0008-101}

The decision point MUST record exactly one outcome for each intent, cancelled or deleted, which never changes afterwards.

### EU-0008-102 {#EU-0008-102}

The decision point MUST NOT record an intent as deleted before every computer it was issued to has answered unreferenced with a barrier while the pairing is unchanged.

### EU-0008-103 {#EU-0008-103}

A holder MUST NOT remove a chunk of an intent before the deleted outcome of that intent is recorded.

### EU-0008-104 {#EU-0008-104}

Where the computer of a decision point is declared lost, each holder of its intent MUST refuse every later message of the lost decision point for that intent.

### EU-0008-105 {#EU-0008-105}

An intent that recovery cannot settle MUST keep its barriers and be shown to the owner as an unresolved deletion.

### EU-0008-106 {#EU-0008-106}

A computer that has not answered an intent MUST NOT hold back the release of copies above the count that the policy sets.

### EU-0008-107 {#EU-0008-107}

Deleting a source original or a recording MUST wait for the approval of the owner of a preview that shows the items, the bytes freed, the revisions and projects that lose them, the copies that remain and whether anything can recreate them.

### EU-0008-108 {#EU-0008-108}

A rendered result whose inputs are retained MAY be evicted under storage pressure without the approval of the owner.

### EU-0008-109 {#EU-0008-109}

Deleting a project MUST NOT delete an item that another project, an export or a keep mark refers to.

### EU-0008-110 {#EU-0008-110}

A deletion by the owner MUST NOT delete a copy that a collaborator holds.

### EU-0008-139 {#EU-0008-139}

Each computer MUST release its barrier for an intent once the recorded outcome of that intent reaches it.
