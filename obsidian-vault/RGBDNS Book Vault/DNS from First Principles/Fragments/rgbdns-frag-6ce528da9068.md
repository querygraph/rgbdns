---
type: "code-fragment"
fragment_id: "rgbdns-frag-6ce528da9068"
source_path: "docs/blog/announcing-rgbdns/post.md"
code_note: "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Running without `supervise`"
kind: "heading"
start_line: 137
end_line: 154
---

# Running without `supervise`

- Fragment ID: `rgbdns-frag-6ce528da9068`
- Source file: [[DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source|docs/blog/announcing-rgbdns/post.md]]
- Lines: 137-154
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-6ce528da9068", "codeNote": "DNS from First Principles/Code/docs/blog/announcing-rgbdns/post.md.source", "heading": "rgbdns-frag-6ce528da9068: heading Running without `supervise`", "sourcePath": "docs/blog/announcing-rgbdns/post.md", "startLine": 137, "endLine": 154}
```

## Excerpt

<span id="rgbdns-frag-6ce528da9068" class="rgbdns-fragment-target"></span>
### rgbdns-frag-6ce528da9068: heading Running without `supervise`

```markdown
## Running without `supervise`

rgbdns still runs correctly under daemontools, but it does not require an old
supervision stack.

For an existing Linux host, systemd is the practical default: foreground
processes, explicit users, restart policy, limits, capability controls, and
central logs. runit is the closest migration when service directories and
`run` scripts are part of the operational model. s6 with s6-rc is the strongest
choice when dependency-aware supervision and a deliberately composed service
graph matter. OpenRC fits systems that already use it, while Kubernetes and
similar orchestrators should supervise one foreground responsibility per
container rather than nesting another restart manager inside the pod.

The key contract is portable: stay in the foreground, log to standard streams,
replace the process directly, expose readiness honestly, and let one supervisor
own restarts.

```
