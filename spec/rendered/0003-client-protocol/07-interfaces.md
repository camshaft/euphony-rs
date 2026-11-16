## The command line, the assistant adapter and other interfaces {#interfaces}

The command line sends one command, or a batch of commands from a file, and prints the typed result
of each, for scripts, tests and use without a window. The assistant adapter offers each command of
the session to assistants as a tool with the same parameters, validation and results, and every
command it forwards carries the origin of an assistant. The terminal interface offers the transport,
a track list with meters, a mixer, a command prompt for the session and a log of refusals, and edits
through the same commands as every other client. The desktop interface reads its layout, the
arrangement of its views, panels and tools, and its theme, its colors, fonts and icons, as data over
views that each have a semantic identity, so a reskinned desktop interface is the same client with
other data. The editor of a built-in processor is a view that each client draws from the typed
parameters of the processor and its declarative panel description, so a phone, a terminal and a
desktop interface on another computer show the same controls.

### EU-0003-091 {#EU-0003-091}

The command line MUST send a single command, or a batch of commands read from a file.

### EU-0003-092 {#EU-0003-092}

The command line MUST print the typed result of each command it sends.

### EU-0003-093 {#EU-0003-093}

The command line MUST work while no other client is attached.

### EU-0003-094 {#EU-0003-094}

The assistant adapter MUST offer each command of the session to assistants as a tool.

### EU-0003-095 {#EU-0003-095}

Each tool of the assistant adapter MUST have the parameters, the validation and the results of the command it carries.

### EU-0003-096 {#EU-0003-096}

A command that the assistant adapter forwards MUST carry the origin of an assistant.

### EU-0003-097 {#EU-0003-097}

The terminal interface MUST offer the transport, a track list with meters, a mixer, a command prompt for the session and a log of refusals.

### EU-0003-098 {#EU-0003-098}

The terminal interface MUST stay usable on a terminal without color.

### EU-0003-099 {#EU-0003-099}

A layout or a theme of the desktop interface MUST be data that the interface reads.

### EU-0003-100 {#EU-0003-100}

Switching the layout or the theme MUST NOT change the project, any command or the audio.

### EU-0003-101 {#EU-0003-101}

A layout MUST refer to views only by their semantic identity.

### EU-0003-102 {#EU-0003-102}

The editor of a built-in processor MUST be drawn by the client from the typed parameters of the processor and its declarative panel description.

### EU-0003-103 {#EU-0003-103}

Every client MUST be able to show the editor of a built-in processor, whatever computer runs the processor.
