---
description: Automated actions that combine multiple functions
---

# Introduction

The Agent CLI supports automations that combine multiple functions into one single command. Currently, the goal of automations is to shortcut some convenient development situations, in the future they might evolve to be more complex.&#x20;

### Usage

```
agent-cli automate <SUBCOMMAND>
```

#### Options

| Alias | Flag     | Description             |
| ----- | -------- | ----------------------- |
| `-h`  | `--help` | Print help information. |

#### Subcommands

| Command                      | Description                                                                    |
| ---------------------------- | ------------------------------------------------------------------------------ |
| [credential-offer](usage.md) | Simple credential offer automation to offer a premade credential to any agent. |
