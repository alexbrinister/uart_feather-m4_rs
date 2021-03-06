MEMORY
{
  /* Per specs found on adafruits website for the Feather
   * (https://www.adafruit.com/product/3857):
   *    - Flash: 512 KB
   *    - RAM: 192 KB
   * 
   * Leave 16k of flash for the default bootloader
   *
   * Origin addresses for RAM and Flash found in section 9.2 of the ATSAMD51J19
   * data sheet, found at https://ww1.microchip.com/downloads/en/DeviceDoc/SAM_D5x_E5x_Family_Data_Sheet_DS60001507G.pdf.
   */
  FLASH : ORIGIN = 0x00000000 + 16k, LENGTH = 512K - 16k
  RAM : ORIGIN = 0x20000000, LENGTH = 192K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/
