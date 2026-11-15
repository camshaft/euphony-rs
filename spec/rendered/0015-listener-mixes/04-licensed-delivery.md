## Licensed spatial delivery formats {#licensed-delivery}

A proprietary licensed spatial delivery format is a format whose renderer, authoring and export
tools, delivery file and validation are licensed by its owner. Delivery in such a format is a
separate integration, and generic spatial rendering, whatever file it writes, isn't that delivery.
The owner of the product decides which licensed deliverables to support and their licensing budget.
Until the license and distribution terms of an integration are known to fit, delivery in that format
stays unavailable, and that part of spatial delivery stays explicitly unresolved. A musician who
owns an external licensed renderer can send the multichannel outputs of a session to it as an
ordinary multichannel sink.

### EU-0015-033 {#EU-0015-033}

The system MUST NOT report a stereo bounce, a generic object file or any other output of generic spatial rendering as delivery in a proprietary licensed spatial format.

### EU-0015-034 {#EU-0015-034}

A public build MUST NOT link or redistribute proprietary code.

### EU-0015-035 {#EU-0015-035}

An integration with a proprietary licensed spatial format MUST be optional.

### EU-0015-036 {#EU-0015-036}

Delivery in a proprietary licensed spatial format MUST stay unavailable until its license and distribution terms are known to fit the build that offers it.

### EU-0015-037 {#EU-0015-037}

An export in a proprietary licensed spatial format MUST NOT be reported as that delivery before it passes the validation its licensor requires.

### EU-0015-038 {#EU-0015-038}

Spatial mixing, object history and public export formats MUST stay usable without any proprietary integration.

### EU-0015-039 {#EU-0015-039}

A musician MAY send the multichannel outputs of a session to an external renderer as an ordinary multichannel sink.

### EU-0015-040 {#EU-0015-040}

The system MUST NOT label a route to an external renderer, or its result, as licensed delivery.
