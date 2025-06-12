<!-- SPDX-License-Identifier: Apache-2.0 -->
# Gitmind Architecture Reference

## 1️⃣ Local‑Machine Stack (Single‑User)

```mermaid
flowchart TD
    subgraph User_Space
        CLI["gitmind CLI<br/>(Go/Rust binary)"] --> GitOps
        CLI -->|optional| Daemon["gitmind serve<br/>(tiny HTTP/WebSocket)"]
        Daemon --> WebUI["D3 Browser UI<br/>(localhost:7420)"]
        TUI["Terminal UI<br/>(ncurses)"] <-->|calls| CLI
    end

    subgraph FS["Local Filesystem"]
        GitRepo[".git object store<br/>(vault / projects / relationships)"]
    end

    GitOps["libgit2 / shell out"] --> GitRepo
    Daemon --> GitRepo
```

---

## 2️⃣ Semantic Link Creation Flow (F001)

```mermaid
sequenceDiagram
    participant User
    participant CLI as gitmind CLI
    participant Git as Local Git
    participant FS as File System

    User->>CLI: gitmind link note.md spec.md
    CLI->>Git: git hash-object note.md
    CLI->>Git: git hash-object spec.md
    CLI->>FS: append "CROSS_REF: note.md -> spec.md" to link.md
    CLI->>Git: git add link.md
    CLI->>Git: git commit -m "link(F001): note -> spec"
    Git-->>CLI: commit SHA
    CLI-->>User: Success + link SHA
```

---

## 3️⃣ Distributed Mesh (Future Phase)

```mermaid
flowchart LR
    subgraph Peer A
        ACLI[gitmind CLI] --> AGit[(Git Repo)]
        AServe[gitmind serve] --> AGit
        AUI[D3 UI]
        AUI --> AServe
    end

    subgraph Peer B
        BCLI[gitmind CLI] --> BGit[(Git Repo)]
        BServe[gitmind serve] --> BGit
        BUI[D3 UI]
        BUI --> BServe
    end

    AServe <-->|gRPC stream / QUIC (Phase 3)| BServe
    AGit <-->|git push/pull (packfiles)| BGit
```

---

## Chaos‑Worker Orchestration

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant CLI as gitmind CLI
    participant CW as Chaos-Manager
    participant W1 as chaos-worker-1
    participant Wn as chaos-worker-N
    participant Git as Local Git
    participant Daemon as gitmind serve
    participant UI as D3 / Browser

    User->>CLI: gitmind chaos --rate 5/s --duration 60
    CLI->>CW: spawn-manager (rate,duration)
    CW->>W1: spawn worker
    CW-->>Wn: …spawn N workers…
    par Parallel chaos
        W1->>Git: random commit
        Wn->>Git: random link/reset
    end
    Git->>Daemon: log event
    Daemon-->>UI: graph-update
    CW-->>CLI: chaos report
    CLI-->>User: metrics
```

---

## Editor Plug‑in Call Flow (VS Code / Obsidian)

```mermaid
flowchart TD
    subgraph Editor
        VS["VS Code Extension (TypeScript)"]
        OBS["Obsidian Plugin (TypeScript)"]
    end

    CLI["gitmind CLI (Rust/Go)"]
    Srv["gitmind serve (daemon)"]

    VS --execSync/json--> CLI
    OBS --execSync/json--> CLI
    VS -.live graph WS.-> Srv
    OBS -.live graph WS.-> Srv
    CLI --> Git[(Git repo)]
    Srv --> Git
```

---

## Recommended Tech Stack

| Component                | Language           | Libraries / Frameworks               | Rationale                              |
| ------------------------ | ------------------ | ------------------------------------ | -------------------------------------- |
| Core library             | **Rust**           | `gitoxide`, `serde_json`             | Fast, static, reused by CLI and daemon |
| CLI (`gitmind`)          | Rust               | `clap` / `structopt`                 | Single binary, easy packaging          |
| Daemon (`gitmind serve`) | Rust               | `axum`, `tokio-tungstenite`, `tonic` | Same code‑base, tiny footprint         |
| Chaos manager            | Rust (or Bash)     | async tasks / POSIX sh               | Demo vs cross‑platform load            |
| D3 Web UI                | TypeScript + React | `d3-force`, `zustand`                | Dev‑friendly, hot reload               |
| VS Code extension        | TypeScript         | `vscode` API                         | Shells to CLI, webview graph           |
| Obsidian plug‑in         | TypeScript         | Obsidian API                         | Same calls, iframe graph               |
| Mesh layer (future)      | Rust               | `tonic` (gRPC) / `quinn` (QUIC)      | Ordered streaming P2P                  |
| Packaging                | Docker             | scratch + copy binary                | Multi‑arch, zero‑deps                  |

---

### Implementation Order

1. Rust core crate (`gitmind-core`)
2. CLI binary (`gitmind`)
3. Local daemon (`gitmind serve`)
4. D3 web UI
5. VS Code / Obsidian plug‑ins
6. Chaos manager subcommand
7. Mesh gRPC/QUIC layer

