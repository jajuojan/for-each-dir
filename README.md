# for-each-dir

## Description
Output a sh/bat for running a command on several directories

## Example usage
```bash
> for-each-dir "mvn clean"

pushd "dir1"
  "mvn clean"
popd

pushd "dir2"
  "mvn clean"
popd
```