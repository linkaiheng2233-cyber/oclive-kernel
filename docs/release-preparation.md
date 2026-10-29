# Source-preview release preparation

This checklist and text template do not prove publication. The three package manifests share
`0.1.0-preview.1` with `publish = false`; [CHANGELOG](../CHANGELOG.md) records the prepared source preview.
The authoritative publication record is the exact tag/commit and GitHub prerelease with its check URLs.
Historical checkpoint tags must keep their existing targets.

## Before a distribution is announced

1. Review the source/CI/documentation diff, freeze its full SHA and run the applicable checks in
   [CONTRIBUTING](../CONTRIBUTING.md). Keep local evidence attached to that SHA.
2. Review the documented security route and any intended GitHub private-reporting setting separately.
   A policy file does not enable that feature. Do not promise a response deadline without capacity.
3. With maintainer authorization, push the frozen source and obtain remote checks for that exact SHA.
   Verify the two OS jobs and strict default/off docs; record the separately run dependency audit and
   advisory-fetch result. A parent SHA's badge is insufficient. Do not turn a failed advisory fetch into a pass.
4. Confirm that About/topics describe code a visitor can actually obtain. Only then apply the metadata.
5. Use `v0.1.0-preview.1` for the source preview, matching package version `0.1.0-preview.1`.
   Verify manifests/lock before freezing; the label in this plan does not create the tag or release.
   Do not force a `0.2.0` bump solely for the canonical rename that preserves source aliases.
6. Review tag target and release text before creation. Mark a preview as prerelease. Do not publish to
   crates.io, change historical tags, or migrate original OCLive's immutable dependency pin as a side effect.

## Proposed repository metadata

About:

> Six independent Rust Base contracts with a default-on optional character profile. Host-owned composition; no scheduler or model adapter.

Topics: `rust`, `ai-agents`, `character-ai`, `kernel`, `contracts`, `traits`.

An optional later social-preview image can use a 1280×640 canvas with project name, one sentence and
the six-slot/profile relationship. Legibility at thumbnail size takes priority over decorative detail.
Artwork is separate from this source/documentation change.

## Release description template

Replace all placeholders with verified values; do not publish this template unchanged.

```text
OCLive Kernel — source preview <version>
Source: <full frozen SHA>

Snapshot: six independent Base contracts; default-on optional character profile;
two deterministic Hosts and a Host-owned local-future integration example.
Profile content: persona + visual references, seven images recommended and one required.

Validation: <local command evidence>; <Windows/Linux CI URLs at this SHA>;
<dependency audit evidence and database fetch status>.
Original OCLive compatibility: <exact consumer SHA / patch or migrated dependency mode>.

Limits: no model inference, scheduler, resource renderer or stable binary ABI supplied.
Host behavior and full product acceptance remain separate. Package publication is disabled.
Maintenance/security route: SECURITY.md. Changes: CHANGELOG.md.
```

The profile revision's earlier isolated consumer test and the main workspace's still-pinned dependency
must not be described as a completed main-workspace migration. Read [extraction](extraction.md) for provenance.
