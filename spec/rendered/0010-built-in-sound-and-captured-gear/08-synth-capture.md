## Capturing an external synthesizer {#synth-capture}

A synthesizer capture uses the capture session of this RFC on the computer whose audio inputs the
synthesizer returns to, with timestamped instrument control messages sent to the synthesizer. Its
plan plays notes with attack and release velocities, pedals, pitch bend, channel and per-note
pressure, controllers and high-resolution controllers, parameter numbers, bank selection and
program changes, and device-specific messages only where the chart of the device verifies them. An
instrument map is a versioned data object that describes the parameters of one device, firmware,
port and channel mode and patch format. The musician selects the patch families and controls the
model supports, with their ranges, before capture. Sampling first maps pitch, velocity, durations
and voice behavior with other settings fixed, then sweeps each chosen control in both directions,
then adds joint samples and interaction cases, then adds samples where held-out errors are large.
Whole takes, histories and parameter regions are held out for validation and the final test. The
plan, its coverage report and its estimates are durable session state, so an interrupted capture
resumes at a confirmed experiment boundary.

### EU-0010-138 {#EU-0010-138}

A synthesizer capture MUST reserve exclusive use of the device, ports and channels it plays for the length of its session.

### EU-0010-139 {#EU-0010-139}

A synthesizer capture that sends messages from another computer MUST measure the mapping and jitter of its timestamps.

### EU-0010-140 {#EU-0010-140}

A synthesizer capture MUST record its planned messages, actual send times, device acknowledgments, audio, clock mapping, calibration and observed failures as one take.

### EU-0010-141 {#EU-0010-141}

A synthesizer capture MUST NOT send a device-specific message other than a read request or a nonpersistent edit of the edit buffer that the chart of the device verifies.

### EU-0010-142 {#EU-0010-142}

A synthesizer capture MUST NOT write firmware, calibration, a factory reset or a persistent patch.

### EU-0010-143 {#EU-0010-143}

A synthesizer capture MUST report a message type that the device lacks as unsupported.

### EU-0010-144 {#EU-0010-144}

A synthesizer capture MUST treat a program change as a discrete patch transition.

### EU-0010-145 {#EU-0010-145}

Synthesizer calibration MUST estimate the distribution of the delay from a sent message to the observed onset from repeated timestamped notes of a reference patch.

### EU-0010-146 {#EU-0010-146}

Where the delays of converter, transport and synthesizer can't be separated, the capture MUST report their aggregate delay with its uncertainty and keep the raw timing.

### EU-0010-147 {#EU-0010-147}

Synthesizer calibration MUST NOT align each target note on its own.

### EU-0010-148 {#EU-0010-148}

A synthesizer capture MUST set its input gain from quiet isolated notes through the loudest authorized chords, with a measured margin below clipping.

### EU-0010-149 {#EU-0010-149}

A synthesizer capture MUST reject a clipped take.

### EU-0010-150 {#EU-0010-150}

A level excursion MUST stop a synthesizer capture and keep the partial take as invalid for fitting.

### EU-0010-151 {#EU-0010-151}

After a device reconnects, a synthesizer capture MUST check the identity and calibration of the device before it resumes.

### EU-0010-152 {#EU-0010-152}

A synthesizer capture MUST begin from a declared patch snapshot and a tested reset sequence, and record which state the sequence restores.

### EU-0010-153 {#EU-0010-153}

Where a reset can't restore the hidden state of a device, the capture MUST mark that uncertainty and validate the model statistically rather than claim waveform identity.

### EU-0010-154 {#EU-0010-154}

Cancellation of a synthesizer capture MUST release each note it owns and each pedal it holds while the device remains reachable.

### EU-0010-155 {#EU-0010-155}

After a device is lost, a new session MUST reconcile the state of the device rather than replay an uncertain outstanding message.

### EU-0010-156 {#EU-0010-156}

An instrument map MUST be a versioned data object tied to a device model, firmware, port and channel mode and patch format.

### EU-0010-157 {#EU-0010-157}

An instrument map MUST record for each parameter its stable identity, unit, range, quantization, type, encoding, dependencies, settling behavior, readback support and whether a change resets voices.

### EU-0010-158 {#EU-0010-158}

euphony MUST treat a dump from a device as untrusted data.

### EU-0010-159 {#EU-0010-159}

A synthesizer capture MUST store the intended and the confirmed value of each parameter apart.

### EU-0010-160 {#EU-0010-160}

A synthesizer capture MUST invalidate a take whose state confirmation fails.

### EU-0010-161 {#EU-0010-161}

A synthesizer capture plan MUST list the patch families and controls the model supports, with their ranges, as the musician selects them.

### EU-0010-162 {#EU-0010-162}

A synthesizer capture MUST hold each control the musician did not select at a recorded value, and show it as fixed.

### EU-0010-163 {#EU-0010-163}

A synthesizer capture MUST NOT split adjacent windows of one performance between training and testing.

### EU-0010-164 {#EU-0010-164}

The plan, coverage report and estimates of a synthesizer capture MUST be durable session state, so that an interrupted capture resumes at a confirmed experiment boundary.

### EU-0010-165 {#EU-0010-165}

A synthesizer capture plan MUST show its estimated physical capture time and the projected remaining time as it grows.

### EU-0010-166 {#EU-0010-166}

A synthesizer capture MUST pace its messages by the documented limits of the device and the measured acknowledgments and settling.

### EU-0010-204 {#EU-0010-204}

A synthesizer capture MUST keep each queued experiment when the device overruns or drops an update.

### EU-0010-205 {#EU-0010-205}

A synthesizer capture MUST lower its message rate when the device overruns or drops an update.

### EU-0010-167 {#EU-0010-167}

The report of a synthesizer capture MUST state its unmet coverage.
