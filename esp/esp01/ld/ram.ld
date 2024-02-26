/* 
 * Aliases for data segments.
 *
 * This set of naming is used if the 'RAMONLY' feature is set, which makes the
 *  
 * */

REGION_ALIAS("ROTEXT", iram_seg); /* Makes static code appear in RAM. */
REGION_ALIAS("RWTEXT", iram_seg);
REGION_ALIAS("RODATA", dram_seg);
REGION_ALIAS("RWDATA", dram_seg);
