## The set and its configuration {#the-set}

The specification of euphony is one set of RFCs. Each RFC has a number, a title, an area and, once
written, a slug and one or more sections. RFC numbers fall in ranges called groups. The foundation
group holds the format of the set and how the set changes. The system group holds the behavior of
euphony, one RFC for each component of its implementation and one overview RFC for the
requirements every component shares, and states the end state of each component, whether or not an implementation has reached it. The contract group holds each interface
that components deployed apart offer each other. Each group has a name and a range of RFC numbers,
which the configuration file states, and the area of an RFC is the name of the group whose range
holds its number.

The configuration file is the file spec.toml at the root of the set. It states the prefix of every
requirement identifier, the levels, the implementation terms on which the check warns, the location
of the sources and of each kind of generated file, and each group. The index is the file index.toml
in the source directory, and lists every RFC by number, title and area. An RFC is planned until
it is written, and it is written where the set holds its directory, named of its four-digit number, a hyphen and its
slug. That directory holds the metadata file rfc.toml and one section file for each section, named
of a two-digit number, a hyphen and a slug; the number orders the sections. The fields of a metadata
file are number, title, area, abstract, retired and sources. The registry file lists each kind of
self-describing identifier an RFC specifies, and holds comment lines alone until an RFC specifies
one.

### EU-0000-001 {#EU-0000-001}

The set MUST hold exactly one configuration file.

### EU-0000-002 {#EU-0000-002}

The configuration file MUST state the prefix of every requirement identifier of the set.

### EU-0000-003 {#EU-0000-003}

The configuration file MUST state the location of the sources and of each kind of generated file.

### EU-0000-004 {#EU-0000-004}

The configuration file MUST state the name and the number range of each group.

### EU-0000-005 {#EU-0000-005}

The number ranges of two groups MUST NOT overlap.

### EU-0000-006 {#EU-0000-006}

A tool of the set MUST take every name, location, prefix and number range from the configuration file.

### EU-0000-007 {#EU-0000-007}

The index MUST list every RFC of the set by its number, title and area.

### EU-0000-008 {#EU-0000-008}

The area of an RFC MUST be the name of the group whose range holds its number.

### EU-0000-009 {#EU-0000-009}

Each written RFC MUST have one metadata file that states its number, title, area and abstract.

### EU-0000-010 {#EU-0000-010}

The metadata file of an RFC MUST list each public source the RFC was written from.

### EU-0000-011 {#EU-0000-011}

Each behavior of euphony MUST be specified in exactly one RFC.

### EU-0000-012 {#EU-0000-012}

Each RFC of the system group MUST cover one component of the implementation, or the requirements every component shares.
