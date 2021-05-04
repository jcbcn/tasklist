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

## Adding a list
```
tl lists add mynewlist
tl l a mynewlist
```

## Active list
```
tl config set-list --current --list=mynewlist
```

## Adding a task

```
tl tasks add -m "Catch up with colleague around X" -d nw -t mytag
tl t a -m "Catch up with colleague around X" -d nw -t mytag
```

## Viewing tasks
Get all tasks in all lists
```
tl tasks get -all
tl t g -a
```
Get all tasks in a specific list
```
tl tasks get -l mynewlist
tl t g -l mynewlist
```
Get all tasks in the current list that are due in the next week
```
tl tasks get -d nw
tl t g -d nw
```
Get all tasks in the current list that are due this week
```
tl tasks get -d tw
tl t g -d tw
```
Get all tasks in the current list that are due today
```
tl tasks get -d td
tl t g -d td
```
Get all tasks in the current list that are due tomorrow
```
tl tasks get -d tm
tl t g -d tm
```
Get all tasks in the current list with a specified tag
```
tl tasks get -t mytag
tl t g -t mytag
```
