## The receiving service {#receiving-service}

The receiving service is the service the project runs to take in what people send. It ingests
signed batches and problem reports, stores them, groups their records into findings, holds the
findings and their aggregates, shows them to the maintainers, and files tracker issues. A tombstone
is the record that an identity's data was deleted. Deploying the service is an act that people
outside the project can see.

### EU-0021-099 {#EU-0021-099}

The receiving service MUST be public code built and released from euphony's public repository.

### EU-0021-100 {#EU-0021-100}

The receiving service MUST be deployed only with the approval of the project's owner.

### EU-0021-101 {#EU-0021-101}

The service MUST bind each submission identity to the public key that its first batch registers.

### EU-0021-102 {#EU-0021-102}

The service MUST refuse a batch with a missing or wrong signature.

### EU-0021-103 {#EU-0021-103}

The service MUST refuse a batch whose identity it already stored with other contents.

### EU-0021-104 {#EU-0021-104}

The service MUST drop an exact replay of a stored batch.

### EU-0021-105 {#EU-0021-105}

The service MUST refuse a batch that holds an unknown field, an unknown schema version or a record of the wrong class.

### EU-0021-106 {#EU-0021-106}

The service MUST NOT keep a client's network address in any record or log.

### EU-0021-107 {#EU-0021-107}

The service MUST NOT set a cookie.

### EU-0021-108 {#EU-0021-108}

Once the comparison of the next release is published, the service MUST reduce the raw events of a release to aggregate counts by release and hardware class without an identity.

### EU-0021-109 {#EU-0021-109}

The service MUST delete a report's attachments once its finding closes and the next release's comparison is published, or once the person deletes them.

### EU-0021-110 {#EU-0021-110}

On a deletion request signed with an identity's private key, the service MUST delete everything it holds under that identity: raw events, batches, problem reports, attachments, its entries in the findings' identity index, grouping work still queued for its batches, and its copies in backups, each backup copy no later than that backup's expiry.

### EU-0021-111 {#EU-0021-111}

The service MUST record a tombstone for an identity before it deletes the identity's data.

### EU-0021-112 {#EU-0021-112}

The service MUST refuse every later upload, retry or grouping job of an identity that has a tombstone.

### EU-0021-173 {#EU-0021-173}

The service MUST NOT restore from a backup any data of an identity that has a tombstone.

### EU-0021-113 {#EU-0021-113}

The receipt of a deletion MUST say which stores are deleted and which still hold a copy, with the time each remaining copy ends.

### EU-0021-114 {#EU-0021-114}

Aggregate counts and tracker issues that don't hold an identity MAY remain after a deletion.

### EU-0021-115 {#EU-0021-115}

When it is congested, the service MUST answer with a retry signal.

### EU-0021-116 {#EU-0021-116}

The service MUST NOT set a sending limit for each client.

### EU-0021-117 {#EU-0021-117}

On a retry signal, the client MUST keep the batch in its outbox and send it again later with growing intervals between attempts.

### EU-0021-118 {#EU-0021-118}

The client MUST NOT drop a batch for want of capacity at the service.

### EU-0021-119 {#EU-0021-119}

Under storage pressure on the computer, the outbox MUST remove the oldest telemetry batches first.

### EU-0021-120 {#EU-0021-120}

The performance and problems view MUST say when the outbox removed batches under storage pressure.

### EU-0021-172 {#EU-0021-172}

The receiving service's keys and the accounts of its maintainers MUST be configuration data.
