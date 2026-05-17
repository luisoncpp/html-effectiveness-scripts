# Editor Team Weekly Sync

Welcome to the weekly triage sync. Below is the current state of the editor's issue backlog. 

Please review the **Triage Queue** before the meeting so we can quickly assign owners.

```yaml
type: triage-board
title: "Current Sprint - Triage"
columns:
  - name: Triage Queue
    issues:
      - id: 112
        title: "Cursor jumps to end of line on save when auto-format is enabled"
        tags: ["bug", "editor-core"]
      - id: 115
        title: "Syntax highlighting completely fails for nested JSX blocks"
        tags: ["bug", "syntax"]
      - id: 118
        title: "Large files (>5MB) cause noticeable typing lag"
        tags: ["performance", "editor-core"]
  - name: Up Next
    issues:
      - id: 98
        title: "Implement undo history grouping for rapid keystrokes"
        tags: ["feature", "history"]
      - id: 104
        title: "Add support for custom ligature fonts"
        tags: ["feature", "rendering"]
  - name: In Progress
    issues:
      - id: 92
        title: "Migrate tree-sitter bindings to WebAssembly"
        tags: ["architecture", "syntax"]

```

### Notes for the team:

* Issue `#112` is highly reproducible on macOS but seems fine on Windows.
* We need a volunteer to look into `#115` as it is blocking the new React tutorial release.



# Implementation Plan: ControlVCode

The development phases for the new creative writing and lore organization tool are broken down below. The initial focus is strictly on the stability of local file handling before moving to the visual interface.

```yaml
type: implementation-plan
title: "Phase 1: Core Structure and Lore Management"
phases:
  - name: "IPC Environment Setup"
    status: "completed"
    tasks:
      - "Initialize Electron with secure contextBridge"
      - "Configure bundler with Preact and Vite"
      - "Integrate Tailwind CSS for the main UI"
  - name: "File Engine (Markdown)"
    status: "in-progress"
    tasks:
      - "Native disk read/write module"
      - "Frontmatter parsing to extract character metadata"
      - "Implement watcher (chokidar) for two-way sync"
  - name: "Core User Interface"
    status: "pending"
    tasks:
      - "Dynamic lateral navigation tree"
      - "Text editor with custom keyboard shortcuts"
      - "Semantic relationship view between documents"

```

### Tech Stack Notes

The strict separation between the main process and the renderer will allow us to keep the interface fluid in Preact, even when massive Markdown directories are being indexed in the background.
