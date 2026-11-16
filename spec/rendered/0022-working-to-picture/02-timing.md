## Frames, timecode and picture-locked positions {#timing}

The project's sample timeline, which RFC 0004 states, is the clock of the project, and a video's
timing comes from the presentation times of its own frames. Each frame's presentation time is a
whole number in the stream's time base, after the container's edit list, which sets where
presentation starts. Frame i starts at the project sample s0 + round((t_i - t_0) x R x b), where
t_i is its presentation time, t_0 the presentation time of the first frame shown, b the time base in
seconds, s0 the start offset in samples and R the project's sample rate, with the rounding that RFC
0004 states for the timeline; the frame lasts until the next frame starts. A file of constant rate
f/d places frame n at s0 + round(n x R x d / f), the same position as its presentation time. This
is the frame mapping of the project.

Timecode is hours, minutes, seconds and frames at the project's timecode rate. The timecode rates
are 23.976, 24, 25, 29.97, 30, 50, 59.94 and 60 frames per second, the fractional ones being
24000/1001, 30000/1001 and 60000/1001. At 29.97 and 59.94 a project labels frames either without
dropping labels or with drop-frame labels. Drop-frame labeling at 29.97 skips the frame labels 00
and 01, and at 59.94 the labels 00 to 03, at the start of each minute except the minutes whose
number is a multiple of ten, so that the labels follow the time of day while every frame is still
shown. Timecode is only a way to read and enter a position. Pull-up and pull-down change the rate
at which the timeline plays against the audio device by a factor of 1001/1000, the change between
24 and 23.976 frames per second.

The position of a marker or a region has a lock. A musical lock keeps the position in bars and
beats. A picture lock keeps it as a frame and a sample offset within that frame.

### EU-0022-018 {#EU-0022-018}

Euphony MUST place every frame of a video on the project's sample timeline by the frame mapping this section states.

### EU-0022-019 {#EU-0022-019}

A sample of the timeline MUST show the frame whose interval holds it.

### EU-0022-020 {#EU-0022-020}

The audio streams, the scrub copy, the thumbnails and the picture-locked positions of a video MUST use the same frame mapping as its picture.

### EU-0022-021 {#EU-0022-021}

A video whose frame intervals vary MUST be placed by the presentation times of its frames.

### EU-0022-022 {#EU-0022-022}

The track of a video whose frame rate varies MUST show that the rate varies.

### EU-0022-023 {#EU-0022-023}

A project MUST offer every timecode rate this section lists.

### EU-0022-024 {#EU-0022-024}

At 29.97 and 59.94 frames per second, a project MUST offer both labeling without dropped labels and drop-frame labeling.

### EU-0022-025 {#EU-0022-025}

Drop-frame labeling MUST skip exactly the labels this section states.

### EU-0022-026 {#EU-0022-026}

Drop-frame labeling MUST NOT leave any frame of the picture unshown.

### EU-0022-027 {#EU-0022-027}

A person MUST be able to read every position as timecode at the project's timecode rate.

### EU-0022-028 {#EU-0022-028}

A person MUST be able to enter every position as timecode at the project's timecode rate.

### EU-0022-029 {#EU-0022-029}

A stored position MUST keep its place on the timeline when the project's timecode rate or labeling changes.

### EU-0022-030 {#EU-0022-030}

A project with a video MUST take its timecode rate and labeling from that video by default.

### EU-0022-031 {#EU-0022-031}

A person MUST be able to set the project's starting timecode.

### EU-0022-032 {#EU-0022-032}

A video whose frame rate differs from the project's timecode rate MUST keep its own frame mapping.

### EU-0022-033 {#EU-0022-033}

The track of a video whose frame rate differs from the project's timecode rate MUST warn of the difference.

### EU-0022-034 {#EU-0022-034}

A project MUST offer pull-up and pull-down as a setting.

### EU-0022-035 {#EU-0022-035}

Pull-up and pull-down MUST NOT change the stored sample rate of any audio.

### EU-0022-036 {#EU-0022-036}

Each marker and each region MUST have a musical lock or a picture lock.

### EU-0022-037 {#EU-0022-037}

A tempo change MUST NOT move a picture-locked position.

### EU-0022-038 {#EU-0022-038}

A tempo change MUST move each musically locked position with the music.

### EU-0022-039 {#EU-0022-039}

A marker placed on the picture MUST have a picture lock by default.

### EU-0022-040 {#EU-0022-040}

A marker placed on the ruler of bars and beats MUST have a musical lock by default.
