Strats
======

Strats calculates stats on streams.


Strats is a command line tool that accepts unix streams on stdin and returns
statistics of that stream. It is composable with other unix tools, and supports
exploratory use cases by optionally displaying stats that update in real time.

### Usage

Use the count, mean, sum, min and max flags to report the corresponding statistics.

By default, output will be a comma delimited list of results that corresponds to
the metric flags passed in.

```
$ seq 1 10000 | strats --min --max --mean
1,10000,5000.5
```

To display the output in a more human friendly format, you can pass the `pretty`
flag:

```
$ seq 1 10000 | strats --min --max --mean --pretty
Mean: 5000.5, Min: 1, Max: 10000
```

The previous examples will consume the entire stream and then report the result.
To update the output as new input is recieved, you can pass the `incremental`
flag.

Combining the `incremental` and `pretty` flags will update the same line.
Without the `pretty` flag, the `incremental` flag will make strats print a new
csv line everytime new output is received.
