/*
 * Memory layout definition
 *
 * This linker script defines all memory layout for ESP8266EX target. Most memory regions
 * are the same as other ESP targets, hovewer some changes exist due to Xtensa CPU and
 * external FLASH mappings.
 * */

/* Overall stack size */
STACK_SIZE = 8K;

/* Specify main memory areas */
MEMORY
{
  reserved_cache_seg    : ORIGIN = 0x40070000, len = 64K                /* SRAM0; reserved for usage as flash cache*/
  vectors_seg       (rx): ORIGIN = 0x40100000, len =  0x100             /* SRAM0 */
  iram_seg          (rx): ORIGIN = 0x40100100, len = 0x8000 - 0x0100    /* SRAM0 */

  reserved_for_rom_seg  : ORIGIN = 0x3FFAE000, len = 8K                /* SRAM2; reserved for usage by the ROM */
  dram_seg          (rw): ORIGIN = 0x3FFE8000, len = 0x14000            /* SRAM2+1; first 64kB used by BT if enable */
  reserved_for_boot_seg : ORIGIN = 0x3FFDC200, len = 144K               /* SRAM1; reserved for static ROM usage; can be used for heap */

  irom_seg          (rx): ORIGIN = 0x40210000, len = 0xfeff0
  drom_seg           (r): ORIGIN = 0x3F400000, len = 4M - 0x20

  /* RTC memory (ESP8266EX specific region.) */
  rtc_seg           (rw): ORIGIN = 0x60001000, len = 768
}

INCLUDE "alias.ld"
