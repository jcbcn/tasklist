# tasklist-cli
[![Rust](https://github.com/jcbcn/tasklist-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/jcbcn/tasklist-cli/actions/workflows/rust.yml)

Command line tool for tracking tasks

## Getting started
```
scoop install tasklist
```

```
tl init
```

# Tasks

## Adding a task

```
tl tasks add -m "Catch up with colleague around X" -d nw
tl t a -m "Catch up with colleague around X" -d nw
```

## Viewing tasks

Get all tasks in the current list that are due today
```
tl tasks get -d td
tl t g -d td
```

# Lists

## Adding a list
```
tl lists add mynewlist
tl l a mynewlist
```

## Active list
```
tl config set-list --current --list=mynewlist
```