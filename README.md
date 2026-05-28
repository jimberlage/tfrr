# tfrr

This is a helper for an opinionated terraform setup.  It allows for shorter commands.

## Install

On macOS:

```zsh
brew tap jimberlage/tfrr https://github.com/jimberlage/tfrr
brew install tfrr
```

## Usage

```
Terraform wrapper that uses directory name for var-file naming

Usage: tfrr [OPTIONS] <COMMAND>

Commands:
  import  Run terraform import with the appropriate var-file
  init    Run terraform init with the appropriate var-file
  plan    Run terraform plan with the appropriate var-file
  apply   Run terraform apply with the generated plan file
  help    Print this message or the help of the given subcommand(s)

Options:
      --dir <DIR>  Directory to run terraform commands in (defaults to current directory)
  -h, --help       Print help
```
