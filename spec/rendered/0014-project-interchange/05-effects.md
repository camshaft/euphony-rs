## Effect settings {#effects}

An effect of a source project carries over to a built-in effect where a mapping covers it. A
mapping is versioned data: it identifies the source effect and the version of its settings, selects
the built-in effect that replaces it, and converts each control's value and automation exactly, by
what the control is and not by how it looks. Some effects store settings that only the effect itself
can read; the import never runs such an effect, so it keeps the effect's identity and settings as
they were and reports it as not converted. A control converts exactly only where its position, unit
and range at the source project's format version are known.

### EU-0014-050 {#EU-0014-050}

The import MUST identify each effect of a source project by its vendor, its identifier and the version of its settings schema.

### EU-0014-051 {#EU-0014-051}

A mapping MUST name the version of the built-in effect that replaces each source effect it covers.

### EU-0014-052 {#EU-0014-052}

A mapping MUST convert each control by its identity, unit, range, enumeration, normalization and conversion curve.

### EU-0014-053 {#EU-0014-053}

A mapping MUST NOT match a control by its display label or its knob position.

### EU-0014-054 {#EU-0014-054}

The import MUST carry each mapped control's automation, with its event positions and interpolation, to the built-in replacement.

### EU-0014-055 {#EU-0014-055}

The import MUST check every control of a source effect against its mapping.

### EU-0014-056 {#EU-0014-056}

The import MUST report each control value that its mapping leaves missing or ambiguous.

### EU-0014-057 {#EU-0014-057}

The import MUST keep the raw settings of each source effect for a later import.

### EU-0014-058 {#EU-0014-058}

The import MUST NOT run a source effect to read its settings.

### EU-0014-059 {#EU-0014-059}

An effect whose settings can't be read without running it MUST import as its identity, slot position and preset name, with its settings kept unchanged.

### EU-0014-060 {#EU-0014-060}

The import MUST report as unmapped each control of a source effect whose position, unit and range it doesn't know at the source project's format version.

### EU-0014-061 {#EU-0014-061}

The import MUST NOT put a default value in place of a setting it can't read.

### EU-0014-062 {#EU-0014-062}

Each mapping MUST carry a version.
