# tasklist
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
tl add -m "Catch up with colleague around X" -d nw
```

## Viewing tasks
```
tl get *
```
```
tl get -d nw
```
```
tl get -d tw
```
```
tl get -d t
```
