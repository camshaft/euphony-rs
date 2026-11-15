## Access and untrusted patches {#access}

Modulation uses the project's existing access rules. The owner's paired devices and connected
assistants create routes. A participant who listens or comments receives only the audio of its
assigned output, which already carries every source mixed into it, so a follower, meter or curve
reveals nothing more. A participant who edits patches its own branch copy. A worker is a paired
computer that receives the inputs its job needs. Patches and scripts are untrusted input.

### EU-0012-144 {#EU-0012-144}

Modulation MUST NOT add a permission of its own.

### EU-0012-145 {#EU-0012-145}

A route in the owner's copy of a project MUST be created only by the owner's paired devices and connected assistants.

### EU-0012-146 {#EU-0012-146}

A participant who listens or comments MUST NOT edit routes.

### EU-0012-147 {#EU-0012-147}

A participant who edits MUST patch its own branch copy of the project.

### EU-0012-148 {#EU-0012-148}

A worker MUST receive only the inputs its job needs.

### EU-0012-149 {#EU-0012-149}

A patch or a script MUST NOT pair a device, make a link or take an action that needs the owner's approval without that approval.

### EU-0012-150 {#EU-0012-150}

The protections against unsafe feedback, excessive graph expansion and stale compilation MUST be enforced outside the user interface.

### EU-0012-151 {#EU-0012-151}

Compiling a graph and allocating its resources MUST happen outside the process that drives the audio device.

### EU-0012-152 {#EU-0012-152}

The behavior of routes, conversions, ranges and timing MUST come from project data, not from identities written into code.
