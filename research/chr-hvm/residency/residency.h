#ifndef CHR_RESIDENCY_H
#define CHR_RESIDENCY_H
#include <assert.h>
#include <stdio.h>
#include <string.h>
typedef struct {unsigned long rss,anonymous,private_dirty,virtual_size;} Nr;
static Nr nr_read(void){
 Nr r={0};unsigned found=0;char line[256],key[64];unsigned long n;
 FILE *f=fopen("/proc/self/smaps_rollup","r");assert(f);
 while(fgets(line,sizeof(line),f))if(sscanf(line,"%63[^:]: %lu",key,&n)==2){
  if(!strcmp(key,"Rss")){r.rss=n;found|=1;}
  if(!strcmp(key,"Anonymous")){r.anonymous=n;found|=2;}
  if(!strcmp(key,"Private_Dirty")){r.private_dirty=n;found|=4;}
 }
 assert(fclose(f)==0);f=fopen("/proc/self/status","r");assert(f);
 while(fgets(line,sizeof(line),f))if(sscanf(line,"%63[^:]: %lu",key,&n)==2 && !strcmp(key,"VmSize")){r.virtual_size=n;found|=8;}
 assert(fclose(f)==0 && found==15);return r;
}
static void nr_emit(const char *phase,unsigned query){
 Nr r=nr_read();fprintf(stderr,"{\"residency\":true,\"phase\":\"%s\",\"query\":%u,\"rss_kib\":%lu,\"anonymous_kib\":%lu,\"private_dirty_kib\":%lu,\"virtual_kib\":%lu}\n",phase,query,r.rss,r.anonymous,r.private_dirty,r.virtual_size);
}
#endif
