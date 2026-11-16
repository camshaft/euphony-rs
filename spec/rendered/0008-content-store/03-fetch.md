## Fetching content for playback and recording {#fetch}

The audio output reads only buffers that are already local, so a fetch never runs where audio is
produced. When a project opens, the items its revision refers to that are held only on storage
computers are fetched in the order in which playback from the current position reaches them, and
then the rest of the arrangement. Before playback starts, the session plans together the fetches of
every region the playback range reads whose content isn't all local. For each region it takes the
bytes still to fetch and the time playback reaches each of them, from the region's own sample rate,
channel count and position, and plans them against the measured capacity of each disk, network path
and decoder they share, after the capacity live playback already uses. Playback starts where the
plan delivers every range before playback reaches it, with the local buffers as the margin. When a
measured capacity falls during playback, the session plans again. Before playing, the person
chooses what happens where a region will miss its time: wait, holding playback before the region
until the plan admits it, or play on, keeping the audio output, monitoring and recording running
and marking the region's place in the arrangement as a missing passage. Recording doesn't depend on
a fetch: captured input is written locally whatever the store holds, and a region that becomes not
ready during a take keeps the take recording and marks the gap in the monitoring mix. Prefetch
covers the common cases: an open project's items fetch at once, an archived project the owner opens
is recalled to the working tier as a whole, an item placed from the catalog fetches while its
preview shows, and the policy can keep a working copy of each item whose project is open on another
of the owner's computers.

### EU-0008-047 {#EU-0008-047}

The audio output MUST read only content that is already held locally.

### EU-0008-048 {#EU-0008-048}

When a project opens, the store MUST fetch each item the project refers to that is not held locally, in the order in which playback from the current position reaches it.

### EU-0008-049 {#EU-0008-049}

Before playback starts, the session MUST plan the fetches of every region the playback range reads together, against the measured capacity of each disk, network path and decoder they share.

### EU-0008-050 {#EU-0008-050}

Playback MUST start only where the plan delivers every range it reads before playback reaches that range.

### EU-0008-051 {#EU-0008-051}

Where a measured capacity falls during playback, the session MUST plan the fetches again.

### EU-0008-052 {#EU-0008-052}

Where a region will miss its time, the session MUST do what the person chose before playing: wait before the region, or play on.

### EU-0008-053 {#EU-0008-053}

A session that plays on MUST mark each passage whose content has not arrived as missing.

### EU-0008-054 {#EU-0008-054}

A session MUST NOT play silence in place of content that has not arrived as though the content were present.

### EU-0008-055 {#EU-0008-055}

A playback or an export that holds a missing passage MUST NOT be presented as complete.

### EU-0008-056 {#EU-0008-056}

Recording, live monitoring and the audio output MUST continue without interruption while content is fetched.

### EU-0008-057 {#EU-0008-057}

Captured input MUST be written to local durable storage whatever the state of the store.

### EU-0008-058 {#EU-0008-058}

Arming a track to record MUST show which regions of the punch range are not ready before recording starts.

### EU-0008-059 {#EU-0008-059}

An archived project that the owner opens MUST be recalled to the working tier as a whole, with its progress shown before playback.

### EU-0008-060 {#EU-0008-060}

Opening a project held only on storage computers and playing it SHOULD NOT cause a missed audio deadline from waiting for its content.
