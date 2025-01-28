# for-each-dir

## Description
Output a sh/bat for running a command on several directories

## Usage
```
Usage: for-each-dir.exe [OPTIONS] <COMMAND>

Arguments:
  <COMMAND>  The command to run on each directory

Options:
  -p, --plain                        Do not transform \n into newline
  -r, --recurse                      Recurse into inner directories
  -c, --contains-dir <CONTAINS_DIR>  Only filter directories containing this directory [default: ]
  -f, --filter <FILTER>              Filter following directories [default: ]
  -i, --ignore <IGNORE>              Ignore following directories [default: ]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Example
```bash
> for-each-dir "mvn clean"

pushd "dir1"
  "mvn clean"
popd

pushd "dir2"
  "mvn clean"
popd
```