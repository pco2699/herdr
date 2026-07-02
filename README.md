# herdr (pco2699 fork)

A personal fork of **[herdr](https://github.com/ogulcancelik/herdr)** — a terminal-based
runtime for coding agents ("tmux, rebuilt for agents").

For what herdr is, install instructions, supported agents, the socket API, and the full
docs, see the **upstream project** — everything there still applies:

- Upstream repo: <https://github.com/ogulcancelik/herdr>
- Upstream README: <https://github.com/ogulcancelik/herdr#readme>
- Docs: <https://herdr.dev/docs/>

This file only documents what **differs in this fork**.

---

## What this fork changes

### 1. Instant new tabs (no name prompt)

Creating a tab no longer opens the rename dialog first — new tabs are created immediately
with a generated name (`1`, `2`, …). Re-enable the prompt if you want it:

```toml
[ui]
prompt_new_tab_name = true
```

### 2. Move between panes with `ctrl+hjkl`

Pane focus is bound to `ctrl+h/j/k/l` (left/down/up/right) in addition to the original
`prefix+h/j/k/l`, so you can move without the prefix.

```toml
[keys]
focus_pane_left  = ["ctrl+h", "prefix+h"]
focus_pane_down  = ["ctrl+j", "prefix+j"]
focus_pane_up    = ["ctrl+k", "prefix+k"]
focus_pane_right = ["ctrl+l", "prefix+l"]
```

> Note: `ctrl+h/j/k/l` overlap with control codes some programs use inside a pane
> (`ctrl+h` is backspace, `ctrl+j` is newline, `ctrl+l` clears the screen); herdr captures
> them before the pane sees them. The `prefix+…` bindings remain as a fallback, and you can
> switch to the collision-free `ctrl+alt+…` family if the defaults get in your way.

### 3. Resize the focused pane with `ctrl+shift+hjkl`

Direct pane resize without entering resize mode:

```toml
[keys]
resize_pane_left  = "ctrl+shift+h"
resize_pane_down  = "ctrl+shift+j"
resize_pane_up    = "ctrl+shift+k"
resize_pane_right = "ctrl+shift+l"
```

The original `prefix+r` resize mode still works.

### 4. `herdr --remote` over Eternal Terminal, with fewer logins

`herdr --remote <host>` now uses **[Eternal Terminal](https://eternalterminal.dev/)** (`et`)
for its persistent data connection by default, instead of the plain ssh bridge. `et` holds a
single authenticated, auto-reconnecting session, so a remote session survives network drops,
suspends, and Wi‑Fi changes without dropping the client.

It also fixes repeated auth prompts: the remote bootstrap (platform + binary + version
checks) now runs in a **single ssh round trip** for an already-provisioned host, so you get
one authentication prompt (e.g. one 2FA challenge) instead of one per probe.

**Installing your fork on the remote:** when a remote host has no matching `herdr`, herdr
provisions one to `~/.local/bin/herdr`. The source is, in order: the `HERDR_REMOTE_BINARY`
env var, then `[remote] binary_path` in config, then your local binary if the remote's OS/arch
matches this machine, then a download from this fork's release manifest
(`raw.githubusercontent.com/pco2699/herdr/master/website/latest.json` → `pco2699/herdr`
releases). To force a specific build (e.g. cross-arch from a macOS laptop to a Linux
devserver):

```toml
[remote]
binary_path = "/path/to/herdr-linux-x86_64"   # a fork build for the remote's platform
```

or `HERDR_REMOTE_BINARY=/path/... herdr --remote <host>` for a one-off.

**Requirements:** `et` must be installed on both the local machine and the remote host.

**Usage** (unchanged):

```sh
herdr --remote user@host
herdr --remote user@host --session work
```

**Config** (`[remote]`):

```toml
[remote]
# "et" (default) uses Eternal Terminal. "ssh" uses the original ssh stdio bridge.
transport = "et"
# keepalive + connection reuse for the ssh bootstrap (unchanged upstream option)
manage_ssh_config = true
# corporate/VPNless et (SSH agent socket + forwarding + x2ssh ProxyCommand,
# et server on port 8080). Leave false for bare et on its default port.
et_corp_internal = false
```

Set `transport = "ssh"` to fall back to the original behavior. Bootstrap always uses ssh
(which `et` itself uses for its handshake).

**Corporate / VPNless (`et_corp_internal = true`):** for environments that reach hosts through
an x2p auth broker, herdr invokes `et` with `--ssh-socket ~/.fb-sks-agent/agent.sock`,
`--forward-ssh-agent`, an `x2ssh` `ProxyCommand`, and the et server port `8080`. The x2p broker
socket is read from `$X2P_SOCK` (falling back to the macOS default path). With the flag `false`,
herdr runs bare `et` on its default port.

---

## Planned

- **1:N remote** — attach one client to multiple servers and see agents across all of them in
  a single unified view. In progress; not yet available.

---

## Building from source

Same as upstream (Rust + Zig 0.15.2 for the vendored libghostty-vt). Use the `just` recipes:

```sh
just check   # formatting + clippy + tests
just test
```
