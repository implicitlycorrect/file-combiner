# file-combiner
file-combiner is a command-line tool for combining multiple files into a single file. It uses the Rayon library for parallel processing, which allows it to efficiently combine large numbers of files.

## usage

To use file-combiner, simply run the following command:

```
file-combiner [file1] [file2] ... [fileN]
```

This will combine the specified files into a single output file called "combined.txt", which will be created in the current directory.

You can also specify directories instead of individual files, in which case all files in the directory and its subdirectories will be combined:

```
file-combiner [directory1] [directory2] ... [directoryN]
```
