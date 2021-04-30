# tasklist-cli
Command line tool for tracking tasks

[![Rust](https://github.com/jcbcn/tasklist-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/jcbcn/tasklist-cli/actions/workflows/rust.yml)

## Getting started
```
scoop install tasklist
```

```
tl init
```

### Defaults
- Persistence: `sqlite`
- List: `default`

## Adding a list
```
tl lists add mynewlist
```

## Active list
```
tl config set-list --current --list=mynewlist
```

## Adding a task

```
tl tasks add -m "Catch up with colleague around X" -d nw -t mytag
```

## Viewing tasks
Get all tasks in all lists
```
tl get tasks -all
```
Get all tasks in a specific list
```
tl get tasks -l mynewlist
```
Get all tasks in the current list that are due in the next week
```
tl get tasks -d nw
```
Get all tasks in the current list that are due this week
```
tl get tasks -d tw
```
Get all tasks in the current list that are due today
```
tl get tasks -d td
```
Get all tasks in the current list that are due tomorrow
```
tl get tasks -d tm
```
Get all tasks in the current list with a specified tag
```
tl get tasks -t mytag
```
