## Rendered text and requirement files {#rendering}

The renderer writes, for each section of each written RFC, one text file under the rendered
location, in a directory named as the directory of the RFC, with the name of the section file and
the extension md. The text opens with a level-two heading of the section title and its anchor, then
the prose, then each requirement as a level-three heading whose title and anchor are its identifier,
followed by its statement as one paragraph. For each requirement the renderer also writes one
requirement file under the requirement location, in the directory of the RFC and then of the
section, named of the identifier with the extension toml. The requirement file states its target,
the location of the text with the identifier as its anchor, and one entry of level and quote. The
tracing tool reads the requirement files directly, and doesn't extract requirements from the
text.

### EU-0000-034 {#EU-0000-034}

For each section of each written RFC, the renderer MUST write one text file in the rendered location.

### EU-0000-035 {#EU-0000-035}

The rendered text of a section MUST begin with the section title in a heading that carries its anchor.

### EU-0000-036 {#EU-0000-036}

The rendered text of a section MUST hold its prose after its heading and before its requirements.

### EU-0000-037 {#EU-0000-037}

The rendered text MUST hold each requirement as a heading of its identifier followed by its statement, in the order of the source.

### EU-0000-038 {#EU-0000-038}

The renderer MUST write, for each requirement, one requirement file that targets the anchor of the requirement in the rendered text.

### EU-0000-039 {#EU-0000-039}

A requirement file MUST hold one entry whose level is the requirement's level and whose quote is its statement without the white space at its ends.

### EU-0000-040 {#EU-0000-040}

Two renderings of the same sources MUST produce the same bytes.

### EU-0000-041 {#EU-0000-041}

A generated file MUST NOT be written by anything other than the renderer.

### EU-0000-042 {#EU-0000-042}

The check MUST fail where a generated file differs from a fresh rendering of the sources.
