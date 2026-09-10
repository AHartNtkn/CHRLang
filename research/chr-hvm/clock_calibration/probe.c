#define _GNU_SOURCE
#include <sched.h>
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
static uint64_t now(void) {
  struct timespec t;
  assert(clock_gettime(CLOCK_MONOTONIC,&t)==0);
  return (uint64_t)t.tv_sec*1000000000ULL+(uint64_t)t.tv_nsec;
}
static uint64_t pair(void) { uint64_t t=now(); return now()-t; }
static uint64_t bulk(int clocked) {
  volatile uint64_t *values=malloc(100000*sizeof(uint64_t)); assert(values);
  uint64_t t=now();
  for(unsigned i=0;i<100000;i++)values[i]=clocked?pair():i;
  uint64_t elapsed=now()-t;
  if(!clocked)for(unsigned i=0;i<100000;i++)assert(values[i]==i);
  free((void*)values);return elapsed;
}
int main(int argc,char **argv) {
  assert(argc==2);int order=atoi(argv[1]);
  for(unsigned i=0;i<1000;i++)(void)pair();
  uint64_t samples[10000];for(unsigned i=0;i<10000;i++)samples[i]=pair();
  uint64_t baseline,measured;
  if(order==0){baseline=bulk(0);measured=bulk(1);}else{measured=bulk(1);baseline=bulk(0);}
  printf("{\"cpu\":%d,\"baseline_ns\":%llu,\"pairs_ns\":%llu,\"baseline_verified\":true,\"samples\":[",sched_getcpu(),(unsigned long long)baseline,(unsigned long long)measured);
  for(unsigned i=0;i<10000;i++)printf("%s%llu",i?",":"",(unsigned long long)samples[i]);
  puts("]}");return 0;
}
