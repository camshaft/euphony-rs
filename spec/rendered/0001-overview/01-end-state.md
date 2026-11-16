## What euphony is {#end-state}

Euphony is a music workstation that one owner runs across the computers they own. It combines
familiar track work, recording, arranging and mixing, with programmable composition, and its sound
processing is part of the product. A session runs in an engine on the owner's computers that keeps
running apart from every interface. The desktop interface, a command line, scripts, a browser
companion, phones and control surfaces, and the assistants the owner connects are all clients of
that engine, and each takes the same session actions with the same results. Audio that can be
prepared before playback is rendered and kept across the owner's computers, while what the musician
plays live is rendered on the computer attached to the speakers, so that network delay doesn't reach
live playing. Other people reach a project only through a share link the owner makes.

This RFC holds what every component shares: this end state, the requirements every component meets,
the rules for the public source, and the trust model. Each other RFC of the system group states one
component: RFC 0002 the engine and the session host, RFC 0003 the client protocol and the assistant
adapter, RFC 0004 the project model, its commands and its history, RFC 0005 the audio device and the
live render, and RFCs 0006 to 0023 the other aspects of the product, as the index lists them. The
first version of euphony doesn't load third-party plug-ins in a traditional plug-in format, and
doesn't include a text terminal interface or a layout that copies another workstation's arrangement.

### EU-0001-001 {#EU-0001-001}

Euphony MUST let one owner run a session across the computers the owner has paired.

### EU-0001-002 {#EU-0001-002}

Euphony MUST offer programmable composition in the same project as recording, arranging and mixing tracks.

### EU-0001-003 {#EU-0001-003}

Every interface and every assistant MUST reach a session as a client of the engine that runs the session.
