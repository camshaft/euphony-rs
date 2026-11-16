## Selecting the supported renderer {#renderer-selection}

A renderer draws the music views. The supported renderer is the one the desktop interface uses on
its supported profiles, and a candidate renderer can run beside it behind a development selection.
A profile is a combination of operating system, display and input device that euphony supports.
The evidence for a renderer is its target conformance on each profile, its surface conformance, the
recorded decision on replacing stock controls with drawn music controls, the provenance and license
records of the build, and the musician's physical acceptance. A conformance record names the
renderer and its version, the panel kind, the profile and the build. Each piece of evidence is a
typed record from a configured publisher or decider. Expressive-control readiness is the claim that
a profile is fast enough for live playing.

### EU-0018-081 {#EU-0018-081}

Selecting the supported renderer MUST take the project, the renderer, the build identity, the supported profiles and the expected configuration revision.

### EU-0018-082 {#EU-0018-082}

The selection MUST require passing target conformance and surface conformance of the selected renderer for the exact build on every supported profile.

### EU-0018-083 {#EU-0018-083}

The selection MUST require the designated approver's recorded decision on replacing stock controls with drawn music controls.

### EU-0018-084 {#EU-0018-084}

The selection MUST require the provenance and license records of the build.

### EU-0018-085 {#EU-0018-085}

The selection MUST require the musician's physical acceptance on every supported profile.

### EU-0018-086 {#EU-0018-086}

A selection with missing, failed, stale or mismatched evidence MUST return its unmet prerequisites.

### EU-0018-087 {#EU-0018-087}

The selection MUST check its evidence and the caller's authority again in the commit that changes the configuration.

### EU-0018-088 {#EU-0018-088}

A change of build, profile or asset MUST invalidate the evidence that applied before it.

### EU-0018-089 {#EU-0018-089}

Evidence for one renderer MUST NOT authorize another renderer.

### EU-0018-090 {#EU-0018-090}

A conformance record without a renderer identity MUST NOT satisfy the selection.

### EU-0018-091 {#EU-0018-091}

The evidence of the selection MUST be typed records from configured publishers and deciders.

### EU-0018-092 {#EU-0018-092}

A rollback of the renderer MUST leave the project's audio and command history unchanged.

### EU-0018-093 {#EU-0018-093}

The desktop interface MUST NOT claim expressive-control readiness on a physical profile that hasn't met the timing criterion the musician selected.

### EU-0018-094 {#EU-0018-094}

A selection with missing, failed, stale or mismatched evidence MUST keep the current renderer.
