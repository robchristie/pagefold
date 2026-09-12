# Pagefold development

Read README.md and docs/content-contract.md before changing the reader or service.
Keep Markdown pages and attachments authoritative and read-only; derived state
must remain separate. Preserve the loopback and path/symlink boundaries.

Run `sh tools/verify.sh` for source, dependency or CI changes. The pinned toolchain
and locked Git dependency support fresh checkouts without a sibling Polyorama.
Use disposable synthetic fixtures; never depend on retained campaign directories.
Browser interaction changes also need focused runtime/UI inspection beyond builds.

The evidence directory and docs/active-plan.md retain historical qualification
records. Do not rewrite them or make ordinary checks depend on them. Record new
review and CI evidence on the owning pull request.
