/* 
 * Aliases for data segments.
 *
 * This set of naming is used if the 'RAMONLY' feature is not set. It will make
 * linker script to include static firmware code to FLASH. 
 * */

REGION_ALIAS("ROTEXT", irom_seg);
REGION_ALIAS("RWTEXT", iram_seg);
REGION_ALIAS("RODATA", dram_seg);
REGION_ALIAS("RWDATA", dram_seg);
