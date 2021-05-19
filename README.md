# tasklist-cli
[![Rust](https://github.com/jcbcn/tasklist-cli/actions/workflows/build.yml/badge.svg)](https://github.com/jcbcn/tasklist-cli/actions/workflows/build.yml)

Command line tool for tracking tasks

## Introduction
### Installation
```
scoop install tasklist
```
```
cargo install tasklist
```
### Getting started
cd to your chosen dir and run
```
tl init
```

## Tasks

### Adding a task

```
tl tasks add -m "Catch up with colleague around X" -d nw
tl t a -m "Catch up with colleague around X" -d nw
```

### Viewing tasks

Get all tasks in the current list that are due today
```
tl tasks get -d td
tl t g -d td
```

### Completing a task

```
tl tasks complete 1234
tl t c 1234
```
