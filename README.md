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

### Example
> The following example uses a Docker container (`ubuntu:latest`) on macOS. 

Start with a privileged container:
```bash
docker run --rm -it -v PATH_TO_RPM:/home \
    --privileged \
    --platform linux/amd64 \
    ubuntu bin/bash
```

Go to `/home`, and set up a target process using `/bin/sleep`:
```bash
root@021488b1d7d7:/> cd home
root@021488b1d7d7:/home> sleep 1000 &
[1] 14
```

`rpm` defaults to dumping the `/proc/<pid>/maps` file when providing only the PID:
```bash
root@021488b1d7d7:/home> ./rpm 14
Memory Region (size)                                             Perm     Offset     Device     Inode      Path
0x0000000000200000-0x00000000002b5000 (  724K)                   r--p     0          7:0        78570      /usr/bin/qemu-x86_64
0x00000000002c4000-0x0000000000461000 ( 1652K)                   r-xp     737280     7:0        78570      /usr/bin/qemu-x86_64
0x0000000000470000-0x00000000004ab000 (  236K)                   rw-p     2424832    7:0        78570      /usr/bin/qemu-x86_64
0x00000000004ba000-0x00000000004cf000 (   84K)                   rw-p     2662400    7:0        78570      /usr/bin/qemu-x86_64
0x00000000004cf000-0x00000000004ef000 (  128K)                   rw-p     0          0:0        0          
0x0000000034506000-0x0000000034507000 (    4K)                   ---p     0          0:0        0          [heap]
0x0000000034507000-0x000000003450b000 (   16K)                   rw-p     0          0:0        0          [heap]
0x0000004000000000-0x0000004000002000 (    8K)                   r--p     0          254:1      1868853    /usr/bin/sleep
0x0000004000002000-0x0000004000006000 (   16K)                   r--p     8192       254:1      1868853    /usr/bin/sleep
0x0000004000006000-0x0000004000007000 (    4K)                   r--p     24576      254:1      1868853    /usr/bin/sleep
0x0000004000007000-0x0000004000008000 (    4K)                   ---p     0          0:0        0          
0x0000004000008000-0x0000004000009000 (    4K)                   r--p     28672      254:1      1868853    /usr/bin/sleep
0x0000004000009000-0x000000400000a000 (    4K)                   rw-p     32768      254:1      1868853    /usr/bin/sleep
0x000000400000a000-0x000000400002b000 (  132K)                   rw-p     0          0:0        0          
0x000000400100a000-0x000000400100b000 (    4K)                   ---p     0          0:0        0          
0x000000400100b000-0x000000400180b000 ( 8192K)                   rw-p     0          0:0        0          
0x000000400180b000-0x000000400180d000 (    8K)                   r--p     0          254:1      1869368    /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x000000400180d000-0x0000004001837000 (  168K)                   r--p     8192       254:1      1869368    /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x0000004001837000-0x0000004001842000 (   44K)                   r--p     180224     254:1      1869368    /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x0000004001842000-0x0000004001843000 (    4K)                   ---p     0          0:0        0          
0x0000004001843000-0x0000004001845000 (    8K)                   r--p     225280     254:1      1869368    /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x0000004001845000-0x0000004001847000 (    8K)                   rw-p     233472     254:1      1869368    /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
0x0000004001847000-0x0000004001849000 (    8K)                   rw-p     0          0:0        0          
0x000000400184b000-0x0000004001873000 (  160K)                   r--p     0          254:1      1869386    /usr/lib/x86_64-linux-gnu/libc.so.6
0x0000004001873000-0x0000004001a08000 ( 1620K)                   r--p     163840     254:1      1869386    /usr/lib/x86_64-linux-gnu/libc.so.6
0x0000004001a08000-0x0000004001a60000 (  352K)                   r--p     1822720    254:1      1869386    /usr/lib/x86_64-linux-gnu/libc.so.6
0x0000004001a60000-0x0000004001a64000 (   16K)                   r--p     2179072    254:1      1869386    /usr/lib/x86_64-linux-gnu/libc.so.6
0x0000004001a64000-0x0000004001a66000 (    8K)                   rw-p     2195456    254:1      1869386    /usr/lib/x86_64-linux-gnu/libc.so.6
0x0000004001a66000-0x0000004001a75000 (   60K)                   rw-p     0          0:0        0          
0x0000ffff7b6f0000-0x0000ffff7b7a1000 (  708K)                   rw-p     0          0:0        0          
0x0000ffff7b7a1000-0x0000ffff837a0000 (131068K)                  rwxp     0          0:0        0          
0x0000ffff837a0000-0x0000ffff837a1000 (    4K)                   ---p     0          0:0        0          
0x0000ffff837a1000-0x0000ffff837a5000 (   16K)                   rw-p     0          0:0        0          
0x0000ffff837a8000-0x0000ffff838ce000 ( 1176K)                   rw-p     0          0:0        0          
0x0000ffff838ce000-0x0000ffff838d0000 (    8K)                   ---p     0          0:0        0          
0x0000ffff838d0000-0x0000ffff838f3000 (  140K)                   rw-p     0          0:0        0          
0x0000ffff838f3000-0x0000ffff838f5000 (    8K)                   r--p     0          0:0        0          [vvar]
0x0000ffff838f5000-0x0000ffff838f6000 (    4K)                   r-xp     0          0:0        0          [vdso]
0x0000ffffe6faf000-0x0000ffffe6fd0000 (  132K)                   rw-p     0          0:0        0          [stack]
```

From which one can pick the virtual page that is of interest, such as:
```bash
root@021488b1d7d7:/home> ./rpm 14 0x00000000004ba000   
VirtAddr            PFN          Present  Swapped  FileMap  Shared   Dirty   
0x00000000004ba000  0x1719d8     [*]      [ ]      [ ]      [ ]      [ ]   
```

Or use `-a` to examine the entire region:
```bash
root@021488b1d7d7:/home> ./rpm 14 0x00000000004ba000 -a                                                                      
VirtAddr            PFN          Present  Swapped  FileMap  Shared   Dirty   
0x00000000004ba000  0x1719d8     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004bb000  0x13b69d     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004bc000  0x13b69e     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004bd000  0x13b69f     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004be000  0x1399a3     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004bf000  0x1399a9     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004c0000  0x17183a     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004c1000  0x1399f7     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004c2000  0x139a06     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004c3000  0x1718bc     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004c4000  0x170ba8     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004c5000  0x1399ff     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004c6000  0x170714     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004c7000  0x16d924     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004c8000  0x17269c     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004c9000  0x139a23     [*]      [ ]      [*]      [*]      [ ]     
0x00000000004ca000  0x17210f     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004cb000  0x16e2eb     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004cc000  0x171d06     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004cd000  0x1729d2     [*]      [ ]      [ ]      [ ]      [ ]     
0x00000000004ce000  0x16fc1b     [*]      [ ]      [ ]      [ ]      [ ]     
```