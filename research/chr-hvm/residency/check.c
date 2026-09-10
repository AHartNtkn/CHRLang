#define _GNU_SOURCE
#include "residency.h"
#include <sys/mman.h>
int main(void){
 Nr before=nr_read();size_t bytes=16U<<20;
 void *p=mmap(NULL,bytes,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);assert(p!=MAP_FAILED);
 Nr reserved=nr_read();
 for(size_t i=0;i<bytes;i+=4096)((volatile unsigned char*)p)[i]=1;
 Nr touched=nr_read();assert(touched.rss>=reserved.rss+15*1024);
 assert(munmap(p,bytes)==0);Nr released=nr_read();assert(touched.rss>=released.rss+15*1024);
 fprintf(stderr,"{\"before_kib\":%lu,\"reserved_kib\":%lu,\"touched_kib\":%lu,\"released_kib\":%lu}\n",before.rss,reserved.rss,touched.rss,released.rss);
 nr_emit("self_check",0);
}
