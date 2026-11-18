# euphony specification

This directory holds the specification of euphony: small RFCs that state the end state of each
component of euphony as requirements with stable identifiers. RFC 0000 states the format of the set
and how it changes, RFC 0001 holds the requirements every component shares, and each other RFC
covers one component.

- `spec.toml`: the configuration of the set: the identifier prefix, the levels, the locations and the RFC number groups.
- `rfcs/index.toml`: every RFC by number, title and area.
- `rfcs/<nnnn>-<slug>/`: a written RFC, its metadata in `rfc.toml` and one file for each section.
- `rendered/`: the text of each section, generated from the sources.
- `.duvet/requirements/`: one requirement file for each requirement, generated from the sources, which duvet reads.
- `.duvet/config.toml`: the duvet configuration that reports on those requirements.
- `registry/identifiers.toml`: the kinds of self-describing identifier the RFCs specify.

The sources are written with partimento: `partimento render` writes the generated files, and
`partimento check` checks the set. Don't edit a generated file by hand.

A change is a pull request against the default branch. It is reviewed by someone other than its
writer, its check passes, and then its writer merges it.
