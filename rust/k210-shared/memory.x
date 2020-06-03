INCLUDE memory-k210.x

REGION_ALIAS("REGION_TEXT", SRAM);
REGION_ALIAS("REGION_RODATA", SRAM);
REGION_ALIAS("REGION_DATA", SRAM);
REGION_ALIAS("REGION_BSS", SRAM);
REGION_ALIAS("REGION_HEAP", SRAM);
REGION_ALIAS("REGION_STACK", SRAM);

/* memory-k210.x already sets _max_hart_id = 1 for us. */

 /* Reserve 4MiB of memory for a heap. This is not used in most of the
    programs.
  */
 _heap_size = 4M;

 /* 128Kib of stack ought to be enough for anyone. */
 _hart_stack_size = 128K;
