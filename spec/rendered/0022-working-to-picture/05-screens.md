## Other screens and shared sessions {#screens}

The picture shows on the computer that plays the audio, on a second display of a computer, on
another of the owner's computers, on a control surface or a phone, and on the screen of a person the
owner shares a session with through a link, as RFC 0013 states for links. Each computer that shows
the picture decodes its own copy of the video. Another of the owner's computers follows the
session's transport through the session timeline, maps each transport change to its own clock with
the session's clock synchronization, and shows each frame at its presentation time. A link has a
picture option, which carries the picture to its participant. A link participant's client shows
each frame at the time its own output plays that frame's sound, with each delay applied once: in
aligned mode at the time the link's shared playout sets for the frame's samples, and in host-live
mode at that time plus the participant's own measured delay. Revoking a link ends its delivery and
can't recall frames that a participant already received.

### EU-0022-062 {#EU-0022-062}

A person MUST be able to show the picture full screen on a second display of the same computer.

### EU-0022-063 {#EU-0022-063}

A person MUST be able to show the picture on another of their computers in step with the session's sound.

### EU-0022-064 {#EU-0022-064}

Each computer that shows the picture MUST decode it from its own copy of the video.

### EU-0022-065 {#EU-0022-065}

A video's items MUST reach each computer that shows its picture before playback reaches them.

### EU-0022-066 {#EU-0022-066}

Another computer that shows the picture MUST show each frame at the presentation time that the session's clock synchronization maps for its samples.

### EU-0022-067 {#EU-0022-067}

The session MUST show the picture alignment of each other computer and of each link participant as it shows the alignment of their sound.

### EU-0022-068 {#EU-0022-068}

A control surface or a phone MUST show the timecode, the current cue and marker, and a countdown to the next hit.

### EU-0022-069 {#EU-0022-069}

A phone MUST NOT show the picture unless its person turns the picture on.

### EU-0022-070 {#EU-0022-070}

A phone whose person turns the picture on MUST show a small picture from the scrub copy.

### EU-0022-071 {#EU-0022-071}

A link MUST carry a picture option that the owner chooses when making it.

### EU-0022-072 {#EU-0022-072}

A new link MUST NOT include the picture unless the owner chooses its picture option.

### EU-0022-073 {#EU-0022-073}

A participant whose link lacks the picture option MUST NOT receive a frame.

### EU-0022-074 {#EU-0022-074}

Euphony MUST check a link's picture option before it delivers each frame to its participant.

### EU-0022-075 {#EU-0022-075}

A link participant's client MUST show each frame at the time its own output plays that frame's sound.

### EU-0022-076 {#EU-0022-076}

After a seek, a loop change, a join, a change of delay or a change between aligned and host-live mode, a participant's picture MUST follow the new timing from its first refresh.

### EU-0022-077 {#EU-0022-077}

The flash-and-tone test on a link participant's client MUST see every flash during the frame whose tone that participant's output plays, in aligned and in host-live mode.

### EU-0022-078 {#EU-0022-078}

An assistant MUST NOT make a link with the picture option without the owner's approval of that link.

### EU-0022-079 {#EU-0022-079}

The dialog that makes a link with the picture option MUST say that revoking the link can't recall frames already received.

### EU-0022-080 {#EU-0022-080}

Revoking a link MUST end the delivery of its frames.

### EU-0022-081 {#EU-0022-081}

A comment link MUST show and accept positions as timecode.
