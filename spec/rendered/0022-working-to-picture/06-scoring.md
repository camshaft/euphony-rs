## Scoring tools {#scoring}

A composer marks the cuts and moments the music must hit, chooses a tempo that puts those moments on
beats, and writes each cue to them. A picture marker holds its position and lock, its kind, a
priority for a hit, text and a color. Its kind is a cut, a hit, a sync point, a cue start or a cue
end, and the priority of a hit is must hit or may miss. Cut detection finds hard cuts and gradual
transitions, such as dissolves and fades, in the picture. Tempo fitting takes a range, the hits in
it, the measure and beat each hit falls on or the nearest beat, and its constraints: one constant
tempo, or tempo changes only at chosen bars, a range of tempos and the meter. The miss of a hit is
the time between the hit and its beat. A cue holds its number, title, start and end, the tracks and
regions it covers, its usage, such as background or visual, and the names and shares of its
composer and publisher. A streamer is a vertical line drawn over the picture that moves from the
left edge to the right over its length and ends on its hit's frame, where a punch, a bright circle,
shows for one frame. A countdown pop is a one-frame tone two seconds before the first frame of
action.

### EU-0022-082 {#EU-0022-082}

A person MUST be able to place a picture marker of each kind this section lists.

### EU-0022-083 {#EU-0022-083}

Euphony MUST suggest the picture's cuts as candidate cut markers, each with its confidence.

### EU-0022-084 {#EU-0022-084}

A person MUST be able to accept a candidate cut marker with one action.

### EU-0022-085 {#EU-0022-085}

A person MUST be able to accept each candidate cut marker at or above a confidence threshold they set for the project.

### EU-0022-086 {#EU-0022-086}

Cut detection MUST find hard cuts and gradual transitions.

### EU-0022-087 {#EU-0022-087}

Analysis of a video MUST find its black frames and its countdown pop.

### EU-0022-088 {#EU-0022-088}

A person MUST be able to fit a tempo map to the hits of a range under the constraints this section lists.

### EU-0022-089 {#EU-0022-089}

Tempo fitting MUST return candidate tempo maps ranked by their largest miss and then by their mean miss.

### EU-0022-090 {#EU-0022-090}

Each candidate tempo map MUST show each hit's miss in milliseconds and in frames.

### EU-0022-091 {#EU-0022-091}

A fit to one constant tempo MUST find the tempo and the first downbeat position whose largest miss is the smallest.

### EU-0022-092 {#EU-0022-092}

Where a fit allows a tempo change at each hit, each segment MUST take the tempo that puts its hit exactly on its beat.

### EU-0022-093 {#EU-0022-093}

A person MUST be able to preview each candidate tempo map before applying it.

### EU-0022-094 {#EU-0022-094}

Euphony MUST list the tempos of a range at which most hits fall on beats or subdivisions within a tolerance, one frame by default.

### EU-0022-095 {#EU-0022-095}

Applying a candidate tempo map MUST change the tempo map within its range alone.

### EU-0022-096 {#EU-0022-096}

Applying a candidate tempo map MUST be one command that can be undone.

### EU-0022-097 {#EU-0022-097}

Euphony MUST number cues by default by reel and music, such as 1M1 for the first cue of reel 1, where reels are known, and as M1, M2 and on otherwise.

### EU-0022-098 {#EU-0022-098}

A person MUST be able to export a cue sheet that lists each cue's number, title, start and end timecode, duration in minutes and seconds, usage, composer and publisher.

### EU-0022-099 {#EU-0022-099}

A cue sheet MUST export both as a table that spreadsheet programs read and as a printable document.

### EU-0022-100 {#EU-0022-100}

Euphony MUST NOT fill in the names or shares of a cue's composer or publisher.

### EU-0022-101 {#EU-0022-101}

A person MUST be able to give a hit a streamer.

### EU-0022-102 {#EU-0022-102}

A streamer MUST be two seconds long by default.

### EU-0022-103 {#EU-0022-103}

A streamer MUST end on its hit's frame, where a punch shows for one frame.

### EU-0022-104 {#EU-0022-104}

The colors of streamers MUST tell a warning from a cue's start and from its end.

### EU-0022-105 {#EU-0022-105}

Streamers and punches MUST show in the picture window, on a second display and on another of the owner's computers that shows the picture.

### EU-0022-106 {#EU-0022-106}

A person MUST be able to show a timecode overlay on the picture.

### EU-0022-107 {#EU-0022-107}

An export MUST NOT carry a streamer, a punch or the timecode overlay.

### EU-0022-108 {#EU-0022-108}

The click MUST follow the fitted tempo, with a count-in before each cue.

### EU-0022-109 {#EU-0022-109}

A person MAY add warning clicks before a cue.
