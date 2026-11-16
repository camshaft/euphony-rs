## Revisions, branches and reads {#revisions}

Each committed batch makes a revision. A revision names the revisions it follows, its parents, so the
revisions of a project form a graph without a cycle. A branch is a name that points at one revision,
its head. Each copy holds its working branches and is the only copy that commits to them, so a
musician edits their branch offline, and the loss of one copy leaves every other copy free to edit.
How copies exchange revisions and merge branches is the subject of the sync and collaboration RFC. A
read of a project answers with a view of one revision and names that revision. Each answer holds at
most one page: a longer answer names the token that reads its next page, each page shows that same
revision, and reading one page costs no more in a project with a long history than in a new one. A
subscriber to the events of a project holds a cursor that a later subscription resumes from. euphony
answers a read or a subscription only for a caller who may read the project, and refuses any other
as denied. It refuses a read or a subscription of a project that doesn't exist in the same way, so
that a refusal doesn't tell a caller whether the project exists.

### EU-0004-032 {#EU-0004-032}

A revision MUST NOT change once it is committed.

### EU-0004-033 {#EU-0004-033}

Each revision MUST name each revision it follows.

### EU-0004-034 {#EU-0004-034}

The revisions of a project MUST NOT form a cycle.

### EU-0004-035 {#EU-0004-035}

Each revision MUST carry a signature by which any copy verifies which copy committed it.

### EU-0004-036 {#EU-0004-036}

A copy MUST NOT accept a revision whose signature it cannot verify.

### EU-0004-037 {#EU-0004-037}

Each branch MUST point at exactly one revision.

### EU-0004-038 {#EU-0004-038}

A copy MUST NOT commit to a working branch that another copy holds.

### EU-0004-055 {#EU-0004-055}

A copy MUST NOT accept a revision on a working branch unless the copy that holds that branch signed it.

### EU-0004-039 {#EU-0004-039}

Committing a batch MUST NOT need any other copy of the project to be reachable.

### EU-0004-040 {#EU-0004-040}

The loss of one copy MUST NOT stop another copy from committing to its branches.

### EU-0004-041 {#EU-0004-041}

Each answer to a read of a project MUST name the revision it shows.

### EU-0004-042 {#EU-0004-042}

Each page of a paged answer MUST show the revision that its first page shows.

### EU-0004-043 {#EU-0004-043}

A subscription resumed from a cursor MUST deliver every event after that cursor.

### EU-0004-044 {#EU-0004-044}

A subscription resumed from a cursor MUST NOT deliver an event at or before that cursor.

### EU-0004-045 {#EU-0004-045}

A subscriber MUST learn of each committed revision of the project without asking for it.

### EU-0004-052 {#EU-0004-052}

Each answer to a read of a project MUST hold at most one page.

### EU-0004-053 {#EU-0004-053}

An answer to a read that does not hold its last page MUST name the token that reads its next page.

### EU-0004-054 {#EU-0004-054}

The cost of reading one page MUST NOT grow with the length of the history or the size of the project.

### EU-0004-050 {#EU-0004-050}

euphony MUST refuse as denied each read of a project, of its history or of its events, and each subscription to its events, from a caller who may not read the project.

### EU-0004-057 {#EU-0004-057}

euphony MUST refuse as denied each read of a project, of its history or of its events, and each subscription to its events, that names a project that does not exist, with an answer that does not differ from its refusal of a caller who may not read an existing project.
