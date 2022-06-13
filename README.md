# Remotex - Remote execution tool

This is my hobby project to execute set of tasks (commands) on the machine.
The application is exposing simple REST API, that is secured by handmade tokens.

## How to use

Clone the repository:

```shell
git clone https://github.com/pestanko/remotex.git
```

Build & Run the application using rust & cargo

```shell
cargo run -- serve
# Or in order to specity the configuration ROOT
cargo run -- -r <PATH_TO_ROOT> serve
```

In order to run the application, you need to have projects configuration (see bellow) defined.

In order to have more verbose logging, you need to set `RUST_LOG` environment variable (Example: `RUST_LOG=debug`).

## Configuration

In order to use this application you need to create a folder, _configuration root_,
(or just use the [`config/`](./config/) - default)
where you would need to create a file `config.yml` that contains YAML definition of the application settings.

```yaml
# Example configuiration

# List of all project file names (file names in projects/ directory)
projects:
  - hello

# Web/REST configuration
web:
  # Listen address
  addr: "127.0.0.1:8030"
```

See [`examples/`](./examples/) for more inspiration.

### Project definition

Each project file needs to be located in the `projects/` within the configuration root.
In the `projects/` directory you can have multiple files, the only one used as project definition
are the ones that are explicitelly mentioned in the `config.yml` in the `projtects:` section (see above).

Example project definition:

```yaml
# Example project definition

# Codename defines the the "handle" of the project - it has to be unique
codename: "hello-example"
# Name defines human readable name
name: "Hello example project"
# You can optionally provide longer description of the project
desc: "Example hello project to print hello world"
# List of tasks that should be executed
tasks:
  # Each task has to have a name
  - name: "Print hello world string"
    # Value defines "what should be done", that kind parameter defines kind/type of the task
    value:
      kind: command # Task is command - it will execute the command with arguments
      name: echo # name of the command (application)
      # List of application/command arguments
      args:
        - "Hello World!"

# Authentication section/configuration
auth:
  # List of all access tokens that can be used to authenticate
  # Tokens are being used for Bearer Authentication
  tokens:
    # Name of the token that will be audited/logged
    - name: "General access token"
      # Expected value of the token
      value: "some-random-token-for-token-based-authentication"
```

## Web API

There API consists of these these endpoints:

- `GET /api/projects` - Get list of all registred projects (it requires admin token, defined in the web configuration)
- `GET /api/projects/{project_codename}` - Get information about a single project, where part of the path is project's `codename`
- `POST /api/projects/{project_codename}/execute` - Execute all the tasks for the project, requires the project token

## Development

To run the tests

```shell
cargo test
```
