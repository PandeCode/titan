# Titan

One command to rule them all

Kinda like make, but works globally
Still in development

[YAML](https://yaml.org)

## Configuration

Titan looks in the following directories in addition to the the current working directory's parents

 - {current working directory}/.titan.toml
 - {current working directory}/titan.toml
 - $HOME/.titan.toml
 - $HOME/titan.toml
 - /etc/.titan.toml
 - /etc/titan.toml

## Example

### Config

```yaml
commands:
    - ifconfig:
          cmd: curl ifconfig.me
          children:
              - ip:
                    cmd: curl ifconfig.me/ip
              - ua:
                    cmd: curl ifconfig.me/ua
              - all:
                    cmd: curl ifconfig.me/all
    - localip:
          cmd: "ifconfig | grep inet"
          shell: true
```

### Commands

```bash

titan ifconfig    # runs curl ifconfig.me
titan ifconfig ip # runs curl ifconfig.me/ip
titan localip     # runs "ifconfig | grep inet"

```

## Todo

### Dev
- [-] Make aliases work
- [-] Fish documentation (sure other shells too)
- [-] Make things acutally execute
- [-] printing tree structure
- [-] pull config files from parent director(y/ies)
- [ ] add more info to tree structure
- [ ] Documentation
- [ ] Get better naming schemes
- [ ] Allow documentation of subcommands
- [ ] Add refs

### Bugs
- [ ] fish completions work only 1 layer deep
