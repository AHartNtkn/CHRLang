#define _GNU_SOURCE
#include "meter.h"
#include <sys/resource.h>
int main(void) {
  void *p=nm_calloc(4,16); assert(p && nm_live==64 && nm_requested==64);
  for(size_t i=0;i<64;i++) assert(((unsigned char*)p)[i]==0);
  p=nm_realloc(p,128);assert(p && nm_live==128 && nm_requested==192);
  p=nm_realloc(p,8);assert(p && nm_live==8 && nm_requested==200);
  assert(nm_realloc(p,SIZE_MAX)==NULL && nm_live==8);
  struct rlimit saved,limited;assert(getrlimit(RLIMIT_AS,&saved)==0);limited=saved;
  limited.rlim_cur=64U<<20;assert(setrlimit(RLIMIT_AS,&limited)==0);
  ((unsigned char*)p)[0]=37;
  assert(nm_realloc(p,1U<<30)==NULL && nm_live==8 && ((unsigned char*)p)[0]==37);
  assert(setrlimit(RLIMIT_AS,&saved)==0);
  nm_free(p);assert(nm_live==0);
  char *s=nm_strdup("abc");assert(strcmp(s,"abc")==0 && nm_live==4);nm_free(s);
  assert(nm_calloc(SIZE_MAX,2)==NULL && nm_live==0);
  void *m=nm_mmap(NULL,4096,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);
  assert(m!=MAP_FAILED && nm_mapped==4096);((volatile char*)m)[0]=1;
  assert(nm_munmap((char*)m+1,4096)==-1 && nm_mapped==4096);
  assert(nm_munmap(m,4096)==0 && nm_mapped==0);
  assert(nm_mmap(NULL,0,PROT_READ,MAP_PRIVATE|MAP_ANONYMOUS,-1,0)==MAP_FAILED && nm_mapped==0);
  assert(nm_live==0 && nm_mapped==0);
  nm_emit("self_check",0);
}
