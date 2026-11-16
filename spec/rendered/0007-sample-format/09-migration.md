## Decoder migration and retirement {#migration}

A store can hold objects written for an earlier decoder, such as an earlier integer working cache.
A new decoder and writer are added beside the old ones, new manifests are published atomically,
and old pins stay until their verified replacements exist. Exactly representable integer originals
convert once, and earlier quantized prepared results are rendered again from their original inputs
where those exist, since a conversion can't restore lost precision. The store keeps typed codec
dependencies for retained objects, revision pins and migration and rollback manifests, and decides
publication and decoder retirement in one atomic step against each other. This is a guarantee of
the local store, not a claim that offline peers have migrated. An import, an archive or a peer
exchange that waits for a decoder or a version is held with its bytes. The registration of what it
waits for marks every such hold ready in one atomic step of a fixed size, without any other prompt,
and each ready item is then read a page at a time and negotiated, verified and published as
durable work of its own, which survives an interruption; a decoder isn't retired while a ready item
depends on it.

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

Migration MUST verify the replacement bytes before it releases an old dependency.

### EU-0007-127 {#EU-0007-127}

Migration MUST update the manifests atomically before it releases an old dependency.

### EU-0007-103 {#EU-0007-103}

Where the inventory of dependencies is uncertain or incomplete, the store MUST keep the decoder available.

### EU-0007-104 {#EU-0007-104}

An import or a restored archive without a verified decoder MUST stay unavailable with its bytes kept.

### EU-0007-105 {#EU-0007-105}

Each held import, archive or peer exchange MUST record the decoder or the schema or profile version it waits for.

### EU-0007-106 {#EU-0007-106}

The registration of a verified decoder or a supported version MUST mark each held item that waits for it ready, in an atomic step that makes a fixed number of store writes however many items wait.

### EU-0007-132 {#EU-0007-132}

The atomic step of a registration MUST NOT negotiate, verify or publish a held item.

### EU-0007-138 {#EU-0007-138}

The store MUST NOT retire a decoder while a ready item depends on it.

### EU-0007-133 {#EU-0007-133}

The store MUST negotiate and verify each ready item as durable work of its own, started by the registration that marks it ready.

### EU-0007-137 {#EU-0007-137}

The store MUST read the ready items a page at a time.

### EU-0007-107 {#EU-0007-107}

A ready item that passes verification MUST be published in an atomic step of its own.

### EU-0007-134 {#EU-0007-134}

A ready item MUST stay recorded as ready until it is published or fails verification.

### EU-0007-135 {#EU-0007-135}

When the store reopens, it MUST resume the work of each ready item that is not yet published.

### EU-0007-136 {#EU-0007-136}

A timer MUST NOT start the work of a ready item.

### EU-0007-108 {#EU-0007-108}

A ready item that fails verification MUST stay held, with its bytes kept and its failure recorded against that registration, until the next registration of what it waits for marks it ready again.

### EU-0007-109 {#EU-0007-109}

A release that removes a decoder MUST NOT be published without passing migration, interruption, rollback and old-revision tests for that exact commit.
