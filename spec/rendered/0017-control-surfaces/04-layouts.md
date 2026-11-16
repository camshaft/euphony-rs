## Layouts and panels {#layouts}

A layout arranges panels over the session's existing controls. The panels are the mixer, transport,
the controls of one built-in processor, smart controls, track parameters and the modulation patch
view, whose values RFC 0012 states. A smart control is a declarative macro that maps one input to
parameters it identifies explicitly, each with its unit, curve and legal range. A layout holds view
kinds, the stable identities of its targets, banks, target ranges and mappings; display labels,
screen position and the current selection are presentation, not part of the layout. Each layout is
saved with the project's session configuration, by project, stable surface identity and user, and
a device can keep a default that seeds its layouts in new projects. The session host validates and
saves every layout change, and the desktop interface, the browser companion, every surface and the
assistant interface read the same records. A tablet can hold a mixer, a phone transport and another
screen a processor or a patch view, each independent of the desktop's selection. The browser
companion sends the same commands as every surface and stays narrower than the desktop interface.
A panel is offered on a surface where a conformance record shows that it passes its capability and
accessibility tests for that surface's profile, its renderer and the running build. Applying a layout
returns the committed revision, or denied, stale revision, unknown target, incompatible capability,
invalid mapping or unverified panel. Attaching returns the session and control generations, an
authorized snapshot and a subscription cursor, or a refusal of the enrollment or the protocol.

### EU-0017-041 {#EU-0017-041}

The system MUST save each surface layout with the project's session configuration, by project, surface identity and user.

### EU-0017-042 {#EU-0017-042}

A device MAY keep a default layout that seeds its layouts in new projects.

### EU-0017-043 {#EU-0017-043}

The system MUST store surface layouts, attachments and device bindings encrypted at rest with the project's other data.

### EU-0017-044 {#EU-0017-044}

A store that cannot encrypt surface records MUST NOT persist them.

### EU-0017-045 {#EU-0017-045}

A layout MUST select each of its targets by the target's stable identity.

### EU-0017-046 {#EU-0017-046}

A display label MUST NOT select a layout's target.

### EU-0017-047 {#EU-0017-047}

A layout MUST NOT hold executable code.

### EU-0017-048 {#EU-0017-048}

Importing a layout MUST NOT import credentials, pairings, links, tools or programs.

### EU-0017-049 {#EU-0017-049}

A layout target that no longer exists MUST be shown as unresolved.

### EU-0017-050 {#EU-0017-050}

The system MUST restore a saved layout only after it validates the layout's targets, its schema and the surface's pairing.

### EU-0017-051 {#EU-0017-051}

The system MUST offer mixer, transport, built-in processor, smart control, track parameter and modulation patch panels.

### EU-0017-052 {#EU-0017-052}

A smart control MUST map its input only to parameters it identifies explicitly, each with its unit, curve and legal range.

### EU-0017-053 {#EU-0017-053}

A surface MUST show a smart control's whole target set before the control is committed.

### EU-0017-054 {#EU-0017-054}

A smart control gesture MUST commit to its whole target set atomically.

### EU-0017-055 {#EU-0017-055}

A smart control gesture with any invalid target MUST be denied as a whole.

### EU-0017-056 {#EU-0017-056}

Editing a sound parameter from a surface MUST NOT write its modulated value into its base value.

### EU-0017-057 {#EU-0017-057}

A surface MUST show a parameter's base value, its modulation contributions and its effective value separately.

### EU-0017-058 {#EU-0017-058}

A surface MUST NOT follow another surface's selected track unless its layout is configured to follow it.

### EU-0017-059 {#EU-0017-059}

The browser companion MUST NOT offer offline editing or render on its own.

### EU-0017-060 {#EU-0017-060}

Each touch panel MUST offer keyboard, screen-reader and single-pointer alternatives.

### EU-0017-061 {#EU-0017-061}

Changing a bank or the orientation of a surface MUST NOT retarget an active gesture.

### EU-0017-062 {#EU-0017-062}

The system MUST apply a layout only where each of its panels has a passing capability and accessibility conformance record for the surface's profile, its renderer and the running build.

### EU-0017-063 {#EU-0017-063}

Restoring a saved layout and activating a panel MUST pass the same conformance check as applying a layout.

### EU-0017-064 {#EU-0017-064}

Previewing a layout MUST NOT change any state.

### EU-0017-065 {#EU-0017-065}

Applying a layout MUST be refused where its expected layout revision is not the current one.

### EU-0017-066 {#EU-0017-066}

Attaching a surface MUST return the session and control generations, an authorized snapshot and a subscription cursor, or the reason it was refused.

### EU-0017-067 {#EU-0017-067}

A surface's snapshot and the changes that follow it MUST share one revision boundary.

### EU-0017-068 {#EU-0017-068}

An assistant MAY arrange a surface's layout.

### EU-0017-154 {#EU-0017-154}

The desktop interface, the browser companion, every surface and the assistant interface MUST share one set of layout records.

### EU-0017-155 {#EU-0017-155}

Applying a layout MUST return the committed revision or one of these refusals: denied, stale revision, unknown target, incompatible capability, invalid mapping or unverified panel.

### EU-0017-156 {#EU-0017-156}

A shared layout template MUST NOT carry or grant a pairing.
