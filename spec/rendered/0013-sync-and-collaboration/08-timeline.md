## Shared listening on one timeline {#timeline}

A shared listening session is a session with remote participants. The host is the native computer
whose session authority owns the transport, and participants join from a native client or from the
browser companion. The transport state is one record of the revision, a transport generation, the
state playing or stopped, the rate, the loop range, and an anchor that pairs a project sample
position P0 with the host media-clock time T0 at which that sample leaves every output. With the
project sample rate f and the transport rate r, sample P leaves every output at host media-clock time
T(P) = T0 + (P - P0) / (r f). Each output converts T(P) to its own clock and writes the sample to its
device at that time less its own output latency. Each participant estimates the offset and drift of
its clock from timestamp exchanges: each exchange proves that the offset lies in a window as wide as
its round trip, and the session intersects the drift-corrected windows of the recent exchanges. Where
the two directions of a path take different times, the offset error can reach half the difference,
which a timestamp exchange cannot detect. In aligned mode the host sets a common presentation delay,
the largest measured end-to-end delay among the participants with that participant's jitter
allowance, so that every output, the host's own included, plays each sample at the time the equation
sets. In host-live mode the host plays with its normal monitoring latency, and each participant plays
behind it by its own measured delay. Sound in air takes several milliseconds to cross a room, which
is what listening in the same room means physically.

### EU-0013-144 {#EU-0013-144}

The transport state of a shared listening session MUST be one record ordered by the control epoch of the session.

### EU-0013-145 {#EU-0013-145}

Every play, stop, seek, loop change, join and change of the common delay MUST make a new transport generation with a new anchor.

### EU-0013-146 {#EU-0013-146}

A transport change MUST NOT add a delay to an existing anchor.

### EU-0013-147 {#EU-0013-147}

Every output MUST write project sample P to its device at host media-clock time T0 + (P - P0) / (r f), converted to its own clock, less its own output latency.

### EU-0013-148 {#EU-0013-148}

The host MUST set the output time of each anchor at least the common presentation delay after it accepts the change.

### EU-0013-149 {#EU-0013-149}

A participant request for a transport change MUST carry the transport generation its sender saw.

### EU-0013-150 {#EU-0013-150}

The host MUST refuse a transport request whose generation is stale.

### EU-0013-151 {#EU-0013-151}

A participant MUST see its transport request as pending until the time the host schedules for it.

### EU-0013-152 {#EU-0013-152}

A participant with transport control MUST be able to play, stop, seek and loop for everyone.

### EU-0013-153 {#EU-0013-153}

Each participant MUST estimate the offset and drift of its clock from the host media clock from timestamp exchanges with the host.

### EU-0013-154 {#EU-0013-154}

The timestamp exchanges of clock sync MUST be carried in the stream packets and the loss and arrival reports of the session.

### EU-0013-155 {#EU-0013-155}

The session MUST report each participant alignment bound as half the width of the intersection of the drift-corrected exchange windows, widened by the uncertainty of the drift fit, plus the uncertainty of the participant output latency.

### EU-0013-156 {#EU-0013-156}

Where the exchange windows of a participant do not intersect, the session MUST show that participant alignment as unverified.

### EU-0013-157 {#EU-0013-157}

Where the exchange windows of a participant do not intersect, the session MUST start its clock estimate again.

### EU-0013-158 {#EU-0013-158}

Where a participant output latency comes only from a platform report that a loopback measurement has not confirmed, the session MUST show that participant alignment as unverified.

### EU-0013-159 {#EU-0013-159}

Where a participant measured delay exceeds the common delay, the session MUST show that participant alignment as unverified.

### EU-0013-160 {#EU-0013-160}

Where a participant plays behind the common delay, the session MUST show by how much.

### EU-0013-161 {#EU-0013-161}

The session MUST NOT present an alignment bound as sample alignment.

### EU-0013-162 {#EU-0013-162}

In aligned mode, every output of the session, the host's included, MUST play the same moment of the project within the alignment bound the session shows for it.

### EU-0013-163 {#EU-0013-163}

Aligned mode MUST be the mode of a shared listening session while nothing records.

### EU-0013-164 {#EU-0013-164}

The host MUST raise the common delay at the next transport change, not during playback.

### EU-0013-165 {#EU-0013-165}

Host-live mode MUST be the mode of the session while it records or monitors a live input.

### EU-0013-166 {#EU-0013-166}

In host-live mode, the session MUST show each participant delay behind the host.

### EU-0013-167 {#EU-0013-167}

Starting a take MUST move the session to host-live mode.

### EU-0013-168 {#EU-0013-168}

The network loss of a participant MUST NOT change the monitoring or the recording of the host.

### EU-0013-169 {#EU-0013-169}

Each player MUST follow the estimated drift by resampling in small continuous amounts.

### EU-0013-170 {#EU-0013-170}

A player MUST NOT drop or repeat samples to follow clock drift.

### EU-0013-171 {#EU-0013-171}

A participant MUST be able to correct its output latency by a loopback measurement or by an offset set by ear.

### EU-0013-172 {#EU-0013-172}

The session MUST show where each output latency value came from.

### EU-0013-173 {#EU-0013-173}

On a new transport generation, a participant MUST discard buffered audio of the previous generation.

### EU-0013-174 {#EU-0013-174}

A participant joining or leaving, or an increase of the common delay, MUST NOT change the playback of other participants before the next transport change.
