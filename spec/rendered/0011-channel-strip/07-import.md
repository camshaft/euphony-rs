## Strips from other workstations {#import}

The project import of RFC 0014 brings in the channel strips of another workstation's project. An
imported processor goes into the stage of its kind only where the whole chain allows it without
changing the sound; the stage then keeps the imported values as a fixed setting, so the strip sounds
as the mapping measured, and Dial in is offered rather than applied. Every other processor goes into
a free slot in its original order. The import report lists what was mapped, approximated and left
unmapped, and why a processor stayed out of a stage.

### EU-0011-146 {#EU-0011-146}

An imported processor MUST go into the strip stage of its kind only where all of these hold: a verified mapping covers every parameter it uses, the stage can represent its setting, it is the only processor of that kind in the chain, and placing it keeps the order and routing of the chain.

### EU-0011-147 {#EU-0011-147}

Where any placement condition fails, the whole imported chain MUST stay in the free slots in its original order, with the stages off.

### EU-0011-148 {#EU-0011-148}

The import report MUST say which placement condition failed.

### EU-0011-149 {#EU-0011-149}

A processor placed in a stage MUST keep its imported values as a fixed setting.

### EU-0011-150 {#EU-0011-150}

An import MUST NOT dial in a stage that holds imported values.

### EU-0011-151 {#EU-0011-151}

A stage that holds imported values MUST offer Dial in.

### EU-0011-152 {#EU-0011-152}

An imported compressor's circuit type MUST NOT map to a character except through a verified mapping.

### EU-0011-153 {#EU-0011-153}

Each imported processor that doesn't go into a stage MUST go into a free slot in its original order, as its mapped built-in processor or as a reported placeholder that keeps its state.

### EU-0011-154 {#EU-0011-154}

The import report MUST list each strip's mapped, approximated and unmapped stages.

### EU-0011-155 {#EU-0011-155}

The import report MUST list each mapping of screen controls to parameters that the source holds and the import doesn't carry.

### EU-0011-156 {#EU-0011-156}

The musician MUST be able to save an imported strip as a fixed preset.

### EU-0011-157 {#EU-0011-157}

Two imports of the same source MUST produce the same strips.
