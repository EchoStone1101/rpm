# `rpm` - Read Linux Page Map

### Usage
```bash
Usage: rpm [OPTIONS] <PID> [ADDR]

Arguments:
  <PID>   the PID of the process to dump from
  [ADDR]  the virtual address of the target pagemap entry

Options:
  -a, --all      dump the entire memory region(s)
  -h, --help     Print help
  -V, --version  Print version
```

### Notes

* Currently only tested on the x86-64/amd64 system.

* May need `sudo` to run (required by reading `/proc/<pid>/pagemap`).
    * If you are using a Docker container, add `--privileged` at `docker run`.
    * **Seeing zeroed PFNs / cannot open pagemap?**\
    Since Linux 4.0, only users with the CAP_SYS_ADMIN capability can get PFNs. \
    In 4.0 and 4.1 opens by unprivileged fail with -EPERM. \
    Starting from
4.2 the PFN field is zeroed if the user does not have CAP_SYS_ADMIN.