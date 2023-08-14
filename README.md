# Titan
Still in development

One command to rule them all

## About
Shorten the commands you type.

Similar to build systems like make, but works globally and has multiple configuration files.

## Configuration

[YAML](https://yaml.org)

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
- [x] Make aliases work
- [x] Fish documentation (sure other shells too)
- [x] Make things acutally execute
- [x] printing tree structure
- [x] pull config files from parent director(y/ies)
- [ ] add more info to tree structure
- [ ] Documentation
- [ ] Get better naming schemes
- [ ] Allow documentation of subcommands
- [ ] Add refs

### Bugs
- [ ] fish completions work only 1 layer deep
