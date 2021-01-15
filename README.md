Strats
======

Strats calculates stats on streams.


Strats is a command line tool that accepts unix streams on stdin and returns
statistics of that stream. It is composable with other unix tools, and supports
exploratory use cases by optionally displaying stats that update in real time.

### Usage

Use the count, count\_values, mean, sum, min and max flags to report the
corresponding statistics.

By default, output will be a json object containing the results that correspond
to the metrics specified.

```
$ seq 1 10000 | strats min max mean
{"max":10000.0,"mean":5000.5,"min":1.0}
```


```
$ echo "yolo
  swag
  yolo
  yolo
  swag" | strats count count_values
{"count":5,"count_values":{"swag":2,"yolo":3}}
```
