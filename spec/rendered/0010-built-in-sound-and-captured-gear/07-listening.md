## Blind listening and acceptance {#listening}

Listening comes in two kinds. Development listening guides the assistant and the musician while
candidates still differ audibly, and its clips and results become development data. The final
evaluation is an ABX test: the listener plays A and B, one the hardware and one the model, and says
which of them an unlabeled clip X is, over level-matched clips of final material. Before it starts,
the musician declares its scope, one candidate chosen in advance in one test or a budget of A
attempts across candidates and retests; the error rate alpha the claim tolerates over that scope;
the largest proportion of correct answers they tolerate, p_tol, above chance and below 1; the L
listeners; and the chance they want of meeting the criterion for a model that a listener truly
can't tell apart. Each attempt tests at level alpha/A. For each listener the trial count is fixed in
advance and computed exactly from the binomial distribution, which assumes independent trials with
one constant probability of a correct answer. A listener meets the criterion when the exact
one-sided upper confidence bound on their proportion of correct answers, at level 1 - alpha/(A L),
is below p_tol. Every final session mixes in a hidden share of trials of the hardware against a
deliberately degraded model, as a positive control. A failure to reject guessing is never evidence
of closeness: the report states detected where an exact binomial test rejects guessing, shown close
only where the upper bound is below p_tol and the listener passed the control, and not shown close
in every other case.

### EU-0010-109 {#EU-0010-109}

The results of development listening MUST become development data for the next refinement round.

### EU-0010-110 {#EU-0010-110}

The final evaluation of a model MUST be an ABX test over level-matched clips of final material.

### EU-0010-111 {#EU-0010-111}

The final evaluation MUST refuse to start before the candidate, the protocol and the criterion are frozen and recorded.

### EU-0010-112 {#EU-0010-112}

The final evaluation MUST refuse final material that fitting, the assistant or development listening read.

### EU-0010-113 {#EU-0010-113}

The final evaluation MUST take from the musician, before it starts, its scope, its tolerated error rate, its tolerated proportion of correct answers, its listeners and the chance they want of meeting the criterion.

### EU-0010-114 {#EU-0010-114}

The scope of the final evaluation MUST default to one candidate chosen in advance and tested once.

### EU-0010-115 {#EU-0010-115}

The tolerated error rate of the final evaluation MUST default to 0.05.

### EU-0010-204 {#EU-0010-204}

The listeners of the final evaluation MUST default to the musician alone.

### EU-0010-116 {#EU-0010-116}

The tolerated proportion of correct answers of the final evaluation MUST be above 0.5 and below 1.

### EU-0010-117 {#EU-0010-117}

Each attempt of a final evaluation with a budget of attempts MUST test at the tolerated error rate divided by the number of attempts in the budget.

### EU-0010-118 {#EU-0010-118}

A final evaluation MUST NOT make a statistical claim after the attempts of its budget are spent.

### EU-0010-119 {#EU-0010-119}

The trial count of each listener MUST be fixed before the test.

### EU-0010-260 {#EU-0010-260}

The trial count of each listener MUST be computed exactly from the binomial distribution for the declared values.

### EU-0010-120 {#EU-0010-120}

The final evaluation MUST draw the identity of X at random for each trial.

### EU-0010-121 {#EU-0010-121}

The final evaluation MUST present its trials in random order.

### EU-0010-122 {#EU-0010-122}

The final evaluation MUST NOT tell a listener the result of a trial before the end of the test.

### EU-0010-123 {#EU-0010-123}

The final evaluation MUST hide which clip is the hardware and which is the model.

### EU-0010-124 {#EU-0010-124}

A listener MUST meet the criterion only when the exact one-sided upper confidence bound on their proportion of correct answers, at a confidence level of one minus the tolerated error rate divided by the number of attempts and the number of listeners, is below the tolerated proportion.

### EU-0010-125 {#EU-0010-125}

The final evaluation MUST NOT pool the trials of different listeners.

### EU-0010-126 {#EU-0010-126}

A claim for the declared listeners MUST hold only when every one of them meets the criterion.

### EU-0010-127 {#EU-0010-127}

The final evaluation MUST run its trials in sessions of at most 25 trials.

### EU-0010-128 {#EU-0010-128}

Every final session MUST mix in, at a share the plan fixes and the listener is not told, trials of the hardware against a deliberately degraded model.

### EU-0010-129 {#EU-0010-129}

The result of a listener who misses the positive control MUST NOT count toward a claim.

### EU-0010-130 {#EU-0010-130}

Level matching MUST apply one constant gain over the whole clip, measured by integrated loudness.

### EU-0010-131 {#EU-0010-131}

Level matching MUST report the level difference that remains after its gain.

### EU-0010-132 {#EU-0010-132}

The clips of the hardware and of the model MUST be aligned to the sample.

### EU-0010-261 {#EU-0010-261}

The clips of the hardware and of the model MUST be trimmed and faded the same way.

### EU-0010-133 {#EU-0010-133}

The report of a final evaluation MUST state for each listener whether an exact binomial test rejects guessing, as the result detected.

### EU-0010-134 {#EU-0010-134}

The report of a final evaluation MUST state shown close for a listener only where their upper bound is below the tolerated proportion and they passed the positive control.

### EU-0010-135 {#EU-0010-135}

The report of a final evaluation MUST NOT present a result of not detected as closeness.

### EU-0010-136 {#EU-0010-136}

The report of a final evaluation MUST list every attempt, with the observed proportion and interval of each listener.

### EU-0010-137 {#EU-0010-137}

A failed final evaluation MUST spend its final partition, so that another final evaluation needs a newly captured final partition.
