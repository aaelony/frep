# frep

`frep` is a command line tool to rename files in the current directory containing a _string to find_ with a _string to replace_.


## Usage
```
frep <find> <replace>
```


## Example

```
$ ls -1
XX_foo.txt
XX_bar.txt

$ frep XX_ ZZ_
Renamed: ./XX_foo.txt -> ./ZZ_foo.txt
Renamed: ./XX_bar.txt -> ./ZZ_bar.txt

$ ls -1
ZZ_foo.txt
ZZ_bar.txt
```
