# tasklist
[![Rust](https://github.com/mentality-tools/tasklist/actions/workflows/build.yml/badge.svg)](https://github.com/mentality-tools/tasklist/actions/workflows/build.yml)

Privacy focused tool for tracking day-to-day tasks.

## Introduction
### Installation
```
scoop bucket add tasklist https://github.com/mentality-tools/tasklist
```
```
scoop install tasklist
```

### Getting started

#### Server

```
tl run
```
Then navigate to `http://localhost:8080` in your browser.

#### CLI

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
