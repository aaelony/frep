# frep

`frep` is a command line tool to rename files in the current directory containing a _string to find_ with a _string to replace_.

**Warning!** This tool WILL RENAME FILES IN THE CURRENT DIRECTORY IF THEY MATCH.

## Usage

```
$ frep
frep version 0.1.1
A utility that will rename file parts that match the <file_pattern>.
Usage: frep <find> <replace> <file_pattern>
```


## Example

```
$ ls -1
XX_foo.txt
XX_bar.txt

$ frep XX_ ZZ_ *
Renamed: XX_bar.txt -> ZZ_bar.txt
Renamed: XX_foo.txt -> ZZ_foo.txt

$ ls -1
ZZ_foo.txt
ZZ_bar.txt
```
