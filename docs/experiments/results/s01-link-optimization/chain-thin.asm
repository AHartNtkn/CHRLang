0000000000028370 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick>:
   28370:	55                   	push   %rbp
   28371:	41 57                	push   %r15
   28373:	41 56                	push   %r14
   28375:	41 55                	push   %r13
   28377:	41 54                	push   %r12
   28379:	53                   	push   %rbx
   2837a:	48 83 ec 78          	sub    $0x78,%rsp
   2837e:	49 89 d7             	mov    %rdx,%r15
   28381:	49 89 f6             	mov    %rsi,%r14
   28384:	48 89 fb             	mov    %rdi,%rbx
   28387:	80 be c0 00 00 00 00 	cmpb   $0x0,0xc0(%rsi)
   2838e:	0f 84 a3 00 00 00    	je     28437 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc7>
   28394:	41 c6 86 c0 00 00 00 	movb   $0x0,0xc0(%r14)
   2839b:	00 
   2839c:	83 39 01             	cmpl   $0x1,(%rcx)
   2839f:	0f 85 92 00 00 00    	jne    28437 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc7>
   283a5:	48 8b 69 08          	mov    0x8(%rcx),%rbp
   283a9:	48 85 ed             	test   %rbp,%rbp
   283ac:	0f 84 85 00 00 00    	je     28437 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc7>
   283b2:	49 8b 87 28 01 00 00 	mov    0x128(%r15),%rax
   283b9:	48 85 c0             	test   %rax,%rax
   283bc:	0f 84 19 06 00 00    	je     289db <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x66b>
   283c2:	48 8b 49 10          	mov    0x10(%rcx),%rcx
   283c6:	49 8b 97 30 01 00 00 	mov    0x130(%r15),%rdx
   283cd:	44 0f b7 80 c2 01 00 	movzwl 0x1c2(%rax),%r8d
   283d4:	00 
   283d5:	45 89 c1             	mov    %r8d,%r9d
   283d8:	41 c1 e1 05          	shl    $0x5,%r9d
   283dc:	48 c7 c7 ff ff ff ff 	mov    $0xffffffffffffffff,%rdi
   283e3:	31 f6                	xor    %esi,%esi
   283e5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   283ec:	00 00 00 00 
   283f0:	49 39 f1             	cmp    %rsi,%r9
   283f3:	74 2b                	je     28420 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xb0>
   283f5:	48 3b 8c f8 70 01 00 	cmp    0x170(%rax,%rdi,8),%rcx
   283fc:	00 
   283fd:	41 0f 97 c2          	seta   %r10b
   28401:	41 80 da 00          	sbb    $0x0,%r10b
   28405:	48 83 c6 20          	add    $0x20,%rsi
   28409:	48 ff c7             	inc    %rdi
   2840c:	41 80 fa 01          	cmp    $0x1,%r10b
   28410:	74 de                	je     283f0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x80>
   28412:	45 0f b6 c2          	movzbl %r10b,%r8d
   28416:	45 85 c0             	test   %r8d,%r8d
   28419:	75 08                	jne    28423 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xb3>
   2841b:	e9 a3 03 00 00       	jmp    287c3 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x453>
   28420:	4c 89 c7             	mov    %r8,%rdi
   28423:	48 83 ea 01          	sub    $0x1,%rdx
   28427:	0f 82 ae 05 00 00    	jb     289db <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x66b>
   2842d:	48 8b 84 f8 c8 01 00 	mov    0x1c8(%rax,%rdi,8),%rax
   28434:	00 
   28435:	eb 96                	jmp    283cd <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5d>
   28437:	49 8b 86 a8 00 00 00 	mov    0xa8(%r14),%rax
   2843e:	48 85 c0             	test   %rax,%rax
   28441:	0f 84 f4 00 00 00    	je     2853b <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1cb>
   28447:	48 83 f8 01          	cmp    $0x1,%rax
   2844b:	0f 84 9d 00 00 00    	je     284ee <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x17e>
   28451:	48 83 f8 02          	cmp    $0x2,%rax
   28455:	0f 85 9a 0e 00 00    	jne    292f5 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xf85>
   2845b:	bf 10 00 00 00       	mov    $0x10,%edi
   28460:	ff 15 92 c4 09 00    	call   *0x9c492(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   28466:	48 85 c0             	test   %rax,%rax
   28469:	0f 84 af 0e 00 00    	je     2931e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xfae>
   2846f:	49 89 c4             	mov    %rax,%r12
   28472:	4d 8d ae b0 00 00 00 	lea    0xb0(%r14),%r13
   28479:	41 0f 10 45 00       	movups 0x0(%r13),%xmm0
   2847e:	0f 11 00             	movups %xmm0,(%rax)
   28481:	48 c7 44 24 50 00 00 	movq   $0x0,0x50(%rsp)
   28488:	00 00 
   2848a:	48 c7 44 24 58 02 00 	movq   $0x2,0x58(%rsp)
   28491:	00 00 
   28493:	48 89 44 24 60       	mov    %rax,0x60(%rsp)
   28498:	48 c7 44 24 68 02 00 	movq   $0x2,0x68(%rsp)
   2849f:	00 00 
   284a1:	49 8b b7 58 01 00 00 	mov    0x158(%r15),%rsi
   284a8:	48 85 f6             	test   %rsi,%rsi
   284ab:	0f 84 f6 01 00 00    	je     286a7 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x337>
   284b1:	49 8b 97 60 01 00 00 	mov    0x160(%r15),%rdx
   284b8:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   284bd:	48 8d 4c 24 50       	lea    0x50(%rsp),%rcx
   284c2:	e8 e9 14 01 00       	call   399b0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, alloc::vec::Vec<u64>), alloc::collections::btree::set_val::SetValZST, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, alloc::vec::Vec<u64>)>>
   284c7:	0f b6 6c 24 28       	movzbl 0x28(%rsp),%ebp
   284cc:	4c 89 e7             	mov    %r12,%rdi
   284cf:	ff 15 33 c4 09 00    	call   *0x9c433(%rip)        # c4908 <free@GLIBC_2.2.5>
   284d5:	40 84 ed             	test   %bpl,%bpl
   284d8:	0f 85 d2 01 00 00    	jne    286b0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x340>
   284de:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   284e5:	01 00 00 00 
   284e9:	e9 d1 0c 00 00       	jmp    291bf <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe4f>
   284ee:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   284f5:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   284fc:	83 39 01             	cmpl   $0x1,(%rcx)
   284ff:	0f 85 d7 00 00 00    	jne    285dc <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x26c>
   28505:	48 83 79 08 01       	cmpq   $0x1,0x8(%rcx)
   2850a:	0f 85 cc 00 00 00    	jne    285dc <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x26c>
   28510:	49 8d 76 40          	lea    0x40(%r14),%rsi
   28514:	41 83 7e 40 ff       	cmpl   $0xffffffff,0x40(%r14)
   28519:	0f 85 ec 00 00 00    	jne    2860b <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x29b>
   2851f:	48 8b 69 10          	mov    0x10(%rcx),%rbp
   28523:	41 bc 02 00 00 00    	mov    $0x2,%r12d
   28529:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   2852f:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   28536:	e9 3e 0c 00 00       	jmp    29179 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe09>
   2853b:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   28542:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   28549:	0f b6 01             	movzbl (%rcx),%eax
   2854c:	48 8b 51 08          	mov    0x8(%rcx),%rdx
   28550:	48 83 fa 02          	cmp    $0x2,%rdx
   28554:	40 0f 92 c6          	setb   %sil
   28558:	40 84 f0             	test   %sil,%al
   2855b:	75 1c                	jne    28579 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x209>
   2855d:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   28564:	00 
   28565:	0f 84 0c 0e 00 00    	je     29377 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1007>
   2856b:	49 8b b6 88 00 00 00 	mov    0x88(%r14),%rsi
   28572:	48 c7 06 02 00 00 00 	movq   $0x2,(%rsi)
   28579:	48 83 fa 01          	cmp    $0x1,%rdx
   2857d:	40 0f 94 c6          	sete   %sil
   28581:	40 84 f0             	test   %sil,%al
   28584:	75 20                	jne    285a6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x236>
   28586:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   2858d:	48 83 fe 02          	cmp    $0x2,%rsi
   28591:	0f 82 03 0e 00 00    	jb     2939a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x102a>
   28597:	49 8b b6 88 00 00 00 	mov    0x88(%r14),%rsi
   2859e:	48 c7 46 10 02 00 00 	movq   $0x2,0x10(%rsi)
   285a5:	00 
   285a6:	41 83 3e ff          	cmpl   $0xffffffff,(%r14)
   285aa:	0f 85 95 03 00 00    	jne    28945 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5d5>
   285b0:	48 85 d2             	test   %rdx,%rdx
   285b3:	0f 94 c2             	sete   %dl
   285b6:	20 d0                	and    %dl,%al
   285b8:	3c 01                	cmp    $0x1,%al
   285ba:	0f 85 01 03 00 00    	jne    288c1 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x551>
   285c0:	48 8b 41 10          	mov    0x10(%rcx),%rax
   285c4:	41 bc 02 00 00 00    	mov    $0x2,%r12d
   285ca:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   285d0:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   285d7:	e9 4a 03 00 00       	jmp    28926 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5b6>
   285dc:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   285e3:	48 83 fe 01          	cmp    $0x1,%rsi
   285e7:	0f 86 9b 0d 00 00    	jbe    29388 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1018>
   285ed:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   285f4:	48 c7 40 10 02 00 00 	movq   $0x2,0x10(%rax)
   285fb:	00 
   285fc:	49 8d 76 40          	lea    0x40(%r14),%rsi
   28600:	41 83 7e 40 ff       	cmpl   $0xffffffff,0x40(%r14)
   28605:	0f 84 1d 02 00 00    	je     28828 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x4b8>
   2860b:	4c 89 ff             	mov    %r15,%rdi
   2860e:	ff 15 f4 c3 09 00    	call   *0x9c3f4(%rip)        # c4a08 <_DYNAMIC+0x2f8>
   28614:	48 83 f8 01          	cmp    $0x1,%rax
   28618:	0f 85 8e 0b 00 00    	jne    291ac <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe3c>
   2861e:	49 39 96 b0 00 00 00 	cmp    %rdx,0xb0(%r14)
   28625:	0f 84 94 0b 00 00    	je     291bf <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe4f>
   2862b:	49 8b 87 28 01 00 00 	mov    0x128(%r15),%rax
   28632:	48 85 c0             	test   %rax,%rax
   28635:	0f 84 84 0b 00 00    	je     291bf <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe4f>
   2863b:	49 8b 8f 30 01 00 00 	mov    0x130(%r15),%rcx
   28642:	44 0f b7 80 c2 01 00 	movzwl 0x1c2(%rax),%r8d
   28649:	00 
   2864a:	45 89 c1             	mov    %r8d,%r9d
   2864d:	41 c1 e1 05          	shl    $0x5,%r9d
   28651:	48 c7 c7 ff ff ff ff 	mov    $0xffffffffffffffff,%rdi
   28658:	31 f6                	xor    %esi,%esi
   2865a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   28660:	49 39 f1             	cmp    %rsi,%r9
   28663:	74 2b                	je     28690 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x320>
   28665:	48 3b 94 f8 70 01 00 	cmp    0x170(%rax,%rdi,8),%rdx
   2866c:	00 
   2866d:	41 0f 97 c2          	seta   %r10b
   28671:	41 80 da 00          	sbb    $0x0,%r10b
   28675:	48 83 c6 20          	add    $0x20,%rsi
   28679:	48 ff c7             	inc    %rdi
   2867c:	41 80 fa 01          	cmp    $0x1,%r10b
   28680:	74 de                	je     28660 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x2f0>
   28682:	45 0f b6 c2          	movzbl %r10b,%r8d
   28686:	45 85 c0             	test   %r8d,%r8d
   28689:	75 08                	jne    28693 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x323>
   2868b:	e9 b7 03 00 00       	jmp    28a47 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x6d7>
   28690:	4c 89 c7             	mov    %r8,%rdi
   28693:	48 83 e9 01          	sub    $0x1,%rcx
   28697:	0f 82 22 0b 00 00    	jb     291bf <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe4f>
   2869d:	48 8b 84 f8 c8 01 00 	mov    0x1c8(%rax,%rdi,8),%rax
   286a4:	00 
   286a5:	eb 9b                	jmp    28642 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x2d2>
   286a7:	4c 89 e7             	mov    %r12,%rdi
   286aa:	ff 15 58 c2 09 00    	call   *0x9c258(%rip)        # c4908 <free@GLIBC_2.2.5>
   286b0:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   286b7:	48 83 fe 02          	cmp    $0x2,%rsi
   286bb:	0f 82 6d 0c 00 00    	jb     2932e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xfbe>
   286c1:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   286c8:	48 8b 68 10          	mov    0x10(%rax),%rbp
   286cc:	48 83 fd 02          	cmp    $0x2,%rbp
   286d0:	75 23                	jne    286f5 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x385>
   286d2:	4d 8b a6 98 00 00 00 	mov    0x98(%r14),%r12
   286d9:	49 8d 4c 24 01       	lea    0x1(%r12),%rcx
   286de:	49 89 8e 98 00 00 00 	mov    %rcx,0x98(%r14)
   286e5:	48 c7 40 10 00 00 00 	movq   $0x0,0x10(%rax)
   286ec:	00 
   286ed:	4c 89 60 18          	mov    %r12,0x18(%rax)
   286f1:	31 ed                	xor    %ebp,%ebp
   286f3:	eb 04                	jmp    286f9 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x389>
   286f5:	4c 8b 60 18          	mov    0x18(%rax),%r12
   286f9:	49 8b 87 08 01 00 00 	mov    0x108(%r15),%rax
   28700:	48 83 78 20 00       	cmpq   $0x0,0x20(%rax)
   28705:	0f 84 35 0c 00 00    	je     29340 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xfd0>
   2870b:	48 8b 40 18          	mov    0x18(%rax),%rax
   2870f:	48 83 78 40 00       	cmpq   $0x0,0x40(%rax)
   28714:	0f 84 37 0c 00 00    	je     29351 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xfe1>
   2871a:	48 8b 40 38          	mov    0x38(%rax),%rax
   2871e:	4c 8b 38             	mov    (%rax),%r15
   28721:	bf 10 00 00 00       	mov    $0x10,%edi
   28726:	ff 15 cc c1 09 00    	call   *0x9c1cc(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   2872c:	48 85 c0             	test   %rax,%rax
   2872f:	0f 84 d9 0b 00 00    	je     2930e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xf9e>
   28735:	48 89 28             	mov    %rbp,(%rax)
   28738:	4c 89 60 08          	mov    %r12,0x8(%rax)
   2873c:	4c 89 7c 24 30       	mov    %r15,0x30(%rsp)
   28741:	48 c7 44 24 38 01 00 	movq   $0x1,0x38(%rsp)
   28748:	00 00 
   2874a:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
   2874f:	48 c7 44 24 48 01 00 	movq   $0x1,0x48(%rsp)
   28756:	00 00 
   28758:	48 c7 44 24 28 00 00 	movq   $0x0,0x28(%rsp)
   2875f:	00 00 
   28761:	bf 10 00 00 00       	mov    $0x10,%edi
   28766:	ff 15 8c c1 09 00    	call   *0x9c18c(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   2876c:	48 85 c0             	test   %rax,%rax
   2876f:	0f 84 ed 0b 00 00    	je     29362 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xff2>
   28775:	41 0f 10 45 00       	movups 0x0(%r13),%xmm0
   2877a:	0f 11 00             	movups %xmm0,(%rax)
   2877d:	48 8b 4c 24 48       	mov    0x48(%rsp),%rcx
   28782:	48 89 4b 20          	mov    %rcx,0x20(%rbx)
   28786:	0f 10 44 24 28       	movups 0x28(%rsp),%xmm0
   2878b:	0f 10 4c 24 38       	movups 0x38(%rsp),%xmm1
   28790:	0f 11 4b 10          	movups %xmm1,0x10(%rbx)
   28794:	0f 11 03             	movups %xmm0,(%rbx)
   28797:	49 8b 8e 98 00 00 00 	mov    0x98(%r14),%rcx
   2879e:	48 c7 43 28 02 00 00 	movq   $0x2,0x28(%rbx)
   287a5:	00 
   287a6:	48 89 43 30          	mov    %rax,0x30(%rbx)
   287aa:	48 c7 43 38 02 00 00 	movq   $0x2,0x38(%rbx)
   287b1:	00 
   287b2:	48 c7 43 40 00 00 00 	movq   $0x0,0x40(%rbx)
   287b9:	00 
   287ba:	48 89 4b 48          	mov    %rcx,0x48(%rbx)
   287be:	e9 03 0a 00 00       	jmp    291c6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe56>
   287c3:	4c 8b 64 30 f0       	mov    -0x10(%rax,%rsi,1),%r12
   287c8:	4d 89 e5             	mov    %r12,%r13
   287cb:	49 c1 e5 04          	shl    $0x4,%r13
   287cf:	4c 89 e1             	mov    %r12,%rcx
   287d2:	48 c1 e9 3c          	shr    $0x3c,%rcx
   287d6:	0f 95 c1             	setne  %cl
   287d9:	48 ba f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rdx
   287e0:	ff ff 7f 
   287e3:	49 39 d5             	cmp    %rdx,%r13
   287e6:	0f 97 c2             	seta   %dl
   287e9:	08 ca                	or     %cl,%dl
   287eb:	0f 85 82 02 00 00    	jne    28a73 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x703>
   287f1:	48 8b 74 30 e8       	mov    -0x18(%rax,%rsi,1),%rsi
   287f6:	4d 85 ed             	test   %r13,%r13
   287f9:	0f 84 ce 02 00 00    	je     28acd <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x75d>
   287ff:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
   28804:	4c 89 ef             	mov    %r13,%rdi
   28807:	ff 15 eb c0 09 00    	call   *0x9c0eb(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   2880d:	48 85 c0             	test   %rax,%rax
   28810:	0f 84 a7 0b 00 00    	je     293bd <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x104d>
   28816:	48 89 c7             	mov    %rax,%rdi
   28819:	4c 89 64 24 18       	mov    %r12,0x18(%rsp)
   2881e:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   28823:	e9 b3 02 00 00       	jmp    28adb <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x76b>
   28828:	49 8b 87 08 01 00 00 	mov    0x108(%r15),%rax
   2882f:	48 83 78 20 00       	cmpq   $0x0,0x20(%rax)
   28834:	0f 84 72 0b 00 00    	je     293ac <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x103c>
   2883a:	48 8b 48 18          	mov    0x18(%rax),%rcx
   2883e:	48 8b 41 10          	mov    0x10(%rcx),%rax
   28842:	48 83 f8 01          	cmp    $0x1,%rax
   28846:	0f 86 7f 0b 00 00    	jbe    293cb <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x105b>
   2884c:	48 8b 41 08          	mov    0x8(%rcx),%rax
   28850:	48 8b 78 38          	mov    0x38(%rax),%rdi
   28854:	41 80 bf f9 03 00 00 	cmpb   $0x0,0x3f9(%r15)
   2885b:	00 
   2885c:	0f 84 1c 02 00 00    	je     28a7e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x70e>
   28862:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   28869:	00 
   2886a:	0f 84 e4 0b 00 00    	je     29454 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10e4>
   28870:	49 8b 8e 88 00 00 00 	mov    0x88(%r14),%rcx
   28877:	48 8b 01             	mov    (%rcx),%rax
   2887a:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   28881:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   28887:	48 83 f8 02          	cmp    $0x2,%rax
   2888b:	0f 85 22 05 00 00    	jne    28db3 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xa43>
   28891:	45 31 e4             	xor    %r12d,%r12d
   28894:	49 8b 86 90 00 00 00 	mov    0x90(%r14),%rax
   2889b:	48 83 f8 02          	cmp    $0x2,%rax
   2889f:	0f 82 9d 08 00 00    	jb     29142 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdd2>
   288a5:	49 8b 8e 88 00 00 00 	mov    0x88(%r14),%rcx
   288ac:	48 8b 41 10          	mov    0x10(%rcx),%rax
   288b0:	48 83 f8 02          	cmp    $0x2,%rax
   288b4:	0f 85 f5 05 00 00    	jne    28eaf <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xb3f>
   288ba:	31 ed                	xor    %ebp,%ebp
   288bc:	e9 b3 08 00 00       	jmp    29174 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe04>
   288c1:	49 8b 87 08 01 00 00 	mov    0x108(%r15),%rax
   288c8:	48 83 78 20 00       	cmpq   $0x0,0x20(%rax)
   288cd:	0f 84 d9 0a 00 00    	je     293ac <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x103c>
   288d3:	48 8b 40 18          	mov    0x18(%rax),%rax
   288d7:	48 83 78 10 00       	cmpq   $0x0,0x10(%rax)
   288dc:	0f 84 fe 0a 00 00    	je     293e0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1070>
   288e2:	48 8b 40 08          	mov    0x8(%rax),%rax
   288e6:	48 8b 48 18          	mov    0x18(%rax),%rcx
   288ea:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   288f0:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   288f7:	41 80 bf f9 03 00 00 	cmpb   $0x0,0x3f9(%r15)
   288fe:	00 
   288ff:	74 22                	je     28923 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5b3>
   28901:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   28908:	00 
   28909:	0f 84 56 0b 00 00    	je     29465 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10f5>
   2890f:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   28916:	48 8b 30             	mov    (%rax),%rsi
   28919:	48 83 fe 02          	cmp    $0x2,%rsi
   2891d:	0f 85 18 05 00 00    	jne    28e3b <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xacb>
   28923:	45 31 e4             	xor    %r12d,%r12d
   28926:	4d 89 26             	mov    %r12,(%r14)
   28929:	4d 89 6e 08          	mov    %r13,0x8(%r14)
   2892d:	49 89 46 10          	mov    %rax,0x10(%r14)
   28931:	49 89 56 18          	mov    %rdx,0x18(%r14)
   28935:	49 c7 46 20 00 00 00 	movq   $0x0,0x20(%r14)
   2893c:	00 
   2893d:	49 89 4e 30          	mov    %rcx,0x30(%r14)
   28941:	49 89 6e 38          	mov    %rbp,0x38(%r14)
   28945:	4c 89 ff             	mov    %r15,%rdi
   28948:	4c 89 f6             	mov    %r14,%rsi
   2894b:	ff 15 b7 c0 09 00    	call   *0x9c0b7(%rip)        # c4a08 <_DYNAMIC+0x2f8>
   28951:	a8 01                	test   $0x1,%al
   28953:	74 7f                	je     289d4 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x664>
   28955:	49 8b 87 28 01 00 00 	mov    0x128(%r15),%rax
   2895c:	48 85 c0             	test   %rax,%rax
   2895f:	0f 84 5a 08 00 00    	je     291bf <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe4f>
   28965:	49 8b 8f 30 01 00 00 	mov    0x130(%r15),%rcx
   2896c:	44 0f b7 80 c2 01 00 	movzwl 0x1c2(%rax),%r8d
   28973:	00 
   28974:	45 89 c1             	mov    %r8d,%r9d
   28977:	41 c1 e1 05          	shl    $0x5,%r9d
   2897b:	48 c7 c7 ff ff ff ff 	mov    $0xffffffffffffffff,%rdi
   28982:	31 f6                	xor    %esi,%esi
   28984:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   2898b:	00 00 00 00 00 
   28990:	49 39 f1             	cmp    %rsi,%r9
   28993:	74 28                	je     289bd <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x64d>
   28995:	48 3b 94 f8 70 01 00 	cmp    0x170(%rax,%rdi,8),%rdx
   2899c:	00 
   2899d:	41 0f 97 c2          	seta   %r10b
   289a1:	41 80 da 00          	sbb    $0x0,%r10b
   289a5:	48 83 c6 20          	add    $0x20,%rsi
   289a9:	48 ff c7             	inc    %rdi
   289ac:	41 80 fa 01          	cmp    $0x1,%r10b
   289b0:	74 de                	je     28990 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x620>
   289b2:	45 0f b6 c2          	movzbl %r10b,%r8d
   289b6:	45 85 c0             	test   %r8d,%r8d
   289b9:	75 05                	jne    289c0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x650>
   289bb:	eb 2a                	jmp    289e7 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x677>
   289bd:	4c 89 c7             	mov    %r8,%rdi
   289c0:	48 83 e9 01          	sub    $0x1,%rcx
   289c4:	0f 82 f5 07 00 00    	jb     291bf <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe4f>
   289ca:	48 8b 84 f8 c8 01 00 	mov    0x1c8(%rax,%rdi,8),%rax
   289d1:	00 
   289d2:	eb 98                	jmp    2896c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5fc>
   289d4:	49 c7 06 ff ff ff ff 	movq   $0xffffffffffffffff,(%r14)
   289db:	48 c7 03 07 00 00 00 	movq   $0x7,(%rbx)
   289e2:	e9 df 07 00 00       	jmp    291c6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe56>
   289e7:	4c 8b 64 30 f0       	mov    -0x10(%rax,%rsi,1),%r12
   289ec:	4d 89 e5             	mov    %r12,%r13
   289ef:	49 c1 e5 04          	shl    $0x4,%r13
   289f3:	4c 89 e1             	mov    %r12,%rcx
   289f6:	48 c1 e9 3c          	shr    $0x3c,%rcx
   289fa:	0f 95 c1             	setne  %cl
   289fd:	48 bf f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rdi
   28a04:	ff ff 7f 
   28a07:	49 39 fd             	cmp    %rdi,%r13
   28a0a:	40 0f 97 c7          	seta   %dil
   28a0e:	40 08 cf             	or     %cl,%dil
   28a11:	75 60                	jne    28a73 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x703>
   28a13:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
   28a18:	48 8b 6c 30 e8       	mov    -0x18(%rax,%rsi,1),%rbp
   28a1d:	4d 85 ed             	test   %r13,%r13
   28a20:	0f 84 74 02 00 00    	je     28c9a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x92a>
   28a26:	4c 89 ef             	mov    %r13,%rdi
   28a29:	ff 15 c9 be 09 00    	call   *0x9bec9(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   28a2f:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   28a34:	48 85 c0             	test   %rax,%rax
   28a37:	0f 84 80 09 00 00    	je     293bd <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x104d>
   28a3d:	4c 89 64 24 20       	mov    %r12,0x20(%rsp)
   28a42:	e9 66 02 00 00       	jmp    28cad <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x93d>
   28a47:	4c 8b 64 30 f0       	mov    -0x10(%rax,%rsi,1),%r12
   28a4c:	4d 89 e5             	mov    %r12,%r13
   28a4f:	49 c1 e5 04          	shl    $0x4,%r13
   28a53:	4c 89 e1             	mov    %r12,%rcx
   28a56:	48 c1 e9 3c          	shr    $0x3c,%rcx
   28a5a:	0f 95 c1             	setne  %cl
   28a5d:	48 bf f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rdi
   28a64:	ff ff 7f 
   28a67:	49 39 fd             	cmp    %rdi,%r13
   28a6a:	40 0f 97 c7          	seta   %dil
   28a6e:	40 08 cf             	or     %cl,%dil
   28a71:	74 20                	je     28a93 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x723>
   28a73:	31 ff                	xor    %edi,%edi
   28a75:	4c 89 ee             	mov    %r13,%rsi
   28a78:	ff 15 c2 be 09 00    	call   *0x9bec2(%rip)        # c4940 <_DYNAMIC+0x230>
   28a7e:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   28a84:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   28a8b:	45 31 e4             	xor    %r12d,%r12d
   28a8e:	e9 e6 06 00 00       	jmp    29179 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe09>
   28a93:	48 8b 74 30 e8       	mov    -0x18(%rax,%rsi,1),%rsi
   28a98:	4d 85 ed             	test   %r13,%r13
   28a9b:	48 89 54 24 18       	mov    %rdx,0x18(%rsp)
   28aa0:	0f 84 9c 04 00 00    	je     28f42 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xbd2>
   28aa6:	48 89 f5             	mov    %rsi,%rbp
   28aa9:	4c 89 ef             	mov    %r13,%rdi
   28aac:	ff 15 46 be 09 00    	call   *0x9be46(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   28ab2:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   28ab7:	48 85 c0             	test   %rax,%rax
   28aba:	0f 84 fd 08 00 00    	je     293bd <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x104d>
   28ac0:	4c 89 64 24 20       	mov    %r12,0x20(%rsp)
   28ac5:	48 89 ee             	mov    %rbp,%rsi
   28ac8:	e9 88 04 00 00       	jmp    28f55 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xbe5>
   28acd:	bf 08 00 00 00       	mov    $0x8,%edi
   28ad2:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
   28ad9:	00 00 
   28adb:	4d 85 e4             	test   %r12,%r12
   28ade:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
   28ae3:	0f 84 08 09 00 00    	je     293f1 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1081>
   28ae9:	4c 89 ea             	mov    %r13,%rdx
   28aec:	49 89 fd             	mov    %rdi,%r13
   28aef:	ff 15 23 be 09 00    	call   *0x9be23(%rip)        # c4918 <memcpy@GLIBC_2.14>
   28af5:	48 83 fd 01          	cmp    $0x1,%rbp
   28af9:	0f 85 05 09 00 00    	jne    29404 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1094>
   28aff:	4d 8b 45 08          	mov    0x8(%r13),%r8
   28b03:	4d 8d 8f 20 01 00 00 	lea    0x120(%r15),%r9
   28b0a:	b9 01 00 00 00       	mov    $0x1,%ecx
   28b0f:	41 80 7d 00 00       	cmpb   $0x0,0x0(%r13)
   28b14:	75 59                	jne    28b6f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7ff>
   28b16:	49 8b 01             	mov    (%r9),%rax
   28b19:	48 85 c0             	test   %rax,%rax
   28b1c:	74 4f                	je     28b6d <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7fd>
   28b1e:	48 89 c2             	mov    %rax,%rdx
   28b21:	eb 1b                	jmp    28b3e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7ce>
   28b23:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   28b2a:	84 00 00 00 00 00 
   28b30:	be 38 00 00 00       	mov    $0x38,%esi
   28b35:	48 8b 14 32          	mov    (%rdx,%rsi,1),%rdx
   28b39:	48 85 d2             	test   %rdx,%rdx
   28b3c:	74 2f                	je     28b6d <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7fd>
   28b3e:	4c 3b 42 20          	cmp    0x20(%rdx),%r8
   28b42:	40 0f 97 c6          	seta   %sil
   28b46:	40 80 de 00          	sbb    $0x0,%sil
   28b4a:	40 80 fe 01          	cmp    $0x1,%sil
   28b4e:	74 e0                	je     28b30 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7c0>
   28b50:	40 0f b6 fe          	movzbl %sil,%edi
   28b54:	be 30 00 00 00       	mov    $0x30,%esi
   28b59:	81 ff ff 00 00 00    	cmp    $0xff,%edi
   28b5f:	74 d4                	je     28b35 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7c5>
   28b61:	4c 8b 42 18          	mov    0x18(%rdx),%r8
   28b65:	83 7a 10 01          	cmpl   $0x1,0x10(%rdx)
   28b69:	75 b3                	jne    28b1e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7ae>
   28b6b:	eb 02                	jmp    28b6f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x7ff>
   28b6d:	31 c9                	xor    %ecx,%ecx
   28b6f:	48 8d 15 5a 91 09 00 	lea    0x9915a(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   28b76:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   28b7d:	00 
   28b7e:	0f 84 7a 08 00 00    	je     293fe <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x108e>
   28b84:	49 8d af c8 01 00 00 	lea    0x1c8(%r15),%rbp
   28b8b:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   28b92:	48 8b 30             	mov    (%rax),%rsi
   28b95:	48 83 fe 02          	cmp    $0x2,%rsi
   28b99:	75 09                	jne    28ba4 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x834>
   28b9b:	48 89 08             	mov    %rcx,(%rax)
   28b9e:	4c 89 40 08          	mov    %r8,0x8(%rax)
   28ba2:	eb 2c                	jmp    28bd0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x860>
   28ba4:	48 8b 50 08          	mov    0x8(%rax),%rdx
   28ba8:	49 8d 7f 10          	lea    0x10(%r15),%rdi
   28bac:	48 89 2c 24          	mov    %rbp,(%rsp)
   28bb0:	4d 89 cd             	mov    %r9,%r13
   28bb3:	ff 15 57 be 09 00    	call   *0x9be57(%rip)        # c4a10 <_DYNAMIC+0x300>
   28bb9:	b9 07 00 00 00       	mov    $0x7,%ecx
   28bbe:	84 c0                	test   %al,%al
   28bc0:	4d 89 e9             	mov    %r13,%r9
   28bc3:	48 8d 15 06 91 09 00 	lea    0x99106(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   28bca:	0f 84 c2 00 00 00    	je     28c92 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x922>
   28bd0:	49 83 fc 01          	cmp    $0x1,%r12
   28bd4:	0f 84 61 08 00 00    	je     2943b <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10cb>
   28bda:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   28bdf:	4c 8b 40 18          	mov    0x18(%rax),%r8
   28be3:	bf 01 00 00 00       	mov    $0x1,%edi
   28be8:	80 78 10 00          	cmpb   $0x0,0x10(%rax)
   28bec:	74 07                	je     28bf5 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x885>
   28bee:	b9 01 00 00 00       	mov    $0x1,%ecx
   28bf3:	eb 4e                	jmp    28c43 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x8d3>
   28bf5:	49 8b 01             	mov    (%r9),%rax
   28bf8:	31 c9                	xor    %ecx,%ecx
   28bfa:	48 85 c0             	test   %rax,%rax
   28bfd:	74 44                	je     28c43 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x8d3>
   28bff:	49 89 c3             	mov    %rax,%r11
   28c02:	eb 0e                	jmp    28c12 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x8a2>
   28c04:	be 38 00 00 00       	mov    $0x38,%esi
   28c09:	4d 8b 1c 33          	mov    (%r11,%rsi,1),%r11
   28c0d:	4d 85 db             	test   %r11,%r11
   28c10:	74 31                	je     28c43 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x8d3>
   28c12:	4d 3b 43 20          	cmp    0x20(%r11),%r8
   28c16:	40 0f 97 c6          	seta   %sil
   28c1a:	40 80 de 00          	sbb    $0x0,%sil
   28c1e:	40 80 fe 01          	cmp    $0x1,%sil
   28c22:	74 e0                	je     28c04 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x894>
   28c24:	44 0f b6 d6          	movzbl %sil,%r10d
   28c28:	be 30 00 00 00       	mov    $0x30,%esi
   28c2d:	41 81 fa ff 00 00 00 	cmp    $0xff,%r10d
   28c34:	74 d3                	je     28c09 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x899>
   28c36:	4d 8b 43 18          	mov    0x18(%r11),%r8
   28c3a:	41 83 7b 10 01       	cmpl   $0x1,0x10(%r11)
   28c3f:	75 be                	jne    28bff <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x88f>
   28c41:	eb ab                	jmp    28bee <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x87e>
   28c43:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   28c4a:	48 83 fe 02          	cmp    $0x2,%rsi
   28c4e:	0f 82 f8 07 00 00    	jb     2944c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10dc>
   28c54:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   28c5b:	48 8b 70 10          	mov    0x10(%rax),%rsi
   28c5f:	48 83 fe 02          	cmp    $0x2,%rsi
   28c63:	75 0a                	jne    28c6f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x8ff>
   28c65:	48 89 48 10          	mov    %rcx,0x10(%rax)
   28c69:	4c 89 40 18          	mov    %r8,0x18(%rax)
   28c6d:	eb 1e                	jmp    28c8d <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x91d>
   28c6f:	48 8b 50 18          	mov    0x18(%rax),%rdx
   28c73:	49 83 c7 10          	add    $0x10,%r15
   28c77:	48 89 2c 24          	mov    %rbp,(%rsp)
   28c7b:	4c 89 ff             	mov    %r15,%rdi
   28c7e:	ff 15 8c bd 09 00    	call   *0x9bd8c(%rip)        # c4a10 <_DYNAMIC+0x300>
   28c84:	b9 07 00 00 00       	mov    $0x7,%ecx
   28c89:	84 c0                	test   %al,%al
   28c8b:	74 05                	je     28c92 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x922>
   28c8d:	b9 06 00 00 00       	mov    $0x6,%ecx
   28c92:	48 89 0b             	mov    %rcx,(%rbx)
   28c95:	e9 76 04 00 00       	jmp    29110 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xda0>
   28c9a:	b8 08 00 00 00       	mov    $0x8,%eax
   28c9f:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   28ca4:	48 c7 44 24 20 00 00 	movq   $0x0,0x20(%rsp)
   28cab:	00 00 
   28cad:	4d 85 e4             	test   %r12,%r12
   28cb0:	0f 84 69 07 00 00    	je     2941f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10af>
   28cb6:	4c 8b 64 24 18       	mov    0x18(%rsp),%r12
   28cbb:	4c 89 e7             	mov    %r12,%rdi
   28cbe:	48 89 ee             	mov    %rbp,%rsi
   28cc1:	4c 89 ea             	mov    %r13,%rdx
   28cc4:	ff 15 4e bc 09 00    	call   *0x9bc4e(%rip)        # c4918 <memcpy@GLIBC_2.14>
   28cca:	4d 8b 44 24 08       	mov    0x8(%r12),%r8
   28ccf:	4d 8d 8f 20 01 00 00 	lea    0x120(%r15),%r9
   28cd6:	b9 01 00 00 00       	mov    $0x1,%ecx
   28cdb:	41 80 3c 24 00       	cmpb   $0x0,(%r12)
   28ce0:	74 07                	je     28ce9 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x979>
   28ce2:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   28ce7:	eb 57                	jmp    28d40 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x9d0>
   28ce9:	49 8b 11             	mov    (%r9),%rdx
   28cec:	48 85 d2             	test   %rdx,%rdx
   28cef:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   28cf4:	74 48                	je     28d3e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x9ce>
   28cf6:	48 89 d6             	mov    %rdx,%rsi
   28cf9:	eb 13                	jmp    28d0e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x99e>
   28cfb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   28d00:	bf 38 00 00 00       	mov    $0x38,%edi
   28d05:	48 8b 34 3e          	mov    (%rsi,%rdi,1),%rsi
   28d09:	48 85 f6             	test   %rsi,%rsi
   28d0c:	74 30                	je     28d3e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x9ce>
   28d0e:	4c 3b 46 20          	cmp    0x20(%rsi),%r8
   28d12:	40 0f 97 c7          	seta   %dil
   28d16:	40 80 df 00          	sbb    $0x0,%dil
   28d1a:	40 80 ff 01          	cmp    $0x1,%dil
   28d1e:	74 e0                	je     28d00 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x990>
   28d20:	44 0f b6 d7          	movzbl %dil,%r10d
   28d24:	bf 30 00 00 00       	mov    $0x30,%edi
   28d29:	41 81 fa ff 00 00 00 	cmp    $0xff,%r10d
   28d30:	74 d3                	je     28d05 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x995>
   28d32:	4c 8b 46 18          	mov    0x18(%rsi),%r8
   28d36:	83 7e 10 01          	cmpl   $0x1,0x10(%rsi)
   28d3a:	75 ba                	jne    28cf6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x986>
   28d3c:	eb 02                	jmp    28d40 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x9d0>
   28d3e:	31 c9                	xor    %ecx,%ecx
   28d40:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   28d47:	00 
   28d48:	0f 84 da 06 00 00    	je     29428 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10b8>
   28d4e:	49 8b 96 88 00 00 00 	mov    0x88(%r14),%rdx
   28d55:	48 8b 32             	mov    (%rdx),%rsi
   28d58:	48 83 fe 02          	cmp    $0x2,%rsi
   28d5c:	75 09                	jne    28d67 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x9f7>
   28d5e:	48 89 0a             	mov    %rcx,(%rdx)
   28d61:	4c 89 42 08          	mov    %r8,0x8(%rdx)
   28d65:	eb 25                	jmp    28d8c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xa1c>
   28d67:	49 8d 87 c8 01 00 00 	lea    0x1c8(%r15),%rax
   28d6e:	48 8b 52 08          	mov    0x8(%rdx),%rdx
   28d72:	49 83 c7 10          	add    $0x10,%r15
   28d76:	48 89 04 24          	mov    %rax,(%rsp)
   28d7a:	4c 89 ff             	mov    %r15,%rdi
   28d7d:	ff 15 8d bc 09 00    	call   *0x9bc8d(%rip)        # c4a10 <_DYNAMIC+0x300>
   28d83:	84 c0                	test   %al,%al
   28d85:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   28d8a:	74 12                	je     28d9e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xa2e>
   28d8c:	49 89 86 b0 00 00 00 	mov    %rax,0xb0(%r14)
   28d93:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   28d9a:	01 00 00 00 
   28d9e:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   28da5:	4c 89 e7             	mov    %r12,%rdi
   28da8:	ff 15 5a bb 09 00    	call   *0x9bb5a(%rip)        # c4908 <free@GLIBC_2.2.5>
   28dae:	e9 13 04 00 00       	jmp    291c6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe56>
   28db3:	48 89 fd             	mov    %rdi,%rbp
   28db6:	48 8b 51 08          	mov    0x8(%rcx),%rdx
   28dba:	4c 89 ff             	mov    %r15,%rdi
   28dbd:	48 89 c6             	mov    %rax,%rsi
   28dc0:	e8 5b 41 02 00       	call   4cf20 <_RNvMs0_Cs3FgqbjBu8z9_12chr_compiledNtB5_4Core10ground_key.llvm.15908011480901089632>
   28dc5:	a8 01                	test   $0x1,%al
   28dc7:	0f 84 53 03 00 00    	je     29120 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdb0>
   28dcd:	48 89 ef             	mov    %rbp,%rdi
   28dd0:	48 89 6c 24 50       	mov    %rbp,0x50(%rsp)
   28dd5:	48 c7 44 24 58 00 00 	movq   $0x0,0x58(%rsp)
   28ddc:	00 00 
   28dde:	48 89 54 24 60       	mov    %rdx,0x60(%rsp)
   28de3:	49 8b 87 c8 03 00 00 	mov    0x3c8(%r15),%rax
   28dea:	48 85 c0             	test   %rax,%rax
   28ded:	49 8d 76 40          	lea    0x40(%r14),%rsi
   28df1:	74 35                	je     28e28 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xab8>
   28df3:	48 89 54 24 18       	mov    %rdx,0x18(%rsp)
   28df8:	49 8b 97 d0 03 00 00 	mov    0x3d0(%r15),%rdx
   28dff:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   28e04:	48 8d 4c 24 50       	lea    0x50(%rsp),%rcx
   28e09:	48 89 c6             	mov    %rax,%rsi
   28e0c:	e8 9f 0c 01 00       	call   39ab0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, usize, usize), alloc::collections::btree::set::BTreeSet<u64>, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, usize, usize)>>
   28e11:	83 7c 24 28 01       	cmpl   $0x1,0x28(%rsp)
   28e16:	0f 85 c8 03 00 00    	jne    291e4 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe74>
   28e1c:	49 8d 76 40          	lea    0x40(%r14),%rsi
   28e20:	48 89 ef             	mov    %rbp,%rdi
   28e23:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   28e28:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   28e2e:	45 31 c0             	xor    %r8d,%r8d
   28e31:	31 ed                	xor    %ebp,%ebp
   28e33:	49 89 fd             	mov    %rdi,%r13
   28e36:	e9 3e 03 00 00       	jmp    29179 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe09>
   28e3b:	48 89 4c 24 10       	mov    %rcx,0x10(%rsp)
   28e40:	48 8b 50 08          	mov    0x8(%rax),%rdx
   28e44:	4c 89 ff             	mov    %r15,%rdi
   28e47:	e8 d4 40 02 00       	call   4cf20 <_RNvMs0_Cs3FgqbjBu8z9_12chr_compiledNtB5_4Core10ground_key.llvm.15908011480901089632>
   28e4c:	a8 01                	test   $0x1,%al
   28e4e:	0f 84 03 03 00 00    	je     29157 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xde7>
   28e54:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
   28e59:	48 89 4c 24 50       	mov    %rcx,0x50(%rsp)
   28e5e:	48 c7 44 24 58 00 00 	movq   $0x0,0x58(%rsp)
   28e65:	00 00 
   28e67:	48 89 54 24 60       	mov    %rdx,0x60(%rsp)
   28e6c:	49 8b b7 c8 03 00 00 	mov    0x3c8(%r15),%rsi
   28e73:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   28e79:	48 85 f6             	test   %rsi,%rsi
   28e7c:	0f 84 56 03 00 00    	je     291d8 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe68>
   28e82:	48 89 54 24 18       	mov    %rdx,0x18(%rsp)
   28e87:	49 8b 97 d0 03 00 00 	mov    0x3d0(%r15),%rdx
   28e8e:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   28e93:	48 8d 4c 24 50       	lea    0x50(%rsp),%rcx
   28e98:	e8 13 0c 01 00       	call   39ab0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, usize, usize), alloc::collections::btree::set::BTreeSet<u64>, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, usize, usize)>>
   28e9d:	83 7c 24 28 01       	cmpl   $0x1,0x28(%rsp)
   28ea2:	0f 85 8e 03 00 00    	jne    29236 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xec6>
   28ea8:	31 ed                	xor    %ebp,%ebp
   28eaa:	e9 a9 03 00 00       	jmp    29258 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xee8>
   28eaf:	4c 89 44 24 20       	mov    %r8,0x20(%rsp)
   28eb4:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
   28eb9:	48 8b 51 18          	mov    0x18(%rcx),%rdx
   28ebd:	4c 89 ff             	mov    %r15,%rdi
   28ec0:	48 89 c6             	mov    %rax,%rsi
   28ec3:	e8 58 40 02 00       	call   4cf20 <_RNvMs0_Cs3FgqbjBu8z9_12chr_compiledNtB5_4Core10ground_key.llvm.15908011480901089632>
   28ec8:	a8 01                	test   $0x1,%al
   28eca:	0f 84 94 02 00 00    	je     29164 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdf4>
   28ed0:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   28ed5:	48 89 7c 24 50       	mov    %rdi,0x50(%rsp)
   28eda:	48 c7 44 24 58 01 00 	movq   $0x1,0x58(%rsp)
   28ee1:	00 00 
   28ee3:	48 89 54 24 60       	mov    %rdx,0x60(%rsp)
   28ee8:	49 8b b7 c8 03 00 00 	mov    0x3c8(%r15),%rsi
   28eef:	bd 01 00 00 00       	mov    $0x1,%ebp
   28ef4:	48 85 f6             	test   %rsi,%rsi
   28ef7:	0f 84 6f 03 00 00    	je     2926c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xefc>
   28efd:	48 89 54 24 70       	mov    %rdx,0x70(%rsp)
   28f02:	49 8b 97 d0 03 00 00 	mov    0x3d0(%r15),%rdx
   28f09:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   28f0e:	48 8d 4c 24 50       	lea    0x50(%rsp),%rcx
   28f13:	e8 98 0b 01 00       	call   39ab0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, usize, usize), alloc::collections::btree::set::BTreeSet<u64>, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, usize, usize)>>
   28f18:	83 7c 24 28 01       	cmpl   $0x1,0x28(%rsp)
   28f1d:	0f 85 5e 03 00 00    	jne    29281 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xf11>
   28f23:	45 31 c0             	xor    %r8d,%r8d
   28f26:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   28f2b:	49 89 fd             	mov    %rdi,%r13
   28f2e:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   28f34:	49 8d 76 40          	lea    0x40(%r14),%rsi
   28f38:	48 8b 54 24 70       	mov    0x70(%rsp),%rdx
   28f3d:	e9 37 02 00 00       	jmp    29179 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe09>
   28f42:	b8 08 00 00 00       	mov    $0x8,%eax
   28f47:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   28f4c:	48 c7 44 24 20 00 00 	movq   $0x0,0x20(%rsp)
   28f53:	00 00 
   28f55:	4d 85 e4             	test   %r12,%r12
   28f58:	0f 84 18 05 00 00    	je     29476 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1106>
   28f5e:	48 8b 6c 24 10       	mov    0x10(%rsp),%rbp
   28f63:	48 89 ef             	mov    %rbp,%rdi
   28f66:	4c 89 ea             	mov    %r13,%rdx
   28f69:	ff 15 a9 b9 09 00    	call   *0x9b9a9(%rip)        # c4918 <memcpy@GLIBC_2.14>
   28f6f:	4c 8b 45 08          	mov    0x8(%rbp),%r8
   28f73:	4d 8d af 20 01 00 00 	lea    0x120(%r15),%r13
   28f7a:	b9 01 00 00 00       	mov    $0x1,%ecx
   28f7f:	80 7d 00 00          	cmpb   $0x0,0x0(%rbp)
   28f83:	75 4d                	jne    28fd2 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc62>
   28f85:	49 8b 45 00          	mov    0x0(%r13),%rax
   28f89:	48 85 c0             	test   %rax,%rax
   28f8c:	74 42                	je     28fd0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc60>
   28f8e:	48 89 c2             	mov    %rax,%rdx
   28f91:	eb 0e                	jmp    28fa1 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc31>
   28f93:	be 38 00 00 00       	mov    $0x38,%esi
   28f98:	48 8b 14 32          	mov    (%rdx,%rsi,1),%rdx
   28f9c:	48 85 d2             	test   %rdx,%rdx
   28f9f:	74 2f                	je     28fd0 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc60>
   28fa1:	4c 3b 42 20          	cmp    0x20(%rdx),%r8
   28fa5:	40 0f 97 c6          	seta   %sil
   28fa9:	40 80 de 00          	sbb    $0x0,%sil
   28fad:	40 80 fe 01          	cmp    $0x1,%sil
   28fb1:	74 e0                	je     28f93 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc23>
   28fb3:	40 0f b6 fe          	movzbl %sil,%edi
   28fb7:	be 30 00 00 00       	mov    $0x30,%esi
   28fbc:	81 ff ff 00 00 00    	cmp    $0xff,%edi
   28fc2:	74 d4                	je     28f98 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc28>
   28fc4:	4c 8b 42 18          	mov    0x18(%rdx),%r8
   28fc8:	83 7a 10 01          	cmpl   $0x1,0x10(%rdx)
   28fcc:	75 c0                	jne    28f8e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc1e>
   28fce:	eb 02                	jmp    28fd2 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc62>
   28fd0:	31 c9                	xor    %ecx,%ecx
   28fd2:	48 8d 15 f7 8c 09 00 	lea    0x98cf7(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   28fd9:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   28fe0:	00 
   28fe1:	0f 84 96 04 00 00    	je     2947d <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x110d>
   28fe7:	49 8d af c8 01 00 00 	lea    0x1c8(%r15),%rbp
   28fee:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   28ff5:	48 8b 30             	mov    (%rax),%rsi
   28ff8:	48 83 fe 02          	cmp    $0x2,%rsi
   28ffc:	75 0e                	jne    2900c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xc9c>
   28ffe:	48 89 08             	mov    %rcx,(%rax)
   29001:	4c 89 40 08          	mov    %r8,0x8(%rax)
   29005:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   2900a:	eb 29                	jmp    29035 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xcc5>
   2900c:	48 8b 50 08          	mov    0x8(%rax),%rdx
   29010:	49 8d 7f 10          	lea    0x10(%r15),%rdi
   29014:	48 89 2c 24          	mov    %rbp,(%rsp)
   29018:	4d 89 e9             	mov    %r13,%r9
   2901b:	ff 15 ef b9 09 00    	call   *0x9b9ef(%rip)        # c4a10 <_DYNAMIC+0x300>
   29021:	84 c0                	test   %al,%al
   29023:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   29028:	48 8d 15 a1 8c 09 00 	lea    0x98ca1(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   2902f:	0f 84 d4 00 00 00    	je     29109 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd99>
   29035:	49 83 fc 01          	cmp    $0x1,%r12
   29039:	0f 84 44 04 00 00    	je     29483 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1113>
   2903f:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
   29044:	4c 8b 41 18          	mov    0x18(%rcx),%r8
   29048:	bf 01 00 00 00       	mov    $0x1,%edi
   2904d:	80 79 10 00          	cmpb   $0x0,0x10(%rcx)
   29051:	74 07                	je     2905a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xcea>
   29053:	b9 01 00 00 00       	mov    $0x1,%ecx
   29058:	eb 50                	jmp    290aa <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd3a>
   2905a:	4d 8b 5d 00          	mov    0x0(%r13),%r11
   2905e:	31 c9                	xor    %ecx,%ecx
   29060:	4d 85 db             	test   %r11,%r11
   29063:	74 45                	je     290aa <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd3a>
   29065:	4c 89 de             	mov    %r11,%rsi
   29068:	eb 0f                	jmp    29079 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd09>
   2906a:	41 b9 38 00 00 00    	mov    $0x38,%r9d
   29070:	4a 8b 34 0e          	mov    (%rsi,%r9,1),%rsi
   29074:	48 85 f6             	test   %rsi,%rsi
   29077:	74 31                	je     290aa <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd3a>
   29079:	4c 3b 46 20          	cmp    0x20(%rsi),%r8
   2907d:	41 0f 97 c1          	seta   %r9b
   29081:	41 80 d9 00          	sbb    $0x0,%r9b
   29085:	41 80 f9 01          	cmp    $0x1,%r9b
   29089:	74 df                	je     2906a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xcfa>
   2908b:	45 0f b6 d1          	movzbl %r9b,%r10d
   2908f:	41 b9 30 00 00 00    	mov    $0x30,%r9d
   29095:	41 81 fa ff 00 00 00 	cmp    $0xff,%r10d
   2909c:	74 d2                	je     29070 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd00>
   2909e:	4c 8b 46 18          	mov    0x18(%rsi),%r8
   290a2:	83 7e 10 01          	cmpl   $0x1,0x10(%rsi)
   290a6:	75 bd                	jne    29065 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xcf5>
   290a8:	eb a9                	jmp    29053 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xce3>
   290aa:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   290b1:	48 83 fe 02          	cmp    $0x2,%rsi
   290b5:	0f 82 d9 03 00 00    	jb     29494 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1124>
   290bb:	49 8b 96 88 00 00 00 	mov    0x88(%r14),%rdx
   290c2:	48 8b 72 10          	mov    0x10(%rdx),%rsi
   290c6:	48 83 fe 02          	cmp    $0x2,%rsi
   290ca:	75 0a                	jne    290d6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd66>
   290cc:	48 89 4a 10          	mov    %rcx,0x10(%rdx)
   290d0:	4c 89 42 18          	mov    %r8,0x18(%rdx)
   290d4:	eb 21                	jmp    290f7 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd87>
   290d6:	48 8b 52 18          	mov    0x18(%rdx),%rdx
   290da:	49 83 c7 10          	add    $0x10,%r15
   290de:	48 89 2c 24          	mov    %rbp,(%rsp)
   290e2:	4c 89 ff             	mov    %r15,%rdi
   290e5:	4d 89 e9             	mov    %r13,%r9
   290e8:	ff 15 22 b9 09 00    	call   *0x9b922(%rip)        # c4a10 <_DYNAMIC+0x300>
   290ee:	84 c0                	test   %al,%al
   290f0:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   290f5:	74 12                	je     29109 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xd99>
   290f7:	49 89 86 b8 00 00 00 	mov    %rax,0xb8(%r14)
   290fe:	49 c7 86 a8 00 00 00 	movq   $0x2,0xa8(%r14)
   29105:	02 00 00 00 
   29109:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   29110:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   29115:	ff 15 ed b7 09 00    	call   *0x9b7ed(%rip)        # c4908 <free@GLIBC_2.2.5>
   2911b:	e9 a6 00 00 00       	jmp    291c6 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe56>
   29120:	45 31 e4             	xor    %r12d,%r12d
   29123:	49 8d 76 40          	lea    0x40(%r14),%rsi
   29127:	48 89 ef             	mov    %rbp,%rdi
   2912a:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   29131:	49 8b 86 90 00 00 00 	mov    0x90(%r14),%rax
   29138:	48 83 f8 02          	cmp    $0x2,%rax
   2913c:	0f 83 63 f7 ff ff    	jae    288a5 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x535>
   29142:	48 8d 15 8f 84 09 00 	lea    0x9848f(%rip),%rdx        # c15d8 <__frame_dummy_init_array_entry+0x298>
   29149:	bf 01 00 00 00       	mov    $0x1,%edi
   2914e:	48 89 c6             	mov    %rax,%rsi
   29151:	ff 15 f9 b7 09 00    	call   *0x9b7f9(%rip)        # c4950 <_DYNAMIC+0x240>
   29157:	45 31 e4             	xor    %r12d,%r12d
   2915a:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
   2915f:	e9 c2 f7 ff ff       	jmp    28926 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5b6>
   29164:	31 ed                	xor    %ebp,%ebp
   29166:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2916a:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   2916f:	4c 8b 44 24 20       	mov    0x20(%rsp),%r8
   29174:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   29179:	4d 89 66 40          	mov    %r12,0x40(%r14)
   2917d:	4d 89 6e 48          	mov    %r13,0x48(%r14)
   29181:	49 89 6e 50          	mov    %rbp,0x50(%r14)
   29185:	49 89 56 58          	mov    %rdx,0x58(%r14)
   29189:	49 c7 46 60 00 00 00 	movq   $0x0,0x60(%r14)
   29190:	00 
   29191:	49 89 7e 70          	mov    %rdi,0x70(%r14)
   29195:	4d 89 46 78          	mov    %r8,0x78(%r14)
   29199:	4c 89 ff             	mov    %r15,%rdi
   2919c:	ff 15 66 b8 09 00    	call   *0x9b866(%rip)        # c4a08 <_DYNAMIC+0x2f8>
   291a2:	48 83 f8 01          	cmp    $0x1,%rax
   291a6:	0f 84 72 f4 ff ff    	je     2861e <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x2ae>
   291ac:	49 c7 46 40 ff ff ff 	movq   $0xffffffffffffffff,0x40(%r14)
   291b3:	ff 
   291b4:	49 c7 86 a8 00 00 00 	movq   $0x0,0xa8(%r14)
   291bb:	00 00 00 00 
   291bf:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   291c6:	48 89 d8             	mov    %rbx,%rax
   291c9:	48 83 c4 78          	add    $0x78,%rsp
   291cd:	5b                   	pop    %rbx
   291ce:	41 5c                	pop    %r12
   291d0:	41 5d                	pop    %r13
   291d2:	41 5e                	pop    %r14
   291d4:	41 5f                	pop    %r15
   291d6:	5d                   	pop    %rbp
   291d7:	c3                   	ret    
   291d8:	31 ed                	xor    %ebp,%ebp
   291da:	49 89 cd             	mov    %rcx,%r13
   291dd:	31 c0                	xor    %eax,%eax
   291df:	e9 42 f7 ff ff       	jmp    28926 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5b6>
   291e4:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   291e9:	48 8b 4c 24 40       	mov    0x40(%rsp),%rcx
   291ee:	48 8d 0c 49          	lea    (%rcx,%rcx,2),%rcx
   291f2:	48 8b 84 c8 20 01 00 	mov    0x120(%rax,%rcx,8),%rax
   291f9:	00 
   291fa:	48 83 f8 ff          	cmp    $0xffffffffffffffff,%rax
   291fe:	49 8d 76 40          	lea    0x40(%r14),%rsi
   29202:	0f 84 bc 00 00 00    	je     292c4 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xf54>
   29208:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2920e:	48 85 c0             	test   %rax,%rax
   29211:	48 89 ef             	mov    %rbp,%rdi
   29214:	0f 84 ce 00 00 00    	je     292e8 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xf78>
   2921a:	49 89 fd             	mov    %rdi,%r13
   2921d:	49 89 c0             	mov    %rax,%r8
   29220:	49 8b 86 90 00 00 00 	mov    0x90(%r14),%rax
   29227:	48 83 f8 02          	cmp    $0x2,%rax
   2922b:	0f 83 74 f6 ff ff    	jae    288a5 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x535>
   29231:	e9 0c ff ff ff       	jmp    29142 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdd2>
   29236:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   2923b:	48 8b 4c 24 40       	mov    0x40(%rsp),%rcx
   29240:	48 8d 0c 49          	lea    (%rcx,%rcx,2),%rcx
   29244:	48 8b ac c8 20 01 00 	mov    0x120(%rax,%rcx,8),%rbp
   2924b:	00 
   2924c:	48 83 fd ff          	cmp    $0xffffffffffffffff,%rbp
   29250:	74 7a                	je     292cc <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xf5c>
   29252:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   29258:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
   2925d:	49 89 cd             	mov    %rcx,%r13
   29260:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   29265:	31 c0                	xor    %eax,%eax
   29267:	e9 ba f6 ff ff       	jmp    28926 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5b6>
   2926c:	45 31 c0             	xor    %r8d,%r8d
   2926f:	49 89 fd             	mov    %rdi,%r13
   29272:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   29278:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2927c:	e9 f8 fe ff ff       	jmp    29179 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe09>
   29281:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   29286:	48 8b 4c 24 40       	mov    0x40(%rsp),%rcx
   2928b:	48 8d 0c 49          	lea    (%rcx,%rcx,2),%rcx
   2928f:	48 8b 84 c8 20 01 00 	mov    0x120(%rax,%rcx,8),%rax
   29296:	00 
   29297:	4c 8b 44 24 20       	mov    0x20(%rsp),%r8
   2929c:	4c 39 c0             	cmp    %r8,%rax
   2929f:	49 8d 76 40          	lea    0x40(%r14),%rsi
   292a3:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   292a8:	0f 83 0c f6 ff ff    	jae    288ba <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x54a>
   292ae:	49 89 c0             	mov    %rax,%r8
   292b1:	49 89 fd             	mov    %rdi,%r13
   292b4:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   292ba:	48 8b 54 24 70       	mov    0x70(%rsp),%rdx
   292bf:	e9 b5 fe ff ff       	jmp    29179 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe09>
   292c4:	45 31 e4             	xor    %r12d,%r12d
   292c7:	e9 5b fe ff ff       	jmp    29127 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xdb7>
   292cc:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   292d2:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   292d9:	45 31 e4             	xor    %r12d,%r12d
   292dc:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
   292e1:	31 c0                	xor    %eax,%eax
   292e3:	e9 3e f6 ff ff       	jmp    28926 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x5b6>
   292e8:	45 31 c0             	xor    %r8d,%r8d
   292eb:	31 ed                	xor    %ebp,%ebp
   292ed:	49 89 fd             	mov    %rdi,%r13
   292f0:	e9 7f fe ff ff       	jmp    29174 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0xe04>
   292f5:	48 8d 3d 35 00 fe ff 	lea    -0x1ffcb(%rip),%rdi        # 9331 <anon.ee651107ab5319c6bc273e1a29320aaf.1.llvm.14746981713632465754+0x351>
   292fc:	48 8d 15 4d 83 09 00 	lea    0x9834d(%rip),%rdx        # c1650 <__frame_dummy_init_array_entry+0x310>
   29303:	be 28 00 00 00       	mov    $0x28,%esi
   29308:	ff 15 0a b7 09 00    	call   *0x9b70a(%rip)        # c4a18 <_DYNAMIC+0x308>
   2930e:	bf 08 00 00 00       	mov    $0x8,%edi
   29313:	be 10 00 00 00       	mov    $0x10,%esi
   29318:	ff 15 6a b6 09 00    	call   *0x9b66a(%rip)        # c4988 <_DYNAMIC+0x278>
   2931e:	bf 08 00 00 00       	mov    $0x8,%edi
   29323:	be 10 00 00 00       	mov    $0x10,%esi
   29328:	ff 15 12 b6 09 00    	call   *0x9b612(%rip)        # c4940 <_DYNAMIC+0x230>
   2932e:	48 8d 15 bb 8a 09 00 	lea    0x98abb(%rip),%rdx        # c1df0 <anon.67153942ca9c76798561ce1001119436.221.llvm.15908011480901089632>
   29335:	bf 01 00 00 00       	mov    $0x1,%edi
   2933a:	ff 15 10 b6 09 00    	call   *0x9b610(%rip)        # c4950 <_DYNAMIC+0x240>
   29340:	48 8d 15 d9 82 09 00 	lea    0x982d9(%rip),%rdx        # c1620 <__frame_dummy_init_array_entry+0x2e0>
   29347:	31 ff                	xor    %edi,%edi
   29349:	31 f6                	xor    %esi,%esi
   2934b:	ff 15 ff b5 09 00    	call   *0x9b5ff(%rip)        # c4950 <_DYNAMIC+0x240>
   29351:	48 8d 15 e0 82 09 00 	lea    0x982e0(%rip),%rdx        # c1638 <__frame_dummy_init_array_entry+0x2f8>
   29358:	31 ff                	xor    %edi,%edi
   2935a:	31 f6                	xor    %esi,%esi
   2935c:	ff 15 ee b5 09 00    	call   *0x9b5ee(%rip)        # c4950 <_DYNAMIC+0x240>
   29362:	bf 08 00 00 00       	mov    $0x8,%edi
   29367:	be 10 00 00 00       	mov    $0x10,%esi
   2936c:	ff 15 ce b5 09 00    	call   *0x9b5ce(%rip)        # c4940 <_DYNAMIC+0x230>
   29372:	e9 23 01 00 00       	jmp    2949a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x112a>
   29377:	48 8d 15 ca 81 09 00 	lea    0x981ca(%rip),%rdx        # c1548 <__frame_dummy_init_array_entry+0x208>
   2937e:	31 ff                	xor    %edi,%edi
   29380:	31 f6                	xor    %esi,%esi
   29382:	ff 15 c8 b5 09 00    	call   *0x9b5c8(%rip)        # c4950 <_DYNAMIC+0x240>
   29388:	48 8d 15 19 82 09 00 	lea    0x98219(%rip),%rdx        # c15a8 <__frame_dummy_init_array_entry+0x268>
   2938f:	bf 01 00 00 00       	mov    $0x1,%edi
   29394:	ff 15 b6 b5 09 00    	call   *0x9b5b6(%rip)        # c4950 <_DYNAMIC+0x240>
   2939a:	48 8d 15 bf 81 09 00 	lea    0x981bf(%rip),%rdx        # c1560 <__frame_dummy_init_array_entry+0x220>
   293a1:	bf 01 00 00 00       	mov    $0x1,%edi
   293a6:	ff 15 a4 b5 09 00    	call   *0x9b5a4(%rip)        # c4950 <_DYNAMIC+0x240>
   293ac:	48 8d 15 15 8b 09 00 	lea    0x98b15(%rip),%rdx        # c1ec8 <anon.67153942ca9c76798561ce1001119436.243.llvm.15908011480901089632>
   293b3:	31 ff                	xor    %edi,%edi
   293b5:	31 f6                	xor    %esi,%esi
   293b7:	ff 15 93 b5 09 00    	call   *0x9b593(%rip)        # c4950 <_DYNAMIC+0x240>
   293bd:	bf 08 00 00 00       	mov    $0x8,%edi
   293c2:	4c 89 ee             	mov    %r13,%rsi
   293c5:	ff 15 75 b5 09 00    	call   *0x9b575(%rip)        # c4940 <_DYNAMIC+0x230>
   293cb:	48 8d 15 0e 8b 09 00 	lea    0x98b0e(%rip),%rdx        # c1ee0 <anon.67153942ca9c76798561ce1001119436.244.llvm.15908011480901089632>
   293d2:	bf 01 00 00 00       	mov    $0x1,%edi
   293d7:	48 89 c6             	mov    %rax,%rsi
   293da:	ff 15 70 b5 09 00    	call   *0x9b570(%rip)        # c4950 <_DYNAMIC+0x240>
   293e0:	48 8d 15 f9 8a 09 00 	lea    0x98af9(%rip),%rdx        # c1ee0 <anon.67153942ca9c76798561ce1001119436.244.llvm.15908011480901089632>
   293e7:	31 ff                	xor    %edi,%edi
   293e9:	31 f6                	xor    %esi,%esi
   293eb:	ff 15 5f b5 09 00    	call   *0x9b55f(%rip)        # c4950 <_DYNAMIC+0x240>
   293f1:	48 83 fd 01          	cmp    $0x1,%rbp
   293f5:	75 0d                	jne    29404 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1094>
   293f7:	48 8d 15 02 81 09 00 	lea    0x98102(%rip),%rdx        # c1500 <__frame_dummy_init_array_entry+0x1c0>
   293fe:	31 ff                	xor    %edi,%edi
   29400:	31 f6                	xor    %esi,%esi
   29402:	eb 48                	jmp    2944c <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10dc>
   29404:	48 8d 3d 26 ff fd ff 	lea    -0x200da(%rip),%rdi        # 9331 <anon.ee651107ab5319c6bc273e1a29320aaf.1.llvm.14746981713632465754+0x351>
   2940b:	48 8d 15 1e 81 09 00 	lea    0x9811e(%rip),%rdx        # c1530 <__frame_dummy_init_array_entry+0x1f0>
   29412:	be 28 00 00 00       	mov    $0x28,%esi
   29417:	ff 15 fb b5 09 00    	call   *0x9b5fb(%rip)        # c4a18 <_DYNAMIC+0x308>
   2941d:	eb 7b                	jmp    2949a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x112a>
   2941f:	48 8d 15 6a 81 09 00 	lea    0x9816a(%rip),%rdx        # c1590 <__frame_dummy_init_array_entry+0x250>
   29426:	eb 07                	jmp    2942f <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x10bf>
   29428:	48 8d 15 a1 88 09 00 	lea    0x988a1(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   2942f:	31 ff                	xor    %edi,%edi
   29431:	31 f6                	xor    %esi,%esi
   29433:	ff 15 17 b5 09 00    	call   *0x9b517(%rip)        # c4950 <_DYNAMIC+0x240>
   29439:	eb 5f                	jmp    2949a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x112a>
   2943b:	48 8d 15 d6 80 09 00 	lea    0x980d6(%rip),%rdx        # c1518 <__frame_dummy_init_array_entry+0x1d8>
   29442:	bf 01 00 00 00       	mov    $0x1,%edi
   29447:	be 01 00 00 00       	mov    $0x1,%esi
   2944c:	ff 15 fe b4 09 00    	call   *0x9b4fe(%rip)        # c4950 <_DYNAMIC+0x240>
   29452:	eb 46                	jmp    2949a <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x112a>
   29454:	48 8d 15 65 81 09 00 	lea    0x98165(%rip),%rdx        # c15c0 <__frame_dummy_init_array_entry+0x280>
   2945b:	31 ff                	xor    %edi,%edi
   2945d:	31 f6                	xor    %esi,%esi
   2945f:	ff 15 eb b4 09 00    	call   *0x9b4eb(%rip)        # c4950 <_DYNAMIC+0x240>
   29465:	48 8d 15 0c 81 09 00 	lea    0x9810c(%rip),%rdx        # c1578 <__frame_dummy_init_array_entry+0x238>
   2946c:	31 ff                	xor    %edi,%edi
   2946e:	31 f6                	xor    %esi,%esi
   29470:	ff 15 da b4 09 00    	call   *0x9b4da(%rip)        # c4950 <_DYNAMIC+0x240>
   29476:	48 8d 15 73 81 09 00 	lea    0x98173(%rip),%rdx        # c15f0 <__frame_dummy_init_array_entry+0x2b0>
   2947d:	31 ff                	xor    %edi,%edi
   2947f:	31 f6                	xor    %esi,%esi
   29481:	eb 11                	jmp    29494 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1124>
   29483:	48 8d 15 7e 81 09 00 	lea    0x9817e(%rip),%rdx        # c1608 <__frame_dummy_init_array_entry+0x2c8>
   2948a:	bf 01 00 00 00       	mov    $0x1,%edi
   2948f:	be 01 00 00 00       	mov    $0x1,%esi
   29494:	ff 15 b6 b4 09 00    	call   *0x9b4b6(%rip)        # c4950 <_DYNAMIC+0x240>
   2949a:	0f 0b                	ud2    
   2949c:	48 89 c3             	mov    %rax,%rbx
   2949f:	48 83 7c 24 20 00    	cmpq   $0x0,0x20(%rsp)
   294a5:	75 3c                	jne    294e3 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1173>
   294a7:	eb 45                	jmp    294ee <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x117e>
   294a9:	48 89 c3             	mov    %rax,%rbx
   294ac:	48 83 7c 24 20 00    	cmpq   $0x0,0x20(%rsp)
   294b2:	74 3a                	je     294ee <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x117e>
   294b4:	48 8b 7c 24 18       	mov    0x18(%rsp),%rdi
   294b9:	eb 2d                	jmp    294e8 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1178>
   294bb:	48 89 c3             	mov    %rax,%rbx
   294be:	4c 89 e7             	mov    %r12,%rdi
   294c1:	eb 25                	jmp    294e8 <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x1178>
   294c3:	48 89 c3             	mov    %rax,%rbx
   294c6:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   294cb:	e8 d0 e3 ff ff       	call   278a0 <core::ptr::drop_glue::<chr_compiled::Work>>
   294d0:	eb 1c                	jmp    294ee <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x117e>
   294d2:	ff 15 00 b5 09 00    	call   *0x9b500(%rip)        # c49d8 <_DYNAMIC+0x2c8>
   294d8:	48 89 c3             	mov    %rax,%rbx
   294db:	48 83 7c 24 18 00    	cmpq   $0x0,0x18(%rsp)
   294e1:	74 0b                	je     294ee <<chain::generated::program_state_0 as chr_compiled::native_access::Continuation>::tick+0x117e>
   294e3:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   294e8:	ff 15 1a b4 09 00    	call   *0x9b41a(%rip)        # c4908 <free@GLIBC_2.2.5>
   294ee:	48 89 df             	mov    %rbx,%rdi
   294f1:	e8 0a 6e 09 00       	call   c0300 <_Unwind_Resume@plt>
   294f6:	cc                   	int3   
   294f7:	cc                   	int3   
   294f8:	cc                   	int3   
   294f9:	cc                   	int3   
   294fa:	cc                   	int3   
   294fb:	cc                   	int3   
   294fc:	cc                   	int3   
   294fd:	cc                   	int3   
   294fe:	cc                   	int3   
   294ff:	cc                   	int3   

0000000000029660 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick>:
   29660:	55                   	push   %rbp
   29661:	41 57                	push   %r15
   29663:	41 56                	push   %r14
   29665:	41 55                	push   %r13
   29667:	41 54                	push   %r12
   29669:	53                   	push   %rbx
   2966a:	48 81 ec a8 00 00 00 	sub    $0xa8,%rsp
   29671:	49 89 d7             	mov    %rdx,%r15
   29674:	49 89 f6             	mov    %rsi,%r14
   29677:	48 89 fb             	mov    %rdi,%rbx
   2967a:	80 be c0 00 00 00 00 	cmpb   $0x0,0xc0(%rsi)
   29681:	0f 84 a0 00 00 00    	je     29727 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xc7>
   29687:	41 c6 86 c0 00 00 00 	movb   $0x0,0xc0(%r14)
   2968e:	00 
   2968f:	83 39 01             	cmpl   $0x1,(%rcx)
   29692:	0f 85 8f 00 00 00    	jne    29727 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xc7>
   29698:	48 8b 69 08          	mov    0x8(%rcx),%rbp
   2969c:	48 85 ed             	test   %rbp,%rbp
   2969f:	0f 84 82 00 00 00    	je     29727 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xc7>
   296a5:	49 8b 87 28 01 00 00 	mov    0x128(%r15),%rax
   296ac:	48 85 c0             	test   %rax,%rax
   296af:	0f 84 c6 07 00 00    	je     29e7b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x81b>
   296b5:	48 8b 49 10          	mov    0x10(%rcx),%rcx
   296b9:	49 8b 97 30 01 00 00 	mov    0x130(%r15),%rdx
   296c0:	44 0f b7 80 c2 01 00 	movzwl 0x1c2(%rax),%r8d
   296c7:	00 
   296c8:	45 89 c1             	mov    %r8d,%r9d
   296cb:	41 c1 e1 05          	shl    $0x5,%r9d
   296cf:	48 c7 c7 ff ff ff ff 	mov    $0xffffffffffffffff,%rdi
   296d6:	31 f6                	xor    %esi,%esi
   296d8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   296df:	00 
   296e0:	49 39 f1             	cmp    %rsi,%r9
   296e3:	74 2b                	je     29710 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb0>
   296e5:	48 3b 8c f8 70 01 00 	cmp    0x170(%rax,%rdi,8),%rcx
   296ec:	00 
   296ed:	41 0f 97 c2          	seta   %r10b
   296f1:	41 80 da 00          	sbb    $0x0,%r10b
   296f5:	48 83 c6 20          	add    $0x20,%rsi
   296f9:	48 ff c7             	inc    %rdi
   296fc:	41 80 fa 01          	cmp    $0x1,%r10b
   29700:	74 de                	je     296e0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x80>
   29702:	45 0f b6 c2          	movzbl %r10b,%r8d
   29706:	45 85 c0             	test   %r8d,%r8d
   29709:	75 08                	jne    29713 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb3>
   2970b:	e9 42 05 00 00       	jmp    29c52 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x5f2>
   29710:	4c 89 c7             	mov    %r8,%rdi
   29713:	48 83 ea 01          	sub    $0x1,%rdx
   29717:	0f 82 5e 07 00 00    	jb     29e7b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x81b>
   2971d:	48 8b 84 f8 c8 01 00 	mov    0x1c8(%rax,%rdi,8),%rax
   29724:	00 
   29725:	eb 99                	jmp    296c0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x60>
   29727:	49 8b 86 a8 00 00 00 	mov    0xa8(%r14),%rax
   2972e:	48 85 c0             	test   %rax,%rax
   29731:	0f 84 f9 00 00 00    	je     29830 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1d0>
   29737:	48 83 f8 01          	cmp    $0x1,%rax
   2973b:	0f 84 a2 00 00 00    	je     297e3 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x183>
   29741:	48 83 f8 02          	cmp    $0x2,%rax
   29745:	0f 85 4d 10 00 00    	jne    2a798 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1138>
   2974b:	bf 10 00 00 00       	mov    $0x10,%edi
   29750:	ff 15 a2 b1 09 00    	call   *0x9b1a2(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29756:	48 85 c0             	test   %rax,%rax
   29759:	0f 84 8c 10 00 00    	je     2a7eb <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x118b>
   2975f:	49 89 c4             	mov    %rax,%r12
   29762:	49 8d 86 b0 00 00 00 	lea    0xb0(%r14),%rax
   29769:	0f 10 00             	movups (%rax),%xmm0
   2976c:	41 0f 11 04 24       	movups %xmm0,(%r12)
   29771:	48 c7 44 24 20 01 00 	movq   $0x1,0x20(%rsp)
   29778:	00 00 
   2977a:	48 c7 44 24 28 02 00 	movq   $0x2,0x28(%rsp)
   29781:	00 00 
   29783:	4c 89 64 24 30       	mov    %r12,0x30(%rsp)
   29788:	48 c7 44 24 38 02 00 	movq   $0x2,0x38(%rsp)
   2978f:	00 00 
   29791:	49 8b b7 58 01 00 00 	mov    0x158(%r15),%rsi
   29798:	48 85 f6             	test   %rsi,%rsi
   2979b:	0f 84 f6 01 00 00    	je     29997 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x337>
   297a1:	49 89 dd             	mov    %rbx,%r13
   297a4:	49 8b 97 60 01 00 00 	mov    0x160(%r15),%rdx
   297ab:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
   297b0:	48 8d 4c 24 20       	lea    0x20(%rsp),%rcx
   297b5:	e8 f6 01 01 00       	call   399b0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, alloc::vec::Vec<u64>), alloc::collections::btree::set_val::SetValZST, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, alloc::vec::Vec<u64>)>>
   297ba:	0f b6 5c 24 48       	movzbl 0x48(%rsp),%ebx
   297bf:	4c 89 e7             	mov    %r12,%rdi
   297c2:	ff 15 40 b1 09 00    	call   *0x9b140(%rip)        # c4908 <free@GLIBC_2.2.5>
   297c8:	84 db                	test   %bl,%bl
   297ca:	4c 89 eb             	mov    %r13,%rbx
   297cd:	0f 85 cd 01 00 00    	jne    299a0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x340>
   297d3:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   297da:	01 00 00 00 
   297de:	e9 7c 0e 00 00       	jmp    2a65f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfff>
   297e3:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   297ea:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   297f1:	83 39 01             	cmpl   $0x1,(%rcx)
   297f4:	0f 85 d7 00 00 00    	jne    298d1 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x271>
   297fa:	48 83 79 08 01       	cmpq   $0x1,0x8(%rcx)
   297ff:	0f 85 cc 00 00 00    	jne    298d1 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x271>
   29805:	49 8d 76 40          	lea    0x40(%r14),%rsi
   29809:	41 83 7e 40 ff       	cmpl   $0xffffffff,0x40(%r14)
   2980e:	0f 85 ec 00 00 00    	jne    29900 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x2a0>
   29814:	48 8b 69 10          	mov    0x10(%rcx),%rbp
   29818:	41 bc 02 00 00 00    	mov    $0x2,%r12d
   2981e:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   29824:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   2982b:	e9 e9 0d 00 00       	jmp    2a619 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb9>
   29830:	49 8b 86 a0 00 00 00 	mov    0xa0(%r14),%rax
   29837:	49 89 86 98 00 00 00 	mov    %rax,0x98(%r14)
   2983e:	0f b6 01             	movzbl (%rcx),%eax
   29841:	48 8b 51 08          	mov    0x8(%rcx),%rdx
   29845:	48 83 fa 02          	cmp    $0x2,%rdx
   29849:	40 0f 92 c6          	setb   %sil
   2984d:	40 84 f0             	test   %sil,%al
   29850:	75 1c                	jne    2986e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x20e>
   29852:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   29859:	00 
   2985a:	0f 84 ef 0f 00 00    	je     2a84f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x11ef>
   29860:	49 8b b6 88 00 00 00 	mov    0x88(%r14),%rsi
   29867:	48 c7 06 02 00 00 00 	movq   $0x2,(%rsi)
   2986e:	48 83 fa 01          	cmp    $0x1,%rdx
   29872:	40 0f 94 c6          	sete   %sil
   29876:	40 84 f0             	test   %sil,%al
   29879:	75 20                	jne    2989b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x23b>
   2987b:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   29882:	48 83 fe 02          	cmp    $0x2,%rsi
   29886:	0f 82 e6 0f 00 00    	jb     2a872 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1212>
   2988c:	49 8b b6 88 00 00 00 	mov    0x88(%r14),%rsi
   29893:	48 c7 46 10 02 00 00 	movq   $0x2,0x10(%rsi)
   2989a:	00 
   2989b:	41 83 3e ff          	cmpl   $0xffffffff,(%r14)
   2989f:	0f 85 41 05 00 00    	jne    29de6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x786>
   298a5:	48 85 d2             	test   %rdx,%rdx
   298a8:	0f 94 c2             	sete   %dl
   298ab:	20 d0                	and    %dl,%al
   298ad:	3c 01                	cmp    $0x1,%al
   298af:	0f 85 a4 04 00 00    	jne    29d59 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x6f9>
   298b5:	48 8b 41 10          	mov    0x10(%rcx),%rax
   298b9:	41 bc 02 00 00 00    	mov    $0x2,%r12d
   298bf:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   298c5:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   298cc:	e9 f6 04 00 00       	jmp    29dc7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x767>
   298d1:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   298d8:	48 83 fe 01          	cmp    $0x1,%rsi
   298dc:	0f 86 7e 0f 00 00    	jbe    2a860 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1200>
   298e2:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   298e9:	48 c7 40 10 02 00 00 	movq   $0x2,0x10(%rax)
   298f0:	00 
   298f1:	49 8d 76 40          	lea    0x40(%r14),%rsi
   298f5:	41 83 7e 40 ff       	cmpl   $0xffffffff,0x40(%r14)
   298fa:	0f 84 b7 03 00 00    	je     29cb7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x657>
   29900:	4c 89 ff             	mov    %r15,%rdi
   29903:	ff 15 ff b0 09 00    	call   *0x9b0ff(%rip)        # c4a08 <_DYNAMIC+0x2f8>
   29909:	48 83 f8 01          	cmp    $0x1,%rax
   2990d:	0f 85 39 0d 00 00    	jne    2a64c <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfec>
   29913:	49 39 96 b0 00 00 00 	cmp    %rdx,0xb0(%r14)
   2991a:	0f 84 3f 0d 00 00    	je     2a65f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfff>
   29920:	49 8b 87 28 01 00 00 	mov    0x128(%r15),%rax
   29927:	48 85 c0             	test   %rax,%rax
   2992a:	0f 84 2f 0d 00 00    	je     2a65f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfff>
   29930:	49 8b 8f 30 01 00 00 	mov    0x130(%r15),%rcx
   29937:	44 0f b7 80 c2 01 00 	movzwl 0x1c2(%rax),%r8d
   2993e:	00 
   2993f:	45 89 c1             	mov    %r8d,%r9d
   29942:	41 c1 e1 05          	shl    $0x5,%r9d
   29946:	48 c7 c7 ff ff ff ff 	mov    $0xffffffffffffffff,%rdi
   2994d:	31 f6                	xor    %esi,%esi
   2994f:	90                   	nop
   29950:	49 39 f1             	cmp    %rsi,%r9
   29953:	74 2b                	je     29980 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x320>
   29955:	48 3b 94 f8 70 01 00 	cmp    0x170(%rax,%rdi,8),%rdx
   2995c:	00 
   2995d:	41 0f 97 c2          	seta   %r10b
   29961:	41 80 da 00          	sbb    $0x0,%r10b
   29965:	48 83 c6 20          	add    $0x20,%rsi
   29969:	48 ff c7             	inc    %rdi
   2996c:	41 80 fa 01          	cmp    $0x1,%r10b
   29970:	74 de                	je     29950 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x2f0>
   29972:	45 0f b6 c2          	movzbl %r10b,%r8d
   29976:	45 85 c0             	test   %r8d,%r8d
   29979:	75 08                	jne    29983 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x323>
   2997b:	e9 67 05 00 00       	jmp    29ee7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x887>
   29980:	4c 89 c7             	mov    %r8,%rdi
   29983:	48 83 e9 01          	sub    $0x1,%rcx
   29987:	0f 82 d2 0c 00 00    	jb     2a65f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfff>
   2998d:	48 8b 84 f8 c8 01 00 	mov    0x1c8(%rax,%rdi,8),%rax
   29994:	00 
   29995:	eb a0                	jmp    29937 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x2d7>
   29997:	4c 89 e7             	mov    %r12,%rdi
   2999a:	ff 15 68 af 09 00    	call   *0x9af68(%rip)        # c4908 <free@GLIBC_2.2.5>
   299a0:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   299a7:	48 83 fe 02          	cmp    $0x2,%rsi
   299ab:	0f 82 4a 0e 00 00    	jb     2a7fb <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x119b>
   299b1:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   299b8:	48 8b 68 10          	mov    0x10(%rax),%rbp
   299bc:	48 83 fd 02          	cmp    $0x2,%rbp
   299c0:	48 89 5c 24 78       	mov    %rbx,0x78(%rsp)
   299c5:	75 45                	jne    29a0c <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x3ac>
   299c7:	4d 8b a6 98 00 00 00 	mov    0x98(%r14),%r12
   299ce:	49 8d 4c 24 01       	lea    0x1(%r12),%rcx
   299d3:	49 89 8e 98 00 00 00 	mov    %rcx,0x98(%r14)
   299da:	48 c7 40 10 00 00 00 	movq   $0x0,0x10(%rax)
   299e1:	00 
   299e2:	4c 89 60 18          	mov    %r12,0x18(%rax)
   299e6:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   299ed:	00 
   299ee:	0f 84 90 0e 00 00    	je     2a884 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1224>
   299f4:	31 ed                	xor    %ebp,%ebp
   299f6:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   299fd:	48 8b 18             	mov    (%rax),%rbx
   29a00:	48 83 fb 02          	cmp    $0x2,%rbx
   29a04:	74 1a                	je     29a20 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x3c0>
   29a06:	4c 8b 68 08          	mov    0x8(%rax),%r13
   29a0a:	eb 33                	jmp    29a3f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x3df>
   29a0c:	4c 8b 60 18          	mov    0x18(%rax),%r12
   29a10:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   29a17:	48 8b 18             	mov    (%rax),%rbx
   29a1a:	48 83 fb 02          	cmp    $0x2,%rbx
   29a1e:	75 e6                	jne    29a06 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x3a6>
   29a20:	4d 8b ae 98 00 00 00 	mov    0x98(%r14),%r13
   29a27:	49 8d 4d 01          	lea    0x1(%r13),%rcx
   29a2b:	49 89 8e 98 00 00 00 	mov    %rcx,0x98(%r14)
   29a32:	48 c7 00 00 00 00 00 	movq   $0x0,(%rax)
   29a39:	4c 89 68 08          	mov    %r13,0x8(%rax)
   29a3d:	31 db                	xor    %ebx,%ebx
   29a3f:	bf 10 00 00 00       	mov    $0x10,%edi
   29a44:	ff 15 ae ae 09 00    	call   *0x9aeae(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29a4a:	48 85 c0             	test   %rax,%rax
   29a4d:	0f 84 5e 0d 00 00    	je     2a7b1 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1151>
   29a53:	48 89 18             	mov    %rbx,(%rax)
   29a56:	4c 89 68 08          	mov    %r13,0x8(%rax)
   29a5a:	48 c7 44 24 48 01 00 	movq   $0x1,0x48(%rsp)
   29a61:	00 00 
   29a63:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
   29a68:	48 c7 44 24 58 01 00 	movq   $0x1,0x58(%rsp)
   29a6f:	00 00 
   29a71:	49 8d 7f 10          	lea    0x10(%r15),%rdi
   29a75:	4d 8d 87 c8 01 00 00 	lea    0x1c8(%r15),%r8
   29a7c:	48 8d 35 85 f6 fd ff 	lea    -0x2097b(%rip),%rsi        # 9108 <anon.ee651107ab5319c6bc273e1a29320aaf.1.llvm.14746981713632465754+0x128>
   29a83:	48 8d 4c 24 48       	lea    0x48(%rsp),%rcx
   29a88:	ba 04 00 00 00       	mov    $0x4,%edx
   29a8d:	ff 15 8d af 09 00    	call   *0x9af8d(%rip)        # c4a20 <_DYNAMIC+0x310>
   29a93:	48 89 ac 24 88 00 00 	mov    %rbp,0x88(%rsp)
   29a9a:	00 
   29a9b:	4c 89 a4 24 90 00 00 	mov    %r12,0x90(%rsp)
   29aa2:	00 
   29aa3:	48 89 84 24 98 00 00 	mov    %rax,0x98(%rsp)
   29aaa:	00 
   29aab:	48 89 94 24 a0 00 00 	mov    %rdx,0xa0(%rsp)
   29ab2:	00 
   29ab3:	48 c7 84 24 80 00 00 	movq   $0x1,0x80(%rsp)
   29aba:	00 01 00 00 00 
   29abf:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   29ac6:	00 
   29ac7:	0f 84 40 0d 00 00    	je     2a80d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x11ad>
   29acd:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   29ad4:	48 8b 28             	mov    (%rax),%rbp
   29ad7:	48 83 fd 02          	cmp    $0x2,%rbp
   29adb:	75 22                	jne    29aff <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x49f>
   29add:	4d 8b a6 98 00 00 00 	mov    0x98(%r14),%r12
   29ae4:	49 8d 4c 24 01       	lea    0x1(%r12),%rcx
   29ae9:	49 89 8e 98 00 00 00 	mov    %rcx,0x98(%r14)
   29af0:	48 c7 00 00 00 00 00 	movq   $0x0,(%rax)
   29af7:	4c 89 60 08          	mov    %r12,0x8(%rax)
   29afb:	31 ed                	xor    %ebp,%ebp
   29afd:	eb 04                	jmp    29b03 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x4a3>
   29aff:	4c 8b 60 08          	mov    0x8(%rax),%r12
   29b03:	49 8b 87 08 01 00 00 	mov    0x108(%r15),%rax
   29b0a:	48 8b 70 20          	mov    0x20(%rax),%rsi
   29b0e:	48 83 fe 02          	cmp    $0x2,%rsi
   29b12:	0f 82 fe 0c 00 00    	jb     2a816 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x11b6>
   29b18:	48 8b 40 18          	mov    0x18(%rax),%rax
   29b1c:	48 83 b8 f8 00 00 00 	cmpq   $0x0,0xf8(%rax)
   29b23:	00 
   29b24:	0f 84 fa 0c 00 00    	je     2a824 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x11c4>
   29b2a:	48 8b 80 f0 00 00 00 	mov    0xf0(%rax),%rax
   29b31:	48 8b 18             	mov    (%rax),%rbx
   29b34:	bf 10 00 00 00       	mov    $0x10,%edi
   29b39:	ff 15 b9 ad 09 00    	call   *0x9adb9(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29b3f:	48 85 c0             	test   %rax,%rax
   29b42:	0f 84 79 0c 00 00    	je     2a7c1 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1161>
   29b48:	48 89 28             	mov    %rbp,(%rax)
   29b4b:	4c 89 60 08          	mov    %r12,0x8(%rax)
   29b4f:	48 89 5c 24 28       	mov    %rbx,0x28(%rsp)
   29b54:	48 c7 44 24 30 01 00 	movq   $0x1,0x30(%rsp)
   29b5b:	00 00 
   29b5d:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   29b62:	48 c7 44 24 40 01 00 	movq   $0x1,0x40(%rsp)
   29b69:	00 00 
   29b6b:	48 c7 44 24 20 00 00 	movq   $0x0,0x20(%rsp)
   29b72:	00 00 
   29b74:	bf 50 00 00 00       	mov    $0x50,%edi
   29b79:	ff 15 79 ad 09 00    	call   *0x9ad79(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29b7f:	48 85 c0             	test   %rax,%rax
   29b82:	0f 84 4e 0c 00 00    	je     2a7d6 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1176>
   29b88:	48 8b 8c 24 a0 00 00 	mov    0xa0(%rsp),%rcx
   29b8f:	00 
   29b90:	48 89 48 20          	mov    %rcx,0x20(%rax)
   29b94:	0f 10 84 24 80 00 00 	movups 0x80(%rsp),%xmm0
   29b9b:	00 
   29b9c:	0f 10 8c 24 90 00 00 	movups 0x90(%rsp),%xmm1
   29ba3:	00 
   29ba4:	0f 11 48 10          	movups %xmm1,0x10(%rax)
   29ba8:	0f 11 00             	movups %xmm0,(%rax)
   29bab:	0f 10 44 24 20       	movups 0x20(%rsp),%xmm0
   29bb0:	0f 10 4c 24 30       	movups 0x30(%rsp),%xmm1
   29bb5:	0f 11 40 28          	movups %xmm0,0x28(%rax)
   29bb9:	0f 11 48 38          	movups %xmm1,0x38(%rax)
   29bbd:	48 8b 4c 24 40       	mov    0x40(%rsp),%rcx
   29bc2:	48 89 48 48          	mov    %rcx,0x48(%rax)
   29bc6:	48 c7 44 24 50 02 00 	movq   $0x2,0x50(%rsp)
   29bcd:	00 00 
   29bcf:	48 89 44 24 58       	mov    %rax,0x58(%rsp)
   29bd4:	48 c7 44 24 60 02 00 	movq   $0x2,0x60(%rsp)
   29bdb:	00 00 
   29bdd:	48 c7 44 24 48 02 00 	movq   $0x2,0x48(%rsp)
   29be4:	00 00 
   29be6:	bf 10 00 00 00       	mov    $0x10,%edi
   29beb:	ff 15 07 ad 09 00    	call   *0x9ad07(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29bf1:	48 85 c0             	test   %rax,%rax
   29bf4:	48 8b 5c 24 78       	mov    0x78(%rsp),%rbx
   29bf9:	49 8d 8e b0 00 00 00 	lea    0xb0(%r14),%rcx
   29c00:	0f 84 34 0c 00 00    	je     2a83a <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x11da>
   29c06:	0f 10 01             	movups (%rcx),%xmm0
   29c09:	0f 11 00             	movups %xmm0,(%rax)
   29c0c:	48 8b 4c 24 68       	mov    0x68(%rsp),%rcx
   29c11:	48 89 4b 20          	mov    %rcx,0x20(%rbx)
   29c15:	0f 10 44 24 48       	movups 0x48(%rsp),%xmm0
   29c1a:	0f 10 4c 24 58       	movups 0x58(%rsp),%xmm1
   29c1f:	0f 11 4b 10          	movups %xmm1,0x10(%rbx)
   29c23:	0f 11 03             	movups %xmm0,(%rbx)
   29c26:	49 8b 8e 98 00 00 00 	mov    0x98(%r14),%rcx
   29c2d:	48 c7 43 28 02 00 00 	movq   $0x2,0x28(%rbx)
   29c34:	00 
   29c35:	48 89 43 30          	mov    %rax,0x30(%rbx)
   29c39:	48 c7 43 38 02 00 00 	movq   $0x2,0x38(%rbx)
   29c40:	00 
   29c41:	48 c7 43 40 01 00 00 	movq   $0x1,0x40(%rbx)
   29c48:	00 
   29c49:	48 89 4b 48          	mov    %rcx,0x48(%rbx)
   29c4d:	e9 14 0a 00 00       	jmp    2a666 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1006>
   29c52:	4c 8b 64 30 f0       	mov    -0x10(%rax,%rsi,1),%r12
   29c57:	4d 89 e5             	mov    %r12,%r13
   29c5a:	49 c1 e5 04          	shl    $0x4,%r13
   29c5e:	4c 89 e1             	mov    %r12,%rcx
   29c61:	48 c1 e9 3c          	shr    $0x3c,%rcx
   29c65:	0f 95 c1             	setne  %cl
   29c68:	48 ba f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rdx
   29c6f:	ff ff 7f 
   29c72:	49 39 d5             	cmp    %rdx,%r13
   29c75:	0f 97 c2             	seta   %dl
   29c78:	08 ca                	or     %cl,%dl
   29c7a:	0f 85 93 02 00 00    	jne    29f13 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8b3>
   29c80:	48 8b 74 30 e8       	mov    -0x18(%rax,%rsi,1),%rsi
   29c85:	4d 85 ed             	test   %r13,%r13
   29c88:	0f 84 df 02 00 00    	je     29f6d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x90d>
   29c8e:	48 89 74 24 08       	mov    %rsi,0x8(%rsp)
   29c93:	4c 89 ef             	mov    %r13,%rdi
   29c96:	ff 15 5c ac 09 00    	call   *0x9ac5c(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29c9c:	48 85 c0             	test   %rax,%rax
   29c9f:	0f 84 f0 0b 00 00    	je     2a895 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1235>
   29ca5:	48 89 c7             	mov    %rax,%rdi
   29ca8:	4c 89 64 24 10       	mov    %r12,0x10(%rsp)
   29cad:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
   29cb2:	e9 c4 02 00 00       	jmp    29f7b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x91b>
   29cb7:	49 8b 8f 08 01 00 00 	mov    0x108(%r15),%rcx
   29cbe:	48 8b 41 20          	mov    0x20(%rcx),%rax
   29cc2:	48 83 f8 02          	cmp    $0x2,%rax
   29cc6:	0f 82 d7 0b 00 00    	jb     2a8a3 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1243>
   29ccc:	48 8b 49 18          	mov    0x18(%rcx),%rcx
   29cd0:	48 8b 81 c8 00 00 00 	mov    0xc8(%rcx),%rax
   29cd7:	48 83 f8 01          	cmp    $0x1,%rax
   29cdb:	0f 86 e9 0b 00 00    	jbe    2a8ca <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x126a>
   29ce1:	48 8b 81 c0 00 00 00 	mov    0xc0(%rcx),%rax
   29ce8:	48 8b 78 38          	mov    0x38(%rax),%rdi
   29cec:	41 80 bf f9 03 00 00 	cmpb   $0x0,0x3f9(%r15)
   29cf3:	00 
   29cf4:	0f 84 24 02 00 00    	je     29f1e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8be>
   29cfa:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   29d01:	00 
   29d02:	0f 84 4b 0c 00 00    	je     2a953 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12f3>
   29d08:	49 8b 8e 88 00 00 00 	mov    0x88(%r14),%rcx
   29d0f:	48 8b 01             	mov    (%rcx),%rax
   29d12:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   29d19:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   29d1f:	48 83 f8 02          	cmp    $0x2,%rax
   29d23:	0f 85 2a 05 00 00    	jne    2a253 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xbf3>
   29d29:	45 31 e4             	xor    %r12d,%r12d
   29d2c:	49 8b 86 90 00 00 00 	mov    0x90(%r14),%rax
   29d33:	48 83 f8 02          	cmp    $0x2,%rax
   29d37:	0f 82 a5 08 00 00    	jb     2a5e2 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf82>
   29d3d:	49 8b 8e 88 00 00 00 	mov    0x88(%r14),%rcx
   29d44:	48 8b 41 10          	mov    0x10(%rcx),%rax
   29d48:	48 83 f8 02          	cmp    $0x2,%rax
   29d4c:	0f 85 fd 05 00 00    	jne    2a34f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xcef>
   29d52:	31 ed                	xor    %ebp,%ebp
   29d54:	e9 bb 08 00 00       	jmp    2a614 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb4>
   29d59:	49 8b 87 08 01 00 00 	mov    0x108(%r15),%rax
   29d60:	48 8b 70 20          	mov    0x20(%rax),%rsi
   29d64:	48 83 fe 02          	cmp    $0x2,%rsi
   29d68:	0f 82 4a 0b 00 00    	jb     2a8b8 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1258>
   29d6e:	48 8b 40 18          	mov    0x18(%rax),%rax
   29d72:	48 83 b8 c8 00 00 00 	cmpq   $0x0,0xc8(%rax)
   29d79:	00 
   29d7a:	0f 84 5f 0b 00 00    	je     2a8df <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x127f>
   29d80:	48 8b 80 c0 00 00 00 	mov    0xc0(%rax),%rax
   29d87:	48 8b 48 18          	mov    0x18(%rax),%rcx
   29d8b:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   29d91:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   29d98:	41 80 bf f9 03 00 00 	cmpb   $0x0,0x3f9(%r15)
   29d9f:	00 
   29da0:	74 22                	je     29dc4 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x764>
   29da2:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   29da9:	00 
   29daa:	0f 84 b4 0b 00 00    	je     2a964 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1304>
   29db0:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   29db7:	48 8b 30             	mov    (%rax),%rsi
   29dba:	48 83 fe 02          	cmp    $0x2,%rsi
   29dbe:	0f 85 17 05 00 00    	jne    2a2db <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xc7b>
   29dc4:	45 31 e4             	xor    %r12d,%r12d
   29dc7:	4d 89 26             	mov    %r12,(%r14)
   29dca:	4d 89 6e 08          	mov    %r13,0x8(%r14)
   29dce:	49 89 46 10          	mov    %rax,0x10(%r14)
   29dd2:	49 89 56 18          	mov    %rdx,0x18(%r14)
   29dd6:	49 c7 46 20 00 00 00 	movq   $0x0,0x20(%r14)
   29ddd:	00 
   29dde:	49 89 4e 30          	mov    %rcx,0x30(%r14)
   29de2:	49 89 6e 38          	mov    %rbp,0x38(%r14)
   29de6:	4c 89 ff             	mov    %r15,%rdi
   29de9:	4c 89 f6             	mov    %r14,%rsi
   29dec:	ff 15 16 ac 09 00    	call   *0x9ac16(%rip)        # c4a08 <_DYNAMIC+0x2f8>
   29df2:	a8 01                	test   $0x1,%al
   29df4:	74 7e                	je     29e74 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x814>
   29df6:	49 8b 87 28 01 00 00 	mov    0x128(%r15),%rax
   29dfd:	48 85 c0             	test   %rax,%rax
   29e00:	0f 84 59 08 00 00    	je     2a65f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfff>
   29e06:	49 8b 8f 30 01 00 00 	mov    0x130(%r15),%rcx
   29e0d:	44 0f b7 80 c2 01 00 	movzwl 0x1c2(%rax),%r8d
   29e14:	00 
   29e15:	45 89 c1             	mov    %r8d,%r9d
   29e18:	41 c1 e1 05          	shl    $0x5,%r9d
   29e1c:	48 c7 c7 ff ff ff ff 	mov    $0xffffffffffffffff,%rdi
   29e23:	31 f6                	xor    %esi,%esi
   29e25:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   29e2c:	00 00 00 00 
   29e30:	49 39 f1             	cmp    %rsi,%r9
   29e33:	74 28                	je     29e5d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7fd>
   29e35:	48 3b 94 f8 70 01 00 	cmp    0x170(%rax,%rdi,8),%rdx
   29e3c:	00 
   29e3d:	41 0f 97 c2          	seta   %r10b
   29e41:	41 80 da 00          	sbb    $0x0,%r10b
   29e45:	48 83 c6 20          	add    $0x20,%rsi
   29e49:	48 ff c7             	inc    %rdi
   29e4c:	41 80 fa 01          	cmp    $0x1,%r10b
   29e50:	74 de                	je     29e30 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7d0>
   29e52:	45 0f b6 c2          	movzbl %r10b,%r8d
   29e56:	45 85 c0             	test   %r8d,%r8d
   29e59:	75 05                	jne    29e60 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x800>
   29e5b:	eb 2a                	jmp    29e87 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x827>
   29e5d:	4c 89 c7             	mov    %r8,%rdi
   29e60:	48 83 e9 01          	sub    $0x1,%rcx
   29e64:	0f 82 f5 07 00 00    	jb     2a65f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfff>
   29e6a:	48 8b 84 f8 c8 01 00 	mov    0x1c8(%rax,%rdi,8),%rax
   29e71:	00 
   29e72:	eb 99                	jmp    29e0d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x7ad>
   29e74:	49 c7 06 ff ff ff ff 	movq   $0xffffffffffffffff,(%r14)
   29e7b:	48 c7 03 07 00 00 00 	movq   $0x7,(%rbx)
   29e82:	e9 df 07 00 00       	jmp    2a666 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1006>
   29e87:	4c 8b 64 30 f0       	mov    -0x10(%rax,%rsi,1),%r12
   29e8c:	4d 89 e5             	mov    %r12,%r13
   29e8f:	49 c1 e5 04          	shl    $0x4,%r13
   29e93:	4c 89 e1             	mov    %r12,%rcx
   29e96:	48 c1 e9 3c          	shr    $0x3c,%rcx
   29e9a:	0f 95 c1             	setne  %cl
   29e9d:	48 bf f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rdi
   29ea4:	ff ff 7f 
   29ea7:	49 39 fd             	cmp    %rdi,%r13
   29eaa:	40 0f 97 c7          	seta   %dil
   29eae:	40 08 cf             	or     %cl,%dil
   29eb1:	75 60                	jne    29f13 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8b3>
   29eb3:	48 89 54 24 08       	mov    %rdx,0x8(%rsp)
   29eb8:	48 8b 6c 30 e8       	mov    -0x18(%rax,%rsi,1),%rbp
   29ebd:	4d 85 ed             	test   %r13,%r13
   29ec0:	0f 84 74 02 00 00    	je     2a13a <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xada>
   29ec6:	4c 89 ef             	mov    %r13,%rdi
   29ec9:	ff 15 29 aa 09 00    	call   *0x9aa29(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29ecf:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   29ed4:	48 85 c0             	test   %rax,%rax
   29ed7:	0f 84 b8 09 00 00    	je     2a895 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1235>
   29edd:	4c 89 64 24 18       	mov    %r12,0x18(%rsp)
   29ee2:	e9 66 02 00 00       	jmp    2a14d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xaed>
   29ee7:	4c 8b 64 30 f0       	mov    -0x10(%rax,%rsi,1),%r12
   29eec:	4d 89 e5             	mov    %r12,%r13
   29eef:	49 c1 e5 04          	shl    $0x4,%r13
   29ef3:	4c 89 e1             	mov    %r12,%rcx
   29ef6:	48 c1 e9 3c          	shr    $0x3c,%rcx
   29efa:	0f 95 c1             	setne  %cl
   29efd:	48 bf f8 ff ff ff ff 	movabs $0x7ffffffffffffff8,%rdi
   29f04:	ff ff 7f 
   29f07:	49 39 fd             	cmp    %rdi,%r13
   29f0a:	40 0f 97 c7          	seta   %dil
   29f0e:	40 08 cf             	or     %cl,%dil
   29f11:	74 20                	je     29f33 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x8d3>
   29f13:	31 ff                	xor    %edi,%edi
   29f15:	4c 89 ee             	mov    %r13,%rsi
   29f18:	ff 15 22 aa 09 00    	call   *0x9aa22(%rip)        # c4940 <_DYNAMIC+0x230>
   29f1e:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   29f24:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   29f2b:	45 31 e4             	xor    %r12d,%r12d
   29f2e:	e9 e6 06 00 00       	jmp    2a619 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb9>
   29f33:	48 8b 74 30 e8       	mov    -0x18(%rax,%rsi,1),%rsi
   29f38:	4d 85 ed             	test   %r13,%r13
   29f3b:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
   29f40:	0f 84 9c 04 00 00    	je     2a3e2 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xd82>
   29f46:	48 89 f5             	mov    %rsi,%rbp
   29f49:	4c 89 ef             	mov    %r13,%rdi
   29f4c:	ff 15 a6 a9 09 00    	call   *0x9a9a6(%rip)        # c48f8 <malloc@GLIBC_2.2.5>
   29f52:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   29f57:	48 85 c0             	test   %rax,%rax
   29f5a:	0f 84 35 09 00 00    	je     2a895 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1235>
   29f60:	4c 89 64 24 18       	mov    %r12,0x18(%rsp)
   29f65:	48 89 ee             	mov    %rbp,%rsi
   29f68:	e9 88 04 00 00       	jmp    2a3f5 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xd95>
   29f6d:	bf 08 00 00 00       	mov    $0x8,%edi
   29f72:	48 c7 44 24 10 00 00 	movq   $0x0,0x10(%rsp)
   29f79:	00 00 
   29f7b:	4d 85 e4             	test   %r12,%r12
   29f7e:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   29f83:	0f 84 67 09 00 00    	je     2a8f0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1290>
   29f89:	4c 89 ea             	mov    %r13,%rdx
   29f8c:	49 89 fd             	mov    %rdi,%r13
   29f8f:	ff 15 83 a9 09 00    	call   *0x9a983(%rip)        # c4918 <memcpy@GLIBC_2.14>
   29f95:	48 83 fd 01          	cmp    $0x1,%rbp
   29f99:	0f 85 64 09 00 00    	jne    2a903 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12a3>
   29f9f:	4d 8b 45 08          	mov    0x8(%r13),%r8
   29fa3:	4d 8d 8f 20 01 00 00 	lea    0x120(%r15),%r9
   29faa:	b9 01 00 00 00       	mov    $0x1,%ecx
   29faf:	41 80 7d 00 00       	cmpb   $0x0,0x0(%r13)
   29fb4:	75 59                	jne    2a00f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x9af>
   29fb6:	49 8b 01             	mov    (%r9),%rax
   29fb9:	48 85 c0             	test   %rax,%rax
   29fbc:	74 4f                	je     2a00d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x9ad>
   29fbe:	48 89 c2             	mov    %rax,%rdx
   29fc1:	eb 1b                	jmp    29fde <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x97e>
   29fc3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   29fca:	84 00 00 00 00 00 
   29fd0:	be 38 00 00 00       	mov    $0x38,%esi
   29fd5:	48 8b 14 32          	mov    (%rdx,%rsi,1),%rdx
   29fd9:	48 85 d2             	test   %rdx,%rdx
   29fdc:	74 2f                	je     2a00d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x9ad>
   29fde:	4c 3b 42 20          	cmp    0x20(%rdx),%r8
   29fe2:	40 0f 97 c6          	seta   %sil
   29fe6:	40 80 de 00          	sbb    $0x0,%sil
   29fea:	40 80 fe 01          	cmp    $0x1,%sil
   29fee:	74 e0                	je     29fd0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x970>
   29ff0:	40 0f b6 fe          	movzbl %sil,%edi
   29ff4:	be 30 00 00 00       	mov    $0x30,%esi
   29ff9:	81 ff ff 00 00 00    	cmp    $0xff,%edi
   29fff:	74 d4                	je     29fd5 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x975>
   2a001:	4c 8b 42 18          	mov    0x18(%rdx),%r8
   2a005:	83 7a 10 01          	cmpl   $0x1,0x10(%rdx)
   2a009:	75 b3                	jne    29fbe <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x95e>
   2a00b:	eb 02                	jmp    2a00f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x9af>
   2a00d:	31 c9                	xor    %ecx,%ecx
   2a00f:	48 8d 15 ba 7c 09 00 	lea    0x97cba(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   2a016:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   2a01d:	00 
   2a01e:	0f 84 d9 08 00 00    	je     2a8fd <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x129d>
   2a024:	49 8d af c8 01 00 00 	lea    0x1c8(%r15),%rbp
   2a02b:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   2a032:	48 8b 30             	mov    (%rax),%rsi
   2a035:	48 83 fe 02          	cmp    $0x2,%rsi
   2a039:	75 09                	jne    2a044 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x9e4>
   2a03b:	48 89 08             	mov    %rcx,(%rax)
   2a03e:	4c 89 40 08          	mov    %r8,0x8(%rax)
   2a042:	eb 2c                	jmp    2a070 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa10>
   2a044:	48 8b 50 08          	mov    0x8(%rax),%rdx
   2a048:	49 8d 7f 10          	lea    0x10(%r15),%rdi
   2a04c:	48 89 2c 24          	mov    %rbp,(%rsp)
   2a050:	4d 89 cd             	mov    %r9,%r13
   2a053:	ff 15 b7 a9 09 00    	call   *0x9a9b7(%rip)        # c4a10 <_DYNAMIC+0x300>
   2a059:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a05e:	84 c0                	test   %al,%al
   2a060:	4d 89 e9             	mov    %r13,%r9
   2a063:	48 8d 15 66 7c 09 00 	lea    0x97c66(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   2a06a:	0f 84 c2 00 00 00    	je     2a132 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xad2>
   2a070:	49 83 fc 01          	cmp    $0x1,%r12
   2a074:	0f 84 c0 08 00 00    	je     2a93a <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12da>
   2a07a:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   2a07f:	4c 8b 40 18          	mov    0x18(%rax),%r8
   2a083:	bf 01 00 00 00       	mov    $0x1,%edi
   2a088:	80 78 10 00          	cmpb   $0x0,0x10(%rax)
   2a08c:	74 07                	je     2a095 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa35>
   2a08e:	b9 01 00 00 00       	mov    $0x1,%ecx
   2a093:	eb 4e                	jmp    2a0e3 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa83>
   2a095:	49 8b 01             	mov    (%r9),%rax
   2a098:	31 c9                	xor    %ecx,%ecx
   2a09a:	48 85 c0             	test   %rax,%rax
   2a09d:	74 44                	je     2a0e3 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa83>
   2a09f:	49 89 c3             	mov    %rax,%r11
   2a0a2:	eb 0e                	jmp    2a0b2 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa52>
   2a0a4:	be 38 00 00 00       	mov    $0x38,%esi
   2a0a9:	4d 8b 1c 33          	mov    (%r11,%rsi,1),%r11
   2a0ad:	4d 85 db             	test   %r11,%r11
   2a0b0:	74 31                	je     2a0e3 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa83>
   2a0b2:	4d 3b 43 20          	cmp    0x20(%r11),%r8
   2a0b6:	40 0f 97 c6          	seta   %sil
   2a0ba:	40 80 de 00          	sbb    $0x0,%sil
   2a0be:	40 80 fe 01          	cmp    $0x1,%sil
   2a0c2:	74 e0                	je     2a0a4 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa44>
   2a0c4:	44 0f b6 d6          	movzbl %sil,%r10d
   2a0c8:	be 30 00 00 00       	mov    $0x30,%esi
   2a0cd:	41 81 fa ff 00 00 00 	cmp    $0xff,%r10d
   2a0d4:	74 d3                	je     2a0a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa49>
   2a0d6:	4d 8b 43 18          	mov    0x18(%r11),%r8
   2a0da:	41 83 7b 10 01       	cmpl   $0x1,0x10(%r11)
   2a0df:	75 be                	jne    2a09f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa3f>
   2a0e1:	eb ab                	jmp    2a08e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xa2e>
   2a0e3:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   2a0ea:	48 83 fe 02          	cmp    $0x2,%rsi
   2a0ee:	0f 82 57 08 00 00    	jb     2a94b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12eb>
   2a0f4:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   2a0fb:	48 8b 70 10          	mov    0x10(%rax),%rsi
   2a0ff:	48 83 fe 02          	cmp    $0x2,%rsi
   2a103:	75 0a                	jne    2a10f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xaaf>
   2a105:	48 89 48 10          	mov    %rcx,0x10(%rax)
   2a109:	4c 89 40 18          	mov    %r8,0x18(%rax)
   2a10d:	eb 1e                	jmp    2a12d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xacd>
   2a10f:	48 8b 50 18          	mov    0x18(%rax),%rdx
   2a113:	49 83 c7 10          	add    $0x10,%r15
   2a117:	48 89 2c 24          	mov    %rbp,(%rsp)
   2a11b:	4c 89 ff             	mov    %r15,%rdi
   2a11e:	ff 15 ec a8 09 00    	call   *0x9a8ec(%rip)        # c4a10 <_DYNAMIC+0x300>
   2a124:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a129:	84 c0                	test   %al,%al
   2a12b:	74 05                	je     2a132 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xad2>
   2a12d:	b9 06 00 00 00       	mov    $0x6,%ecx
   2a132:	48 89 0b             	mov    %rcx,(%rbx)
   2a135:	e9 76 04 00 00       	jmp    2a5b0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf50>
   2a13a:	b8 08 00 00 00       	mov    $0x8,%eax
   2a13f:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   2a144:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
   2a14b:	00 00 
   2a14d:	4d 85 e4             	test   %r12,%r12
   2a150:	0f 84 c8 07 00 00    	je     2a91e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12be>
   2a156:	4c 8b 64 24 10       	mov    0x10(%rsp),%r12
   2a15b:	4c 89 e7             	mov    %r12,%rdi
   2a15e:	48 89 ee             	mov    %rbp,%rsi
   2a161:	4c 89 ea             	mov    %r13,%rdx
   2a164:	ff 15 ae a7 09 00    	call   *0x9a7ae(%rip)        # c4918 <memcpy@GLIBC_2.14>
   2a16a:	4d 8b 44 24 08       	mov    0x8(%r12),%r8
   2a16f:	4d 8d 8f 20 01 00 00 	lea    0x120(%r15),%r9
   2a176:	b9 01 00 00 00       	mov    $0x1,%ecx
   2a17b:	41 80 3c 24 00       	cmpb   $0x0,(%r12)
   2a180:	74 07                	je     2a189 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb29>
   2a182:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   2a187:	eb 57                	jmp    2a1e0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb80>
   2a189:	49 8b 11             	mov    (%r9),%rdx
   2a18c:	48 85 d2             	test   %rdx,%rdx
   2a18f:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   2a194:	74 48                	je     2a1de <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb7e>
   2a196:	48 89 d6             	mov    %rdx,%rsi
   2a199:	eb 13                	jmp    2a1ae <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb4e>
   2a19b:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   2a1a0:	bf 38 00 00 00       	mov    $0x38,%edi
   2a1a5:	48 8b 34 3e          	mov    (%rsi,%rdi,1),%rsi
   2a1a9:	48 85 f6             	test   %rsi,%rsi
   2a1ac:	74 30                	je     2a1de <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb7e>
   2a1ae:	4c 3b 46 20          	cmp    0x20(%rsi),%r8
   2a1b2:	40 0f 97 c7          	seta   %dil
   2a1b6:	40 80 df 00          	sbb    $0x0,%dil
   2a1ba:	40 80 ff 01          	cmp    $0x1,%dil
   2a1be:	74 e0                	je     2a1a0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb40>
   2a1c0:	44 0f b6 d7          	movzbl %dil,%r10d
   2a1c4:	bf 30 00 00 00       	mov    $0x30,%edi
   2a1c9:	41 81 fa ff 00 00 00 	cmp    $0xff,%r10d
   2a1d0:	74 d3                	je     2a1a5 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb45>
   2a1d2:	4c 8b 46 18          	mov    0x18(%rsi),%r8
   2a1d6:	83 7e 10 01          	cmpl   $0x1,0x10(%rsi)
   2a1da:	75 ba                	jne    2a196 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb36>
   2a1dc:	eb 02                	jmp    2a1e0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xb80>
   2a1de:	31 c9                	xor    %ecx,%ecx
   2a1e0:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   2a1e7:	00 
   2a1e8:	0f 84 39 07 00 00    	je     2a927 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12c7>
   2a1ee:	49 8b 96 88 00 00 00 	mov    0x88(%r14),%rdx
   2a1f5:	48 8b 32             	mov    (%rdx),%rsi
   2a1f8:	48 83 fe 02          	cmp    $0x2,%rsi
   2a1fc:	75 09                	jne    2a207 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xba7>
   2a1fe:	48 89 0a             	mov    %rcx,(%rdx)
   2a201:	4c 89 42 08          	mov    %r8,0x8(%rdx)
   2a205:	eb 25                	jmp    2a22c <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xbcc>
   2a207:	49 8d 87 c8 01 00 00 	lea    0x1c8(%r15),%rax
   2a20e:	48 8b 52 08          	mov    0x8(%rdx),%rdx
   2a212:	49 83 c7 10          	add    $0x10,%r15
   2a216:	48 89 04 24          	mov    %rax,(%rsp)
   2a21a:	4c 89 ff             	mov    %r15,%rdi
   2a21d:	ff 15 ed a7 09 00    	call   *0x9a7ed(%rip)        # c4a10 <_DYNAMIC+0x300>
   2a223:	84 c0                	test   %al,%al
   2a225:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   2a22a:	74 12                	je     2a23e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xbde>
   2a22c:	49 89 86 b0 00 00 00 	mov    %rax,0xb0(%r14)
   2a233:	49 c7 86 a8 00 00 00 	movq   $0x1,0xa8(%r14)
   2a23a:	01 00 00 00 
   2a23e:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   2a245:	4c 89 e7             	mov    %r12,%rdi
   2a248:	ff 15 ba a6 09 00    	call   *0x9a6ba(%rip)        # c4908 <free@GLIBC_2.2.5>
   2a24e:	e9 13 04 00 00       	jmp    2a666 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1006>
   2a253:	48 89 fd             	mov    %rdi,%rbp
   2a256:	48 8b 51 08          	mov    0x8(%rcx),%rdx
   2a25a:	4c 89 ff             	mov    %r15,%rdi
   2a25d:	48 89 c6             	mov    %rax,%rsi
   2a260:	e8 bb 2c 02 00       	call   4cf20 <_RNvMs0_Cs3FgqbjBu8z9_12chr_compiledNtB5_4Core10ground_key.llvm.15908011480901089632>
   2a265:	a8 01                	test   $0x1,%al
   2a267:	0f 84 53 03 00 00    	je     2a5c0 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf60>
   2a26d:	48 89 ef             	mov    %rbp,%rdi
   2a270:	48 89 6c 24 20       	mov    %rbp,0x20(%rsp)
   2a275:	48 c7 44 24 28 00 00 	movq   $0x0,0x28(%rsp)
   2a27c:	00 00 
   2a27e:	48 89 54 24 30       	mov    %rdx,0x30(%rsp)
   2a283:	49 8b 87 c8 03 00 00 	mov    0x3c8(%r15),%rax
   2a28a:	48 85 c0             	test   %rax,%rax
   2a28d:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a291:	74 35                	je     2a2c8 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xc68>
   2a293:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
   2a298:	49 8b 97 d0 03 00 00 	mov    0x3d0(%r15),%rdx
   2a29f:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
   2a2a4:	48 8d 4c 24 20       	lea    0x20(%rsp),%rcx
   2a2a9:	48 89 c6             	mov    %rax,%rsi
   2a2ac:	e8 ff f7 00 00       	call   39ab0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, usize, usize), alloc::collections::btree::set::BTreeSet<u64>, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, usize, usize)>>
   2a2b1:	83 7c 24 48 01       	cmpl   $0x1,0x48(%rsp)
   2a2b6:	0f 85 cb 03 00 00    	jne    2a687 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1027>
   2a2bc:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a2c0:	48 89 ef             	mov    %rbp,%rdi
   2a2c3:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
   2a2c8:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2a2ce:	45 31 c0             	xor    %r8d,%r8d
   2a2d1:	31 ed                	xor    %ebp,%ebp
   2a2d3:	49 89 fd             	mov    %rdi,%r13
   2a2d6:	e9 3e 03 00 00       	jmp    2a619 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb9>
   2a2db:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
   2a2e0:	48 8b 50 08          	mov    0x8(%rax),%rdx
   2a2e4:	4c 89 ff             	mov    %r15,%rdi
   2a2e7:	e8 34 2c 02 00       	call   4cf20 <_RNvMs0_Cs3FgqbjBu8z9_12chr_compiledNtB5_4Core10ground_key.llvm.15908011480901089632>
   2a2ec:	a8 01                	test   $0x1,%al
   2a2ee:	0f 84 03 03 00 00    	je     2a5f7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf97>
   2a2f4:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2a2f9:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   2a2fe:	48 c7 44 24 28 00 00 	movq   $0x0,0x28(%rsp)
   2a305:	00 00 
   2a307:	48 89 54 24 30       	mov    %rdx,0x30(%rsp)
   2a30c:	49 8b b7 c8 03 00 00 	mov    0x3c8(%r15),%rsi
   2a313:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2a319:	48 85 f6             	test   %rsi,%rsi
   2a31c:	0f 84 59 03 00 00    	je     2a67b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x101b>
   2a322:	48 89 54 24 10       	mov    %rdx,0x10(%rsp)
   2a327:	49 8b 97 d0 03 00 00 	mov    0x3d0(%r15),%rdx
   2a32e:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
   2a333:	48 8d 4c 24 20       	lea    0x20(%rsp),%rcx
   2a338:	e8 73 f7 00 00       	call   39ab0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, usize, usize), alloc::collections::btree::set::BTreeSet<u64>, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, usize, usize)>>
   2a33d:	83 7c 24 48 01       	cmpl   $0x1,0x48(%rsp)
   2a342:	0f 85 91 03 00 00    	jne    2a6d9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1079>
   2a348:	31 ed                	xor    %ebp,%ebp
   2a34a:	e9 ac 03 00 00       	jmp    2a6fb <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x109b>
   2a34f:	4c 89 44 24 18       	mov    %r8,0x18(%rsp)
   2a354:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   2a359:	48 8b 51 18          	mov    0x18(%rcx),%rdx
   2a35d:	4c 89 ff             	mov    %r15,%rdi
   2a360:	48 89 c6             	mov    %rax,%rsi
   2a363:	e8 b8 2b 02 00       	call   4cf20 <_RNvMs0_Cs3FgqbjBu8z9_12chr_compiledNtB5_4Core10ground_key.llvm.15908011480901089632>
   2a368:	a8 01                	test   $0x1,%al
   2a36a:	0f 84 94 02 00 00    	je     2a604 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfa4>
   2a370:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   2a375:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
   2a37a:	48 c7 44 24 28 01 00 	movq   $0x1,0x28(%rsp)
   2a381:	00 00 
   2a383:	48 89 54 24 30       	mov    %rdx,0x30(%rsp)
   2a388:	49 8b b7 c8 03 00 00 	mov    0x3c8(%r15),%rsi
   2a38f:	bd 01 00 00 00       	mov    $0x1,%ebp
   2a394:	48 85 f6             	test   %rsi,%rsi
   2a397:	0f 84 72 03 00 00    	je     2a70f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x10af>
   2a39d:	48 89 54 24 70       	mov    %rdx,0x70(%rsp)
   2a3a2:	49 8b 97 d0 03 00 00 	mov    0x3d0(%r15),%rdx
   2a3a9:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
   2a3ae:	48 8d 4c 24 20       	lea    0x20(%rsp),%rcx
   2a3b3:	e8 f8 f6 00 00       	call   39ab0 <<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, (usize, usize, usize), alloc::collections::btree::set::BTreeSet<u64>, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<(usize, usize, usize)>>
   2a3b8:	83 7c 24 48 01       	cmpl   $0x1,0x48(%rsp)
   2a3bd:	0f 85 61 03 00 00    	jne    2a724 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x10c4>
   2a3c3:	45 31 c0             	xor    %r8d,%r8d
   2a3c6:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   2a3cb:	49 89 fd             	mov    %rdi,%r13
   2a3ce:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2a3d4:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a3d8:	48 8b 54 24 70       	mov    0x70(%rsp),%rdx
   2a3dd:	e9 37 02 00 00       	jmp    2a619 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb9>
   2a3e2:	b8 08 00 00 00       	mov    $0x8,%eax
   2a3e7:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   2a3ec:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
   2a3f3:	00 00 
   2a3f5:	4d 85 e4             	test   %r12,%r12
   2a3f8:	0f 84 77 05 00 00    	je     2a975 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1315>
   2a3fe:	48 8b 6c 24 08       	mov    0x8(%rsp),%rbp
   2a403:	48 89 ef             	mov    %rbp,%rdi
   2a406:	4c 89 ea             	mov    %r13,%rdx
   2a409:	ff 15 09 a5 09 00    	call   *0x9a509(%rip)        # c4918 <memcpy@GLIBC_2.14>
   2a40f:	4c 8b 45 08          	mov    0x8(%rbp),%r8
   2a413:	4d 8d af 20 01 00 00 	lea    0x120(%r15),%r13
   2a41a:	b9 01 00 00 00       	mov    $0x1,%ecx
   2a41f:	80 7d 00 00          	cmpb   $0x0,0x0(%rbp)
   2a423:	75 4d                	jne    2a472 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe12>
   2a425:	49 8b 45 00          	mov    0x0(%r13),%rax
   2a429:	48 85 c0             	test   %rax,%rax
   2a42c:	74 42                	je     2a470 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe10>
   2a42e:	48 89 c2             	mov    %rax,%rdx
   2a431:	eb 0e                	jmp    2a441 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xde1>
   2a433:	be 38 00 00 00       	mov    $0x38,%esi
   2a438:	48 8b 14 32          	mov    (%rdx,%rsi,1),%rdx
   2a43c:	48 85 d2             	test   %rdx,%rdx
   2a43f:	74 2f                	je     2a470 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe10>
   2a441:	4c 3b 42 20          	cmp    0x20(%rdx),%r8
   2a445:	40 0f 97 c6          	seta   %sil
   2a449:	40 80 de 00          	sbb    $0x0,%sil
   2a44d:	40 80 fe 01          	cmp    $0x1,%sil
   2a451:	74 e0                	je     2a433 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xdd3>
   2a453:	40 0f b6 fe          	movzbl %sil,%edi
   2a457:	be 30 00 00 00       	mov    $0x30,%esi
   2a45c:	81 ff ff 00 00 00    	cmp    $0xff,%edi
   2a462:	74 d4                	je     2a438 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xdd8>
   2a464:	4c 8b 42 18          	mov    0x18(%rdx),%r8
   2a468:	83 7a 10 01          	cmpl   $0x1,0x10(%rdx)
   2a46c:	75 c0                	jne    2a42e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xdce>
   2a46e:	eb 02                	jmp    2a472 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe12>
   2a470:	31 c9                	xor    %ecx,%ecx
   2a472:	48 8d 15 57 78 09 00 	lea    0x97857(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   2a479:	49 83 be 90 00 00 00 	cmpq   $0x0,0x90(%r14)
   2a480:	00 
   2a481:	0f 84 f5 04 00 00    	je     2a97c <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x131c>
   2a487:	49 8d af c8 01 00 00 	lea    0x1c8(%r15),%rbp
   2a48e:	49 8b 86 88 00 00 00 	mov    0x88(%r14),%rax
   2a495:	48 8b 30             	mov    (%rax),%rsi
   2a498:	48 83 fe 02          	cmp    $0x2,%rsi
   2a49c:	75 0e                	jne    2a4ac <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe4c>
   2a49e:	48 89 08             	mov    %rcx,(%rax)
   2a4a1:	4c 89 40 08          	mov    %r8,0x8(%rax)
   2a4a5:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   2a4aa:	eb 29                	jmp    2a4d5 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe75>
   2a4ac:	48 8b 50 08          	mov    0x8(%rax),%rdx
   2a4b0:	49 8d 7f 10          	lea    0x10(%r15),%rdi
   2a4b4:	48 89 2c 24          	mov    %rbp,(%rsp)
   2a4b8:	4d 89 e9             	mov    %r13,%r9
   2a4bb:	ff 15 4f a5 09 00    	call   *0x9a54f(%rip)        # c4a10 <_DYNAMIC+0x300>
   2a4c1:	84 c0                	test   %al,%al
   2a4c3:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   2a4c8:	48 8d 15 01 78 09 00 	lea    0x97801(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   2a4cf:	0f 84 d4 00 00 00    	je     2a5a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf49>
   2a4d5:	49 83 fc 01          	cmp    $0x1,%r12
   2a4d9:	0f 84 a3 04 00 00    	je     2a982 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1322>
   2a4df:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2a4e4:	4c 8b 41 18          	mov    0x18(%rcx),%r8
   2a4e8:	bf 01 00 00 00       	mov    $0x1,%edi
   2a4ed:	80 79 10 00          	cmpb   $0x0,0x10(%rcx)
   2a4f1:	74 07                	je     2a4fa <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe9a>
   2a4f3:	b9 01 00 00 00       	mov    $0x1,%ecx
   2a4f8:	eb 50                	jmp    2a54a <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xeea>
   2a4fa:	4d 8b 5d 00          	mov    0x0(%r13),%r11
   2a4fe:	31 c9                	xor    %ecx,%ecx
   2a500:	4d 85 db             	test   %r11,%r11
   2a503:	74 45                	je     2a54a <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xeea>
   2a505:	4c 89 de             	mov    %r11,%rsi
   2a508:	eb 0f                	jmp    2a519 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xeb9>
   2a50a:	41 b9 38 00 00 00    	mov    $0x38,%r9d
   2a510:	4a 8b 34 0e          	mov    (%rsi,%r9,1),%rsi
   2a514:	48 85 f6             	test   %rsi,%rsi
   2a517:	74 31                	je     2a54a <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xeea>
   2a519:	4c 3b 46 20          	cmp    0x20(%rsi),%r8
   2a51d:	41 0f 97 c1          	seta   %r9b
   2a521:	41 80 d9 00          	sbb    $0x0,%r9b
   2a525:	41 80 f9 01          	cmp    $0x1,%r9b
   2a529:	74 df                	je     2a50a <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xeaa>
   2a52b:	45 0f b6 d1          	movzbl %r9b,%r10d
   2a52f:	41 b9 30 00 00 00    	mov    $0x30,%r9d
   2a535:	41 81 fa ff 00 00 00 	cmp    $0xff,%r10d
   2a53c:	74 d2                	je     2a510 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xeb0>
   2a53e:	4c 8b 46 18          	mov    0x18(%rsi),%r8
   2a542:	83 7e 10 01          	cmpl   $0x1,0x10(%rsi)
   2a546:	75 bd                	jne    2a505 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xea5>
   2a548:	eb a9                	jmp    2a4f3 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xe93>
   2a54a:	49 8b b6 90 00 00 00 	mov    0x90(%r14),%rsi
   2a551:	48 83 fe 02          	cmp    $0x2,%rsi
   2a555:	0f 82 38 04 00 00    	jb     2a993 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1333>
   2a55b:	49 8b 96 88 00 00 00 	mov    0x88(%r14),%rdx
   2a562:	48 8b 72 10          	mov    0x10(%rdx),%rsi
   2a566:	48 83 fe 02          	cmp    $0x2,%rsi
   2a56a:	75 0a                	jne    2a576 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf16>
   2a56c:	48 89 4a 10          	mov    %rcx,0x10(%rdx)
   2a570:	4c 89 42 18          	mov    %r8,0x18(%rdx)
   2a574:	eb 21                	jmp    2a597 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf37>
   2a576:	48 8b 52 18          	mov    0x18(%rdx),%rdx
   2a57a:	49 83 c7 10          	add    $0x10,%r15
   2a57e:	48 89 2c 24          	mov    %rbp,(%rsp)
   2a582:	4c 89 ff             	mov    %r15,%rdi
   2a585:	4d 89 e9             	mov    %r13,%r9
   2a588:	ff 15 82 a4 09 00    	call   *0x9a482(%rip)        # c4a10 <_DYNAMIC+0x300>
   2a58e:	84 c0                	test   %al,%al
   2a590:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   2a595:	74 12                	je     2a5a9 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf49>
   2a597:	49 89 86 b8 00 00 00 	mov    %rax,0xb8(%r14)
   2a59e:	49 c7 86 a8 00 00 00 	movq   $0x2,0xa8(%r14)
   2a5a5:	02 00 00 00 
   2a5a9:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   2a5b0:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   2a5b5:	ff 15 4d a3 09 00    	call   *0x9a34d(%rip)        # c4908 <free@GLIBC_2.2.5>
   2a5bb:	e9 a6 00 00 00       	jmp    2a666 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1006>
   2a5c0:	45 31 e4             	xor    %r12d,%r12d
   2a5c3:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a5c7:	48 89 ef             	mov    %rbp,%rdi
   2a5ca:	49 c7 c0 ff ff ff ff 	mov    $0xffffffffffffffff,%r8
   2a5d1:	49 8b 86 90 00 00 00 	mov    0x90(%r14),%rax
   2a5d8:	48 83 f8 02          	cmp    $0x2,%rax
   2a5dc:	0f 83 5b f7 ff ff    	jae    29d3d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x6dd>
   2a5e2:	48 8d 15 57 71 09 00 	lea    0x97157(%rip),%rdx        # c1740 <__frame_dummy_init_array_entry+0x400>
   2a5e9:	bf 01 00 00 00       	mov    $0x1,%edi
   2a5ee:	48 89 c6             	mov    %rax,%rsi
   2a5f1:	ff 15 59 a3 09 00    	call   *0x9a359(%rip)        # c4950 <_DYNAMIC+0x240>
   2a5f7:	45 31 e4             	xor    %r12d,%r12d
   2a5fa:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2a5ff:	e9 c3 f7 ff ff       	jmp    29dc7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x767>
   2a604:	31 ed                	xor    %ebp,%ebp
   2a606:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a60a:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   2a60f:	4c 8b 44 24 18       	mov    0x18(%rsp),%r8
   2a614:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
   2a619:	4d 89 66 40          	mov    %r12,0x40(%r14)
   2a61d:	4d 89 6e 48          	mov    %r13,0x48(%r14)
   2a621:	49 89 6e 50          	mov    %rbp,0x50(%r14)
   2a625:	49 89 56 58          	mov    %rdx,0x58(%r14)
   2a629:	49 c7 46 60 00 00 00 	movq   $0x0,0x60(%r14)
   2a630:	00 
   2a631:	49 89 7e 70          	mov    %rdi,0x70(%r14)
   2a635:	4d 89 46 78          	mov    %r8,0x78(%r14)
   2a639:	4c 89 ff             	mov    %r15,%rdi
   2a63c:	ff 15 c6 a3 09 00    	call   *0x9a3c6(%rip)        # c4a08 <_DYNAMIC+0x2f8>
   2a642:	48 83 f8 01          	cmp    $0x1,%rax
   2a646:	0f 84 c7 f2 ff ff    	je     29913 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x2b3>
   2a64c:	49 c7 46 40 ff ff ff 	movq   $0xffffffffffffffff,0x40(%r14)
   2a653:	ff 
   2a654:	49 c7 86 a8 00 00 00 	movq   $0x0,0xa8(%r14)
   2a65b:	00 00 00 00 
   2a65f:	48 c7 03 06 00 00 00 	movq   $0x6,(%rbx)
   2a666:	48 89 d8             	mov    %rbx,%rax
   2a669:	48 81 c4 a8 00 00 00 	add    $0xa8,%rsp
   2a670:	5b                   	pop    %rbx
   2a671:	41 5c                	pop    %r12
   2a673:	41 5d                	pop    %r13
   2a675:	41 5e                	pop    %r14
   2a677:	41 5f                	pop    %r15
   2a679:	5d                   	pop    %rbp
   2a67a:	c3                   	ret    
   2a67b:	31 ed                	xor    %ebp,%ebp
   2a67d:	49 89 cd             	mov    %rcx,%r13
   2a680:	31 c0                	xor    %eax,%eax
   2a682:	e9 40 f7 ff ff       	jmp    29dc7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x767>
   2a687:	48 8b 44 24 50       	mov    0x50(%rsp),%rax
   2a68c:	48 8b 4c 24 60       	mov    0x60(%rsp),%rcx
   2a691:	48 8d 0c 49          	lea    (%rcx,%rcx,2),%rcx
   2a695:	48 8b 84 c8 20 01 00 	mov    0x120(%rax,%rcx,8),%rax
   2a69c:	00 
   2a69d:	48 83 f8 ff          	cmp    $0xffffffffffffffff,%rax
   2a6a1:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a6a5:	0f 84 bc 00 00 00    	je     2a767 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1107>
   2a6ab:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2a6b1:	48 85 c0             	test   %rax,%rax
   2a6b4:	48 89 ef             	mov    %rbp,%rdi
   2a6b7:	0f 84 ce 00 00 00    	je     2a78b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x112b>
   2a6bd:	49 89 fd             	mov    %rdi,%r13
   2a6c0:	49 89 c0             	mov    %rax,%r8
   2a6c3:	49 8b 86 90 00 00 00 	mov    0x90(%r14),%rax
   2a6ca:	48 83 f8 02          	cmp    $0x2,%rax
   2a6ce:	0f 83 69 f6 ff ff    	jae    29d3d <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x6dd>
   2a6d4:	e9 09 ff ff ff       	jmp    2a5e2 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf82>
   2a6d9:	48 8b 44 24 50       	mov    0x50(%rsp),%rax
   2a6de:	48 8b 4c 24 60       	mov    0x60(%rsp),%rcx
   2a6e3:	48 8d 0c 49          	lea    (%rcx,%rcx,2),%rcx
   2a6e7:	48 8b ac c8 20 01 00 	mov    0x120(%rax,%rcx,8),%rbp
   2a6ee:	00 
   2a6ef:	48 83 fd ff          	cmp    $0xffffffffffffffff,%rbp
   2a6f3:	74 7a                	je     2a76f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x110f>
   2a6f5:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2a6fb:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2a700:	49 89 cd             	mov    %rcx,%r13
   2a703:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
   2a708:	31 c0                	xor    %eax,%eax
   2a70a:	e9 b8 f6 ff ff       	jmp    29dc7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x767>
   2a70f:	45 31 c0             	xor    %r8d,%r8d
   2a712:	49 89 fd             	mov    %rdi,%r13
   2a715:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2a71b:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a71f:	e9 f5 fe ff ff       	jmp    2a619 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb9>
   2a724:	48 8b 44 24 50       	mov    0x50(%rsp),%rax
   2a729:	48 8b 4c 24 60       	mov    0x60(%rsp),%rcx
   2a72e:	48 8d 0c 49          	lea    (%rcx,%rcx,2),%rcx
   2a732:	48 8b 84 c8 20 01 00 	mov    0x120(%rax,%rcx,8),%rax
   2a739:	00 
   2a73a:	4c 8b 44 24 18       	mov    0x18(%rsp),%r8
   2a73f:	4c 39 c0             	cmp    %r8,%rax
   2a742:	49 8d 76 40          	lea    0x40(%r14),%rsi
   2a746:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   2a74b:	0f 83 01 f6 ff ff    	jae    29d52 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x6f2>
   2a751:	49 89 c0             	mov    %rax,%r8
   2a754:	49 89 fd             	mov    %rdi,%r13
   2a757:	41 bc 01 00 00 00    	mov    $0x1,%r12d
   2a75d:	48 8b 54 24 70       	mov    0x70(%rsp),%rdx
   2a762:	e9 b2 fe ff ff       	jmp    2a619 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb9>
   2a767:	45 31 e4             	xor    %r12d,%r12d
   2a76a:	e9 58 fe ff ff       	jmp    2a5c7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xf67>
   2a76f:	41 bd 01 00 00 00    	mov    $0x1,%r13d
   2a775:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   2a77c:	45 31 e4             	xor    %r12d,%r12d
   2a77f:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2a784:	31 c0                	xor    %eax,%eax
   2a786:	e9 3c f6 ff ff       	jmp    29dc7 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x767>
   2a78b:	45 31 c0             	xor    %r8d,%r8d
   2a78e:	31 ed                	xor    %ebp,%ebp
   2a790:	49 89 fd             	mov    %rdi,%r13
   2a793:	e9 7c fe ff ff       	jmp    2a614 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0xfb4>
   2a798:	48 8d 3d 92 eb fd ff 	lea    -0x2146e(%rip),%rdi        # 9331 <anon.ee651107ab5319c6bc273e1a29320aaf.1.llvm.14746981713632465754+0x351>
   2a79f:	48 8d 15 e2 6f 09 00 	lea    0x96fe2(%rip),%rdx        # c1788 <__frame_dummy_init_array_entry+0x448>
   2a7a6:	be 28 00 00 00       	mov    $0x28,%esi
   2a7ab:	ff 15 67 a2 09 00    	call   *0x9a267(%rip)        # c4a18 <_DYNAMIC+0x308>
   2a7b1:	bf 08 00 00 00       	mov    $0x8,%edi
   2a7b6:	be 10 00 00 00       	mov    $0x10,%esi
   2a7bb:	ff 15 c7 a1 09 00    	call   *0x9a1c7(%rip)        # c4988 <_DYNAMIC+0x278>
   2a7c1:	bf 08 00 00 00       	mov    $0x8,%edi
   2a7c6:	be 10 00 00 00       	mov    $0x10,%esi
   2a7cb:	ff 15 b7 a1 09 00    	call   *0x9a1b7(%rip)        # c4988 <_DYNAMIC+0x278>
   2a7d1:	e9 c3 01 00 00       	jmp    2a999 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1339>
   2a7d6:	bf 08 00 00 00       	mov    $0x8,%edi
   2a7db:	be 50 00 00 00       	mov    $0x50,%esi
   2a7e0:	ff 15 a2 a1 09 00    	call   *0x9a1a2(%rip)        # c4988 <_DYNAMIC+0x278>
   2a7e6:	e9 ae 01 00 00       	jmp    2a999 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1339>
   2a7eb:	bf 08 00 00 00       	mov    $0x8,%edi
   2a7f0:	be 10 00 00 00       	mov    $0x10,%esi
   2a7f5:	ff 15 45 a1 09 00    	call   *0x9a145(%rip)        # c4940 <_DYNAMIC+0x230>
   2a7fb:	48 8d 15 ee 75 09 00 	lea    0x975ee(%rip),%rdx        # c1df0 <anon.67153942ca9c76798561ce1001119436.221.llvm.15908011480901089632>
   2a802:	bf 01 00 00 00       	mov    $0x1,%edi
   2a807:	ff 15 43 a1 09 00    	call   *0x9a143(%rip)        # c4950 <_DYNAMIC+0x240>
   2a80d:	48 8d 15 dc 75 09 00 	lea    0x975dc(%rip),%rdx        # c1df0 <anon.67153942ca9c76798561ce1001119436.221.llvm.15908011480901089632>
   2a814:	eb 15                	jmp    2a82b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x11cb>
   2a816:	48 8d 15 03 6e 09 00 	lea    0x96e03(%rip),%rdx        # c1620 <__frame_dummy_init_array_entry+0x2e0>
   2a81d:	bf 01 00 00 00       	mov    $0x1,%edi
   2a822:	eb 0b                	jmp    2a82f <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x11cf>
   2a824:	48 8d 15 0d 6e 09 00 	lea    0x96e0d(%rip),%rdx        # c1638 <__frame_dummy_init_array_entry+0x2f8>
   2a82b:	31 ff                	xor    %edi,%edi
   2a82d:	31 f6                	xor    %esi,%esi
   2a82f:	ff 15 1b a1 09 00    	call   *0x9a11b(%rip)        # c4950 <_DYNAMIC+0x240>
   2a835:	e9 5f 01 00 00       	jmp    2a999 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1339>
   2a83a:	bf 08 00 00 00       	mov    $0x8,%edi
   2a83f:	be 10 00 00 00       	mov    $0x10,%esi
   2a844:	ff 15 f6 a0 09 00    	call   *0x9a0f6(%rip)        # c4940 <_DYNAMIC+0x230>
   2a84a:	e9 4a 01 00 00       	jmp    2a999 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1339>
   2a84f:	48 8d 15 5a 6e 09 00 	lea    0x96e5a(%rip),%rdx        # c16b0 <__frame_dummy_init_array_entry+0x370>
   2a856:	31 ff                	xor    %edi,%edi
   2a858:	31 f6                	xor    %esi,%esi
   2a85a:	ff 15 f0 a0 09 00    	call   *0x9a0f0(%rip)        # c4950 <_DYNAMIC+0x240>
   2a860:	48 8d 15 a9 6e 09 00 	lea    0x96ea9(%rip),%rdx        # c1710 <__frame_dummy_init_array_entry+0x3d0>
   2a867:	bf 01 00 00 00       	mov    $0x1,%edi
   2a86c:	ff 15 de a0 09 00    	call   *0x9a0de(%rip)        # c4950 <_DYNAMIC+0x240>
   2a872:	48 8d 15 4f 6e 09 00 	lea    0x96e4f(%rip),%rdx        # c16c8 <__frame_dummy_init_array_entry+0x388>
   2a879:	bf 01 00 00 00       	mov    $0x1,%edi
   2a87e:	ff 15 cc a0 09 00    	call   *0x9a0cc(%rip)        # c4950 <_DYNAMIC+0x240>
   2a884:	48 8d 15 65 75 09 00 	lea    0x97565(%rip),%rdx        # c1df0 <anon.67153942ca9c76798561ce1001119436.221.llvm.15908011480901089632>
   2a88b:	31 ff                	xor    %edi,%edi
   2a88d:	31 f6                	xor    %esi,%esi
   2a88f:	ff 15 bb a0 09 00    	call   *0x9a0bb(%rip)        # c4950 <_DYNAMIC+0x240>
   2a895:	bf 08 00 00 00       	mov    $0x8,%edi
   2a89a:	4c 89 ee             	mov    %r13,%rsi
   2a89d:	ff 15 9d a0 09 00    	call   *0x9a09d(%rip)        # c4940 <_DYNAMIC+0x230>
   2a8a3:	48 8d 15 1e 76 09 00 	lea    0x9761e(%rip),%rdx        # c1ec8 <anon.67153942ca9c76798561ce1001119436.243.llvm.15908011480901089632>
   2a8aa:	bf 01 00 00 00       	mov    $0x1,%edi
   2a8af:	48 89 c6             	mov    %rax,%rsi
   2a8b2:	ff 15 98 a0 09 00    	call   *0x9a098(%rip)        # c4950 <_DYNAMIC+0x240>
   2a8b8:	48 8d 15 09 76 09 00 	lea    0x97609(%rip),%rdx        # c1ec8 <anon.67153942ca9c76798561ce1001119436.243.llvm.15908011480901089632>
   2a8bf:	bf 01 00 00 00       	mov    $0x1,%edi
   2a8c4:	ff 15 86 a0 09 00    	call   *0x9a086(%rip)        # c4950 <_DYNAMIC+0x240>
   2a8ca:	48 8d 15 0f 76 09 00 	lea    0x9760f(%rip),%rdx        # c1ee0 <anon.67153942ca9c76798561ce1001119436.244.llvm.15908011480901089632>
   2a8d1:	bf 01 00 00 00       	mov    $0x1,%edi
   2a8d6:	48 89 c6             	mov    %rax,%rsi
   2a8d9:	ff 15 71 a0 09 00    	call   *0x9a071(%rip)        # c4950 <_DYNAMIC+0x240>
   2a8df:	48 8d 15 fa 75 09 00 	lea    0x975fa(%rip),%rdx        # c1ee0 <anon.67153942ca9c76798561ce1001119436.244.llvm.15908011480901089632>
   2a8e6:	31 ff                	xor    %edi,%edi
   2a8e8:	31 f6                	xor    %esi,%esi
   2a8ea:	ff 15 60 a0 09 00    	call   *0x9a060(%rip)        # c4950 <_DYNAMIC+0x240>
   2a8f0:	48 83 fd 01          	cmp    $0x1,%rbp
   2a8f4:	75 0d                	jne    2a903 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12a3>
   2a8f6:	48 8d 15 6b 6d 09 00 	lea    0x96d6b(%rip),%rdx        # c1668 <__frame_dummy_init_array_entry+0x328>
   2a8fd:	31 ff                	xor    %edi,%edi
   2a8ff:	31 f6                	xor    %esi,%esi
   2a901:	eb 48                	jmp    2a94b <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12eb>
   2a903:	48 8d 3d 27 ea fd ff 	lea    -0x215d9(%rip),%rdi        # 9331 <anon.ee651107ab5319c6bc273e1a29320aaf.1.llvm.14746981713632465754+0x351>
   2a90a:	48 8d 15 87 6d 09 00 	lea    0x96d87(%rip),%rdx        # c1698 <__frame_dummy_init_array_entry+0x358>
   2a911:	be 28 00 00 00       	mov    $0x28,%esi
   2a916:	ff 15 fc a0 09 00    	call   *0x9a0fc(%rip)        # c4a18 <_DYNAMIC+0x308>
   2a91c:	eb 7b                	jmp    2a999 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1339>
   2a91e:	48 8d 15 d3 6d 09 00 	lea    0x96dd3(%rip),%rdx        # c16f8 <__frame_dummy_init_array_entry+0x3b8>
   2a925:	eb 07                	jmp    2a92e <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x12ce>
   2a927:	48 8d 15 a2 73 09 00 	lea    0x973a2(%rip),%rdx        # c1cd0 <anon.67153942ca9c76798561ce1001119436.208.llvm.15908011480901089632>
   2a92e:	31 ff                	xor    %edi,%edi
   2a930:	31 f6                	xor    %esi,%esi
   2a932:	ff 15 18 a0 09 00    	call   *0x9a018(%rip)        # c4950 <_DYNAMIC+0x240>
   2a938:	eb 5f                	jmp    2a999 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1339>
   2a93a:	48 8d 15 3f 6d 09 00 	lea    0x96d3f(%rip),%rdx        # c1680 <__frame_dummy_init_array_entry+0x340>
   2a941:	bf 01 00 00 00       	mov    $0x1,%edi
   2a946:	be 01 00 00 00       	mov    $0x1,%esi
   2a94b:	ff 15 ff 9f 09 00    	call   *0x99fff(%rip)        # c4950 <_DYNAMIC+0x240>
   2a951:	eb 46                	jmp    2a999 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1339>
   2a953:	48 8d 15 ce 6d 09 00 	lea    0x96dce(%rip),%rdx        # c1728 <__frame_dummy_init_array_entry+0x3e8>
   2a95a:	31 ff                	xor    %edi,%edi
   2a95c:	31 f6                	xor    %esi,%esi
   2a95e:	ff 15 ec 9f 09 00    	call   *0x99fec(%rip)        # c4950 <_DYNAMIC+0x240>
   2a964:	48 8d 15 75 6d 09 00 	lea    0x96d75(%rip),%rdx        # c16e0 <__frame_dummy_init_array_entry+0x3a0>
   2a96b:	31 ff                	xor    %edi,%edi
   2a96d:	31 f6                	xor    %esi,%esi
   2a96f:	ff 15 db 9f 09 00    	call   *0x99fdb(%rip)        # c4950 <_DYNAMIC+0x240>
   2a975:	48 8d 15 dc 6d 09 00 	lea    0x96ddc(%rip),%rdx        # c1758 <__frame_dummy_init_array_entry+0x418>
   2a97c:	31 ff                	xor    %edi,%edi
   2a97e:	31 f6                	xor    %esi,%esi
   2a980:	eb 11                	jmp    2a993 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1333>
   2a982:	48 8d 15 e7 6d 09 00 	lea    0x96de7(%rip),%rdx        # c1770 <__frame_dummy_init_array_entry+0x430>
   2a989:	bf 01 00 00 00       	mov    $0x1,%edi
   2a98e:	be 01 00 00 00       	mov    $0x1,%esi
   2a993:	ff 15 b7 9f 09 00    	call   *0x99fb7(%rip)        # c4950 <_DYNAMIC+0x240>
   2a999:	0f 0b                	ud2    
   2a99b:	48 89 c3             	mov    %rax,%rbx
   2a99e:	48 83 7c 24 18 00    	cmpq   $0x0,0x18(%rsp)
   2a9a4:	75 4e                	jne    2a9f4 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x1394>
   2a9a6:	eb 7e                	jmp    2aa26 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x13c6>
   2a9a8:	48 89 c3             	mov    %rax,%rbx
   2a9ab:	48 83 7c 24 18 00    	cmpq   $0x0,0x18(%rsp)
   2a9b1:	74 73                	je     2aa26 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x13c6>
   2a9b3:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   2a9b8:	ff 15 4a 9f 09 00    	call   *0x99f4a(%rip)        # c4908 <free@GLIBC_2.2.5>
   2a9be:	48 89 df             	mov    %rbx,%rdi
   2a9c1:	e8 3a 59 09 00       	call   c0300 <_Unwind_Resume@plt>
   2a9c6:	48 89 c3             	mov    %rax,%rbx
   2a9c9:	4c 89 e7             	mov    %r12,%rdi
   2a9cc:	ff 15 36 9f 09 00    	call   *0x99f36(%rip)        # c4908 <free@GLIBC_2.2.5>
   2a9d2:	48 89 df             	mov    %rbx,%rdi
   2a9d5:	e8 26 59 09 00       	call   c0300 <_Unwind_Resume@plt>
   2a9da:	48 89 c3             	mov    %rax,%rbx
   2a9dd:	48 8d 7c 24 48       	lea    0x48(%rsp),%rdi
   2a9e2:	e8 b9 ce ff ff       	call   278a0 <core::ptr::drop_glue::<chr_compiled::Work>>
   2a9e7:	eb 3d                	jmp    2aa26 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x13c6>
   2a9e9:	48 89 c3             	mov    %rax,%rbx
   2a9ec:	48 83 7c 24 10 00    	cmpq   $0x0,0x10(%rsp)
   2a9f2:	74 32                	je     2aa26 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x13c6>
   2a9f4:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   2a9f9:	ff 15 09 9f 09 00    	call   *0x99f09(%rip)        # c4908 <free@GLIBC_2.2.5>
   2a9ff:	48 89 df             	mov    %rbx,%rdi
   2aa02:	e8 f9 58 09 00       	call   c0300 <_Unwind_Resume@plt>
   2aa07:	48 89 c3             	mov    %rax,%rbx
   2aa0a:	48 8d 7c 24 20       	lea    0x20(%rsp),%rdi
   2aa0f:	e8 8c ce ff ff       	call   278a0 <core::ptr::drop_glue::<chr_compiled::Work>>
   2aa14:	eb 03                	jmp    2aa19 <<chain::generated::program_state_1 as chr_compiled::native_access::Continuation>::tick+0x13b9>
   2aa16:	48 89 c3             	mov    %rax,%rbx
   2aa19:	48 8d bc 24 80 00 00 	lea    0x80(%rsp),%rdi
   2aa20:	00 
   2aa21:	e8 7a ce ff ff       	call   278a0 <core::ptr::drop_glue::<chr_compiled::Work>>
   2aa26:	48 89 df             	mov    %rbx,%rdi
   2aa29:	e8 d2 58 09 00       	call   c0300 <_Unwind_Resume@plt>
   2aa2e:	ff 15 a4 9f 09 00    	call   *0x99fa4(%rip)        # c49d8 <_DYNAMIC+0x2c8>
   2aa34:	cc                   	int3   
   2aa35:	cc                   	int3   
   2aa36:	cc                   	int3   
   2aa37:	cc                   	int3   
   2aa38:	cc                   	int3   
   2aa39:	cc                   	int3   
   2aa3a:	cc                   	int3   
   2aa3b:	cc                   	int3   
   2aa3c:	cc                   	int3   
   2aa3d:	cc                   	int3   
   2aa3e:	cc                   	int3   
   2aa3f:	cc                   	int3   