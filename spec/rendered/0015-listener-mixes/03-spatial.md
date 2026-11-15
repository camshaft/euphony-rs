## Spatial mixing {#spatial}

A speaker bed is a set of channels meant for fixed speaker positions. A sound object is a sound with
its own identity, source, position, trajectory, spread, gain and spatial automation, placed on the
project's sample timeline. The project declares the coordinate system, the units and the
interpolation of its spatial data. A spatial renderer maps the beds and the objects to an explicit
speaker layout or to a headphone rendering profile; the layout a listener plays is a property of
their sink, and rendering for it leaves the objects as they are. A headphone render and a speaker
render are different profiles, and neither is a bit-exact copy of the other. Prepared playback holds
the spatial render its sink selects, and a live object plays on the local live path with its measured
latency. Source material rendered once serves stereo, spatial and listener outputs alike, and a
change of layout prepares again only the audio that depends on that layout.

### EU-0015-022 {#EU-0015-022}

The project MUST hold speaker beds and independently positioned sound objects.

### EU-0015-023 {#EU-0015-023}

The identity, source, position, trajectory, spread, gain and spatial automation of each sound object MUST be versioned data of the project, placed on its sample timeline.

### EU-0015-024 {#EU-0015-024}

The project MUST declare the coordinate system, the units and the interpolation of its spatial data.

### EU-0015-025 {#EU-0015-025}

Rendering a spatial mix to a speaker layout or a headphone profile MUST NOT change its beds or objects.

### EU-0015-026 {#EU-0015-026}

The playback layout of a spatial mix MUST be a property of the sink that plays it.

### EU-0015-027 {#EU-0015-027}

The metadata of each sound object MUST be preserved through collaboration, history and the preparation of source material.

### EU-0015-028 {#EU-0015-028}

A change of a sink's layout MUST NOT cause a new rendering of audio that doesn't depend on that layout.

### EU-0015-029 {#EU-0015-029}

A spatial render MUST keep the motion of each sound object continuous.

### EU-0015-030 {#EU-0015-030}

A spatial render MUST apply spatial automation at the sample time the project holds for it.

### EU-0015-031 {#EU-0015-031}

A spatial render MUST map each output channel to the speaker its layout declares for that channel.

### EU-0015-032 {#EU-0015-032}

A live sound object MUST play on the local live path.
