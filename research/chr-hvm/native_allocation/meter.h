// Direct-call diagnostic only. Header storage and libc-internal allocations are excluded.
#ifndef CHR_NATIVE_METER_H
#define CHR_NATIVE_METER_H
#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <sys/mman.h>
static size_t nm_live,nm_peak,nm_requested,nm_calls,nm_frees;
static size_t nm_mapped,nm_map_peak,nm_map_requested,nm_maps,nm_unmaps;
typedef union { max_align_t alignment; struct {size_t size;uint64_t magic;} data; } NmHeader;
#define NM_MAGIC UINT64_C(0x4e41544956454d45)
static void nm_added(size_t bytes) {
  nm_live+=bytes;nm_requested+=bytes;nm_calls++;
  if(nm_live>nm_peak)nm_peak=nm_live;
}
static void *nm_malloc(size_t bytes) {
  if(bytes>SIZE_MAX-sizeof(NmHeader)){errno=ENOMEM;return NULL;}
  NmHeader *h=malloc(sizeof(NmHeader)+bytes);if(!h)return NULL;
  h->data.size=bytes;h->data.magic=NM_MAGIC;nm_added(bytes);return h+1;
}
static void nm_free(void *p) {
  if(!p)return;
  NmHeader *h=(NmHeader*)p-1;assert(h->data.magic==NM_MAGIC);
  assert(nm_live>=h->data.size);nm_live-=h->data.size;nm_frees++;
  h->data.magic=0;free(h);
}
static void *nm_calloc(size_t count,size_t size) {
  if(size && count>SIZE_MAX/size){errno=ENOMEM;return NULL;}
  size_t bytes=count*size;
  if(bytes>SIZE_MAX-sizeof(NmHeader)){errno=ENOMEM;return NULL;}
  NmHeader *h=calloc(1,sizeof(NmHeader)+bytes);if(!h)return NULL;
  h->data.size=bytes;h->data.magic=NM_MAGIC;nm_added(bytes);return h+1;
}
static void *nm_realloc(void *p,size_t bytes) {
  if(!p)return nm_malloc(bytes);
  if(!bytes){nm_free(p);return NULL;}
  if(bytes>SIZE_MAX-sizeof(NmHeader)){errno=ENOMEM;return NULL;}
  NmHeader *h=(NmHeader*)p-1;assert(h->data.magic==NM_MAGIC);size_t old=h->data.size;
  NmHeader *next=realloc(h,sizeof(NmHeader)+bytes);if(!next)return NULL;
  assert(nm_live>=old);nm_live-=old;next->data.size=bytes;nm_added(bytes);return next+1;
}
static char *nm_strdup(const char *s) {
  size_t n=strlen(s)+1;char *p=nm_malloc(n);if(p)memcpy(p,s,n);return p;
}
static void *nm_mmap(void *addr,size_t bytes,int prot,int flags,int fd,off_t offset) {
  void *p=mmap(addr,bytes,prot,flags,fd,offset);
  if(p!=MAP_FAILED){nm_mapped+=bytes;nm_map_requested+=bytes;nm_maps++;if(nm_mapped>nm_map_peak)nm_map_peak=nm_mapped;}
  return p;
}
static int nm_munmap(void *addr,size_t bytes) {
  int result=munmap(addr,bytes);
  if(!result){assert(nm_mapped>=bytes);nm_mapped-=bytes;nm_unmaps++;}return result;
}
static void nm_emit(const char *phase,unsigned query) {
  fprintf(stderr,"{\"memory\":true,\"phase\":\"%s\",\"query\":%u,\"live\":%zu,\"peak\":%zu,\"requested\":%zu,\"calls\":%zu,\"frees\":%zu,\"mapped\":%zu,\"map_peak\":%zu,\"map_requested\":%zu,\"maps\":%zu,\"unmaps\":%zu}\n",phase,query,nm_live,nm_peak,nm_requested,nm_calls,nm_frees,nm_mapped,nm_map_peak,nm_map_requested,nm_maps,nm_unmaps);
}
#endif
