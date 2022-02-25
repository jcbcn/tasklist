# tasklist-cli
[![Rust](https://github.com/mentality-tools/tasklist/actions/workflows/build.yml/badge.svg)](https://github.com/mentality-tools/tasklist/actions/workflows/build.yml)

Command line tool for tracking tasks

## Introduction
### Installation
```
scoop bucket add tasklist https://github.com/mentality-tools/tasklist
```
```
scoop install tasklist
```
### Getting started
`cd` to your chosen directory and run
```
tl init
```

## Tasks

### Adding a task

```
tl tasks add -m "Catch up with colleague around X" -d tm
tl t a -m "Catch up with colleague around X" -d tm
```

### Viewing tasks

Get all tasks due today
```
tl tasks get -d td
tl t g -d td
```

### Completing a task

```
tl tasks complete 1234
tl t c 1234
```
