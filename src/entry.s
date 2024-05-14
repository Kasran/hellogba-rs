@ --
@ a
@ --

.arm
.section .entrypoint, "ax"
.align

.global __start
__start:
    @ header goes here
    @ the blank spots get filled in by gbafix
    @ ref: https://www.akkit.org/info/gbatek.htm#gbacartridgeheader
    b init
    .fill 156, 1, 0  @ nintendo logo char data
    .fill 16, 1, 0   @ title (12 chars uppercase) + gamecode (4 chars u.c.)
    .byte 0x30, 0x31 @ maker code (2 chars uppercase)
    .byte 0x96       @ fixed value (has to be 0x96 for some reason)
    .byte 0x00       @ "main unit code" (0x00)
    .byte 0x00       @ device type
    .fill 7, 1, 0    @ reserved for something or another
    .byte 0x00       @ software version (usually 0x00)
    .byte 0xf0       @ header something something
    .byte 0x00, 0x00 @ two bytes reserved
    @ addl. multiboot header stuff goes here
    b init           @ this should be an address to ram (since multiboot)
    .byte 0x00, 0x00 @ boot mode + slave id no. (bios overwrites these)
    .fill 26, 1, 0   @ 26 seemingly unused bytes
    .fill 4, 1, 0    @ JOYBUS branch opcode (i dont know what that means)

init:
    @ init code goes here
    @ copy ewram LMA to VMA
    ldr r0, =__ewram_rom_start
    ldr r1, =__ewram_data_start
    ldr r2, =__ewram_rom_length_halfwords
    swi 0x000B0000  @ syscall CpuSet

    @ copy iwram LMA to VMA
    ldr r0, =__iwram_rom_start
    ldr r1, =__iwram_data_start
    ldr r2, =__iwram_rom_length_halfwords
    swi 0x000B0000  @ syscall CpuSet again

    @ jump to main
    ldr r0, =0      @ zero in r0 and r1 means no argc and no argv
    mov r1, r0      @ (c runtime cares about that, we don't really)
    ldr r2, =main
    bx r2
1:
    b 1b
.pool
