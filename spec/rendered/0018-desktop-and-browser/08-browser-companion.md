## The browser companion {#browser-companion}

The browser companion is the interface euphony serves to a web browser. It connects to an
authorized desktop session through the approved control path, inspects project revisions, sends
commands that the native host runs, and plays prepared audio that it downloads. Prepared audio is
audio the native hosts rendered already, listed in a pinned manifest. A browser can deny a
permission, require a person's gesture before it plays audio, evict stored data, change its audio
device and suspend a page in the background. Local network discovery, multicast audio and network
clock synchronization stay with the native hosts. Narrowing the browser leaves distributed
rendering on native workers and collaboration between native hosts as they are.

### EU-0018-058 {#EU-0018-058}

The browser companion MUST control an authorized desktop session through the client protocol.

### EU-0018-059 {#EU-0018-059}

Each command the browser companion sends MUST run on the authorized native host.

### EU-0018-060 {#EU-0018-060}

The browser companion MUST let a person inspect the revisions of a project.

### EU-0018-061 {#EU-0018-061}

The browser companion MUST play downloaded prepared audio that a pinned manifest lists.

### EU-0018-062 {#EU-0018-062}

The browser companion MUST verify all audio a manifest requires before it reports playback as ready.

### EU-0018-063 {#EU-0018-063}

The browser companion MUST report each capability it lacks as unavailable.

### EU-0018-064 {#EU-0018-064}

The browser companion MUST NOT edit a project or run a composition program offline.

### EU-0018-065 {#EU-0018-065}

The browser companion MUST NOT record audio locally.

### EU-0018-066 {#EU-0018-066}

The browser companion MUST NOT offer itself as a rendering worker.

### EU-0018-067 {#EU-0018-067}

When the host is lost, the browser companion MUST keep downloaded prepared audio playable.

### EU-0018-068 {#EU-0018-068}

When the host is lost, the browser companion MUST show remote commands as unavailable.

### EU-0018-069 {#EU-0018-069}

The browser companion MUST report to the person a denied permission, a required gesture to start audio, evicted storage, a changed device and a background suspension.

### EU-0018-070 {#EU-0018-070}

The prepared player MUST play when memory shared between threads is unavailable.

### EU-0018-071 {#EU-0018-071}

The browser companion MUST NOT claim direct access to local network discovery, multicast audio or network clock synchronization.

### EU-0018-072 {#EU-0018-072}

The controls of the browser companion MUST be accessible to the browser's assistive technology.
