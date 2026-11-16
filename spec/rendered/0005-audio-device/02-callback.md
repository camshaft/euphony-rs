## The real-time callback {#callback}

The device asks the audio device process for each period of audio, and the callback fills the
output of the period and takes its input before the deadline of the device. Everything the callback
reads is ready before it runs: the session host delivers buffers whose memory is already allocated,
and each compiled graph revision before the callback plays it. The callback passes buffers and
revisions to and from the session host without either side waiting on the other. Work whose time
has no bound, such as waiting on a lock, a read of storage, a call to another computer, or
allocating or releasing memory, happens outside the callback. The callback counts positions in whole
samples; exact musical time is resolved to samples before a revision reaches it.

### EU-0005-010 {#EU-0005-010}

The callback MUST NOT make a network call.

### EU-0005-011 {#EU-0005-011}

The callback MUST NOT allocate memory.

### EU-0005-012 {#EU-0005-012}

The callback MUST NOT release memory.

### EU-0005-013 {#EU-0005-013}

The callback MUST NOT wait on a lock.

### EU-0005-014 {#EU-0005-014}

The callback MUST NOT read or write storage.

### EU-0005-015 {#EU-0005-015}

The session host MUST deliver each audio buffer it sends to the audio device process with its memory already allocated.

### EU-0005-016 {#EU-0005-016}

The session host MUST deliver each compiled graph revision to the audio device process before the callback plays it.

### EU-0005-017 {#EU-0005-017}

Passing a buffer or a revision between the session host and the callback MUST NOT make either side wait on the other.

### EU-0005-018 {#EU-0005-018}

The callback MUST count positions in whole samples.
