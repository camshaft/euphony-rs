## Learning work and workflow mechanisms {#mechanisms}

Learning work records a scoped problem: its evidence, its proposer, its status, the external
assistant it is assigned to and its typed dependencies on decisions, tests or other work. Repeated
manual steps, repeated corrections and forgotten checks become findings of learning work. Euphony
keeps this work and serves it whenever an assistant attaches; while no assistant is attached, the
work waits where the owner sees it. A mechanism version has a kind: template, default, macro,
composition program, validation check or event rule. It holds its input and output schema, its
applicability predicates, its trigger event types, the digest of its definition, the capabilities
it needs, a preview of its effects, its test specification and results, its baseline metrics, its
proposer, its approval, its prior version and its rollback operation. A template or a default
changes newly requested work and leaves earlier music as it was. A mechanism is tested on a pinned
copy of the project and activated by the owner's accept command, and each of its invocations is
authorized again as it runs. The executor runs active mechanisms through the command gateway; it
isn't a scheduler of assistants or a model runtime.

### EU-0019-103 {#EU-0019-103}

Learning work MUST record its problem, evidence, proposer, status, assigned assistant and typed dependencies.

### EU-0019-104 {#EU-0019-104}

Duplicate findings MUST join the same learning work by an explicit key and the affected workflow or domain.

### EU-0019-105 {#EU-0019-105}

Similar prose alone MUST NOT join two findings.

### EU-0019-106 {#EU-0019-106}

A finding MUST NOT be an executable command or an approval.

### EU-0019-107 {#EU-0019-107}

Euphony MUST serve pending learning work to an assistant when the assistant attaches.

### EU-0019-108 {#EU-0019-108}

Euphony MUST NOT start an assistant to work on learning work.

### EU-0019-109 {#EU-0019-109}

Pending learning work MUST stay visible to the owner while no assistant is attached.

### EU-0019-110 {#EU-0019-110}

A mechanism version MUST hold its kind, its schemas, its predicates, its trigger event types, its definition digest, its required capabilities, its effect preview, its tests and results, its baseline metrics, its proposer, its approval, its prior version and its rollback operation.

### EU-0019-111 {#EU-0019-111}

A template or default mechanism MUST NOT rewrite existing music.

### EU-0019-112 {#EU-0019-112}

A macro or a composition program MUST use the same typed commands and the same restricted runner as every other client.

### EU-0019-113 {#EU-0019-113}

A validation check MUST return a structured refusal with an explanation the caller can read.

### EU-0019-114 {#EU-0019-114}

An event rule MUST invoke only the commands it declares.

### EU-0019-115 {#EU-0019-115}

A mechanism proposal MUST NOT change the command gateway, the approval operations or its own authority.

### EU-0019-116 {#EU-0019-116}

A mechanism MUST NOT carry new native code.

### EU-0019-117 {#EU-0019-117}

A mechanism proposal MUST reference learning work of its subject and project scope that the caller can read.

### EU-0019-118 {#EU-0019-118}

A mechanism proposal whose learning work reference is missing, inaccessible or of another scope MUST be refused without being stored.

### EU-0019-119 {#EU-0019-119}

A refusal for an inaccessible learning work reference MUST NOT reveal whether that work exists.

### EU-0019-120 {#EU-0019-120}

Testing a mechanism MUST run its exact digest on a pinned copy of the project with deterministic inputs and simulated dependencies.

### EU-0019-121 {#EU-0019-121}

A mechanism test MUST NOT use real devices, write outside its pinned copy or export anything.

### EU-0019-122 {#EU-0019-122}

A test result MUST hold the command diff, an audio comparison where audio changes, the failures, the resource use and a replay of the rollback.

### EU-0019-123 {#EU-0019-123}

Activating a mechanism MUST be the owner's accept command.

### EU-0019-124 {#EU-0019-124}

Activation MUST be refused unless the exact version digest, the current scope, the test profile and a passing result match the preview the owner accepted.

### EU-0019-125 {#EU-0019-125}

An assistant, a script or a mechanism MUST NOT activate a mechanism.

### EU-0019-126 {#EU-0019-126}

An import, an update or a rollback of a mechanism MUST repeat the activation checks against the current runtime.

### EU-0019-127 {#EU-0019-127}

A change to a mechanism's definition MUST invalidate its tests and its acceptance.

### EU-0019-128 {#EU-0019-128}

Euphony MUST authorize each invocation of a mechanism again as it runs.

### EU-0019-129 {#EU-0019-129}

An invocation whose effect needs the owner's approval of the exact effect MUST wait for that approval of that invocation.

### EU-0019-130 {#EU-0019-130}

Activating a mechanism MUST NOT approve an effect of a later invocation in advance.

### EU-0019-131 {#EU-0019-131}

A macro whose combined steps hold an effect that needs approval MUST receive that approval for its combined preview before any effect commits.

### EU-0019-132 {#EU-0019-132}

The executor MUST deduplicate invocations by mechanism version, trigger event and operation identity.

### EU-0019-133 {#EU-0019-133}

Each event a mechanism generates MUST carry its causal chain.

### EU-0019-134 {#EU-0019-134}

Euphony MUST reject a cycle of triggers and effects that lacks a validated finite execution plan.

### EU-0019-135 {#EU-0019-135}

A failed invocation MUST roll back its command transaction.

### EU-0019-136 {#EU-0019-136}

A failed invocation MUST suspend the mechanism that failed.

### EU-0019-137 {#EU-0019-137}

Ordinary editing MUST stay available after a mechanism invocation fails.

### EU-0019-200 {#EU-0019-200}

Euphony MUST derive the rate and concurrency of learning work and of transfers from the answers of the resources they use.

### EU-0019-201 {#EU-0019-201}

Accepted learning work past the rate its resources answer MUST stay queued.

### EU-0019-211 {#EU-0019-211}

Transfers past the rate their resources answer MUST stay queued.

### EU-0019-138 {#EU-0019-138}

The protection of recording and audible output MUST take precedence over any mechanism.

### EU-0019-139 {#EU-0019-139}

A storage or capacity failure MUST keep accepted learning work, visible to the owner.
