## Shortcuts and collisions {#shortcuts}

A shortcut mapping binds keys to actions for a project, for each platform profile. A platform
profile states the shortcuts its operating system reserves and the bindings that exist already
there. A collision is a binding of the mapping that a profile reserves or binds already. A
resolution remaps or disables the conflicting binding of the application. An imported mapping is
a file that goes through the normal checked import.

### EU-0018-042 {#EU-0018-042}

Musical shortcut mappings MUST be configurable.

### EU-0018-043 {#EU-0018-043}

The desktop interface MUST display the effective binding of each shortcut.

### EU-0018-044 {#EU-0018-044}

Saving a shortcut mapping MUST check it, at its expected revision, against the reserved and existing bindings of each supported platform profile.

### EU-0018-045 {#EU-0018-045}

A shortcut mapping MUST be saved whole or not at all.

### EU-0018-046 {#EU-0018-046}

A save with an unresolved collision MUST be refused with each collision and its platform.

### EU-0018-047 {#EU-0018-047}

A resolution MUST NOT override a shortcut the operating system reserves.

### EU-0018-048 {#EU-0018-048}

A save for a platform whose binding data is missing MUST report that platform as unverified.

### EU-0018-049 {#EU-0018-049}

Activating or importing a shortcut mapping MUST repeat its checks.

### EU-0018-050 {#EU-0018-050}

A binding that a changed platform profile leaves unresolved MUST NOT become active.

### EU-0018-051 {#EU-0018-051}

The desktop interface and an assistant MUST change a shortcut mapping through the same command and its revision checks.
