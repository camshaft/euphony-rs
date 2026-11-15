## Text, scale and displays {#text-and-scale}

Geometry is in logical units, converted with the actual scale of the window that shows it. A window
can move between monitors of different scales, and a scale can be fractional. Names and text in a
project can be in any script, written from right to left or from left to right, with combining
marks and glyphs that come from a fallback font. Musical time is a position in the project, not a
position on the screen.

### EU-0018-027 {#EU-0018-027}

The desktop interface MUST lay out each view in logical units converted with the actual scale of its window.

### EU-0018-028 {#EU-0018-028}

A view MUST stay correct when its window moves to a monitor of another scale, is resized, or its display changes.

### EU-0018-029 {#EU-0018-029}

The mapping between a position on screen and musical time MUST NOT round musical time to pixels.

### EU-0018-030 {#EU-0018-030}

Text entry MUST support the operating system's input methods for composing characters.

### EU-0018-031 {#EU-0018-031}

The desktop interface MUST display and edit bidirectional text, combining marks and fallback glyphs with correct selection and caret placement.

### EU-0018-032 {#EU-0018-032}

A font the product includes MUST NOT override the person's system text size or contrast preferences.
