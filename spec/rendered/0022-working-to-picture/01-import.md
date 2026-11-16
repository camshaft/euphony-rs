## Importing a video {#import}

A video is a file that a film or video editor delivers, holding a picture stream and any number of
audio streams. Importing it adds the file to the project's content store, as RFC 0008 states for
every item, and adds a video record to the project. The video record holds the file's content
identity, its container, and each stream's codec, profile, frame size and scan; its frame rate as
an exact ratio, such as 24000/1001 for 23.976 frames per second; its starting timecode; its start
offset, the project position of its first shown frame; and its audio streams, each with its channel
layout. From the file, import derives a frame index, which holds the presentation time, the key
frame flag and the byte position of each frame; thumbnails for the video's track at several zoom
levels; a scrub copy, in which every frame is encoded on its own at a reduced size so that any frame
decodes without its neighbors; and the analysis results that the scoring tools and post-production
sections use. Each computer reads the formats its own decoders read. Euphony doesn't edit, color or
re-encode a picture: its only change to a video is a copy with the mix's sound added.

### EU-0022-001 {#EU-0022-001}

A person MUST be able to import a video in any container and codec that a decoder of the computer showing it reads.

### EU-0022-002 {#EU-0022-002}

Import MUST NOT change or re-encode the video file it stores.

### EU-0022-003 {#EU-0022-003}

The video record MUST hold the frame rate as an exact ratio of two whole numbers.

### EU-0022-004 {#EU-0022-004}

The video record MUST hold the starting timecode that the file holds, or that the person enters where the file holds none.

### EU-0022-005 {#EU-0022-005}

The video record MUST hold its start offset, the project position of its first shown frame.

### EU-0022-006 {#EU-0022-006}

Each audio stream of an imported video MUST become a region on a track of its own, placed by the same timing as the picture.

### EU-0022-007 {#EU-0022-007}

Audio decoded from a lossy compressed stream MUST be marked as coming from a lossy source.

### EU-0022-008 {#EU-0022-008}

Import MUST refuse, with the reason, a stream whose presentation times are missing or go backward.

### EU-0022-009 {#EU-0022-009}

Each computer MUST report the video formats that its decoders read.

### EU-0022-010 {#EU-0022-010}

A video in a format that no decoder of a computer reads MUST show as unavailable on that computer, with the reason.

### EU-0022-011 {#EU-0022-011}

A person MUST be able to derive the scrub copy and the thumbnails of a video on a computer whose decoders read its format.

### EU-0022-012 {#EU-0022-012}

Import MUST derive the scrub copy of a video by default.

### EU-0022-013 {#EU-0022-013}

Euphony MUST be able to show any frame of an imported video from its scrub copy without decoding any other frame.

### EU-0022-014 {#EU-0022-014}

Import MUST derive thumbnails of the video at several zoom levels for its track.

### EU-0022-015 {#EU-0022-015}

An item derived from a video MUST be computed again only where its inputs differ from those of an item already computed.

### EU-0022-016 {#EU-0022-016}

The catalog MUST list each item derived from a video as derived from that video.

### EU-0022-017 {#EU-0022-017}

Euphony MUST NOT edit, color or re-encode the picture of a video.
