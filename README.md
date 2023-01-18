# File rename using regular expression

`frn` renames files using regular expression supported by the [`regex` crate]
(https://docs.rs/regex/latest/regex/).

## Compilation

```{bash}
git clone https://github.com/djhshih/frn
cd frn
cargo build
```

Static binaries for x86_64 Linux is provided with each release.

## Usage

See `frn --help`

## Examples

```
$ touch aqqle orange banana
```

We can fix the spelling mistake by first doing a dry run
```
$ frn 's/q/p/g' *
aqqle -> apple
No operation performed; confirm by `frn -r`
```

After verifying the proposed changes, we can apply the change by
```
$ frn -r 's/q/p/g' *
aqqle -> apple
```

Now, the rename operation will be applied and this will be recorded 
in `.frn_history` within the current directory.

We can also change the directory structure by
```
$ frn -p -r 's|(.*)|fruits/$1|' *
apple -> fruits/apple
banana -> fruits/banana
orange -> fruits/orange
```
where `|` is the delimiter, and `-p` creates the parent directory.
Backreference can be done with `$1` (or `\1`).

Now, all the files are placed within the `fruits` directory.

