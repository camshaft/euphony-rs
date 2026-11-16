## Storing each item once {#deduplication}

Deduplication is by whole item. An item whose identity a computer already holds isn't stored again
on it, and the new reference points at the existing item. Two computers exchange identities and
send only the items the receiver lacks, and of an item it holds in part, only the missing chunks. A
storage computer holds each item once for every project and every working computer it serves. The
saving is largest for a sample pack reused across projects: n projects that use a pack store it
once instead of n times. Two takes don't share samples, and a render after an edit shares nothing with
the render before it, so each stays its own item; an export, a freeze or consolidated audio with no
transform applied equals its source and is stored once. Note sequences, MIDI and automation are
small, so their identity matters for finding where a clip is used more than for disk space. The
library scan reads an existing library without changing it, and reports the duplicate bytes it
finds by item kind and by the point that would save them, so the owner sees the measured saving on
their own library before migrating anything.

### EU-0008-061 {#EU-0008-061}

A computer MUST store each item at most once, however many projects refer to it.

### EU-0008-062 {#EU-0008-062}

Publishing an item whose identity a computer already holds MUST add a reference to the existing item instead of a second copy.

### EU-0008-063 {#EU-0008-063}

A transfer MUST send only the items, and of a partly held item only the chunks, that the receiver lacks.

### EU-0008-064 {#EU-0008-064}

The library scan MUST NOT change the library it reads.

### EU-0008-065 {#EU-0008-065}

The library scan MUST report the duplicate bytes it finds by item kind and by the point at which storing each item once would save them.

### EU-0008-066 {#EU-0008-066}

The owner SHOULD see the measured saving of storing each item once before any content of their library moves into the store.
