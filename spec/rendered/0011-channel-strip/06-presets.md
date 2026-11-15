## Strip presets and learning {#presets}

A strip preset is a versioned record of a strip: its name, role and genre tags, character, layout
version, each stage's on state and processor version, its sends, and its provenance, which is built
in, the musician's own, learned or imported. Each stage of a preset is adaptive or fixed. An adaptive
stage holds targets and offsets relative to the starting point, so applying it to another track dials
that stage in from the track's own audio; a fixed stage holds exact values, for a sound to reproduce
exactly. Presets are items of the catalog that RFC 0008 specifies. The offsets the musician keeps are
evidence for the preference learning of RFC 0019, which proposes preferences and learned presets that
the musician approves, edits or rejects.

### EU-0011-129 {#EU-0011-129}

The musician MUST be able to save a track's strip as a strip preset.

### EU-0011-130 {#EU-0011-130}

A strip preset MUST record its name, role and genre tags, character, strip layout version, each stage's on state and processor version, its sends and its provenance.

### EU-0011-131 {#EU-0011-131}

The provenance of a strip preset MUST be one of built in, the musician's own, learned and imported.

### EU-0011-132 {#EU-0011-132}

Each stage of a strip preset MUST be adaptive or fixed.

### EU-0011-133 {#EU-0011-133}

Applying a preset's adaptive stage MUST dial that stage in from the new track's own audio with the preset's targets and offsets.

### EU-0011-134 {#EU-0011-134}

Applying a preset's fixed stage MUST set the stage's exact parameter values.

### EU-0011-135 {#EU-0011-135}

Applying a preset's fixed stage MUST level-match it unless the musician turns level matching off for that apply.

### EU-0011-136 {#EU-0011-136}

The musician MUST be able to place a strip preset on any track of any project.

### EU-0011-137 {#EU-0011-137}

Strip presets MUST be items of the catalog, found by role, genre, character and provenance.

### EU-0011-138 {#EU-0011-138}

The musician's strip presets MUST sync to their paired devices.

### EU-0011-139 {#EU-0011-139}

The project SHOULD release original built-in presets for each role and for several genres.

### EU-0011-140 {#EU-0011-140}

Each offset the musician keeps after a starting point MUST be recorded as evidence of the musician's adjustment, linked to the starting point's inputs.

### EU-0011-141 {#EU-0011-141}

A preference MUST NOT enter the starting-point calculation before the musician approves it.

### EU-0011-142 {#EU-0011-142}

A preference MUST NOT be learned by counting the musician's adjustments.

### EU-0011-143 {#EU-0011-143}

A learned strip preset MUST NOT appear among the musician's presets before the musician approves it.

### EU-0011-144 {#EU-0011-144}

A proposal of a learned strip preset MUST show the preset and the evidence it was learned from.

### EU-0011-145 {#EU-0011-145}

A learned strip preset MUST NOT be shared with a collaborator without the musician's approval.
