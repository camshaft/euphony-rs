## Refinement by an assistant {#refinement}

A refinement run is project data: its identity, its declared stop rules and the result of each of
its evaluations. Each round the assistant reads, as typed data, the measurements of the gear, the
error and random-part comparisons of each candidate on validation material with their intervals,
the spectrum and envelope of the residual over time, the measured cost, added delay and missed
deadlines of each candidate, and the results of development listening with the clips the listener
identified. It changes the structure of a structured model, the initial values and fit settings,
the model family and size, and the capture plan. A run stops when the interval of the
conditional-mean error lies within the musician's tolerance for each signal kind and the
random-part comparisons pass, when the best candidate stops improving beyond its intervals over the
number of rounds the musician sets, or when no candidate that reduces the error fits the processor
cost the musician accepts. The assistant can't listen: its measures are evidence for the musician,
not a verdict. The musician connects the gear, approves each capture session, declares the final
test, accepts a model, and decides whether a model is shared.

### EU-0010-094 {#EU-0010-094}

A refinement run MUST be project data that holds its identity, its declared stop rules and the result of each of its evaluations.

### EU-0010-095 {#EU-0010-095}

Each refinement round MUST offer the assistant the measurements, the validation errors with their intervals, the residual over time, the measured cost and delay of each candidate, and the development listening results, as typed data.

### EU-0010-096 {#EU-0010-096}

The assistant MUST NOT read the final partition or the trials of the final evaluation before the musician records that evaluation.

### EU-0010-097 {#EU-0010-097}

A capture that a refinement run plans MUST wait for the owner's approval of its capture session before it plays a signal through the gear.

### EU-0010-098 {#EU-0010-098}

When an evaluation result establishes a stop rule of a run, euphony MUST record the run as stopped in the same transaction as that result.

### EU-0010-099 {#EU-0010-099}

The stagnation rule of a refinement run MUST default to three rounds without an improvement beyond the intervals.

### EU-0010-100 {#EU-0010-100}

euphony MUST refuse a fit, evaluation or capture plan submitted for a stopped run, with the stop rule that holds and the report of the best candidates.

### EU-0010-101 {#EU-0010-101}

euphony MUST refuse a job of a stopped run at admission, a job queued before the stop included.

### EU-0010-102 {#EU-0010-102}

A submission to a refinement run MUST carry the run revision it read, so that a submission that races the result that stopped the run is refused.

### EU-0010-103 {#EU-0010-103}

A job of a run that was already running when the run stopped MUST finish and have its result recorded without resuming the run.

### EU-0010-104 {#EU-0010-104}

The stop of a refinement run MUST NOT affect accepted models, completed jobs or other runs.

### EU-0010-105 {#EU-0010-105}

A stopped run MUST resume only when the musician resumes it.

### EU-0010-106 {#EU-0010-106}

The resume of a run MUST record who resumed it and each stop rule they changed.

### EU-0010-107 {#EU-0010-107}

The stop check MUST take the metrics, tolerances, stagnation rule and resume authority of any refinement run, so that every refinement loop of euphony uses it.

### EU-0010-108 {#EU-0010-108}

A stopped run MUST report its best candidates with their error and their cost.
