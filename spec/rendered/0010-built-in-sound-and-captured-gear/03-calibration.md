## Calibration and test signals {#calibration}

Calibration comes before measurement. A direct cable from an output to an input measures the round
trip delay, frequency response and noise of the interface. A measurement through the gear then
reads the delay of the gear as the difference, to a fraction of a sample, and checks its polarity.
The delay of digital gear is part of its model; the delay of the interface isn't. A tone at a
known digital level, read on the meter of the gear or on a voltmeter, maps digital full scale to
the operating level of the gear, and the rig rises to the highest level the musician sets in steps
from a low level. A reference segment plays at the start and at the end of a capture, and a
difference between them larger than the spread among repeated passes shows that the gear drifted.
The test signal set holds sine sweeps and noise bursts at several levels, transients, level steps
and tone bursts of several sizes and lengths, silence, a long steady tone at 3150 Hz and a pulse
train for pitch modulation, and the musician's own recordings of the material the model will
process. The set plays in K repeated passes, K at least 2, each after the same preroll. Each kind
of signal is split into a training partition, a validation partition for comparing candidates, and
a final partition with its listening clips, which stays sealed until the musician freezes a
candidate. How much capture a model needs is measured, not fixed.

### EU-0010-033 {#EU-0010-033}

Calibration MUST measure the round trip delay, the frequency response and the noise of the audio interface on its own, before a measurement of the gear.

### EU-0010-034 {#EU-0010-034}

A model MUST describe the gear without the delay, response or noise of the audio interface.

### EU-0010-035 {#EU-0010-035}

Calibration MUST measure the delay of the gear to a fraction of a sample.

### EU-0010-036 {#EU-0010-036}

Calibration MUST detect an inverted polarity of the gear.

### EU-0010-037 {#EU-0010-037}

Calibration MUST map digital full scale to the operating level of the gear.

### EU-0010-038 {#EU-0010-038}

The capture rig MUST rise to the highest level the musician sets in steps from a low level.

### EU-0010-039 {#EU-0010-039}

A capture of gear with a digital clock of its own MUST correct for the measured drift between that clock and the clock of the audio interface.

### EU-0010-040 {#EU-0010-040}

A capture MUST compare a reference segment played at its start with one played at its end.

### EU-0010-041 {#EU-0010-041}

Where the reference segments of a capture differ by more than the spread among its repeated passes, the capture rig MUST ask the musician to let the gear warm up and capture again.

### EU-0010-042 {#EU-0010-042}

Each test signal MUST have a stored seed and version, so that a capture can be repeated.

### EU-0010-043 {#EU-0010-043}

The test signal set MUST hold sine sweeps and noise bursts at several levels, transients, level steps and tone bursts of several sizes and lengths, silence, a steady tone, a pulse train and the musician's own program material.

### EU-0010-044 {#EU-0010-044}

A capture MUST play each test segment in at least two repeated passes, each after the same preroll.

### EU-0010-045 {#EU-0010-045}

A capture MUST split each kind of test signal into a training partition, a validation partition and a final partition.

### EU-0010-046 {#EU-0010-046}

Fitting, candidate selection and development listening MUST NOT read the final partition.

### EU-0010-047 {#EU-0010-047}

A capture MUST stop once its error lies within the tolerance the musician declares and its random-part comparisons pass, once more capture no longer improves its estimates beyond their intervals, or on the musician's stop.

### EU-0010-048 {#EU-0010-048}

A model MUST store the calibrated level of its capture.

### EU-0010-049 {#EU-0010-049}

A model MUST report when its input goes outside the levels its capture covered.

### EU-0010-050 {#EU-0010-050}

A model MUST keep the description of the control settings of the gear that the musician provides with its capture.
