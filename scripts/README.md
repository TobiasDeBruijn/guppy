# Guppy scripts
These scripts are used to back up applications.
The structure of invocation is as follows:

Cron > guppy-prive.sh > guppy.sh > guppy binary
The `period.sh` script is expanded when invoking the `guppy-prive.sh` script.

The cron line could be as follows:
```bash
30 2 1 * * /srv/guppy/guppy-prive.sh $(/srv/guppy/period.sh) 2>&1 | logger -t guppy-prive
```
The logs will end up in the syslog, and can be viewed with `journalctl`

For this setup to work, the scripts responsible for exporting the backups, should set the name of the file correctly: `$PREFIX-$YYYY-$mm-*.tar.gz`

Guppy.sh takes the source files from `"/mnt/backups/$PARENT/$PREFIX-$PERIOD-*.tar.gz"`, but this can be customized by changing either guppy.sh, or the `parent` argument passed to it from `guppy-prive.sh` (e.g.).