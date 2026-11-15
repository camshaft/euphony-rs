## Evaluation and release {#evaluation}

The evaluation corpus is versioned. It holds licensed audio with the provenance of its labels, the
disagreements of its annotators, its recording conditions, genres, instruments and languages, and
the identities of its splits. Test artists and recordings are kept apart from training and tuning
material where those are known, and the overlap is recorded as unknown where it can't be checked.
The cases the corpus covers are project configuration, and start with silence, noise, clipping,
tempo changes, rubato, odd meter, detuning and mixed sources. Each method is evaluated with the
metrics of its family, such as tonic and mode scored together with related-key errors apart, chord
accuracy for each vocabulary weighted by duration, beat and downbeat precision and recall at a
declared tolerance, note onset and offset scores, pitch error in cents, and loudness against
reference vectors of the standard it computes; no single accuracy spans the families. Calibration
is derived from held-out labeled examples. A release record of a method version holds its validated
outputs, the material it supports and its coverage, the verified rights of each dependency
category, passing tests of its resources and of its isolation from live audio, its comparison with
the pinned previous version on the same corpus and device profiles, or for a first version a record
that it has none, and an explicit acceptance criterion for its intended use with its result. Someone
authorized can still inspect a method as a prototype, and a prototype isn't released. Each
result family advances on its own, through the same result contract and evaluation. A person can
let a corrected example join the corpus, and can withdraw it later.

### EU-0020-091 {#EU-0020-091}

The system MUST refuse to include a method in a build where its conformance tests do not include a request it abstains from.

### EU-0020-092 {#EU-0020-092}

The system MUST refuse to include a method version in a build without a passing release record for that version, model digest, execution profile and build.

### EU-0020-093 {#EU-0020-093}

The system MUST refuse to apply a result automatically without a passing release record for its method version, model digest, execution profile and build.

### EU-0020-094 {#EU-0020-094}

The release record of a method version MUST hold its comparison with the pinned previous version on the same corpus and device profiles.

### EU-0020-095 {#EU-0020-095}

The system MUST refuse a release record that doesn't name a previous version, for a method whose earlier version has a release record.

### EU-0020-096 {#EU-0020-096}

The release record of a method version MUST state an explicit acceptance criterion for its intended use and the result against that criterion.

### EU-0020-097 {#EU-0020-097}

The system MUST refuse to publish a corpus version that lacks an example of each required case.

### EU-0020-098 {#EU-0020-098}

An evaluation report MUST state its exact input set, its metric definitions and tolerances, the model and preprocessing versions, its abstention coverage, its uncertainty and its results for each subgroup.

### EU-0020-099 {#EU-0020-099}

The system MUST refuse an evaluation report whose test artists or recordings overlap a known training or tuning split.

### EU-0020-100 {#EU-0020-100}

The system MUST refuse a calibration record whose examples overlap the final test split.

### EU-0020-101 {#EU-0020-101}

Where training overlap cannot be checked, an evaluation report MUST record the overlap as unknown with its missing provenance.

### EU-0020-102 {#EU-0020-102}

A report whose overlap is unknown MUST NOT satisfy an acceptance criterion that requires verified separation.

### EU-0020-103 {#EU-0020-103}

A comparison that shows a regression past a method's acceptance criterion, or a disagreement between methods, MUST open or join one work item for its project, method, profile, corpus and metric.

### EU-0020-104 {#EU-0020-104}

A later comparison that shows a resolved regression again MUST reopen its work item.

### EU-0020-105 {#EU-0020-105}

The system MUST refuse to admit an example derived from a correction into the corpus without the owner's recorded permission for that example and for evaluation.

### EU-0020-106 {#EU-0020-106}

A withdrawn example MUST be left out of every later corpus version, including one being assembled when it is withdrawn.

### EU-0020-107 {#EU-0020-107}

A report published before a withdrawal MUST keep showing its original corpus together with the withdrawal.

### EU-0020-108 {#EU-0020-108}

Where a method runs on more than one kind of hardware, its results on each kind MUST be compared for accuracy as well as for speed.
