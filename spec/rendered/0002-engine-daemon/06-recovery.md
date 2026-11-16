## Session host failure and recovery {#recovery}

In this version, a failure of the session host ends the session's playback and take rather than
continuing them. The audio device process learns of the loss, fades its output out and closes the
take file, as the RFC of the audio device states. The host service then starts the session host
again, which restores the committed project state from its journal and adopts the closed take. A
new audio device process starts in a new generation, and accepts commands only from the generation
of the session host that started it. Continuing playback and a take through a failure of the session
host needs a recovery handshake between a surviving audio device process and a new session host,
and is left to a later version.

### EU-0002-069 {#EU-0002-069}

After a session host fails, the host service MUST start the session host again.

### EU-0002-070 {#EU-0002-070}

A restarted session host MUST restore the committed project state from its journal.

### EU-0002-071 {#EU-0002-071}

A restarted session host MUST adopt the take its failure closed exactly once, by the identity of the take.

### EU-0002-072 {#EU-0002-072}

A restarted session host MUST start the audio device process in a new generation.

### EU-0002-073 {#EU-0002-073}

A restarted session host MUST NOT run an external effect of a journaled command again.

### EU-0002-074 {#EU-0002-074}

A restarted session host MUST NOT restart a play or record action.

### EU-0002-075 {#EU-0002-075}

After recovery, the transport MUST stay stopped until a person or a client starts it.

### EU-0002-076 {#EU-0002-076}

A command whose commit was journaled before the failure MUST be part of the restored state, whether or not it was acknowledged.

### EU-0002-077 {#EU-0002-077}

A command that was not journaled before the failure MUST NOT be part of the restored state.

### EU-0002-078 {#EU-0002-078}

On reattachment, the session host MUST answer the outcome of each command a client sent before the failure by its idempotency key.

### EU-0002-079 {#EU-0002-079}

Until the restarted session host has restored the session, each attached client MUST show the session as stopped and recovering.
