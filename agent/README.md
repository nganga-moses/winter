# 🧠 Winter Edge – Local AI Software Engineer

Winter Edge is your autonomous, local-first AI developer.  
It reads your codebase, understands your architecture, and builds or reviews software like a real engineer — right from your machine.

---

## 🚀 Features

✅ **Offline AI Agent** – No cloud needed for most tasks  
✅ **LLM-Aware Code Context** – Selects only the most relevant code chunks  
✅ **Memory-Persistent Reasoning** – Remembers your requirements, architecture, and rules  
✅ **Diff-Based Code Reviews** – Optional AI-based review before every commit  
✅ **Bootstrap from GitHub** – Load an existing project and extract requirements automatically  

---

## 🧱 Core Modules

### 🤖 Agents
| Name           | Role                                |
|----------------|--------------------------------------|
| `BuilderAgent` | Writes/refactors code from tasks     |
| `ReviewerAgent`| Reviews code diffs with an LLM       |

---

### 🧠 Context & Memory
| Module                 | Description                                      |
|------------------------|--------------------------------------------------|
| `CodeChunkRegistry`    | Indexes functions/classes from project files     |
| `DependencyGraph`      | Maps imports, calls, and class inheritance       |
| `MemorySQLite`         | Stores requirements, architecture, rules         |
| `ContextManager`       | Combines context for any task under token limit  |

---

### 🔌 LLM Routing
| Module        | Description                                   |
|----------------|-----------------------------------------------|
| `LLMRouter`    | Routes to local (Ollama) or cloud (OpenAI) LLMs |

---

### ⚡ Bootstrapping
| Module             | Description                                  |
|--------------------|----------------------------------------------|
| `BootstrapRunner`  | Parses repo, extracts features, populates memory |

---

## 📦 Dev Flow (CLI-based MVP)

```bash
# Bootstrap from existing repo
Winter bootstrap https://github.com/user/repo

# Run a new task
Winter task "Add endpoint to list leaderboard scores"

# Review your changes
Winter review

```
---
Built with ❤️ by real devs, for real devs.
Not just “AI-enhanced”. AI-empowered.