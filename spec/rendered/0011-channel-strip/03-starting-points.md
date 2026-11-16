## Starting points {#starting-points}

Switching a stage on sets it to a starting point calculated for that track. The calculation is a
versioned, deterministic function of recorded inputs: the track's role, such as lead vocal, kick or
bass; the analysis results of the track's audio as it reaches the stage, as RFC 0020 specifies them;
the targets of the role profile for that role, genre and character; the musician's approved
preferences for strip settings, as RFC 0019 specifies them; the calculation's version; and the
stage's processor version. A role profile is a versioned record of the project that states, for each
role and optionally a genre, the target each stage's calculation aims for. Each stage's rule works
from these inputs with defaults from common mixing practice, which the musician changes in the
profile or with a preference. Every starting point is matched in loudness with the stage off, so it
sounds different, not only louder. Pan and fader stay the musician's, since the balance of a mix is
theirs. A track that has no audio yet starts from its role's defaults, and Listen measures a moment
of the live input to dial in from it.

### EU-0011-037 {#EU-0011-037}

Switching a stage on MUST set it to a starting point calculated for that track.

### EU-0011-038 {#EU-0011-038}

A stage's starting point MUST be a deterministic function of exactly these inputs: the track's role, the analysis results of the track's audio as it reaches the stage, the role profile's targets, the musician's approved preferences for strip settings, the calculation's version and the stage's processor version.

### EU-0011-039 {#EU-0011-039}

The strip MUST record each input of a stage's starting point with the stage.

### EU-0011-040 {#EU-0011-040}

Two calculations with the same inputs MUST return the same starting point on every computer of a compatible numerical profile.

### EU-0011-041 {#EU-0011-041}

A track's role MUST be one the musician set, one a preset or an import set, or an instrument candidate of analysis that the musician accepted.

### EU-0011-042 {#EU-0011-042}

The musician MUST be able to accept an instrument candidate of analysis as the track's role with one action.

### EU-0011-043 {#EU-0011-043}

An instrument candidate of analysis MUST NOT be used as a track's role before the musician accepts it.

### EU-0011-044 {#EU-0011-044}

A track without a role MUST use the general role profile.

### EU-0011-045 {#EU-0011-045}

The analysis results of a starting point MUST be measured on the track's audio as it reaches the stage, after the stages before it.

### EU-0011-046 {#EU-0011-046}

A starting point MUST NOT use an analysis result that analysis marks unsupported or below its accepted confidence.

### EU-0011-047 {#EU-0011-047}

Where an analysis result is missing or unsupported, the stage MUST use the role's default for that input.

### EU-0011-048 {#EU-0011-048}

A stage MUST report each input for which its starting point used the role's default.

### EU-0011-049 {#EU-0011-049}

The musician's approved preferences for strip settings MUST override the role profile's targets within the preferences' scope.

### EU-0011-050 {#EU-0011-050}

Where approved preferences conflict and the conflict is unresolved, the starting point MUST use the role profile's target.

### EU-0011-051 {#EU-0011-051}

The strip MUST show each unresolved conflict of preferences that a starting point met.

### EU-0011-052 {#EU-0011-052}

A role profile MUST state, for its role and optionally a genre and a character, the target each stage's calculation aims for.

### EU-0011-053 {#EU-0011-053}

Role profiles MUST be versioned records that the project authors and releases.

### EU-0011-054 {#EU-0011-054}

The released role profiles SHOULD cover at least lead vocal, backing vocal, kick, snare, drum overheads, bass, electric guitar, acoustic guitar, piano, pad and a general role.

### EU-0011-055 {#EU-0011-055}

A starting point MUST make the track's loudness through the stage equal its loudness with the stage off, within the level-match tolerance.

### EU-0011-056 {#EU-0011-056}

The level-match tolerance MUST be a setting the musician changes.

### EU-0011-057 {#EU-0011-057}

The level-match tolerance SHOULD default to 0.5 loudness units.

### EU-0011-058 {#EU-0011-058}

The input stage's starting trim SHOULD bring the track's short-term loudness to the strip's operating level.

### EU-0011-059 {#EU-0011-059}

The strip's operating level SHOULD default to 18 decibels below full scale.

### EU-0011-060 {#EU-0011-060}

The low cut's starting point SHOULD sit just below the lowest sustained fundamental of the track, or below the low edge of its spectrum where no pitch is found.

### EU-0011-061 {#EU-0011-061}

The starting point MUST NOT switch the low cut on for the kick and bass roles.

### EU-0011-062 {#EU-0011-062}

The starting point MUST NOT switch the high cut on unless the role profile or an approved preference asks for it.

### EU-0011-063 {#EU-0011-063}

The starting point MUST NOT switch the gate on unless analysis finds a steady level of bleed or noise between onsets.

### EU-0011-064 {#EU-0011-064}

Where the gate stays off at its starting point, the stage MUST report why.

### EU-0011-065 {#EU-0011-065}

The gate's starting threshold SHOULD sit between the level of the bleed and the level of the onsets.

### EU-0011-066 {#EU-0011-066}

The gate's starting range SHOULD reduce the bleed without silencing it.

### EU-0011-067 {#EU-0011-067}

The gate's starting hold SHOULD follow the tempo.

### EU-0011-068 {#EU-0011-068}

The equalizer's starting point SHOULD fit the difference between the track's long-term spectrum and the role's target curve with its four bands.

### EU-0011-069 {#EU-0011-069}

Each equalizer band's starting gain MUST stay within the role profile's bound.

### EU-0011-070 {#EU-0011-070}

The role profile's bound on an equalizer band's starting gain SHOULD default to 4 decibels.

### EU-0011-071 {#EU-0011-071}

A peaking band that the broad fit doesn't need SHOULD take the strongest resonance that stands above its neighbors, narrowed for a cut.

### EU-0011-072 {#EU-0011-072}

Where more corrections qualify than four bands hold, the starting point MUST keep the set that most reduces the difference from the target.

### EU-0011-073 {#EU-0011-073}

The equalizer MUST report each correction its four bands leave out.

### EU-0011-074 {#EU-0011-074}

The response that the starting point calculates MUST equal the equalizer's rendered response.

### EU-0011-075 {#EU-0011-075}

The starting point MUST NOT cut a resonance at a note of the song's key or at its octaves unless the musician asks.

### EU-0011-076 {#EU-0011-076}

The compressor's starting threshold SHOULD bring the loud passages to the role's target of average gain reduction.

### EU-0011-077 {#EU-0011-077}

The compressor's starting ratio SHOULD follow the role and the track's loudness range.

### EU-0011-078 {#EU-0011-078}

The compressor's starting attack SHOULD follow the sharpness of the onsets, slower where the transients carry the part.

### EU-0011-079 {#EU-0011-079}

The compressor's starting release SHOULD follow the tempo, near an eighth note, within the role profile's range.

### EU-0011-080 {#EU-0011-080}

The de-esser's starting point SHOULD center it on the band between about 4 and 10 kilohertz where short events stand above that band's long-term level.

### EU-0011-081 {#EU-0011-081}

The de-esser's starting threshold SHOULD reduce those events by the role's target reduction.

### EU-0011-180 {#EU-0011-180}

The starting point SHOULD switch the de-esser on only for vocal roles.

### EU-0011-082 {#EU-0011-082}

The color stage's starting drive SHOULD be light, as the character and the role set it.

### EU-0011-083 {#EU-0011-083}

Switching a send on SHOULD route it to the project's shared reverb or delay return, created from the role profile where the project has none.

### EU-0011-181 {#EU-0011-181}

A send's starting amount SHOULD follow the track's role.

### EU-0011-182 {#EU-0011-182}

The starting delay time of a delay return that a send creates SHOULD follow the tempo.

### EU-0011-084 {#EU-0011-084}

A starting point MUST NOT set pan or the fader.

### EU-0011-085 {#EU-0011-085}

Where a stage's input needs measuring again, the stage MUST pass its audio unchanged until its starting point is ready.

### EU-0011-086 {#EU-0011-086}

A stage whose starting point isn't ready MUST show that it is dialing in.

### EU-0011-087 {#EU-0011-087}

A switch and its starting point MUST be committed to the project together, as one change.

### EU-0011-088 {#EU-0011-088}

A starting point calculated for an earlier revision of the track MUST NOT be committed.

### EU-0011-089 {#EU-0011-089}

A track without audio MUST start from its role's defaults.

### EU-0011-090 {#EU-0011-090}

The strip MUST offer Listen on a track without audio, which measures a moment of the live input and dials the stages in from it.

### EU-0011-091 {#EU-0011-091}

Audio that Listen measures MUST NOT be recorded unless the musician records.

### EU-0011-092 {#EU-0011-092}

The musician MUST be able to dial a stage in on several tracks with one command.

### EU-0011-093 {#EU-0011-093}

The musician and an assistant MUST be able to preview a stage's starting point before it applies.
