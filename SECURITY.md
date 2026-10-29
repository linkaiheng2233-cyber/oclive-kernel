# Security policy

## Report privately

Send security reports to **[ocliveproject@163.com](mailto:ocliveproject@163.com)**.
Include the affected full commit SHA, enabled features, Rust/OS versions, impact and a minimal,
sanitized reproduction. Do not post exploitable details in a public issue while disclosure is being coordinated.

If GitHub offers a **Report a vulnerability** button for this repository, that private form is also
an option. This policy file does not enable GitHub private vulnerability reporting; that is a separate
repository setting. Email is the documented route regardless of whether that button is available.

This is a single-maintainer project. Reports are handled on a best-effort basis, with no fixed response
or remediation deadline. A follow-up on the same email thread is welcome. Coordinate disclosure timing
with the maintainer when possible; there is no implied support contract or bug bounty.

## Scope and support

The supported investigation target is the currently maintained source revision. There is no released
stable support series or promised backport matrix yet. Always identify the exact SHA; a checkpoint tag
or historical sample does not imply indefinite maintenance. Share a reproduction on current source if possible.

The library defines Base calls and optional character-content validation. It does not implement file
access, asset rendering, authorization, transport, model execution or process isolation. Host-specific
security issues should be reported to the Host's maintainers, with this project included when the
Base contract or validator contributes to the issue. See [responsibility boundaries](docs/boundary.md).

Character-profile JSON/field diagnostics omit supplied persona/asset values. This is not a general
error-redaction guarantee: `BaseCallError.detail` is supplied by the caller and is displayed verbatim
when nonempty. Hosts own redaction before exposing their errors or logs.

## Dependency checks

`Cargo.lock` is committed to reproduce the source examples and CI. The separate
[dependency audit workflow](.github/workflows/security.yml) runs manually or weekly and fetches the
RustSec advisory database before checking that lockfile. It denies advisory warnings as well as
vulnerabilities; a failed fetch is not a clean security result.
Local offline checks must be labeled as using cached advisories. An audit pass covers known advisories
in the inspected graph, not an absence of unknown defects or full Host security acceptance.
