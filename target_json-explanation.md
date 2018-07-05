##Explanation of arch-spren_os.json fields

 * panic-trategy: don't stack unwind
 * linker-flavor: use cross platform linker
 * disable-redzone: disable redzone  pointer optimization, which would cause stack corruption
 * features:
   * -mmx,-sse: disableds simd for kernel (not applications) to speed up kernel
   * +soft-float: emulate fp operations through integers, to avoid simd usage (see above)