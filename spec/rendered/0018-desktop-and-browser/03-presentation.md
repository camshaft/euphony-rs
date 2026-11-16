## Presentation and the audio path {#presentation}

A view is built from a projection of the project that the session host authorized for the client,
with the layout, the display scale and the person's font and accessibility preferences. Input on a
view becomes an intent: a local view action, such as a scroll or a zoom, or a gesture that the
desktop interface sends to the session host as a command. A gesture on a continuous control, such
as a fader, has a begin, updates and an end. A meter shows a snapshot of levels that the audio
render publishes without waiting for any reader. A presentation failure is the loss of a graphics
device, a closed window, a display that becomes unavailable or a renderer that restarts. The
desktop interface reaches the operating system through platform services: menus, file selection,
the clipboard, drag and drop, text input methods, focus and window hosting. Each view has a
generation, and the map from positions on the view to its controls has a revision: a rebuilt view
has a new generation, and a changed layout a new revision.

### EU-0018-014 {#EU-0018-014}

The desktop interface MUST build each view from a projection of the project that the session host authorized for it.

### EU-0018-015 {#EU-0018-015}

The presentation of a view MUST NOT widen what a person or an assistant may read or do.

### EU-0018-016 {#EU-0018-016}

Windows, drawing state and graphics resources MUST NOT be part of project state, sound definitions or render cache keys.

### EU-0018-017 {#EU-0018-017}

Input on a view MUST reach the project only as a command sent to the session host.

### EU-0018-018 {#EU-0018-018}

The audio render MUST NOT wait for a frame, a display or a graphics device.

### EU-0018-019 {#EU-0018-019}

A presentation failure MUST NOT stop a recording.

### EU-0018-020 {#EU-0018-020}

After a presentation failure, the desktop interface MUST rebuild the view from a fresh authorized snapshot.

### EU-0018-021 {#EU-0018-021}

The recovery of a view MUST NOT replay a gesture.

### EU-0018-022 {#EU-0018-022}

The desktop interface MUST report a presentation failure without showing the transport as stopped.

### EU-0018-023 {#EU-0018-023}

A meter MUST show the newest authorized level snapshot with its age.

### EU-0018-024 {#EU-0018-024}

A continuous control MUST show local feedback within the tolerance of the selected profile, distinct from the value the session host acknowledged.

### EU-0018-025 {#EU-0018-025}

A gesture on a continuous control MUST reach the session host as a begin, updates and an end.

### EU-0018-026 {#EU-0018-026}

Coalescing display work MUST NOT drop an accepted automation value or a forced end of a gesture.

### EU-0018-096 {#EU-0018-096}

A file selection MUST NOT grant an assistant access to any file beyond the files selected.

### EU-0018-097 {#EU-0018-097}

Each request to a platform service MUST return a cancellation, a denial, an unsupported result or a validated result.

### EU-0018-098 {#EU-0018-098}

The desktop interface MUST refuse as stale any input made against a view generation or a map revision that is no longer current.

### EU-0018-099 {#EU-0018-099}

The desktop interface MUST refuse as invalid or unsupported any input it can't interpret against the current view.
