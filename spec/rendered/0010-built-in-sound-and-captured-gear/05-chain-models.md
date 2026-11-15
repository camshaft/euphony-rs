## Models of an external chain {#chain-models}

A model of an external chain belongs to one of two families. A structured model is a typed graph of
euphony's own built-in processors, such as filters, a level detector, a gain curve and a
waveshaper, whose parameters are fitted to the capture, with small learned parts where a fixed
formula falls short and, only where those don't meet the musician's tolerances, a small learned
model of the remaining error. A learned model is one trained network of the whole chain. Both
families are fitted to the same capture, and the evaluation of each device decides which one the
musician accepts. A model is a versioned, content-addressed object, and one built-in processor for
each family loads model objects as data. The modeled delay is the behavior of the hardware itself;
the added delay is what the model adds beyond it, such as buffering, look-ahead and resampling.
The random part of a model is a random process with a stored seed. A model of an external chain
captured at one setting of its controls copies the chain at that setting and offers one control, a
blend of the unprocessed and processed sound. A chain captured at several settings of its controls
is offered both as a set of models at fixed settings that the musician switches between and as one
model conditioned on the captured settings, which reports its error at each captured setting and
marks each setting between them as unverified. Gear that recalls its settings digitally lets the
capture rig step the settings between captures, instead of a person turning each control.
Training, fitting and evaluation run as durable jobs on the owner's paired computers, and playback
runs live on the audio computer or ahead into the render cache.

### EU-0010-063 {#EU-0010-063}

For each capture of an external chain, euphony MUST fit a structured model and a learned model to the same capture.

### EU-0010-064 {#EU-0010-064}

A structured model MUST be a typed graph of built-in processors with fitted parameters.

### EU-0010-065 {#EU-0010-065}

Each fitted parameter of a built-in processor in a structured model MUST be readable and adjustable by the musician.

### EU-0010-066 {#EU-0010-066}

A model object MUST NOT hold code that euphony runs.

### EU-0010-067 {#EU-0010-067}

A model MUST be a versioned, content-addressed object.

### EU-0010-068 {#EU-0010-068}

A model object MUST hold its family, structure, weights, sample rate, calibrated level, modeled delay, declared added delay, random-part statistics, capture identities, weight license and error report.

### EU-0010-069 {#EU-0010-069}

euphony MUST load a model into its built-in processors without a new release of euphony.

### EU-0010-070 {#EU-0010-070}

Before activation, the loader MUST refuse a model whose graph holds an operator outside the built-in set, a cycle without delay, a dimension mismatch or a coefficient that is not finite, with the reason.

### EU-0010-071 {#EU-0010-071}

Before activation, the loader MUST refuse a model whose measured resource demand exceeds the capacity measured on the computer, with the reason.

### EU-0010-072 {#EU-0010-072}

The validation of a model MUST run outside the real-time audio processing.

### EU-0010-073 {#EU-0010-073}

A model whose output becomes non-finite while it plays MUST be replaced by its dry signal with a fade.

### EU-0010-202 {#EU-0010-202}

euphony MUST report each model whose output became non-finite while it played.

### EU-0010-074 {#EU-0010-074}

A fault of a model MUST NOT interrupt unrelated recording.

### EU-0010-075 {#EU-0010-075}

A model MUST declare its added delay apart from its modeled delay.

### EU-0010-076 {#EU-0010-076}

euphony MUST measure the total delay of each candidate model at each device period of each target computer.

### EU-0010-077 {#EU-0010-077}

euphony MUST refuse a candidate model whose measured total delay differs from its declared modeled delay plus its declared added delay.

### EU-0010-078 {#EU-0010-078}

Delay compensation and the blend control MUST use the measured added delay of a model.

### EU-0010-079 {#EU-0010-079}

A model of an external chain captured at one setting of its controls MUST copy the chain at that setting.

### EU-0010-240 {#EU-0010-240}

euphony MUST let the musician switch between the models of one external chain captured at different settings of its controls.

### EU-0010-241 {#EU-0010-241}

euphony MUST offer, for an external chain captured at several settings of its controls, a model conditioned on those settings.

### EU-0010-242 {#EU-0010-242}

A model conditioned on control settings MUST report its held-out error at each captured setting.

### EU-0010-243 {#EU-0010-243}

A model conditioned on control settings MUST mark each setting between the captured settings as unverified.

### EU-0010-244 {#EU-0010-244}

Where the gear recalls its control settings digitally, the capture rig MUST step the settings between captures.

### EU-0010-080 {#EU-0010-080}

A model of an external chain MUST offer a control that blends the unprocessed and processed sound.

### EU-0010-081 {#EU-0010-081}

A model played at a sample rate other than the one it was trained at MUST be resampled or refused.

### EU-0010-203 {#EU-0010-203}

euphony MUST report each sample rate change it makes to play a model, and each model it refuses for its sample rate.

### EU-0010-082 {#EU-0010-082}

The random part of a model MUST be a random process with a stored seed, not a copy of one recorded pass.

### EU-0010-083 {#EU-0010-083}

Each rendered region that plays a model MUST store its seed in its cache identity.

### EU-0010-084 {#EU-0010-084}

Two renders of a region with one seed MUST be identical.

### EU-0010-085 {#EU-0010-085}

A render with a new seed MUST produce a new pass with the statistics of the random part of the model.

### EU-0010-086 {#EU-0010-086}

The cache identity of audio that plays a model MUST include the identity of the model object and the processor version.

### EU-0010-087 {#EU-0010-087}

Refining a model MUST NOT invalidate audio that did not use that model.

### EU-0010-088 {#EU-0010-088}

A saved project MUST keep playing the model version it names after a refined version exists.

### EU-0010-089 {#EU-0010-089}

euphony MUST admit a model for live playback only where its measured added delay is within the musician's live latency setting and its measured processor cost is within the capacity measured on that computer.

### EU-0010-090 {#EU-0010-090}

A model that euphony does not admit for live playback MUST be offered for prepared playback.

### EU-0010-091 {#EU-0010-091}

Training, fitting and evaluation of a model MUST run as durable jobs on the owner's paired computers.

### EU-0010-092 {#EU-0010-092}

euphony MUST admit the training, fitting and evaluation jobs of a model on every paired computer that the scheduler admits.

### EU-0010-093 {#EU-0010-093}

The report of a model MUST list the processor cost, the added delay and the missed deadlines of each candidate at each device period.
