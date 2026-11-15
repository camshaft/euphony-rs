## Decoder migration and retirement {#migration}

A store can hold objects written for an earlier decoder, such as an earlier integer working cache.
A new decoder and writer are added beside the old ones, new manifests are published atomically,
and old pins stay until their verified replacements exist. Exactly representable integer originals
convert once, and earlier quantized prepared results are rendered again from their original inputs
where those exist, since a conversion can't restore lost precision. The store keeps typed codec
dependencies for retained objects, revision pins and migration and rollback manifests, and decides
publication and decoder retirement in one atomic step against each other. This is a guarantee of
the local store, not a claim that offline peers have migrated. An import, an archive or a peer
exchange that waits for a decoder or a version is held with its bytes, and the registration of what
it waits for runs it again, without any other prompt.

### EU-0007-094 {#EU-0007-094}

Adding a decoder or a writer MUST NOT overwrite existing objects.

### EU-0007-095 {#EU-0007-095}

The store MUST publish each new manifest atomically.

### EU-0007-096 {#EU-0007-096}

The store MUST keep each old pin until a verified replacement of it exists.

### EU-0007-097 {#EU-0007-097}

An earlier quantized prepared result MUST be rendered again from its original inputs where they exist, instead of being converted.

### EU-0007-098 {#EU-0007-098}

The store MUST keep a typed codec dependency for each retained object, revision pin and migration or rollback manifest.

### EU-0007-099 {#EU-0007-099}

The store MUST NOT retire a decoder while a retained dependency requires it.

### EU-0007-100 {#EU-0007-100}

The store MUST NOT publish an object or a pin that references a retired decoder.

### EU-0007-101 {#EU-0007-101}

Publication of an object or a pin and retirement of a decoder MUST be decided atomically against each other.

### EU-0007-102 {#EU-0007-102}

Migration MUST verify the replacement bytes and update the manifests atomically before it releases an old dependency.

### EU-0007-103 {#EU-0007-103}

Where the inventory of dependencies is uncertain or incomplete, the store MUST keep the decoder available.

### EU-0007-104 {#EU-0007-104}

An import or a restored archive without a verified decoder MUST stay unavailable with its bytes kept.

### EU-0007-105 {#EU-0007-105}

Each held import, archive or peer exchange MUST record the decoder or the schema or profile version it waits for.

### EU-0007-106 {#EU-0007-106}

The registration of a verified decoder or a supported version MUST negotiate again, in the same atomic step, each held item that waits for it.

### EU-0007-107 {#EU-0007-107}

A held item that passes verification after a registration MUST be published.

### EU-0007-108 {#EU-0007-108}

A held item that fails verification MUST stay held, with its bytes kept and its failure recorded.

### EU-0007-109 {#EU-0007-109}

A release that removes a decoder MUST NOT be published without passing migration, interruption, rollback and old-revision tests for that exact commit.
