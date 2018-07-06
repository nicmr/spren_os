## Explanation of arch-spren_os.json fields

 * panic-trategy: don't unwind stack
 * linker-flavor: use cross platform linker
 * disable-redzone: disable redzone  pointer optimization, which would cause stack corruption
 * features:
   * -mmx,-sse: disables simd for kernel (not applications) to speed up kernel, because simd register are costly to store on stack on context switch
   * +soft-float: emulate fp operations through integers, to avoid simd usage (see above)