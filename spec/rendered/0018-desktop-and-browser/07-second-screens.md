## Second screens and touch monitors {#second-screens}

A second screen is another window or view of the session on the same computer, and a touch monitor
reports each contact apart. Both are surfaces as RFC 0017 defines them, and its requirements on a
surface's authority, on touch contacts and on gestures hold for them. Phones, tablets and touch
monitors share layout schemas, parameter ranges, gesture meanings and conformance cases with the
desktop interface. An assistant works through typed commands and surface layouts.

### EU-0018-055 {#EU-0018-055}

A layout MUST adapt its density, safe areas, orientation and touch targets to the actual device.

### EU-0018-056 {#EU-0018-056}

An assistant MUST NOT act on a view through pixels or injected pointer events.

### EU-0018-057 {#EU-0018-057}

A drawn panel on a phone or in a browser MUST NOT replace the semantic access that its platform's own controls provide.
