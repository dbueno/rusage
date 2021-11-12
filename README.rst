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

Building
--------

If you're not using Nix, you can do:

::

    export CFLAGS=-Wall
    make crusage

Then put ``rusage`` wherever you like and point it to ``crusage``.
