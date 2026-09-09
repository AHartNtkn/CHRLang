00000000000354d0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick>:
   354d0:	55                   	push   %rbp
   354d1:	41 57                	push   %r15
   354d3:	41 56                	push   %r14
   354d5:	41 55                	push   %r13
   354d7:	41 54                	push   %r12
   354d9:	53                   	push   %rbx
   354da:	48 83 ec 68          	sub    $0x68,%rsp
   354de:	49 89 d7             	mov    %rdx,%r15
   354e1:	49 89 f6             	mov    %rsi,%r14
   354e4:	48 89 fb             	mov    %rdi,%rbx
   354e7:	80 be c0 00 00 00 00 	cmpb   $0x0,0xc0(%rsi)
   354ee:	0f 84 b7 00 00 00    	je     355ab <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdb>
   354f4:	41 c6 86 c0 00 00 00 	movb   $0x0,0xc0(%r14)
   354fb:	00 
   354fc:	83 39 01             	cmpl   $0x1,(%rcx)
   354ff:	0f 85 a6 00 00 00    	jne    355ab <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdb>
   35505:	48 8b 69 08          	mov    0x8(%rcx),%rbp
   35509:	48 85 ed             	test   %rbp,%rbp
   3550c:	0f 84 99 00 00 00    	je     355ab <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdb>
   35512:	48 8b 51 10          	mov    0x10(%rcx),%rdx
   35516:	48 89 e7             	mov    %rsp,%rdi
   35519:	4c 89 fe             	mov    %r15,%rsi
   3551c:	ff 15 a6 d5 09 00    	call   *0x9d5a6(%rip)        # d2ac8 <_DYNAMIC+0x290>
   35522:	4c 8b 24 24          	mov    (%rsp),%r12
   35526:	49 83 fc ff          	cmp    $0xffffffffffffffff,%r12
   3552a:	0f 84 84 04 00 00    	je     359b4 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x4e4>
   35530:	4c 8b 6c 24 08       	mov    0x8(%rsp),%r13
   35535:	48 83 fd 01          	cmp    $0x1,%rbp
   35539:	0f 85 24 06 00 00    	jne    35b63 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x693>
   3553f:	48 8b 6c 24 10       	mov    0x10(%rsp),%rbp
   35544:	48 85 ed             	test   %rbp,%rbp
   35547:	0f 84 a0 06 00 00    	je     35bed <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x71d>
   3554d:	49 83 ee 80          	sub    $0xffffffffffffff80,%r14
   35551:	49 8b 4d 00          	mov    0x0(%r13),%rcx
   35555:	4d 8b 45 08          	mov    0x8(%r13),%r8
   35559:	4c 89 ff             	mov    %r15,%rdi
   3555c:	4c 89 f6             	mov    %r14,%rsi
   3555f:	31 d2                	xor    %edx,%edx
   35561:	ff 15 69 d5 09 00    	call   *0x9d569(%rip)        # d2ad0 <_DYNAMIC+0x298>
   35567:	b9 07 00 00 00       	mov    $0x7,%ecx
   3556c:	84 c0                	test   %al,%al
   3556e:	74 2a                	je     3559a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xca>
   35570:	48 83 fd 01          	cmp    $0x1,%rbp
   35574:	0f 84 b3 06 00 00    	je     35c2d <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x75d>
   3557a:	49 8b 4d 10          	mov    0x10(%r13),%rcx
   3557e:	4d 8b 45 18          	mov    0x18(%r13),%r8
   35582:	ba 01 00 00 00       	mov    $0x1,%edx
   35587:	4c 89 ff             	mov    %r15,%rdi
   3558a:	4c 89 f6             	mov    %r14,%rsi
   3558d:	ff 15 3d d5 09 00    	call   *0x9d53d(%rip)        # d2ad0 <_DYNAMIC+0x298>
   35593:	0f b6 c8             	movzbl %al,%ecx
   35596:	48 83 f1 07          	xor    $0x7,%rcx
   3559a:	48 89 0b             	mov    %rcx,(%rbx)
   3559d:	4d 85 e4             	test   %r12,%r12
   355a0:	0f 85 37 05 00 00    	jne    35add <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x60d>
   355a6:	e9 63 05 00 00       	jmp    35b0e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x63e>
   355ab:	49 8b 86 a8 00 00 00 	mov    0xa8(%r14),%rax
   355b2:	48 85 c0             	test   %rax,%rax
   355b5:	0f 84 e3 01 00 00    	je     3579e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x2ce>
   355bb:	48 83 f8 01          	cmp    $0x1,%rax
   355bf:	0f 84 25 01 00 00    	je     356ea <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x21a>
   355c5:	48 83 f8 02          	cmp    $0x2,%rax
   355c9:	0f 85 51 05 00 00    	jne    35b20 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x650>
   355cf:	4d 8d a6 b0 00 00 00 	lea    0xb0(%r14),%r12
   355d6:	b9 02 00 00 00       	mov    $0x2,%ecx
   355db:	4c 89 ff             	mov    %r15,%rdi
   355de:	31 f6                	xor    %esi,%esi
   355e0:	4c 89 e2             	mov    %r12,%rdx
   355e3:	ff 15 ef d4 09 00    	call   *0x9d4ef(%rip)        # d2ad8 <_DYNAMIC+0x2a0>
   355e9:	84 c0                	test   %al,%al
   355eb:	0f 84 88 02 00 00    	je     35879 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x3a9>
   355f1:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   355f8:	ba 01 00 00 00       	mov    $0x1,%edx
   355fd:	4c 89 ff             	mov    %r15,%rdi
   35600:	ff 15 da d4 09 00    	call   *0x9d4da(%rip)        # d2ae0 <_DYNAMIC+0x2a8>
   35606:	49 89 c5             	mov    %rax,%r13
   35609:	49 8b 87 08 01 00 00 	mov    0x108(%r15),%rax
   35610:	48 83 78 20 00       	cmpq   $0x0,0x20(%rax)
   35615:	0f 84 66 05 00 00    	je     35b81 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x6b1>
   3561b:	48 8b 40 18          	mov    0x18(%rax),%rax
   3561f:	48 83 78 40 00       	cmpq   $0x0,0x40(%rax)
   35624:	0f 84 9d 05 00 00    	je     35bc7 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x6f7>
   3562a:	48 89 d5             	mov    %rdx,%rbp
   3562d:	48 8b 40 38          	mov    0x38(%rax),%rax
   35631:	4c 8b 38             	mov    (%rax),%r15
   35634:	ff 15 5e d4 09 00    	call   *0x9d45e(%rip)        # d2a98 <_DYNAMIC+0x260>
   3563a:	bf 10 00 00 00       	mov    $0x10,%edi
   3563f:	be 08 00 00 00       	mov    $0x8,%esi
   35644:	ff 15 56 d4 09 00    	call   *0x9d456(%rip)        # d2aa0 <_DYNAMIC+0x268>
   3564a:	48 85 c0             	test   %rax,%rax
   3564d:	0f 84 00 05 00 00    	je     35b53 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x683>
   35653:	4c 89 28             	mov    %r13,(%rax)
   35656:	48 89 68 08          	mov    %rbp,0x8(%rax)
   3565a:	4c 89 7c 24 08       	mov    %r15,0x8(%rsp)
   3565f:	48 c7 44 24 10 01 00 	movq   $0x1,0x10(%rsp)
   35666:	00 00 
   35668:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   3566d:	48 c7 44 24 20 01 00 	movq   $0x1,0x20(%rsp)
   35674:	00 00 
   35676:	48 c7 04 24 00 00 00 	movq   $0x0,(%rsp)
   3567d:	00 
   3567e:	ff 15 14 d4 09 00    	call   *0x9d414(%rip)        # d2a98 <_DYNAMIC+0x260>
   35684:	bf 10 00 00 00       	mov    $0x10,%edi
   35689:	be 08 00 00 00       	mov    $0x8,%esi
   3568e:	ff 15 0c d4 09 00    	call   *0x9d40c(%rip)        # d2aa0 <_DYNAMIC+0x268>
   35694:	48 85 c0             	test   %rax,%rax
   35697:	0f 84 3b 05 00 00    	je     35bd8 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x708>
   3569d:	41 0f 10 04 24       	movups (%r12),%xmm0
   356a2:	0f 11 00             	movups %xmm0,(%rax)
   356a5:	48 8b 4c 24 20       	mov    0x20(%rsp),%rcx
   356aa:	48 89 4b 20          	mov    %rcx,0x20(%rbx)
   356ae:	0f 10 04 24          	movups (%rsp),%xmm0
   356b2:	0f 10 4c 24 10       	movups 0x10(%rsp),%xmm1
   356b7:	0f 11 4b 10          	movups %xmm1,0x10(%rbx)
   356bb:	0f 11 03             	movups %xmm0,(%rbx)
   356be:	49 8b 8e 98 00 00 00 	mov    0x98(%r14),%rcx
   356c5:	48 c7 43 28 02 00 00 	movq   $0x2,0x28(%rbx)
   356cc:	00 
   356cd:	48 89 43 30          	mov    %rax,0x30(%rbx)
   356d1:	48 c7 43 38 02 00 00 	movq   $0x2,0x38(%rbx)
   356d8:	00 
   356d9:	48 c7 43 40 00 00 00 	movq   $0x0,0x40(%rbx)
   356e0:	00 
   356e1:	48 89 4b 48          	mov    %rcx,0x48(%rbx)
   356e5:	e9 24 04 00 00       	jmp    35b0e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x63e>
   356ea:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   356f1:	48 83 79 08 01       	cmpq   $0x1,0x8(%rcx)
   356f6:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   356fd:	40 0f 94 c5          	sete   %bpl
   35701:	40 22 29             	and    (%rcx),%bpl
   35704:	75 20                	jne    35726 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x256>
   35706:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   3570d:	48 83 fe 02          	cmp    $0x2,%rsi
   35711:	0f 82 7b 04 00 00    	jb     35b92 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x6c2>
   35717:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   3571e:	48 c7 40 10 02 00 00 	movq   $0x2,0x10(%rax)
   35725:	00 
   35726:	4d 8d 66 40          	lea    0x40(%r14),%r12
   3572a:	41 83 7e 40 ff       	cmpl   $0xffffffff,0x40(%r14)
   3572f:	0f 85 e7 02 00 00    	jne    35a1c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x54c>
   35735:	48 8b 41 10          	mov    0x10(%rcx),%rax
   35739:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
   3573e:	0f 10 01             	movups (%rcx),%xmm0
   35741:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   35746:	48 89 e7             	mov    %rsp,%rdi
   35749:	4c 8d 44 24 40       	lea    0x40(%rsp),%r8
   3574e:	b9 01 00 00 00       	mov    $0x1,%ecx
   35753:	4c 89 fe             	mov    %r15,%rsi
   35756:	31 d2                	xor    %edx,%edx
   35758:	ff 15 8a d3 09 00    	call   *0x9d38a(%rip)        # d2ae8 <_DYNAMIC+0x2b0>
   3575e:	41 0f b6 87 f9 03 00 	movzbl 0x3f9(%r15),%eax
   35765:	00 
   35766:	34 01                	xor    $0x1,%al
   35768:	40 08 c5             	or     %al,%bpl
   3576b:	40 80 fd 01          	cmp    $0x1,%bpl
   3576f:	0f 84 72 02 00 00    	je     359e7 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x517>
   35775:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   3577c:	00 
   3577d:	0f 84 88 04 00 00    	je     35c0b <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x73b>
   35783:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   3578a:	48 8b 30             	mov    (%rax),%rsi
   3578d:	48 83 fe 02          	cmp    $0x2,%rsi
   35791:	0f 85 f2 00 00 00    	jne    35889 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x3b9>
   35797:	31 c9                	xor    %ecx,%ecx
   35799:	e9 01 01 00 00       	jmp    3589f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x3cf>
   3579e:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   357a5:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   357ac:	0f b6 29             	movzbl (%rcx),%ebp
   357af:	4c 8b 61 08          	mov    0x8(%rcx),%r12
   357b3:	49 83 fc 02          	cmp    $0x2,%r12
   357b7:	0f 92 c0             	setb   %al
   357ba:	40 84 c5             	test   %al,%bpl
   357bd:	75 1c                	jne    357db <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x30b>
   357bf:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   357c6:	00 
   357c7:	0f 84 d7 03 00 00    	je     35ba4 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x6d4>
   357cd:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   357d4:	48 c7 00 02 00 00 00 	movq   $0x2,(%rax)
   357db:	49 83 fc 01          	cmp    $0x1,%r12
   357df:	0f 94 c0             	sete   %al
   357e2:	40 84 c5             	test   %al,%bpl
   357e5:	75 20                	jne    35807 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x337>
   357e7:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   357ee:	48 83 fe 02          	cmp    $0x2,%rsi
   357f2:	0f 82 bd 03 00 00    	jb     35bb5 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x6e5>
   357f8:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   357ff:	48 c7 40 10 02 00 00 	movq   $0x2,0x10(%rax)
   35806:	00 
   35807:	41 83 3e ff          	cmpl   $0xffffffff,(%r14)
   3580b:	0f 85 25 01 00 00    	jne    35936 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x466>
   35811:	48 8b 41 10          	mov    0x10(%rcx),%rax
   35815:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
   3581a:	0f 10 01             	movups (%rcx),%xmm0
   3581d:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   35822:	48 89 e7             	mov    %rsp,%rdi
   35825:	4c 8d 44 24 40       	lea    0x40(%rsp),%r8
   3582a:	4c 89 fe             	mov    %r15,%rsi
   3582d:	31 d2                	xor    %edx,%edx
   3582f:	31 c9                	xor    %ecx,%ecx
   35831:	ff 15 b1 d2 09 00    	call   *0x9d2b1(%rip)        # d2ae8 <_DYNAMIC+0x2b0>
   35837:	41 0f b6 87 f9 03 00 	movzbl 0x3f9(%r15),%eax
   3583e:	00 
   3583f:	34 01                	xor    $0x1,%al
   35841:	4d 85 e4             	test   %r12,%r12
   35844:	0f 94 c1             	sete   %cl
   35847:	40 20 cd             	and    %cl,%bpl
   3584a:	40 08 c5             	or     %al,%bpl
   3584d:	40 80 fd 01          	cmp    $0x1,%bpl
   35851:	0f 84 af 00 00 00    	je     35906 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x436>
   35857:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   3585e:	00 
   3585f:	0f 84 b7 03 00 00    	je     35c1c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x74c>
   35865:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   3586c:	48 8b 30             	mov    (%rax),%rsi
   3586f:	48 83 fe 02          	cmp    $0x2,%rsi
   35873:	75 6d                	jne    358e2 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x412>
   35875:	31 c9                	xor    %ecx,%ecx
   35877:	eb 7f                	jmp    358f8 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x428>
   35879:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   35880:	01 00 00 00 
   35884:	e9 7e 02 00 00       	jmp    35b07 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x637>
   35889:	48 8b 50 08          	mov    0x8(%rax),%rdx
   3588d:	4c 89 ff             	mov    %r15,%rdi
   35890:	ff 15 5a d2 09 00    	call   *0x9d25a(%rip)        # d2af0 <_DYNAMIC+0x2b8>
   35896:	48 89 c1             	mov    %rax,%rcx
   35899:	49 89 d0             	mov    %rdx,%r8
   3589c:	83 e1 01             	and    $0x1,%ecx
   3589f:	48 89 e6             	mov    %rsp,%rsi
   358a2:	4c 89 ff             	mov    %r15,%rdi
   358a5:	31 d2                	xor    %edx,%edx
   358a7:	ff 15 4b d2 09 00    	call   *0x9d24b(%rip)        # d2af8 <_DYNAMIC+0x2c0>
   358ad:	84 c0                	test   %al,%al
   358af:	0f 85 32 01 00 00    	jne    359e7 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x517>
   358b5:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   358bc:	48 83 fe 02          	cmp    $0x2,%rsi
   358c0:	0f 82 89 03 00 00    	jb     35c4f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x77f>
   358c6:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   358cd:	48 8b 70 10          	mov    0x10(%rax),%rsi
   358d1:	48 83 fe 02          	cmp    $0x2,%rsi
   358d5:	0f 85 e5 00 00 00    	jne    359c0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x4f0>
   358db:	31 c9                	xor    %ecx,%ecx
   358dd:	e9 f4 00 00 00       	jmp    359d6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x506>
   358e2:	48 8b 50 08          	mov    0x8(%rax),%rdx
   358e6:	4c 89 ff             	mov    %r15,%rdi
   358e9:	ff 15 01 d2 09 00    	call   *0x9d201(%rip)        # d2af0 <_DYNAMIC+0x2b8>
   358ef:	48 89 c1             	mov    %rax,%rcx
   358f2:	49 89 d0             	mov    %rdx,%r8
   358f5:	83 e1 01             	and    $0x1,%ecx
   358f8:	48 89 e6             	mov    %rsp,%rsi
   358fb:	4c 89 ff             	mov    %r15,%rdi
   358fe:	31 d2                	xor    %edx,%edx
   35900:	ff 15 f2 d1 09 00    	call   *0x9d1f2(%rip)        # d2af8 <_DYNAMIC+0x2c0>
   35906:	0f 10 04 24          	movups (%rsp),%xmm0
   3590a:	0f 10 4c 24 10       	movups 0x10(%rsp),%xmm1
   3590f:	0f 10 54 24 20       	movups 0x20(%rsp),%xmm2
   35914:	0f 10 5c 24 30       	movups 0x30(%rsp),%xmm3
   35919:	41 0f 11 5e 30       	movups %xmm3,0x30(%r14)
   3591e:	41 0f 11 56 20       	movups %xmm2,0x20(%r14)
   35923:	41 0f 11 4e 10       	movups %xmm1,0x10(%r14)
   35928:	41 0f 11 06          	movups %xmm0,(%r14)
   3592c:	49 83 3e ff          	cmpq   $0xffffffffffffffff,(%r14)
   35930:	0f 84 10 02 00 00    	je     35b46 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x676>
   35936:	4c 89 ff             	mov    %r15,%rdi
   35939:	4c 89 f6             	mov    %r14,%rsi
   3593c:	ff 15 be d1 09 00    	call   *0x9d1be(%rip)        # d2b00 <_DYNAMIC+0x2c8>
   35942:	a8 01                	test   $0x1,%al
   35944:	74 67                	je     359ad <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x4dd>
   35946:	48 89 d5             	mov    %rdx,%rbp
   35949:	48 89 e7             	mov    %rsp,%rdi
   3594c:	4c 89 fe             	mov    %r15,%rsi
   3594f:	ff 15 73 d1 09 00    	call   *0x9d173(%rip)        # d2ac8 <_DYNAMIC+0x290>
   35955:	4c 8b 24 24          	mov    (%rsp),%r12
   35959:	49 83 fc ff          	cmp    $0xffffffffffffffff,%r12
   3595d:	0f 84 a4 01 00 00    	je     35b07 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x637>
   35963:	4c 8b 6c 24 08       	mov    0x8(%rsp),%r13
   35968:	48 83 7c 24 10 00    	cmpq   $0x0,0x10(%rsp)
   3596e:	0f 84 84 02 00 00    	je     35bf8 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x728>
   35974:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   3597b:	49 8b 4d 00          	mov    0x0(%r13),%rcx
   3597f:	4d 8b 45 08          	mov    0x8(%r13),%r8
   35983:	4c 89 ff             	mov    %r15,%rdi
   35986:	31 d2                	xor    %edx,%edx
   35988:	ff 15 42 d1 09 00    	call   *0x9d142(%rip)        # d2ad0 <_DYNAMIC+0x298>
   3598e:	84 c0                	test   %al,%al
   35990:	0f 84 3b 01 00 00    	je     35ad1 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x601>
   35996:	49 89 ae b0 00 00 00 	mov    %rbp,0xb0(%r14)
   3599d:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   359a4:	01 00 00 00 
   359a8:	e9 24 01 00 00       	jmp    35ad1 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x601>
   359ad:	49 c7 06 ff ff ff ff 	movq   $0xffffffffffffffff,(%r14)
   359b4:	48 c7 03 07 00 00 00 	movq   $0x7,(%rbx)
   359bb:	e9 4e 01 00 00       	jmp    35b0e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x63e>
   359c0:	48 8b 50 18          	mov    0x18(%rax),%rdx
   359c4:	4c 89 ff             	mov    %r15,%rdi
   359c7:	ff 15 23 d1 09 00    	call   *0x9d123(%rip)        # d2af0 <_DYNAMIC+0x2b8>
   359cd:	48 89 c1             	mov    %rax,%rcx
   359d0:	49 89 d0             	mov    %rdx,%r8
   359d3:	83 e1 01             	and    $0x1,%ecx
   359d6:	48 89 e6             	mov    %rsp,%rsi
   359d9:	ba 01 00 00 00       	mov    $0x1,%edx
   359de:	4c 89 ff             	mov    %r15,%rdi
   359e1:	ff 15 11 d1 09 00    	call   *0x9d111(%rip)        # d2af8 <_DYNAMIC+0x2c0>
   359e7:	0f 10 04 24          	movups (%rsp),%xmm0
   359eb:	0f 10 4c 24 10       	movups 0x10(%rsp),%xmm1
   359f0:	0f 10 54 24 20       	movups 0x20(%rsp),%xmm2
   359f5:	0f 10 5c 24 30       	movups 0x30(%rsp),%xmm3
   359fa:	41 0f 11 5c 24 30    	movups %xmm3,0x30(%r12)
   35a00:	41 0f 11 54 24 20    	movups %xmm2,0x20(%r12)
   35a06:	41 0f 11 4c 24 10    	movups %xmm1,0x10(%r12)
   35a0c:	41 0f 11 04 24       	movups %xmm0,(%r12)
   35a11:	49 83 3c 24 ff       	cmpq   $0xffffffffffffffff,(%r12)
   35a16:	0f 84 1d 01 00 00    	je     35b39 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x669>
   35a1c:	4c 89 ff             	mov    %r15,%rdi
   35a1f:	4c 89 e6             	mov    %r12,%rsi
   35a22:	ff 15 d8 d0 09 00    	call   *0x9d0d8(%rip)        # d2b00 <_DYNAMIC+0x2c8>
   35a28:	48 83 f8 01          	cmp    $0x1,%rax
   35a2c:	0f 85 c2 00 00 00    	jne    35af4 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x624>
   35a32:	49 39 96 b0 00 00 00 	cmp    %rdx,0xb0(%r14)
   35a39:	0f 84 c8 00 00 00    	je     35b07 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x637>
   35a3f:	48 89 e7             	mov    %rsp,%rdi
   35a42:	4c 89 fe             	mov    %r15,%rsi
   35a45:	48 89 54 24 60       	mov    %rdx,0x60(%rsp)
   35a4a:	ff 15 78 d0 09 00    	call   *0x9d078(%rip)        # d2ac8 <_DYNAMIC+0x290>
   35a50:	4c 8b 24 24          	mov    (%rsp),%r12
   35a54:	49 83 fc ff          	cmp    $0xffffffffffffffff,%r12
   35a58:	0f 84 a9 00 00 00    	je     35b07 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x637>
   35a5e:	4c 8b 6c 24 08       	mov    0x8(%rsp),%r13
   35a63:	48 8b 6c 24 10       	mov    0x10(%rsp),%rbp
   35a68:	48 85 ed             	test   %rbp,%rbp
   35a6b:	0f 84 d3 01 00 00    	je     35c44 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x774>
   35a71:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   35a78:	49 8b 4d 00          	mov    0x0(%r13),%rcx
   35a7c:	4d 8b 45 08          	mov    0x8(%r13),%r8
   35a80:	4c 89 ff             	mov    %r15,%rdi
   35a83:	31 d2                	xor    %edx,%edx
   35a85:	ff 15 45 d0 09 00    	call   *0x9d045(%rip)        # d2ad0 <_DYNAMIC+0x298>
   35a8b:	84 c0                	test   %al,%al
   35a8d:	74 42                	je     35ad1 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x601>
   35a8f:	48 83 fd 01          	cmp    $0x1,%rbp
   35a93:	0f 84 c8 01 00 00    	je     35c61 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x791>
   35a99:	49 8b 4d 10          	mov    0x10(%r13),%rcx
   35a9d:	4d 8b 45 18          	mov    0x18(%r13),%r8
   35aa1:	ba 01 00 00 00       	mov    $0x1,%edx
   35aa6:	4c 89 ff             	mov    %r15,%rdi
   35aa9:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   35ab0:	ff 15 1a d0 09 00    	call   *0x9d01a(%rip)        # d2ad0 <_DYNAMIC+0x298>
   35ab6:	84 c0                	test   %al,%al
   35ab8:	48 8b 44 24 60       	mov    0x60(%rsp),%rax
   35abd:	74 12                	je     35ad1 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x601>
   35abf:	49 89 86 b8 00 00 00 	mov    %rax,0xb8(%r14)
   35ac6:	49 c7 86 a8 00 00 00 	movq   $0x2,0xa8(%r14)
   35acd:	02 00 00 00 
   35ad1:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   35ad8:	4d 85 e4             	test   %r12,%r12
   35adb:	74 31                	je     35b0e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x63e>
   35add:	49 c1 e4 04          	shl    $0x4,%r12
   35ae1:	ba 08 00 00 00       	mov    $0x8,%edx
   35ae6:	4c 89 ef             	mov    %r13,%rdi
   35ae9:	4c 89 e6             	mov    %r12,%rsi
   35aec:	ff 15 7e cf 09 00    	call   *0x9cf7e(%rip)        # d2a70 <_DYNAMIC+0x238>
   35af2:	eb 1a                	jmp    35b0e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x63e>
   35af4:	49 c7 46 40 ff ff ff 	movq   $0xffffffffffffffff,0x40(%r14)
   35afb:	ff 
   35afc:	49 c7 86 a8 00 00 00 	movq   $0x0,0xa8(%r14)
   35b03:	00 00 00 00 
   35b07:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   35b0e:	48 89 d8             	mov    %rbx,%rax
   35b11:	48 83 c4 68          	add    $0x68,%rsp
   35b15:	5b                   	pop    %rbx
   35b16:	41 5c                	pop    %r12
   35b18:	41 5d                	pop    %r13
   35b1a:	41 5e                	pop    %r14
   35b1c:	41 5f                	pop    %r15
   35b1e:	5d                   	pop    %rbp
   35b1f:	c3                   	ret    
   35b20:	48 8d 3d 06 6a fe ff 	lea    -0x195fa(%rip),%rdi        # 1c52d <anon.031e5539418131618fb4d42bde787972.0.llvm.14684163749403725453+0x6b9>
   35b27:	48 8d 15 0a 9c 09 00 	lea    0x99c0a(%rip),%rdx        # cf738 <__frame_dummy_init_array_entry+0x2a8>
   35b2e:	be 28 00 00 00       	mov    $0x28,%esi
   35b33:	ff 15 cf cf 09 00    	call   *0x9cfcf(%rip)        # d2b08 <_DYNAMIC+0x2d0>
   35b39:	48 8d 3d 80 9b 09 00 	lea    0x99b80(%rip),%rdi        # cf6c0 <__frame_dummy_init_array_entry+0x230>
   35b40:	ff 15 42 cf 09 00    	call   *0x9cf42(%rip)        # d2a88 <_DYNAMIC+0x250>
   35b46:	48 8d 3d fb 9a 09 00 	lea    0x99afb(%rip),%rdi        # cf648 <__frame_dummy_init_array_entry+0x1b8>
   35b4d:	ff 15 35 cf 09 00    	call   *0x9cf35(%rip)        # d2a88 <_DYNAMIC+0x250>
   35b53:	bf 08 00 00 00       	mov    $0x8,%edi
   35b58:	be 10 00 00 00       	mov    $0x10,%esi
   35b5d:	ff 15 45 cf 09 00    	call   *0x9cf45(%rip)        # d2aa8 <_DYNAMIC+0x270>
   35b63:	48 8d 3d c3 69 fe ff 	lea    -0x1963d(%rip),%rdi        # 1c52d <anon.031e5539418131618fb4d42bde787972.0.llvm.14684163749403725453+0x6b9>
   35b6a:	48 8d 15 77 9a 09 00 	lea    0x99a77(%rip),%rdx        # cf5e8 <__frame_dummy_init_array_entry+0x158>
   35b71:	be 28 00 00 00       	mov    $0x28,%esi
   35b76:	ff 15 8c cf 09 00    	call   *0x9cf8c(%rip)        # d2b08 <_DYNAMIC+0x2d0>
   35b7c:	e9 f5 00 00 00       	jmp    35c76 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   35b81:	48 8d 15 80 9b 09 00 	lea    0x99b80(%rip),%rdx        # cf708 <__frame_dummy_init_array_entry+0x278>
   35b88:	31 ff                	xor    %edi,%edi
   35b8a:	31 f6                	xor    %esi,%esi
   35b8c:	ff 15 7e cf 09 00    	call   *0x9cf7e(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35b92:	48 8d 15 df 9a 09 00 	lea    0x99adf(%rip),%rdx        # cf678 <__frame_dummy_init_array_entry+0x1e8>
   35b99:	bf 01 00 00 00       	mov    $0x1,%edi
   35b9e:	ff 15 6c cf 09 00    	call   *0x9cf6c(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35ba4:	48 8d 15 55 9a 09 00 	lea    0x99a55(%rip),%rdx        # cf600 <__frame_dummy_init_array_entry+0x170>
   35bab:	31 ff                	xor    %edi,%edi
   35bad:	31 f6                	xor    %esi,%esi
   35baf:	ff 15 5b cf 09 00    	call   *0x9cf5b(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35bb5:	48 8d 15 5c 9a 09 00 	lea    0x99a5c(%rip),%rdx        # cf618 <__frame_dummy_init_array_entry+0x188>
   35bbc:	bf 01 00 00 00       	mov    $0x1,%edi
   35bc1:	ff 15 49 cf 09 00    	call   *0x9cf49(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35bc7:	48 8d 15 52 9b 09 00 	lea    0x99b52(%rip),%rdx        # cf720 <__frame_dummy_init_array_entry+0x290>
   35bce:	31 ff                	xor    %edi,%edi
   35bd0:	31 f6                	xor    %esi,%esi
   35bd2:	ff 15 38 cf 09 00    	call   *0x9cf38(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35bd8:	bf 08 00 00 00       	mov    $0x8,%edi
   35bdd:	be 10 00 00 00       	mov    $0x10,%esi
   35be2:	ff 15 30 cf 09 00    	call   *0x9cf30(%rip)        # d2b18 <_DYNAMIC+0x2e0>
   35be8:	e9 89 00 00 00       	jmp    35c76 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   35bed:	48 8d 15 c4 99 09 00 	lea    0x999c4(%rip),%rdx        # cf5b8 <__frame_dummy_init_array_entry+0x128>
   35bf4:	31 ff                	xor    %edi,%edi
   35bf6:	eb 41                	jmp    35c39 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x769>
   35bf8:	48 8d 15 61 9a 09 00 	lea    0x99a61(%rip),%rdx        # cf660 <__frame_dummy_init_array_entry+0x1d0>
   35bff:	31 ff                	xor    %edi,%edi
   35c01:	31 f6                	xor    %esi,%esi
   35c03:	ff 15 07 cf 09 00    	call   *0x9cf07(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35c09:	eb 6b                	jmp    35c76 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   35c0b:	48 8d 15 7e 9a 09 00 	lea    0x99a7e(%rip),%rdx        # cf690 <__frame_dummy_init_array_entry+0x200>
   35c12:	31 ff                	xor    %edi,%edi
   35c14:	31 f6                	xor    %esi,%esi
   35c16:	ff 15 f4 ce 09 00    	call   *0x9cef4(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35c1c:	48 8d 15 0d 9a 09 00 	lea    0x99a0d(%rip),%rdx        # cf630 <__frame_dummy_init_array_entry+0x1a0>
   35c23:	31 ff                	xor    %edi,%edi
   35c25:	31 f6                	xor    %esi,%esi
   35c27:	ff 15 e3 ce 09 00    	call   *0x9cee3(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35c2d:	48 8d 15 9c 99 09 00 	lea    0x9999c(%rip),%rdx        # cf5d0 <__frame_dummy_init_array_entry+0x140>
   35c34:	bf 01 00 00 00       	mov    $0x1,%edi
   35c39:	48 89 fe             	mov    %rdi,%rsi
   35c3c:	ff 15 ce ce 09 00    	call   *0x9cece(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35c42:	eb 32                	jmp    35c76 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   35c44:	48 8d 15 8d 9a 09 00 	lea    0x99a8d(%rip),%rdx        # cf6d8 <__frame_dummy_init_array_entry+0x248>
   35c4b:	31 ff                	xor    %edi,%edi
   35c4d:	eb 1e                	jmp    35c6d <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x79d>
   35c4f:	48 8d 15 52 9a 09 00 	lea    0x99a52(%rip),%rdx        # cf6a8 <__frame_dummy_init_array_entry+0x218>
   35c56:	bf 01 00 00 00       	mov    $0x1,%edi
   35c5b:	ff 15 af ce 09 00    	call   *0x9ceaf(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35c61:	48 8d 15 88 9a 09 00 	lea    0x99a88(%rip),%rdx        # cf6f0 <__frame_dummy_init_array_entry+0x260>
   35c68:	bf 01 00 00 00       	mov    $0x1,%edi
   35c6d:	48 89 fe             	mov    %rdi,%rsi
   35c70:	ff 15 9a ce 09 00    	call   *0x9ce9a(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   35c76:	0f 0b                	ud2    
   35c78:	eb 15                	jmp    35c8f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7bf>
   35c7a:	48 89 c3             	mov    %rax,%rbx
   35c7d:	48 89 e7             	mov    %rsp,%rdi
   35c80:	e8 0b ed ff ff       	call   34990 <core::ptr::drop_glue::<chr_compiled::Work>>
   35c85:	eb 25                	jmp    35cac <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7dc>
   35c87:	ff 15 eb cd 09 00    	call   *0x9cdeb(%rip)        # d2a78 <_DYNAMIC+0x240>
   35c8d:	eb 00                	jmp    35c8f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7bf>
   35c8f:	48 89 c3             	mov    %rax,%rbx
   35c92:	4d 85 e4             	test   %r12,%r12
   35c95:	74 15                	je     35cac <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7dc>
   35c97:	49 c1 e4 04          	shl    $0x4,%r12
   35c9b:	ba 08 00 00 00       	mov    $0x8,%edx
   35ca0:	4c 89 ef             	mov    %r13,%rdi
   35ca3:	4c 89 e6             	mov    %r12,%rsi
   35ca6:	ff 15 c4 cd 09 00    	call   *0x9cdc4(%rip)        # d2a70 <_DYNAMIC+0x238>
   35cac:	48 89 df             	mov    %rbx,%rdi
   35caf:	e8 9c 87 09 00       	call   ce450 <_Unwind_Resume@plt>
   35cb4:	cc                   	int3   
   35cb5:	cc                   	int3   
   35cb6:	cc                   	int3   
   35cb7:	cc                   	int3   
   35cb8:	cc                   	int3   
   35cb9:	cc                   	int3   
   35cba:	cc                   	int3   
   35cbb:	cc                   	int3   
   35cbc:	cc                   	int3   
   35cbd:	cc                   	int3   
   35cbe:	cc                   	int3   
   35cbf:	cc                   	int3   

0000000000035e40 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick>:
   35e40:	55                   	push   %rbp
   35e41:	41 57                	push   %r15
   35e43:	41 56                	push   %r14
   35e45:	41 55                	push   %r13
   35e47:	41 54                	push   %r12
   35e49:	53                   	push   %rbx
   35e4a:	48 81 ec a8 00 00 00 	sub    $0xa8,%rsp
   35e51:	49 89 d7             	mov    %rdx,%r15
   35e54:	49 89 f6             	mov    %rsi,%r14
   35e57:	48 89 fb             	mov    %rdi,%rbx
   35e5a:	80 be c0 00 00 00 00 	cmpb   $0x0,0xc0(%rsi)
   35e61:	0f 84 ba 00 00 00    	je     35f21 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe1>
   35e67:	41 c6 86 c0 00 00 00 	movb   $0x0,0xc0(%r14)
   35e6e:	00 
   35e6f:	83 39 01             	cmpl   $0x1,(%rcx)
   35e72:	0f 85 a9 00 00 00    	jne    35f21 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe1>
   35e78:	48 8b 69 08          	mov    0x8(%rcx),%rbp
   35e7c:	48 85 ed             	test   %rbp,%rbp
   35e7f:	0f 84 9c 00 00 00    	je     35f21 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe1>
   35e85:	48 8b 51 10          	mov    0x10(%rcx),%rdx
   35e89:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
   35e8e:	4c 89 fe             	mov    %r15,%rsi
   35e91:	ff 15 31 cc 09 00    	call   *0x9cc31(%rip)        # d2ac8 <_DYNAMIC+0x290>
   35e97:	4c 8b 64 24 08       	mov    0x8(%rsp),%r12
   35e9c:	49 83 fc ff          	cmp    $0xffffffffffffffff,%r12
   35ea0:	0f 84 e0 05 00 00    	je     36486 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x646>
   35ea6:	4c 8b 6c 24 10       	mov    0x10(%rsp),%r13
   35eab:	48 83 fd 01          	cmp    $0x1,%rbp
   35eaf:	0f 85 b3 07 00 00    	jne    36668 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x828>
   35eb5:	48 8b 6c 24 18       	mov    0x18(%rsp),%rbp
   35eba:	48 85 ed             	test   %rbp,%rbp
   35ebd:	0f 84 31 08 00 00    	je     366f4 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8b4>
   35ec3:	49 83 ee 80          	sub    $0xffffffffffffff80,%r14
   35ec7:	49 8b 4d 00          	mov    0x0(%r13),%rcx
   35ecb:	4d 8b 45 08          	mov    0x8(%r13),%r8
   35ecf:	4c 89 ff             	mov    %r15,%rdi
   35ed2:	4c 89 f6             	mov    %r14,%rsi
   35ed5:	31 d2                	xor    %edx,%edx
   35ed7:	ff 15 f3 cb 09 00    	call   *0x9cbf3(%rip)        # d2ad0 <_DYNAMIC+0x298>
   35edd:	b9 07 00 00 00       	mov    $0x7,%ecx
   35ee2:	84 c0                	test   %al,%al
   35ee4:	74 2a                	je     35f10 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xd0>
   35ee6:	48 83 fd 01          	cmp    $0x1,%rbp
   35eea:	0f 84 44 08 00 00    	je     36734 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8f4>
   35ef0:	49 8b 4d 10          	mov    0x10(%r13),%rcx
   35ef4:	4d 8b 45 18          	mov    0x18(%r13),%r8
   35ef8:	ba 01 00 00 00       	mov    $0x1,%edx
   35efd:	4c 89 ff             	mov    %r15,%rdi
   35f00:	4c 89 f6             	mov    %r14,%rsi
   35f03:	ff 15 c7 cb 09 00    	call   *0x9cbc7(%rip)        # d2ad0 <_DYNAMIC+0x298>
   35f09:	0f b6 c8             	movzbl %al,%ecx
   35f0c:	48 83 f1 07          	xor    $0x7,%rcx
   35f10:	48 89 0b             	mov    %rcx,(%rbx)
   35f13:	4d 85 e4             	test   %r12,%r12
   35f16:	0f 85 99 06 00 00    	jne    365b5 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x775>
   35f1c:	e9 c5 06 00 00       	jmp    365e6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   35f21:	49 8b 86 a8 00 00 00 	mov    0xa8(%r14),%rax
   35f28:	48 85 c0             	test   %rax,%rax
   35f2b:	0f 84 2f 03 00 00    	je     36260 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x420>
   35f31:	48 83 f8 01          	cmp    $0x1,%rax
   35f35:	0f 84 6c 02 00 00    	je     361a7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x367>
   35f3b:	48 83 f8 02          	cmp    $0x2,%rax
   35f3f:	0f 85 b6 06 00 00    	jne    365fb <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7bb>
   35f45:	49 8d 96 b0 00 00 00 	lea    0xb0(%r14),%rdx
   35f4c:	be 01 00 00 00       	mov    $0x1,%esi
   35f51:	b9 02 00 00 00       	mov    $0x2,%ecx
   35f56:	4c 89 ff             	mov    %r15,%rdi
   35f59:	ff 15 79 cb 09 00    	call   *0x9cb79(%rip)        # d2ad8 <_DYNAMIC+0x2a0>
   35f5f:	84 c0                	test   %al,%al
   35f61:	0f 84 dc 03 00 00    	je     36343 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x503>
   35f67:	4d 8d ae 80 00 00 00 	lea    0x80(%r14),%r13
   35f6e:	4c 8b 25 6b cb 09 00 	mov    0x9cb6b(%rip),%r12        # d2ae0 <_DYNAMIC+0x2a8>
   35f75:	ba 01 00 00 00       	mov    $0x1,%edx
   35f7a:	4c 89 ff             	mov    %r15,%rdi
   35f7d:	4c 89 ee             	mov    %r13,%rsi
   35f80:	41 ff d4             	call   *%r12
   35f83:	48 89 c5             	mov    %rax,%rbp
   35f86:	48 89 54 24 48       	mov    %rdx,0x48(%rsp)
   35f8b:	4c 89 ff             	mov    %r15,%rdi
   35f8e:	4c 89 ee             	mov    %r13,%rsi
   35f91:	31 d2                	xor    %edx,%edx
   35f93:	41 ff d4             	call   *%r12
   35f96:	49 89 c5             	mov    %rax,%r13
   35f99:	49 89 d4             	mov    %rdx,%r12
   35f9c:	ff 15 f6 ca 09 00    	call   *0x9caf6(%rip)        # d2a98 <_DYNAMIC+0x260>
   35fa2:	bf 10 00 00 00       	mov    $0x10,%edi
   35fa7:	be 08 00 00 00       	mov    $0x8,%esi
   35fac:	ff 15 ee ca 09 00    	call   *0x9caee(%rip)        # d2aa0 <_DYNAMIC+0x268>
   35fb2:	48 85 c0             	test   %rax,%rax
   35fb5:	0f 84 73 06 00 00    	je     3662e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7ee>
   35fbb:	4c 89 28             	mov    %r13,(%rax)
   35fbe:	4c 89 60 08          	mov    %r12,0x8(%rax)
   35fc2:	48 c7 44 24 08 01 00 	movq   $0x1,0x8(%rsp)
   35fc9:	00 00 
   35fcb:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   35fd0:	48 c7 44 24 18 01 00 	movq   $0x1,0x18(%rsp)
   35fd7:	00 00 
   35fd9:	48 8d 35 20 63 fe ff 	lea    -0x19ce0(%rip),%rsi        # 1c300 <anon.031e5539418131618fb4d42bde787972.0.llvm.14684163749403725453+0x48c>
   35fe0:	48 8d 4c 24 08       	lea    0x8(%rsp),%rcx
   35fe5:	ba 04 00 00 00       	mov    $0x4,%edx
   35fea:	4c 89 ff             	mov    %r15,%rdi
   35fed:	ff 15 35 cb 09 00    	call   *0x9cb35(%rip)        # d2b28 <_DYNAMIC+0x2f0>
   35ff3:	48 89 ac 24 88 00 00 	mov    %rbp,0x88(%rsp)
   35ffa:	00 
   35ffb:	48 8b 4c 24 48       	mov    0x48(%rsp),%rcx
   36000:	48 89 8c 24 90 00 00 	mov    %rcx,0x90(%rsp)
   36007:	00 
   36008:	48 89 84 24 98 00 00 	mov    %rax,0x98(%rsp)
   3600f:	00 
   36010:	48 89 94 24 a0 00 00 	mov    %rdx,0xa0(%rsp)
   36017:	00 
   36018:	48 c7 84 24 80 00 00 	movq   $0x1,0x80(%rsp)
   3601f:	00 01 00 00 00 
   36024:	4c 89 ff             	mov    %r15,%rdi
   36027:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   3602e:	31 d2                	xor    %edx,%edx
   36030:	ff 15 aa ca 09 00    	call   *0x9caaa(%rip)        # d2ae0 <_DYNAMIC+0x2a8>
   36036:	49 89 c4             	mov    %rax,%r12
   36039:	49 8b 87 08 01 00 00 	mov    0x108(%r15),%rax
   36040:	48 8b 70 20          	mov    0x20(%rax),%rsi
   36044:	48 83 fe 02          	cmp    $0x2,%rsi
   36048:	0f 82 6d 06 00 00    	jb     366bb <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x87b>
   3604e:	48 8b 40 18          	mov    0x18(%rax),%rax
   36052:	48 83 b8 f8 00 00 00 	cmpq   $0x0,0xf8(%rax)
   36059:	00 
   3605a:	0f 84 69 06 00 00    	je     366c9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x889>
   36060:	49 89 d5             	mov    %rdx,%r13
   36063:	48 8b 80 f0 00 00 00 	mov    0xf0(%rax),%rax
   3606a:	4c 8b 38             	mov    (%rax),%r15
   3606d:	ff 15 25 ca 09 00    	call   *0x9ca25(%rip)        # d2a98 <_DYNAMIC+0x260>
   36073:	bf 10 00 00 00       	mov    $0x10,%edi
   36078:	be 08 00 00 00       	mov    $0x8,%esi
   3607d:	ff 15 1d ca 09 00    	call   *0x9ca1d(%rip)        # d2aa0 <_DYNAMIC+0x268>
   36083:	48 85 c0             	test   %rax,%rax
   36086:	0f 84 b2 05 00 00    	je     3663e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7fe>
   3608c:	4c 89 20             	mov    %r12,(%rax)
   3608f:	4c 89 68 08          	mov    %r13,0x8(%rax)
   36093:	4c 89 7c 24 58       	mov    %r15,0x58(%rsp)
   36098:	48 c7 44 24 60 01 00 	movq   $0x1,0x60(%rsp)
   3609f:	00 00 
   360a1:	48 89 44 24 68       	mov    %rax,0x68(%rsp)
   360a6:	48 c7 44 24 70 01 00 	movq   $0x1,0x70(%rsp)
   360ad:	00 00 
   360af:	48 c7 44 24 50 00 00 	movq   $0x0,0x50(%rsp)
   360b6:	00 00 
   360b8:	ff 15 da c9 09 00    	call   *0x9c9da(%rip)        # d2a98 <_DYNAMIC+0x260>
   360be:	bf 50 00 00 00       	mov    $0x50,%edi
   360c3:	be 08 00 00 00       	mov    $0x8,%esi
   360c8:	ff 15 d2 c9 09 00    	call   *0x9c9d2(%rip)        # d2aa0 <_DYNAMIC+0x268>
   360ce:	48 85 c0             	test   %rax,%rax
   360d1:	0f 84 7c 05 00 00    	je     36653 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x813>
   360d7:	48 8b 8c 24 a0 00 00 	mov    0xa0(%rsp),%rcx
   360de:	00 
   360df:	48 89 48 20          	mov    %rcx,0x20(%rax)
   360e3:	0f 10 84 24 80 00 00 	movups 0x80(%rsp),%xmm0
   360ea:	00 
   360eb:	0f 10 8c 24 90 00 00 	movups 0x90(%rsp),%xmm1
   360f2:	00 
   360f3:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   360f7:	0f 11 00             	movups %xmm0,(%rax)
   360fa:	0f 10 44 24 50       	movups 0x50(%rsp),%xmm0
   360ff:	0f 10 4c 24 60       	movups 0x60(%rsp),%xmm1
   36104:	0f 11 40 28          	movups %xmm0,0x28(%rax)
   36108:	0f 11 48 38          	movups %xmm1,0x38(%rax)
   3610c:	48 8b 4c 24 70       	mov    0x70(%rsp),%rcx
   36111:	48 89 48 48          	mov    %rcx,0x48(%rax)
   36115:	48 c7 44 24 10 02 00 	movq   $0x2,0x10(%rsp)
   3611c:	00 00 
   3611e:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   36123:	48 c7 44 24 20 02 00 	movq   $0x2,0x20(%rsp)
   3612a:	00 00 
   3612c:	48 c7 44 24 08 02 00 	movq   $0x2,0x8(%rsp)
   36133:	00 00 
   36135:	ff 15 5d c9 09 00    	call   *0x9c95d(%rip)        # d2a98 <_DYNAMIC+0x260>
   3613b:	bf 10 00 00 00       	mov    $0x10,%edi
   36140:	be 08 00 00 00       	mov    $0x8,%esi
   36145:	ff 15 55 c9 09 00    	call   *0x9c955(%rip)        # d2aa0 <_DYNAMIC+0x268>
   3614b:	48 85 c0             	test   %rax,%rax
   3614e:	49 8d 8e b0 00 00 00 	lea    0xb0(%r14),%rcx
   36155:	0f 84 84 05 00 00    	je     366df <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x89f>
   3615b:	0f 10 01             	movups (%rcx),%xmm0
   3615e:	0f 11 00             	movups %xmm0,(%rax)
   36161:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   36166:	48 89 4b 20          	mov    %rcx,0x20(%rbx)
   3616a:	0f 10 44 24 08       	movups 0x8(%rsp),%xmm0
   3616f:	0f 10 4c 24 18       	movups 0x18(%rsp),%xmm1
   36174:	0f 11 4b 10          	movups %xmm1,0x10(%rbx)
   36178:	0f 11 03             	movups %xmm0,(%rbx)
   3617b:	49 8b 8e 98 00 00 00 	mov    0x98(%r14),%rcx
   36182:	48 c7 43 28 02 00 00 	movq   $0x2,0x28(%rbx)
   36189:	00 
   3618a:	48 89 43 30          	mov    %rax,0x30(%rbx)
   3618e:	48 c7 43 38 02 00 00 	movq   $0x2,0x38(%rbx)
   36195:	00 
   36196:	48 c7 43 40 01 00 00 	movq   $0x1,0x40(%rbx)
   3619d:	00 
   3619e:	48 89 4b 48          	mov    %rcx,0x48(%rbx)
   361a2:	e9 3f 04 00 00       	jmp    365e6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   361a7:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   361ae:	48 83 79 08 01       	cmpq   $0x1,0x8(%rcx)
   361b3:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   361ba:	40 0f 94 c5          	sete   %bpl
   361be:	40 22 29             	and    (%rcx),%bpl
   361c1:	75 20                	jne    361e3 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x3a3>
   361c3:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   361ca:	48 83 fe 02          	cmp    $0x2,%rsi
   361ce:	0f 82 b2 04 00 00    	jb     36686 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x846>
   361d4:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   361db:	48 c7 40 10 02 00 00 	movq   $0x2,0x10(%rax)
   361e2:	00 
   361e3:	4d 8d 66 40          	lea    0x40(%r14),%r12
   361e7:	41 83 7e 40 ff       	cmpl   $0xffffffff,0x40(%r14)
   361ec:	0f 85 ff 02 00 00    	jne    364f1 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x6b1>
   361f2:	48 8b 41 10          	mov    0x10(%rcx),%rax
   361f6:	48 89 44 24 60       	mov    %rax,0x60(%rsp)
   361fb:	0f 10 01             	movups (%rcx),%xmm0
   361fe:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   36203:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
   36208:	4c 8d 44 24 50       	lea    0x50(%rsp),%r8
   3620d:	ba 01 00 00 00       	mov    $0x1,%edx
   36212:	b9 01 00 00 00       	mov    $0x1,%ecx
   36217:	4c 89 fe             	mov    %r15,%rsi
   3621a:	ff 15 c8 c8 09 00    	call   *0x9c8c8(%rip)        # d2ae8 <_DYNAMIC+0x2b0>
   36220:	41 0f b6 87 f9 03 00 	movzbl 0x3f9(%r15),%eax
   36227:	00 
   36228:	34 01                	xor    $0x1,%al
   3622a:	40 08 c5             	or     %al,%bpl
   3622d:	40 80 fd 01          	cmp    $0x1,%bpl
   36231:	0f 84 84 02 00 00    	je     364bb <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x67b>
   36237:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   3623e:	00 
   3623f:	0f 84 cd 04 00 00    	je     36712 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8d2>
   36245:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   3624c:	48 8b 30             	mov    (%rax),%rsi
   3624f:	48 83 fe 02          	cmp    $0x2,%rsi
   36253:	0f 85 fa 00 00 00    	jne    36353 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x513>
   36259:	31 c9                	xor    %ecx,%ecx
   3625b:	e9 09 01 00 00       	jmp    36369 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x529>
   36260:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   36267:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   3626e:	0f b6 29             	movzbl (%rcx),%ebp
   36271:	4c 8b 61 08          	mov    0x8(%rcx),%r12
   36275:	49 83 fc 02          	cmp    $0x2,%r12
   36279:	0f 92 c0             	setb   %al
   3627c:	40 84 c5             	test   %al,%bpl
   3627f:	75 1c                	jne    3629d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x45d>
   36281:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   36288:	00 
   36289:	0f 84 09 04 00 00    	je     36698 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x858>
   3628f:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   36296:	48 c7 00 02 00 00 00 	movq   $0x2,(%rax)
   3629d:	49 83 fc 01          	cmp    $0x1,%r12
   362a1:	0f 94 c0             	sete   %al
   362a4:	40 84 c5             	test   %al,%bpl
   362a7:	75 20                	jne    362c9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x489>
   362a9:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   362b0:	48 83 fe 02          	cmp    $0x2,%rsi
   362b4:	0f 82 ef 03 00 00    	jb     366a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x869>
   362ba:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   362c1:	48 c7 40 10 02 00 00 	movq   $0x2,0x10(%rax)
   362c8:	00 
   362c9:	41 83 3e ff          	cmpl   $0xffffffff,(%r14)
   362cd:	0f 85 32 01 00 00    	jne    36405 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x5c5>
   362d3:	48 8b 41 10          	mov    0x10(%rcx),%rax
   362d7:	48 89 44 24 60       	mov    %rax,0x60(%rsp)
   362dc:	0f 10 01             	movups (%rcx),%xmm0
   362df:	0f 29 44 24 50       	movaps %xmm0,0x50(%rsp)
   362e4:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
   362e9:	4c 8d 44 24 50       	lea    0x50(%rsp),%r8
   362ee:	ba 01 00 00 00       	mov    $0x1,%edx
   362f3:	4c 89 fe             	mov    %r15,%rsi
   362f6:	31 c9                	xor    %ecx,%ecx
   362f8:	ff 15 ea c7 09 00    	call   *0x9c7ea(%rip)        # d2ae8 <_DYNAMIC+0x2b0>
   362fe:	41 0f b6 87 f9 03 00 	movzbl 0x3f9(%r15),%eax
   36305:	00 
   36306:	34 01                	xor    $0x1,%al
   36308:	4d 85 e4             	test   %r12,%r12
   3630b:	0f 94 c1             	sete   %cl
   3630e:	40 20 cd             	and    %cl,%bpl
   36311:	40 08 c5             	or     %al,%bpl
   36314:	40 80 fd 01          	cmp    $0x1,%bpl
   36318:	0f 84 b6 00 00 00    	je     363d4 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x594>
   3631e:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   36325:	00 
   36326:	0f 84 f7 03 00 00    	je     36723 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8e3>
   3632c:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   36333:	48 8b 30             	mov    (%rax),%rsi
   36336:	48 83 fe 02          	cmp    $0x2,%rsi
   3633a:	75 72                	jne    363ae <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x56e>
   3633c:	31 c9                	xor    %ecx,%ecx
   3633e:	e9 81 00 00 00       	jmp    363c4 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x584>
   36343:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   3634a:	01 00 00 00 
   3634e:	e9 8c 02 00 00       	jmp    365df <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x79f>
   36353:	48 8b 50 08          	mov    0x8(%rax),%rdx
   36357:	4c 89 ff             	mov    %r15,%rdi
   3635a:	ff 15 90 c7 09 00    	call   *0x9c790(%rip)        # d2af0 <_DYNAMIC+0x2b8>
   36360:	48 89 c1             	mov    %rax,%rcx
   36363:	49 89 d0             	mov    %rdx,%r8
   36366:	83 e1 01             	and    $0x1,%ecx
   36369:	48 8d 74 24 08       	lea    0x8(%rsp),%rsi
   3636e:	4c 89 ff             	mov    %r15,%rdi
   36371:	31 d2                	xor    %edx,%edx
   36373:	ff 15 7f c7 09 00    	call   *0x9c77f(%rip)        # d2af8 <_DYNAMIC+0x2c0>
   36379:	84 c0                	test   %al,%al
   3637b:	0f 85 3a 01 00 00    	jne    364bb <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x67b>
   36381:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   36388:	48 83 fe 02          	cmp    $0x2,%rsi
   3638c:	0f 82 c4 03 00 00    	jb     36756 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x916>
   36392:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   36399:	48 8b 70 10          	mov    0x10(%rax),%rsi
   3639d:	48 83 fe 02          	cmp    $0x2,%rsi
   363a1:	0f 85 eb 00 00 00    	jne    36492 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x652>
   363a7:	31 c9                	xor    %ecx,%ecx
   363a9:	e9 fa 00 00 00       	jmp    364a8 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x668>
   363ae:	48 8b 50 08          	mov    0x8(%rax),%rdx
   363b2:	4c 89 ff             	mov    %r15,%rdi
   363b5:	ff 15 35 c7 09 00    	call   *0x9c735(%rip)        # d2af0 <_DYNAMIC+0x2b8>
   363bb:	48 89 c1             	mov    %rax,%rcx
   363be:	49 89 d0             	mov    %rdx,%r8
   363c1:	83 e1 01             	and    $0x1,%ecx
   363c4:	48 8d 74 24 08       	lea    0x8(%rsp),%rsi
   363c9:	4c 89 ff             	mov    %r15,%rdi
   363cc:	31 d2                	xor    %edx,%edx
   363ce:	ff 15 24 c7 09 00    	call   *0x9c724(%rip)        # d2af8 <_DYNAMIC+0x2c0>
   363d4:	0f 10 44 24 08       	movups 0x8(%rsp),%xmm0
   363d9:	0f 10 4c 24 18       	movups 0x18(%rsp),%xmm1
   363de:	0f 10 54 24 28       	movups 0x28(%rsp),%xmm2
   363e3:	0f 10 5c 24 38       	movups 0x38(%rsp),%xmm3
   363e8:	41 0f 11 5e 30       	movups %xmm3,0x30(%r14)
   363ed:	41 0f 11 56 20       	movups %xmm2,0x20(%r14)
   363f2:	41 0f 11 4e 10       	movups %xmm1,0x10(%r14)
   363f7:	41 0f 11 06          	movups %xmm0,(%r14)
   363fb:	49 83 3e ff          	cmpq   $0xffffffffffffffff,(%r14)
   363ff:	0f 84 1c 02 00 00    	je     36621 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7e1>
   36405:	4c 89 ff             	mov    %r15,%rdi
   36408:	4c 89 f6             	mov    %r14,%rsi
   3640b:	ff 15 ef c6 09 00    	call   *0x9c6ef(%rip)        # d2b00 <_DYNAMIC+0x2c8>
   36411:	a8 01                	test   $0x1,%al
   36413:	74 6a                	je     3647f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x63f>
   36415:	48 89 d5             	mov    %rdx,%rbp
   36418:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
   3641d:	4c 89 fe             	mov    %r15,%rsi
   36420:	ff 15 a2 c6 09 00    	call   *0x9c6a2(%rip)        # d2ac8 <_DYNAMIC+0x290>
   36426:	4c 8b 64 24 08       	mov    0x8(%rsp),%r12
   3642b:	49 83 fc ff          	cmp    $0xffffffffffffffff,%r12
   3642f:	0f 84 aa 01 00 00    	je     365df <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x79f>
   36435:	4c 8b 6c 24 10       	mov    0x10(%rsp),%r13
   3643a:	48 83 7c 24 18 00    	cmpq   $0x0,0x18(%rsp)
   36440:	0f 84 b9 02 00 00    	je     366ff <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8bf>
   36446:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   3644d:	49 8b 4d 00          	mov    0x0(%r13),%rcx
   36451:	4d 8b 45 08          	mov    0x8(%r13),%r8
   36455:	4c 89 ff             	mov    %r15,%rdi
   36458:	31 d2                	xor    %edx,%edx
   3645a:	ff 15 70 c6 09 00    	call   *0x9c670(%rip)        # d2ad0 <_DYNAMIC+0x298>
   36460:	84 c0                	test   %al,%al
   36462:	0f 84 41 01 00 00    	je     365a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x769>
   36468:	49 89 ae b0 00 00 00 	mov    %rbp,0xb0(%r14)
   3646f:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   36476:	01 00 00 00 
   3647a:	e9 2a 01 00 00       	jmp    365a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x769>
   3647f:	49 c7 06 ff ff ff ff 	movq   $0xffffffffffffffff,(%r14)
   36486:	48 c7 03 07 00 00 00 	movq   $0x7,(%rbx)
   3648d:	e9 54 01 00 00       	jmp    365e6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   36492:	48 8b 50 18          	mov    0x18(%rax),%rdx
   36496:	4c 89 ff             	mov    %r15,%rdi
   36499:	ff 15 51 c6 09 00    	call   *0x9c651(%rip)        # d2af0 <_DYNAMIC+0x2b8>
   3649f:	48 89 c1             	mov    %rax,%rcx
   364a2:	49 89 d0             	mov    %rdx,%r8
   364a5:	83 e1 01             	and    $0x1,%ecx
   364a8:	48 8d 74 24 08       	lea    0x8(%rsp),%rsi
   364ad:	ba 01 00 00 00       	mov    $0x1,%edx
   364b2:	4c 89 ff             	mov    %r15,%rdi
   364b5:	ff 15 3d c6 09 00    	call   *0x9c63d(%rip)        # d2af8 <_DYNAMIC+0x2c0>
   364bb:	0f 10 44 24 08       	movups 0x8(%rsp),%xmm0
   364c0:	0f 10 4c 24 18       	movups 0x18(%rsp),%xmm1
   364c5:	0f 10 54 24 28       	movups 0x28(%rsp),%xmm2
   364ca:	0f 10 5c 24 38       	movups 0x38(%rsp),%xmm3
   364cf:	41 0f 11 5c 24 30    	movups %xmm3,0x30(%r12)
   364d5:	41 0f 11 54 24 20    	movups %xmm2,0x20(%r12)
   364db:	41 0f 11 4c 24 10    	movups %xmm1,0x10(%r12)
   364e1:	41 0f 11 04 24       	movups %xmm0,(%r12)
   364e6:	49 83 3c 24 ff       	cmpq   $0xffffffffffffffff,(%r12)
   364eb:	0f 84 23 01 00 00    	je     36614 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7d4>
   364f1:	4c 89 ff             	mov    %r15,%rdi
   364f4:	4c 89 e6             	mov    %r12,%rsi
   364f7:	ff 15 03 c6 09 00    	call   *0x9c603(%rip)        # d2b00 <_DYNAMIC+0x2c8>
   364fd:	48 83 f8 01          	cmp    $0x1,%rax
   36501:	0f 85 c5 00 00 00    	jne    365cc <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x78c>
   36507:	49 39 96 b0 00 00 00 	cmp    %rdx,0xb0(%r14)
   3650e:	0f 84 cb 00 00 00    	je     365df <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x79f>
   36514:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
   36519:	4c 89 fe             	mov    %r15,%rsi
   3651c:	48 89 54 24 48       	mov    %rdx,0x48(%rsp)
   36521:	ff 15 a1 c5 09 00    	call   *0x9c5a1(%rip)        # d2ac8 <_DYNAMIC+0x290>
   36527:	4c 8b 64 24 08       	mov    0x8(%rsp),%r12
   3652c:	49 83 fc ff          	cmp    $0xffffffffffffffff,%r12
   36530:	0f 84 a9 00 00 00    	je     365df <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x79f>
   36536:	4c 8b 6c 24 10       	mov    0x10(%rsp),%r13
   3653b:	48 8b 6c 24 18       	mov    0x18(%rsp),%rbp
   36540:	48 85 ed             	test   %rbp,%rbp
   36543:	0f 84 02 02 00 00    	je     3674b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x90b>
   36549:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   36550:	49 8b 4d 00          	mov    0x0(%r13),%rcx
   36554:	4d 8b 45 08          	mov    0x8(%r13),%r8
   36558:	4c 89 ff             	mov    %r15,%rdi
   3655b:	31 d2                	xor    %edx,%edx
   3655d:	ff 15 6d c5 09 00    	call   *0x9c56d(%rip)        # d2ad0 <_DYNAMIC+0x298>
   36563:	84 c0                	test   %al,%al
   36565:	74 42                	je     365a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x769>
   36567:	48 83 fd 01          	cmp    $0x1,%rbp
   3656b:	0f 84 f7 01 00 00    	je     36768 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x928>
   36571:	49 8b 4d 10          	mov    0x10(%r13),%rcx
   36575:	4d 8b 45 18          	mov    0x18(%r13),%r8
   36579:	ba 01 00 00 00       	mov    $0x1,%edx
   3657e:	4c 89 ff             	mov    %r15,%rdi
   36581:	49 8d b6 80 00 00 00 	lea    0x80(%r14),%rsi
   36588:	ff 15 42 c5 09 00    	call   *0x9c542(%rip)        # d2ad0 <_DYNAMIC+0x298>
   3658e:	84 c0                	test   %al,%al
   36590:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   36595:	74 12                	je     365a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x769>
   36597:	49 89 86 b8 00 00 00 	mov    %rax,0xb8(%r14)
   3659e:	49 c7 86 a8 00 00 00 	movq   $0x2,0xa8(%r14)
   365a5:	02 00 00 00 
   365a9:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   365b0:	4d 85 e4             	test   %r12,%r12
   365b3:	74 31                	je     365e6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   365b5:	49 c1 e4 04          	shl    $0x4,%r12
   365b9:	ba 08 00 00 00       	mov    $0x8,%edx
   365be:	4c 89 ef             	mov    %r13,%rdi
   365c1:	4c 89 e6             	mov    %r12,%rsi
   365c4:	ff 15 a6 c4 09 00    	call   *0x9c4a6(%rip)        # d2a70 <_DYNAMIC+0x238>
   365ca:	eb 1a                	jmp    365e6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7a6>
   365cc:	49 c7 46 40 ff ff ff 	movq   $0xffffffffffffffff,0x40(%r14)
   365d3:	ff 
   365d4:	49 c7 86 a8 00 00 00 	movq   $0x0,0xa8(%r14)
   365db:	00 00 00 00 
   365df:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   365e6:	48 89 d8             	mov    %rbx,%rax
   365e9:	48 81 c4 a8 00 00 00 	add    $0xa8,%rsp
   365f0:	5b                   	pop    %rbx
   365f1:	41 5c                	pop    %r12
   365f3:	41 5d                	pop    %r13
   365f5:	41 5e                	pop    %r14
   365f7:	41 5f                	pop    %r15
   365f9:	5d                   	pop    %rbp
   365fa:	c3                   	ret    
   365fb:	48 8d 3d 2b 5f fe ff 	lea    -0x1a0d5(%rip),%rdi        # 1c52d <anon.031e5539418131618fb4d42bde787972.0.llvm.14684163749403725453+0x6b9>
   36602:	48 8d 15 97 92 09 00 	lea    0x99297(%rip),%rdx        # cf8a0 <__frame_dummy_init_array_entry+0x410>
   36609:	be 28 00 00 00       	mov    $0x28,%esi
   3660e:	ff 15 f4 c4 09 00    	call   *0x9c4f4(%rip)        # d2b08 <_DYNAMIC+0x2d0>
   36614:	48 8d 3d 3d 92 09 00 	lea    0x9923d(%rip),%rdi        # cf858 <__frame_dummy_init_array_entry+0x3c8>
   3661b:	ff 15 67 c4 09 00    	call   *0x9c467(%rip)        # d2a88 <_DYNAMIC+0x250>
   36621:	48 8d 3d b8 91 09 00 	lea    0x991b8(%rip),%rdi        # cf7e0 <__frame_dummy_init_array_entry+0x350>
   36628:	ff 15 5a c4 09 00    	call   *0x9c45a(%rip)        # d2a88 <_DYNAMIC+0x250>
   3662e:	bf 08 00 00 00       	mov    $0x8,%edi
   36633:	be 10 00 00 00       	mov    $0x10,%esi
   36638:	ff 15 6a c4 09 00    	call   *0x9c46a(%rip)        # d2aa8 <_DYNAMIC+0x270>
   3663e:	bf 08 00 00 00       	mov    $0x8,%edi
   36643:	be 10 00 00 00       	mov    $0x10,%esi
   36648:	ff 15 5a c4 09 00    	call   *0x9c45a(%rip)        # d2aa8 <_DYNAMIC+0x270>
   3664e:	e9 2a 01 00 00       	jmp    3677d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x93d>
   36653:	bf 08 00 00 00       	mov    $0x8,%edi
   36658:	be 50 00 00 00       	mov    $0x50,%esi
   3665d:	ff 15 45 c4 09 00    	call   *0x9c445(%rip)        # d2aa8 <_DYNAMIC+0x270>
   36663:	e9 15 01 00 00       	jmp    3677d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x93d>
   36668:	48 8d 3d be 5e fe ff 	lea    -0x1a142(%rip),%rdi        # 1c52d <anon.031e5539418131618fb4d42bde787972.0.llvm.14684163749403725453+0x6b9>
   3666f:	48 8d 15 0a 91 09 00 	lea    0x9910a(%rip),%rdx        # cf780 <__frame_dummy_init_array_entry+0x2f0>
   36676:	be 28 00 00 00       	mov    $0x28,%esi
   3667b:	ff 15 87 c4 09 00    	call   *0x9c487(%rip)        # d2b08 <_DYNAMIC+0x2d0>
   36681:	e9 f7 00 00 00       	jmp    3677d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x93d>
   36686:	48 8d 15 83 91 09 00 	lea    0x99183(%rip),%rdx        # cf810 <__frame_dummy_init_array_entry+0x380>
   3668d:	bf 01 00 00 00       	mov    $0x1,%edi
   36692:	ff 15 78 c4 09 00    	call   *0x9c478(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   36698:	48 8d 15 f9 90 09 00 	lea    0x990f9(%rip),%rdx        # cf798 <__frame_dummy_init_array_entry+0x308>
   3669f:	31 ff                	xor    %edi,%edi
   366a1:	31 f6                	xor    %esi,%esi
   366a3:	ff 15 67 c4 09 00    	call   *0x9c467(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   366a9:	48 8d 15 00 91 09 00 	lea    0x99100(%rip),%rdx        # cf7b0 <__frame_dummy_init_array_entry+0x320>
   366b0:	bf 01 00 00 00       	mov    $0x1,%edi
   366b5:	ff 15 55 c4 09 00    	call   *0x9c455(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   366bb:	48 8d 15 46 90 09 00 	lea    0x99046(%rip),%rdx        # cf708 <__frame_dummy_init_array_entry+0x278>
   366c2:	bf 01 00 00 00       	mov    $0x1,%edi
   366c7:	eb 0b                	jmp    366d4 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x894>
   366c9:	48 8d 15 50 90 09 00 	lea    0x99050(%rip),%rdx        # cf720 <__frame_dummy_init_array_entry+0x290>
   366d0:	31 ff                	xor    %edi,%edi
   366d2:	31 f6                	xor    %esi,%esi
   366d4:	ff 15 36 c4 09 00    	call   *0x9c436(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   366da:	e9 9e 00 00 00       	jmp    3677d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x93d>
   366df:	bf 08 00 00 00       	mov    $0x8,%edi
   366e4:	be 10 00 00 00       	mov    $0x10,%esi
   366e9:	ff 15 29 c4 09 00    	call   *0x9c429(%rip)        # d2b18 <_DYNAMIC+0x2e0>
   366ef:	e9 89 00 00 00       	jmp    3677d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x93d>
   366f4:	48 8d 15 55 90 09 00 	lea    0x99055(%rip),%rdx        # cf750 <__frame_dummy_init_array_entry+0x2c0>
   366fb:	31 ff                	xor    %edi,%edi
   366fd:	eb 41                	jmp    36740 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x900>
   366ff:	48 8d 15 f2 90 09 00 	lea    0x990f2(%rip),%rdx        # cf7f8 <__frame_dummy_init_array_entry+0x368>
   36706:	31 ff                	xor    %edi,%edi
   36708:	31 f6                	xor    %esi,%esi
   3670a:	ff 15 00 c4 09 00    	call   *0x9c400(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   36710:	eb 6b                	jmp    3677d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x93d>
   36712:	48 8d 15 0f 91 09 00 	lea    0x9910f(%rip),%rdx        # cf828 <__frame_dummy_init_array_entry+0x398>
   36719:	31 ff                	xor    %edi,%edi
   3671b:	31 f6                	xor    %esi,%esi
   3671d:	ff 15 ed c3 09 00    	call   *0x9c3ed(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   36723:	48 8d 15 9e 90 09 00 	lea    0x9909e(%rip),%rdx        # cf7c8 <__frame_dummy_init_array_entry+0x338>
   3672a:	31 ff                	xor    %edi,%edi
   3672c:	31 f6                	xor    %esi,%esi
   3672e:	ff 15 dc c3 09 00    	call   *0x9c3dc(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   36734:	48 8d 15 2d 90 09 00 	lea    0x9902d(%rip),%rdx        # cf768 <__frame_dummy_init_array_entry+0x2d8>
   3673b:	bf 01 00 00 00       	mov    $0x1,%edi
   36740:	48 89 fe             	mov    %rdi,%rsi
   36743:	ff 15 c7 c3 09 00    	call   *0x9c3c7(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   36749:	eb 32                	jmp    3677d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x93d>
   3674b:	48 8d 15 1e 91 09 00 	lea    0x9911e(%rip),%rdx        # cf870 <__frame_dummy_init_array_entry+0x3e0>
   36752:	31 ff                	xor    %edi,%edi
   36754:	eb 1e                	jmp    36774 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x934>
   36756:	48 8d 15 e3 90 09 00 	lea    0x990e3(%rip),%rdx        # cf840 <__frame_dummy_init_array_entry+0x3b0>
   3675d:	bf 01 00 00 00       	mov    $0x1,%edi
   36762:	ff 15 a8 c3 09 00    	call   *0x9c3a8(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   36768:	48 8d 15 19 91 09 00 	lea    0x99119(%rip),%rdx        # cf888 <__frame_dummy_init_array_entry+0x3f8>
   3676f:	bf 01 00 00 00       	mov    $0x1,%edi
   36774:	48 89 fe             	mov    %rdi,%rsi
   36777:	ff 15 93 c3 09 00    	call   *0x9c393(%rip)        # d2b10 <_DYNAMIC+0x2d8>
   3677d:	0f 0b                	ud2    
   3677f:	eb 11                	jmp    36792 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x952>
   36781:	48 89 c3             	mov    %rax,%rbx
   36784:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
   36789:	e8 02 e2 ff ff       	call   34990 <core::ptr::drop_glue::<chr_compiled::Work>>
   3678e:	eb 46                	jmp    367d6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x996>
   36790:	eb 00                	jmp    36792 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x952>
   36792:	48 89 c3             	mov    %rax,%rbx
   36795:	4d 85 e4             	test   %r12,%r12
   36798:	74 3c                	je     367d6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x996>
   3679a:	49 c1 e4 04          	shl    $0x4,%r12
   3679e:	ba 08 00 00 00       	mov    $0x8,%edx
   367a3:	4c 89 ef             	mov    %r13,%rdi
   367a6:	4c 89 e6             	mov    %r12,%rsi
   367a9:	ff 15 c1 c2 09 00    	call   *0x9c2c1(%rip)        # d2a70 <_DYNAMIC+0x238>
   367af:	48 89 df             	mov    %rbx,%rdi
   367b2:	e8 99 7c 09 00       	call   ce450 <_Unwind_Resume@plt>
   367b7:	48 89 c3             	mov    %rax,%rbx
   367ba:	48 8d 7c 24 50       	lea    0x50(%rsp),%rdi
   367bf:	e8 cc e1 ff ff       	call   34990 <core::ptr::drop_glue::<chr_compiled::Work>>
   367c4:	eb 03                	jmp    367c9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x989>
   367c6:	48 89 c3             	mov    %rax,%rbx
   367c9:	48 8d bc 24 80 00 00 	lea    0x80(%rsp),%rdi
   367d0:	00 
   367d1:	e8 ba e1 ff ff       	call   34990 <core::ptr::drop_glue::<chr_compiled::Work>>
   367d6:	48 89 df             	mov    %rbx,%rdi
   367d9:	e8 72 7c 09 00       	call   ce450 <_Unwind_Resume@plt>
   367de:	ff 15 94 c2 09 00    	call   *0x9c294(%rip)        # d2a78 <_DYNAMIC+0x240>
   367e4:	cc                   	int3   
   367e5:	cc                   	int3   
   367e6:	cc                   	int3   
   367e7:	cc                   	int3   
   367e8:	cc                   	int3   
   367e9:	cc                   	int3   
   367ea:	cc                   	int3   
   367eb:	cc                   	int3   
   367ec:	cc                   	int3   
   367ed:	cc                   	int3   
   367ee:	cc                   	int3   
   367ef:	cc                   	int3   