// Author: Denis Bueno

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>
#include <sys/time.h>

typedef struct {
  struct rusage ru;
  struct timeval start, elapsed;
} resources;

// https://man7.org/tlpi/code/online/diff/procres/print_rusage.c.html
void printRusage(const char *leader, const struct rusage *ru) {
  const char *ldr = (leader == NULL) ? "" : leader;

  fprintf(stderr, "%sCPU time (secs):         user=%.3f; system=%.3f\n", ldr,
          ru->ru_utime.tv_sec + ru->ru_utime.tv_usec / 1000000.0,
          ru->ru_stime.tv_sec + ru->ru_stime.tv_usec / 1000000.0);
  fprintf(stderr, "%sMax resident set size:   %ld\n", ldr, ru->ru_maxrss);
  fprintf(stderr, "%sIntegral shared memory:  %ld\n", ldr, ru->ru_ixrss);
  fprintf(stderr, "%sIntegral unshared data:  %ld\n", ldr, ru->ru_idrss);
  fprintf(stderr, "%sIntegral unshared stack: %ld\n", ldr, ru->ru_isrss);
  fprintf(stderr, "%sPage reclaims:           %ld\n", ldr, ru->ru_minflt);
  fprintf(stderr, "%sPage faults:             %ld\n", ldr, ru->ru_majflt);
  fprintf(stderr, "%sSwaps:                   %ld\n", ldr, ru->ru_nswap);
  fprintf(stderr, "%sBlock I/Os:              input=%ld; output=%ld\n",
          ldr, ru->ru_inblock, ru->ru_oublock);
  fprintf(stderr, "%sSignals received:        %ld\n", ldr, ru->ru_nsignals);
  fprintf(stderr, "%sIPC messages:            sent=%ld; received=%ld\n",
          ldr, ru->ru_msgsnd, ru->ru_msgrcv);
  fprintf(stderr, "%sContext switches:        voluntary=%ld; "
          "involuntary=%ld\n", ldr, ru->ru_nvcsw, ru->ru_nivcsw);
}

void printElapsed(const char *leader, const resources *res) {
  const char *ldr = (leader == NULL) ? "" : leader;
  fprintf(stderr, "%sWall time (secs):        %.3f\n", ldr,
          res->elapsed.tv_sec + res->elapsed.tv_usec / 1000000.0);
}

void printResources(const char *leader, const resources *res) {
  printElapsed(leader, res);
  printRusage(leader, &res->ru);
}

//^----------------------------------------------------------------------------^

void measure_start(resources *res) {
  gettimeofday(&res->start, (struct timezone *) 0);
}

void measure_end(resources *res) {
  gettimeofday(&res->elapsed, (struct timezone *) 0);
  // Gets usage of children.
  getrusage(RUSAGE_CHILDREN, &res->ru);

  res->elapsed.tv_sec -= res->start.tv_sec;
  if (res->elapsed.tv_usec < res->start.tv_usec) {
    // Carries 1 from seconds field
    res->elapsed.tv_usec += 1000000;
    --res->elapsed.tv_sec;
  }
  res->elapsed.tv_usec -= res->start.tv_usec;
}

// Takes a single argument designating the command to run.
int main(int argc, char** argv) {
  resources res;

  if (!argv[1]) {
    fprintf(stderr, "NULL argv[1]\n");
    return -1;
  }

  measure_start(&res);

  int r = system(argv[1]);
  if (-1 == r) {
    fprintf(stderr, "system() error: %d: %s\n", errno, strerror(errno));
    return r;
  }

  if (127 == r) {
    fprintf(stderr, "system() shell could not be executed\n");
    return r;
  }

  measure_end(&res);

  fprintf(stderr, "\n");
  printResources(NULL, &res);
  return 0;
}
