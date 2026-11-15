## Patching, commands and import {#editing}

The patch view shows source sockets and target controls beside a project's tracks and automation
lanes. Dragging a source to a control opens a preview of the route before anything changes. The
command interface offers three commands for routes: one lists the sources and targets of a
project, one previews a route without activating it, and one applies a batch of route and target
edits as one change. Composition programs build routes with the same typed data. Clients follow a
graph through its committed, compiled, activated and failed events. Importing a project from
another workstation brings its sidechains and automation into routes, with a report of each.
RFC 0009 states that the code of a composition program doesn't run on the audio path.

### EU-0012-115 {#EU-0012-115}

The patch view MUST show source sockets and target controls beside the tracks and automation lanes.

### EU-0012-116 {#EU-0012-116}

Connecting a source to a control MUST open a preview that shows the tap, conversion, units, effective range, depth, offset, curve, combination mode, timing capability and any cycle delay.

### EU-0012-117 {#EU-0012-117}

The patch view MUST show a target's base, each contribution and its effective value separately.

### EU-0012-118 {#EU-0012-118}

The patch view MUST show where a value is clipped and where a source is unavailable.

### EU-0012-119 {#EU-0012-119}

The patch view MUST show the physical range a modulated target reaches.

### EU-0012-120 {#EU-0012-120}

Removing, bypassing and undoing a route MUST use the same kind of change as creating it.

### EU-0012-121 {#EU-0012-121}

The patch view MUST distinguish the saved graph from the graph that is playing.

### EU-0012-122 {#EU-0012-122}

The patch view MUST show, for each voice someone watches, its contribution and effective value beside the global contribution.

### EU-0012-123 {#EU-0012-123}

The system MUST NOT publish per-voice effective values of a target that no one watches.

### EU-0012-124 {#EU-0012-124}

The command interface MUST offer a command that lists the modulation sources and targets of a project.

### EU-0012-125 {#EU-0012-125}

The command interface MUST offer a command that previews a modulation.

### EU-0012-126 {#EU-0012-126}

The command interface MUST offer a command that applies a batch of route and target edits as one change.

### EU-0012-127 {#EU-0012-127}

A batch MUST carry its project, the revision it expects, an idempotency identity and its edits.

### EU-0012-128 {#EU-0012-128}

A batch MUST answer with the new revision and the graph's diagnostics, or with a refusal that states its cause: denied, stale revision, missing endpoint, incompatible units or rate, invalid cycle or unavailable resources.

### EU-0012-129 {#EU-0012-129}

A preview MUST NOT activate anything.

### EU-0012-130 {#EU-0012-130}

A composition program MUST build routes with the same typed data as the commands, using stable track and parameter handles.

### EU-0012-132 {#EU-0012-132}

Editors, scripts and assistant clients MUST receive the same committed, compiled, activated and failed events of a graph.

### EU-0012-133 {#EU-0012-133}

A stale compile result MUST NOT replace a newer graph.

### EU-0012-134 {#EU-0012-134}

The same edit made from the interface, a script or an assistant MUST produce the same graph.

### EU-0012-135 {#EU-0012-135}

A malformed batch, a stale revision, an unauthorized edit or an edit that needs more computation than is available MUST fail visibly.

### EU-0012-159 {#EU-0012-159}

A failed batch MUST NOT interrupt unrelated recording.

### EU-0012-136 {#EU-0012-136}

Resource pressure MUST NOT lose an edit.

### EU-0012-137 {#EU-0012-137}

A saved project MUST keep its routes and its original automation apart.

### EU-0012-138 {#EU-0012-138}

Importing a project from another workstation MUST bring its sidechains and automation into routes of the native graph.

### EU-0012-139 {#EU-0012-139}

Imported track and region automation MUST become the base of its target.

### EU-0012-140 {#EU-0012-140}

A sidechain MUST be imported only where the mapping of its source and target processors is verified.

### EU-0012-141 {#EU-0012-141}

An automation mode of another workstation MUST map onto a combination mode only where their equations are verified to match.

### EU-0012-142 {#EU-0012-142}

An import MUST NOT discard an unsupported connection silently.

### EU-0012-143 {#EU-0012-143}

The import report MUST list each imported connection as imported or unsupported, and each unsupported one with its source, target and reason.
