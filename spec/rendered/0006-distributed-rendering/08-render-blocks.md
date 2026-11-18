## Render blocks {#render-blocks}

A render processes the project graph in render blocks: in each block, each node takes its inputs
for that block and writes that many samples of its output. A sink is the node that writes a
render's result, and the rules of this section hold for sinks as for every other node. An input of
a node is another node's output or a value, such as a constant, which holds as many samples as a
node asks of it. Every render holds the same block length, on a worker and in the live render
island of RFC 0005 alike, and a render whose length isn't a whole number of blocks ends with a
shorter block.

### EU-0006-098 {#EU-0006-098}

A render block other than the last block of a render MUST hold 256 samples of each node output.

### EU-0006-099 {#EU-0006-099}

The last block of a render MUST hold the samples that remain of the render.

### EU-0006-100 {#EU-0006-100}

In each render block, each node, each sink among them, MUST take at most as many samples from each of its inputs as the block holds, whatever the input holds.
