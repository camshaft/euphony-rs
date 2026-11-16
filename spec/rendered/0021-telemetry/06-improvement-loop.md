## Findings and the improvement loop {#improvement-loop}

A finding is a group of records that share a finding key, which the service reads from typed fields
alone. Each finding has one tracker issue, which is private to the maintainers until the publication
policy lets the service publish it on the project's public issue tracker. The publication threshold
is the least count of reporting identities that a published combination of finding key, release and
hardware class needs, and the project sets it. A valid signature shows only that a batch came from
the holder of its identity's key, so a sender can still invent measurements or make many
identities: an anomaly is a provisional signal and never proof of forgery, and each result is shown
with and without the identities that the project's anomaly policy marks. A comparison reads the
records of one release against the release before it. A fix-effect report reads the rate of a
finding before and after the release that fixes it, over the exposure that fits the work that can
fail, and only over complete exposure intervals. A complete interval is one whose family recorded
throughout, which didn't lose an event, for which the service received every batch the client
built, and whose session ended normally or with the crash it records.

### EU-0021-121 {#EU-0021-121}

The service MUST assign each record a finding key from its typed fields alone.

### EU-0021-122 {#EU-0021-122}

The finding key of a crash MUST be its process, its exception kind and the frames of euphony's own code on the faulting thread.

### EU-0021-123 {#EU-0021-123}

The finding key of an error MUST be its interface, its operation and its result kind.

### EU-0021-124 {#EU-0021-124}

The finding key of a tool call MUST be its tool and its result kind.

### EU-0021-125 {#EU-0021-125}

The finding key of a performance record MUST be its measure, its component and its hardware class.

### EU-0021-126 {#EU-0021-126}

A finding MUST count its cases and its distinct submission identities by release.

### EU-0021-127 {#EU-0021-127}

Each comparison MUST state that a count of distinct submission identities isn't a verified count of computers.

### EU-0021-128 {#EU-0021-128}

The service MUST keep one tracker issue for each finding key.

### EU-0021-129 {#EU-0021-129}

A later case of a finding MUST add to that finding's count instead of filing a new issue.

### EU-0021-130 {#EU-0021-130}

A closed finding that recurs in a later release MUST open again.

### EU-0021-131 {#EU-0021-131}

The service MUST NOT publish a finding until the publication policy passes.

### EU-0021-132 {#EU-0021-132}

Each published combination of finding key, release and hardware class MUST hold cases from at least the publication threshold's count of reporting identities.

### EU-0021-133 {#EU-0021-133}

A combination below the publication threshold MUST be published only with its release or hardware class generalized to a coarser level that meets the threshold.

### EU-0021-134 {#EU-0021-134}

A published trail MUST hold only the event kinds present in cases from at least the publication threshold's count of identities.

### EU-0021-135 {#EU-0021-135}

A later update of a published finding MUST publish new counts only where the change itself comes from at least the publication threshold's count of identities.

### EU-0021-136 {#EU-0021-136}

While the project hasn't set a publication threshold, every finding MUST stay private.

### EU-0021-137 {#EU-0021-137}

Where the publication policy can't establish that a finding meets it, the finding MUST stay private until a maintainer reviews it.

### EU-0021-138 {#EU-0021-138}

A maintainer MUST NOT publish a part of a finding that the publication policy doesn't pass without the consent of each reporter whose cases it holds.

### EU-0021-139 {#EU-0021-139}

Each comparison and each fix-effect report MUST be computed with all records and again without the identities that the anomaly policy marks, and show both results with the count left out.

### EU-0021-140 {#EU-0021-140}

The anomaly policy MUST be project configuration.

### EU-0021-141 {#EU-0021-141}

Where the two results disagree on the direction of a change, the result MUST be reported as unverified.

### EU-0021-142 {#EU-0021-142}

An unverified result MUST NOT confirm a fix.

### EU-0021-143 {#EU-0021-143}

A timing comparison MUST use only records with the same job kind, a matching frame count and the same cache result.

### EU-0021-144 {#EU-0021-144}

For each measure and hardware class, the service MUST compare the distribution in a release with the distribution in the release before it, with the count of submission identities and an interval of uncertainty.

### EU-0021-145 {#EU-0021-145}

The service MUST flag a regression where the worse direction passes the margin that the project configures.

### EU-0021-146 {#EU-0021-146}

A hardware class with too few computers to give an interval MUST be reported as unverified.

### EU-0021-147 {#EU-0021-147}

Each comparison MUST state its coverage.

### EU-0021-148 {#EU-0021-148}

Each comparison MUST state any change in who sends, such as a new release cohort or another mix of hardware classes.

### EU-0021-149 {#EU-0021-149}

Regression detection MUST run when a stored batch completes the data it needs.

### EU-0021-150 {#EU-0021-150}

The issue of a fix MUST state the finding key it fixes.

### EU-0021-151 {#EU-0021-151}

Before the release with a fix, the project MUST declare the criterion that decides the fix's effect: the exposure that fits the finding, the least exposure and the confidence level.

### EU-0021-152 {#EU-0021-152}

The exposure that fits a finding MUST be transport-active hours for a deadline miss, render jobs of the finding's job kind for a render failure, attempts of the finding's tool for a tool failure, and hours with the finding's process running for a crash.

### EU-0021-153 {#EU-0021-153}

An exposure interval sent again in a later batch MUST count once.

### EU-0021-154 {#EU-0021-154}

The service MUST leave each incomplete exposure interval out of both the cases and the exposure.

### EU-0021-155 {#EU-0021-155}

Each fix-effect report MUST state how much exposure it left out as incomplete.

### EU-0021-156 {#EU-0021-156}

Once the release with a fix has reports, the service MUST post the change in the finding's rate with its interval of uncertainty, the exposure and the cohort of each release.

### EU-0021-157 {#EU-0021-157}

The service MUST close a fixed finding only where its declared criterion is met.

### EU-0021-158 {#EU-0021-158}

Zero cases over less exposure than the criterion requires MUST be reported as not observed over this exposure.

### EU-0021-159 {#EU-0021-159}

A performance fix MUST report the change in its measure as a fix-effect report does.

### EU-0021-160 {#EU-0021-160}

A fix-effect report without comparable evidence MUST say that the fix's benefit is unverified.

### EU-0021-161 {#EU-0021-161}

The notes of each release MUST list the findings it fixed and their measured effect.

### EU-0021-162 {#EU-0021-162}

After a release, the service MUST post its comparison on the release's tracking issue.
