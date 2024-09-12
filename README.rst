rusage
======

Like ``time(1)`` but for resource usage.

Usage
-----

::

    ; rusage ls
    LICENSE		default.nix	flake.nix	rusage.c
    README.rst	flake.lock	rusage.sh

    Wall time (secs):        0.013
    CPU time (secs):         user=0.003; system=0.006
    Max resident set size:   1347584
    Integral shared memory:  0
    Integral unshared data:  0
    Integral unshared stack: 0
    Page reclaims:           863
    Page faults:             0
    Swaps:                   0
    Block I/Os:              input=0; output=0
    Signals received:        0
    IPC messages:            sent=0; received=0
    Context switches:        voluntary=0; involuntary=7

::

    ; rusage -q ls
    Wall time (secs):        0.013
    CPU time (secs):         user=0.003; system=0.006
    Max resident set size:   1347584
    Integral shared memory:  0
    Integral unshared data:  0
    Integral unshared stack: 0
    Page reclaims:           863
    Page faults:             0
    Swaps:                   0
    Block I/Os:              input=0; output=0
    Signals received:        0
    IPC messages:            sent=0; received=0
    Context switches:        voluntary=0; involuntary=7

::

    ; rusage --json rusage.json ls
    LICENSE		default.nix	flake.nix	rusage.c
    README.rst	flake.lock	rusage.sh
    ; cat rusage.json
    {
    "wall time": 0.010,
    "user time": 0.001,
    "system time": 0.001,
    "max rss": 2179072,
    "integral shared memory": 0,
    "integral unshared data": 0,
    "integral unshared stack": 0,
    "page reclaims": 185,
    "page faults": 0,
    "swaps": 0,
    "block reads": 0,
    "block writes": 0,
    "signals received": 0,
    "ipc sends": 0,
    "ipc receives": 0,
    "voluntary context switches": 0,
    "involuntary context switches": 19
    }


Building
--------

Just use cargo.
