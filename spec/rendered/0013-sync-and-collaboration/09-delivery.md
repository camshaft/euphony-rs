## Delivery and quality of the listening audio {#delivery}

The host's audio process already calculates the main mix and the mix of each sink. The session
streams each participant's assigned sink from the host, and plays a collaborator's own prepared copy
of the playing revision and sink while that copy's manifest digest equals the host's. Rendering the
project on each participant's computer would need the whole project on every computer and would
diverge as soon as the host edits, so it is not a way of delivery. The lossy stream uses a standard
lossy codec for music, and the lossless stream carries the exact samples of the mix. Each session
has a listening-quality policy, which the owner chooses: lossless required, lossless preferred or
lossy. Lossy audio suits discussion of an arrangement, and judging a mix needs the lossless stream or
the prepared copy. The listening stream is playback delivery alone. A forwarding peer, which
receives one copy and sends it on, is a later addition for audiences larger than the upload of one
host.

### EU-0013-175 {#EU-0013-175}

The host MUST stream each participant assigned sink as a live stream that follows every edit, live input and automation change at once.

### EU-0013-176 {#EU-0013-176}

The listening stream MUST NOT carry stems or project files.

### EU-0013-177 {#EU-0013-177}

While the prepared copy of a collaborator holds the playing revision and sink with a manifest digest equal to that of the host, the session MUST play that prepared copy for the collaborator.

### EU-0013-178 {#EU-0013-178}

When the manifest digest of the host changes, the session MUST return the collaborator to the stream, aligned on the shared timeline, before the changed audio plays.

### EU-0013-179 {#EU-0013-179}

Playback of a prepared copy MUST be bit-identical to the prepared mix of the host.

### EU-0013-180 {#EU-0013-180}

Each stream message MUST carry the transport generation, the sample position of its first sample, the host send time and the codec of its frames.

### EU-0013-181 {#EU-0013-181}

A stream packet MUST NOT be sent again after its output time has passed.

### EU-0013-182 {#EU-0013-182}

The lossless stream MUST decode to exactly the samples of the host.

### EU-0013-183 {#EU-0013-183}

The lossy stream MUST carry full-band stereo audio at 48 kHz.

### EU-0013-184 {#EU-0013-184}

The bit rate of the lossy stream MUST follow the congestion control of the path.

### EU-0013-185 {#EU-0013-185}

A path that cannot carry the lossy stream at its current rate MUST receive a lower rate instead of silence.

### EU-0013-186 {#EU-0013-186}

Each participant MUST see the bit rate they receive.

### EU-0013-187 {#EU-0013-187}

Each session MUST have a listening-quality policy of lossless required, lossless preferred or lossy, which the owner chooses.

### EU-0013-188 {#EU-0013-188}

Every session MUST start with the lossless required policy.

### EU-0013-189 {#EU-0013-189}

Every participant MUST see the listening-quality policy of the session and the quality they actually receive.

### EU-0013-190 {#EU-0013-190}

Under lossless required, a participant whose path cannot carry the lossless stream MUST have its playback stopped with the reason instead of receiving lossy audio.

### EU-0013-191 {#EU-0013-191}

Under lossless preferred, every move of the stream between lossless and lossy MUST change the quality label of the participant.

### EU-0013-192 {#EU-0013-192}

A participant receiving lossy audio MUST NOT be shown as receiving lossless audio.

### EU-0013-193 {#EU-0013-193}

At join, a client MUST check its decoders and report their measured decoding time.

### EU-0013-194 {#EU-0013-194}

A client that cannot run a decoder that the listening-quality policy needs MUST be refused as a listener with the reason.

### EU-0013-195 {#EU-0013-195}

A participant client MUST NOT keep the listening stream as a project object, use it for a render or offer it to another peer.

### EU-0013-196 {#EU-0013-196}

Project objects, cached renders and audio that peers exchange MUST stay exact in every session, whatever its listening-quality policy.

### EU-0013-197 {#EU-0013-197}

The host MUST encode once for each distinct sink and send one encrypted copy to each participant.

### EU-0013-198 {#EU-0013-198}

The session MUST NOT cap the number of participants.

### EU-0013-199 {#EU-0013-199}

The host MUST admit participants while its measured upload carries them.
