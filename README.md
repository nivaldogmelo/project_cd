# Project cd

## Table of Contents

* [Motivation](#motivation)
* [Requirements](#requirements)
* [Installation](#Installation)
* [Usage](#Usage)
* [Contributing](#Contributing)
* [License](#License)


## Motivation

Sometimes you need to switch between project folders in your terminal too often, this project tries to make that a little easier.

## Requirements

- [Cargo](https://github.com/rust-lang/cargo)

## Installation

To install you just need to execute

```
## Install package
cargo install --locked project_cd

## Install wrapper script
project_cd -i
```

### Source `pcd` script

It's a bit tricky change folder from a script in a terminal. Usually scripts create a child process to execute its commands, so when you change the folder you're only changing in the child process, that it's terminated at the end of the execution. So to allow the pcd to change the folder at the parent directory you need to add the following line into your `.bashrc` or equivalent

```
# Pcd activate
source ~/.cargo/bin/pcd
```

It might vary depending on your cargo binary folder, but the `project_cd -i` command will show you the exact path of the `pcd` wrapper

## Usage

After you're done with the installation, you just need to execute the `pcd` with the desired flags `use the --help` to more information about it

### Adding a new project to the list
``` 
> pcd -a <path-to-project> 
```

### Removing a project from the list
``` 
> pcd -r <project_name>
```
### Switching to a project 
``` 
> pcd
```
## Contributing

If you feel like something is missing/broken, feel free to create an issue or submit a PR.

## License

This project is under the `MIT License`. See the [LICENSE](LICENSE.md) file for more details.
