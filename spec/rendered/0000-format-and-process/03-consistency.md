## Relations and the consistency check {#consistency}

Each requirement identifier is an atom of propositional logic, true where the obligation holds in
the target system. A requirement without conditions that isn't a part of a composed requirement is
asserted. A requirement with conditions holds where all of its conditions hold. A relation of kind
depends_on, implies or refines from A to B adds that A implies B, conflicts_with adds that A and B
don't both hold, and supersedes retires its target. A requirement composed as all_of, any_of or
exactly_one holds exactly where all, any or exactly one of its parts hold, and one composed as
unless holds exactly where its exception doesn't. Each logic formula is added as written, over
identifiers, negation, conjunction, disjunction, implication, equivalence and parentheses. Each
parameter a requirement sets has a name, a type, a unit and a bound. A relation to a requirement of
a planned RFC, one the index lists whose directory the set doesn't hold, is a warning until that RFC
is written. The check reads the whole set this way on every change.

### EU-0000-027 {#EU-0000-027}

Each relation and each logic formula MUST name only identifiers of requirements the set holds or of requirements of a planned RFC, one the index lists whose directory the set doesn't hold.

### EU-0000-028 {#EU-0000-028}

The check MUST read the requirements, conditions, compositions, relations and logic formulas of the set as the propositional logic this section states.

### EU-0000-029 {#EU-0000-029}

A change MUST NOT merge while the formulas of the set are unsatisfiable together.

### EU-0000-030 {#EU-0000-030}

Where the formulas of the set are unsatisfiable, the check MUST report a minimal subset of them with the identifier of each requirement it names.

### EU-0000-031 {#EU-0000-031}

Each condition of a requirement MUST be able to hold together with the rest of the set.

### EU-0000-032 {#EU-0000-032}

The relations of kind implies and refines MUST NOT form a cycle.

### EU-0000-058 {#EU-0000-058}

The relations of kind supersedes MUST NOT form a cycle.

### EU-0000-033 {#EU-0000-033}

The bounds that requirements set on one parameter MUST leave at least one value.
