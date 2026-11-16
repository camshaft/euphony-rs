## Mixing and delivery {#delivery}

A delivery holds the stems to make, by default dialogue, music, effects, music-and-effects and the
full mix, each as a set of stem buses; its channel layout, such as stereo, 5.1 or 7.1, or a spatial
bed; its sample rate and bit depth; its file format, by default an audio file whose metadata holds
its starting timecode; a file naming pattern; its loudness profile; and the program's range, from
its first frame of action to its last, with any head and tail pops its specification asks for.

The sums of a delivery are explicit: dialogue, music and effects add up to the full mix, and
music-and-effects is the sum of music and effects, a separate file that isn't added to the first sum
a second time. Three things break the sums. A send that reaches a return of another stem mixes two
stems. Gain-only processing on the main output, such as a compressor or a limiter, changes the full
mix by a gain that the stems don't carry. Other nonlinear processing on the main output, such as
saturation or clipping, can't be reproduced on each stem.

A loudness profile is versioned data that names its source and version and sets targets for
integrated loudness, true peak and loudness range. The built-in profiles are the European broadcast
profile, an integrated loudness of -23 LUFS with a true peak of at most -1 dBTP, and the North
American broadcast profile, an integrated loudness of -24 LKFS. Integrated loudness, true peak and
loudness range are measured as the international broadcast recommendations for loudness state.
Sending an export to another person follows the approval of what leaves the owner's computers that
RFC 0001 states.

### EU-0022-140 {#EU-0022-140}

A person MUST be able to define a delivery with each field this section lists.

### EU-0022-141 {#EU-0022-141}

One export MUST render every stem of a delivery from the same revision of the project and the same numerical profile.

### EU-0022-142 {#EU-0022-142}

The delivery check MUST test that dialogue, music and effects add up to the full mix within the profile's numerical tolerance.

### EU-0022-143 {#EU-0022-143}

The delivery check MUST test that music-and-effects equals music plus effects within the profile's numerical tolerance.

### EU-0022-144 {#EU-0022-144}

A delivery MUST call its stems complete only while its check holds.

### EU-0022-145 {#EU-0022-145}

A failed delivery check MUST say which sum failed, by how much and where.

### EU-0022-146 {#EU-0022-146}

The delivery check MUST report each send that reaches a return of another stem, with its track, the return and the stems.

### EU-0022-147 {#EU-0022-147}

Gain-only processing on the main output MUST apply to each stem the gain it computes from the full mix, sample by sample and channel by channel.

### EU-0022-148 {#EU-0022-148}

Other nonlinear processing on the main output MUST fail the delivery check.

### EU-0022-149 {#EU-0022-149}

Where nonlinear processing on the main output fails the check, the delivery MUST offer to deliver the stems and a reference mix taken before that processing, beside the processed full mix as a separate labeled file, or to move that processing onto the stems.

### EU-0022-150 {#EU-0022-150}

A delivery MUST NOT apply nonlinear main-output processing to each stem and report the sum as passing.

### EU-0022-151 {#EU-0022-151}

Integrated loudness, true peak and loudness range MUST be measured on each exported file.

### EU-0022-152 {#EU-0022-152}

Each loudness profile MUST name its source and version.

### EU-0022-153 {#EU-0022-153}

Euphony MUST hold the built-in loudness profiles this section lists.

### EU-0022-154 {#EU-0022-154}

A person MUST be able to enter a loudness profile from a delivery specification they received.

### EU-0022-155 {#EU-0022-155}

A loudness profile MAY measure dialogue alone, on the dialogue stem or on the passages with speech.

### EU-0022-156 {#EU-0022-156}

The loudness report MUST list each file's values and whether it meets the profile.

### EU-0022-157 {#EU-0022-157}

A loudness correction MUST NOT change the mix until the person applies it.

### EU-0022-158 {#EU-0022-158}

A delivery MUST offer stereo, 5.1 and 7.1 channel layouts.

### EU-0022-159 {#EU-0022-159}

An immersive master MUST show as unavailable until a licensed integration provides it.

### EU-0022-160 {#EU-0022-160}

A person MUST be able to export the mix inside a copy of the video whose picture stream is copied byte for byte.

### EU-0022-161 {#EU-0022-161}

The copy of the video MUST hold the mix as uncompressed audio tracks.

### EU-0022-162 {#EU-0022-162}

A person MUST be able to keep or replace the original audio tracks of the video in its copy.

### EU-0022-163 {#EU-0022-163}

A person MUST be able to export the stems and the mix as audio files whose metadata holds their starting timecode, so that a video editing program places each at its time.

### EU-0022-164 {#EU-0022-164}

A person MUST be able to export a project exchange file that carries each track's clips with their handles, fades and clip gain at their timecode, with the media embedded or referenced.

### EU-0022-165 {#EU-0022-165}

Exchange MUST offer a format that each video editing program in common use opens.

### EU-0022-166 {#EU-0022-166}

An exchange export MUST report each element that doesn't transfer.

### EU-0022-167 {#EU-0022-167}

A delivery MUST report each item whose license is unknown.

### EU-0022-168 {#EU-0022-168}

Sending an export to another person MUST take the owner's approval of what leaves their computers.
