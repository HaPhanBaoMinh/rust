
## Usage

```shell
rlog <level> <file>
```

## Generate a log file

Use the generator script with the minimum and maximum message length followed
by the number of lines to create. The message length is inclusive and counts
the text between the log level and `event_id`. It uses varied Lorem Ipsum text
for each message and writes to `app.log` by default (next to the generator
script):

```shell
./generate-log.sh 20 80 100
```

An optional fourth argument changes the output path:

```shell
./generate-log.sh 20 80 100 /tmp/app.log
```
