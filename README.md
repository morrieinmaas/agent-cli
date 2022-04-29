<p align="center">
  <br />
  <img
    alt="Agent CLI logo"
    src="./images/agent-cli-dark.svg#gh-dark-mode-only"
    height="250px"
  />
   <img
    alt="Agent CLI logo"
    src="./images/agent-cli-light.svg#gh-light-mode-only"
    height="250px"
  />
</p>

<h1 align="center" ><b>Agent CLI</b></h1>

<h3 align="center">Powered by Hyperledger Aries and &nbsp; <img src="./images/animo-logo-dark-background.png#gh-dark-mode-only" height="12px"><img src="./images/animo-logo-light-background.png#gh-light-mode-only" height="12px"></h3><br>

<p align="center">
<a href="#getting-started">Getting started</a> &nbsp;|&nbsp;
  <a href="#roadmap">Roadmap</a> &nbsp;|&nbsp;
  <a href="#contributing">Contributing</a> 
    
</p>

<!-- Add badges? -->

The Agent CLI is the most convenient way for self-sovereign identity (SSI) developers to interact with SSI agents.

- **Environments** to easily manage configuration for multiple projects and agents
- **Automations** that you can perform against an agent
- **Mock data** so that you can focus on the important task of building your application instead of other foobar (coming soon 🚧)

If you are looking for more information about the concepts, example code and tutorials on how to use the CLI we recommend you check out our extensive [docs](https://docs.agent-cli.animo.id/).

## Installation

Heres how to install the Agent CLI using the most popular package managers. For advanced installation options, binaries and troubleshooting we recommend checking out the [installation guide](https://docs.agent-cli.animo.id/guides/installation).

### macOS using Brew

```sh
brew tap animo/agent-cli
brew install agent-cli
```

### Cargo install

```sh
cargo install --git https://github.com/animo/agent-cli
```

### Manual build from source

```sh
git clone https://github.com/animo/agent-cli
cd agent-cli

# Run ONE of the following commands to build:
make build

cargo build --release

# Run ONE of the following commands to add install the agent-cli into your PATH
make install

cargo install --path ./cli

```

## Getting started

In order to work with the Agent CLI an agent needs to be configured. With the following command you connect with our community agent and can access all the functionality.

```sh
agent-cli configuration add --default
```

This command will set up the community agent. To set up your own agent or use our multitenant agent to get your own environment, see the [set up guide](https://docs.agent-cli.animo.id/guides/configuration).

If you are getting started with the tool we recommend enabling informational logs by passing the `--verbose` (or `-v`) flag.

To see all actions simply use the `--help` or `-h` flag or see the [feature overview](https://docs.agent-cli.animo.id/features/introduction).

## Roadmap

We intend to support multiple versions of the Aries agent. See the CLI help `agent-cli --help` for a list of actions we currently support.

| Feature                             | Status | Description                                                                     |
| ----------------------------------- | ------ | ------------------------------------------------------------------------------- |
| Environments                        | ✅     | Support multiple environments.                                                  |
| Connections                         | ✅     | Retrieve connections or create invitations.                                     |
| Schemas                             | ✅     | Retrieve or create schemas.                                                     |
| Credentials                         | ✅     | Offer or propose credentials.                                                   |
| ACA-Py 0.7.3 support                | ✅     | ACA-Py 0.7.3 support.                                                           |
| Verbosity                           | ✅     | Three verbosity levels and error messages.                                      |
| Automation: offer credential mocked | ✅     | Offer a mocked data credential through an automation.                           |
| Community agent                     | ✅     | Default use with the Animo community agent.                                     |
| Multitenancy                        | ✅     | Use a personal agent environment with your personal token.                      |
| Brew install                        | ✅     | Brew install.                                                                   |
| Proofs                              | ✅     | Proofs.                                                                         |
| Apt-get install                     | 🚧     | Apt-get install.                                                                |
| Chocolaty install                   | 🚧     | Chocolaty install.                                                              |
| Automation: offer credential custom | 🚧     | Offer a custom data credential through an automation.                           |
| Filters                             | 🚧     | Use filters to determine what output you want returned.                         |
| Automation: definitions custom      | 🚧     | Create a schema + credential definition with custom data through an automation. |
| Automation: proofs mock             | 🚧     | Present proof with mock data through an automation.                             |
| Automation: proofs custom           | 🚧     | Present proof with custom data through an automation.                           |

## Contributing

Is there something you'd like to fix or add to the CLI? Great! We 💗 community
contributions. [Get involved](https://docs.agent-cli.animo.id/community/contributing).
