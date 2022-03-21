# gitignore

Fetch the .gitignore templates from Github automatically, for easy inclusion in your projects.
Mostly I just wrote this because I was tired of copying from [github's
repo](https://github.com/github/gitignore). This uses the [gitignore
API](https://docs.github.com/en/rest/reference/gitignore).

## installation

```sh
$ git clone git@github.com:pkage/focusd && cd focusd
$ cargo build --release
$ cp target/release/gitignore ~/.local/bin
```

## usage

List available languages:

```sh
$ gitignore --list
```

Print a language's .gitignore to stdout:

```sh
$ gitignore Python
```

Print a language's .gitignore to the current directory

```sh
$ gitignore Python >> .gitignore
```
