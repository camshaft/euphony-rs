## The conversion report {#report}

The conversion report lists every item of a source project and how it came over, so that nothing is
lost or replaced without the musician seeing it. The musician reads it with the proposed project,
before accepting it, and its states stay visible in the arrangement afterwards. A musician can
accept an import that didn't convert everything, and still hear an unresolved part through a render
they already made, while editing that part stays incomplete.

### EU-0014-063 {#EU-0014-063}

The import MUST report, before the musician accepts it, each feature, control setting and file that does not convert, with its place in the project.

### EU-0014-064 {#EU-0014-064}

The conversion report MUST give each item one of the states complete mapping, known lossy conversion, unsupported feature and missing file.

### EU-0014-065 {#EU-0014-065}

An imported project MUST show in its arrangement each item that didn't convert completely.

### EU-0014-066 {#EU-0014-066}

The musician MAY accept an import as partial.

### EU-0014-067 {#EU-0014-067}

A partial import MUST NOT be reported as preserving the sound completely.

### EU-0014-068 {#EU-0014-068}

A render or an export of an imported project MUST identify each replacement that is still unresolved.

### EU-0014-069 {#EU-0014-069}

An imported project MAY keep the musician's own render of an unresolved part, so that the part can still be heard.
