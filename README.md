# tasklist
[![Rust](https://github.com/mentality-tools/tasklist/actions/workflows/build.yml/badge.svg)](https://github.com/mentality-tools/tasklist/actions/workflows/build.yml)

Privacy focused tool for tracking day-to-day tasks.

## Installation

#### Scoop 

```
scoop bucket add tasklist https://github.com/mentality-tools/tasklist
```
```
scoop install tasklist
```

#### Brew 

```
Coming Soon...
```

## Getting started

`cd` to your chosen directory and run

### Web UI

```
tl run
```

navigate to `http://localhost:8080` in your browser.

### Command Line

```
tl init
```

#### Tasks

Adding a task
```
tl tasks add -m "Catch up with colleague around X" -d tm
tl t a -m "Catch up with colleague around X" -d tm
```

Get all tasks due today
```
tl tasks get -d td
tl t g -d td
```

Completing a task
```
tl tasks complete 1234
tl t c 1234
```
