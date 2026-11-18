## Render blocks {#render-blocks}

A render advances the project graph from one instruction, such as a node's start, a parameter
change or a node's end, to the next, in render blocks: in each block, each node takes its inputs
for that block and writes that many samples of its output. A render block holds 256 samples, and
the block that reaches the render's next instruction or the render's end holds the samples that
remain up to it. A sink is the node that writes a render's result, and the rules of this section
hold for sinks as for every other node. An input of a node is another node's output or a value,
such as a constant, which holds as many samples as a node asks of it. Every render holds the same
block length, on a worker and in the live render island of RFC 0005 alike.

### EU-0006-098 {#EU-0006-098}

A render block MUST hold at most 256 samples of each node output.

### EU-0006-099 {#EU-0006-099}

A render block MUST hold fewer than 256 samples only where it ends at the render's next instruction or at the render's end.

### EU-0006-101 {#EU-0006-101}

A render block that ends at the render's next instruction or at the render's end MUST hold the samples that remain up to that point.

### EU-0006-100 {#EU-0006-100}

In each render block, each node, each sink among them, MUST take at most as many samples from each of its inputs as the block holds, whatever the input holds.
