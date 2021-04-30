# tasklist-cli
Command line tool for tracking tasks

## Getting started
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
tl tasks add -m "Catch up with colleague around X" -d nw
```

## Viewing tasks
```
tl get tasks -all
```
```
tl get tasks -d nw
```
```
tl get tasks -d tw
```
```
tl get tasks -d td
```
```
tl get tasks -d tm
```
