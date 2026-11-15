## Access, rights and trust {#trust}

Analysis works under the owner's device trust and the collaboration boundaries the system overview
states. It adds neither permissions on single notes nor an approval for each local analysis, which
can be undone. The owner's paired devices can analyze the owner's material. A collaborator's
boundary of listening, commenting or editing controls their reads of the source, their reads of
results and their changes to the project, and a result derived from a source is withheld wherever
the source is. The assets at risk are unreleased audio, inferred lyrics and tags, model files and
project edits; the ways in are media decoders, model loaders, assistant requests, worker results and
imported annotations. A malformed file or model can exhaust memory or run unsafe operations, and a
hostile result can hold injected instructions or false note events. The rights of each method are
recorded for its exact version, each category apart: the license of its code, the license of its
model weights, the terms of its training data, the license of its evaluation data, its notices, its
patent and codec questions, and any restriction on commercial use or redistribution. A permissive
code license doesn't license downloaded weights, reference songs or generated stems, and the right
to analyze a personal recording isn't a right to publish its source, transcription, lyrics or
stems. Audio and results already disclosed to a collaborator can't be erased remotely; the consent
the collaboration took covers that risk.

### EU-0020-079 {#EU-0020-079}

A derived result MUST NOT reveal material that the boundary of a reader withholds from that reader.

### EU-0020-080 {#EU-0020-080}

The system MUST check the requester's boundary again before it dispatches a job and before it delivers a result.

### EU-0020-081 {#EU-0020-081}

The system MUST NOT reveal to a reader whether a result exists that the reader cannot read.

### EU-0020-082 {#EU-0020-082}

A worker MUST refuse an input or a model that carries an executable payload or an unsupported operation.

### EU-0020-083 {#EU-0020-083}

The system MUST verify the identity of each method and model artifact before a worker loads it.

### EU-0020-084 {#EU-0020-084}

Ordinary transfers and storage of audio and results MUST be encrypted.

### EU-0020-085 {#EU-0020-085}

The system MUST record the rights of the code, the model weights, the training data and the evaluation data of each method for its exact version.

### EU-0020-086 {#EU-0020-086}

A distributed build MUST NOT include a method whose rights record is missing, failed, unknown, incompatible or for another version.

### EU-0020-087 {#EU-0020-087}

The system MUST NOT download a model or send material to an outside provider without the person's explicit choice of that destination and that material.

### EU-0020-088 {#EU-0020-088}

A local analysis MUST NOT count as permission for telemetry or for training.

### EU-0020-089 {#EU-0020-089}

A local analysis MUST NOT ask for an approval beyond the owner's existing device trust and collaboration boundaries.

### EU-0020-090 {#EU-0020-090}

Each public test of analysis MUST use synthetic or explicitly licensed audio.
