/*
 * Main linker script that defines memory sections of ESP8266EX target.
 *
 * This script defines all special sections, which are ESP8266EX specific. Naming are used so
 * regular linker scripts which are used for all esp targets could map other memory regions. 
 * */

/* Entry function */
ENTRY(ESP8266Reset)

/* Memory regions definitions */
INCLUDE "memory.ld"

/* ESP8266 specific regions */
SECTIONS {
    .rtc.text : {
        . = ALIGN(4);
        *(.rtc.literal .rtc.text .rtc.literal.* .rtc.text.*)
    } > rtc_seg AT > RODATA

    .rtc.data : {
        _rtc_data_start = ABSOLUTE(.);
        . = ALIGN(4);
        *(.rtc.data .rtc.data.*)
        _rtc_data_end = ABSOLUTE(.);
    } > rtc_seg AT > RODATA

    .rtc.bss (NOLOAD) : {
        _rtc_bss_start = ABSOLUTE(.);
        . = ALIGN(4);
        *(.rtc.bss .rtc.bss.*)
        _rtc_bss_end = ABSOLUTE(.);
    } > rtc_seg

    .rtc.noinit (NOLOAD) : {
        . = ALIGN(4);
        *(.rtc.noinit .rtc.noinit.*)
    } > rtc_seg
}

/* Memory related symbols */
_heap_end = ABSOLUTE(ORIGIN(dram_seg))+LENGTH(dram_seg)+LENGTH(reserved_for_boot_seg) - 2*STACK_SIZE;
_text_heap_end = ABSOLUTE(ORIGIN(iram_seg)+LENGTH(iram_seg));

_stack_start_cpu1 = _heap_end;
_stack_end_cpu1 = _stack_start_cpu1 + STACK_SIZE;
_stack_start_cpu0 = _stack_end_cpu1;
_stack_end_cpu0 = _stack_start_cpu0 + STACK_SIZE;

EXTERN(DefaultHandler);

/* interrupt handlers */
PROVIDE(__slc_interrupt = __default_interrupt);
PROVIDE(__spi_interrupt = __default_interrupt);
PROVIDE(__gpio_interrupt = __default_interrupt);
PROVIDE(__uart_interrupt = __default_interrupt);
PROVIDE(__ccompare_interrupt = __default_interrupt);
PROVIDE(__soft_interrupt = __default_interrupt);
PROVIDE(__wdt_interrupt = __default_interrupt);
PROVIDE(__timer1_interrupt = __default_interrupt);

PROVIDE(__slc_hal_interrupt = __default_interrupt);
PROVIDE(__spi_hal_interrupt = __default_interrupt);
PROVIDE(__gpio_hal_interrupt = __default_interrupt);
PROVIDE(__uart_hal_interrupt = __default_interrupt);
PROVIDE(__ccompare_hal_interrupt = __default_interrupt);
PROVIDE(__soft_hal_interrupt = __default_interrupt);
PROVIDE(__wdt_hal_interrupt = __default_interrupt);
PROVIDE(__timer1_hal_interrupt = __default_interrupt);
