## How closely a model matches its hardware {#closeness}

The output of gear for an input is its conditional mean, the output averaged over many passes of
the same input from the same starting state, plus a random part that differs on every pass. The
hardware variance is estimated from the deviations of the K passes about their average. The
conditional-mean error is the mean squared difference between the conditional mean of a model and
the hardware average, less the share that the finite number of passes and renders adds, so that an
exact independent random replica of the hardware has an expected error of zero. The conditional
mean of a model is rendered exactly only where its expected output is proven, such as zero-mean
noise added after all nonlinear processing; elsewhere it is the average of M renders with
independent seeds. The random part is compared in its level, its spectrum, its dependence on the
input level, and the spectrum and depth of its pitch modulation, with the deviations of the model
formed from K renders as those of the hardware are formed from K passes. Each estimate is made for
each segment and frequency band, with an interval from a resampling of whole segments. A report
also holds a null test of the conditional mean against the hardware average, error measures on
held-out material, and the results of blind listening, each apart from the others.

### EU-0010-051 {#EU-0010-051}

The report of a model MUST estimate the hardware variance for each test segment and frequency band.

### EU-0010-052 {#EU-0010-052}

The report of a model MUST estimate its conditional-mean error so that an exact independent random replica of the hardware has an expected estimate of zero.

### EU-0010-053 {#EU-0010-053}

Where the expected output of a model is not proven, its report MUST estimate the conditional mean of the model as the average of renders with independent seeds.

### EU-0010-054 {#EU-0010-054}

The report of a model MUST compare the random part of the model with that of the hardware in level, spectrum, dependence on input level, and spectrum and depth of pitch modulation.

### EU-0010-055 {#EU-0010-055}

The comparison of random parts MUST fail a model whose random part is absent or replays one recorded pass whatever its seed.

### EU-0010-056 {#EU-0010-056}

The report of a model MUST state an interval for each estimate it holds.

### EU-0010-057 {#EU-0010-057}

The report of a model MUST show whether the passes of its capture come from one steady state.

### EU-0010-058 {#EU-0010-058}

The report of a model MUST hold a null test, error measures on held-out material and the results of blind listening, each apart from the others.

### EU-0010-059 {#EU-0010-059}

The error measures in the report of a model MUST be computed on material that fitting and selection did not use.

### EU-0010-060 {#EU-0010-060}

The report of a model MUST NOT call the model close enough before the musician declares tolerances.

### EU-0010-061 {#EU-0010-061}

The report of a model MUST call the model as close as the capture can show only once the interval of its conditional-mean error and the intervals of its random-part comparisons lie within the tolerances the musician declares.

### EU-0010-062 {#EU-0010-062}

The report of a model MUST record whether the musician accepted the model, apart from its statistical results.
