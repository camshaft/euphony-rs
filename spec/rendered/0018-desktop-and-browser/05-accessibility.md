## Accessibility and the keyboard {#accessibility}

An interactive element is any control a person can focus or act on, such as a fader, a clip, a
note or a patch point. Its semantics are its role, label, unit, range, base value, effective value,
focus and permitted actions. A long project holds more tracks and clips than a screen shows, and
the ones off screen still exist for navigation and for assistive technology.

### EU-0018-033 {#EU-0018-033}

The desktop interface MUST expose the role, label, unit, range, base value, effective value, focus and permitted actions of each interactive element to the operating system's assistive technology.

### EU-0018-034 {#EU-0018-034}

Every action of the desktop interface MUST be reachable from the keyboard.

### EU-0018-035 {#EU-0018-035}

Each shortcut MUST be discoverable from the interface.

### EU-0018-036 {#EU-0018-036}

Each continuous control MUST offer fine adjustment, coarse adjustment and numeric entry.

### EU-0018-037 {#EU-0018-037}

Patching MUST have a way that doesn't need dragging.

### EU-0018-038 {#EU-0018-038}

A track or clip that is off screen MUST keep a stable identity for assistive technology.

### EU-0018-039 {#EU-0018-039}

A track or clip that is off screen MUST stay reachable by navigation.

### EU-0018-040 {#EU-0018-040}

An action reached through assistive technology MUST use the same action identifier and current selection as the same action reached by pointer.

### EU-0018-041 {#EU-0018-041}

The accessibility of the desktop interface MUST be verified with each supported operating system's screen reader on an actual build.
