#let title = [
  Analog/RF Architecture
]

#let name = [
  Andrew Friedman
]


#set page(
  header: align(
    right + horizon,
    title
  ),
  footer: align(
    left + horizon,
    name
  ),
)

#align(center, text(20pt)[
  *#title*
])

#align(center, text(17pt)[
  *#name*
])

= Architecture Information
This architecture is specialized for DSP applications. It is a clean sheet architecture based on parts from ARM, RISC-V, and x86. The architecture is a 32 bit Harvard architecture with 32 32-bit integers registers, 32 single-precision floating-point registers. The registers are in little endian. \

The Integer Registers:
- R0-R20: General purpose register
- R20-R23 (A1-A4): ADC Output Register
- R24-R27 (D1-D4): DAC Input Register
- R28 (PC): Program Counter
- R29 (LR): Link Register
- R30 (ST): Status Register
- R31 (SP): Stack Pointer


Bits of Status Register and Short Condition Code
0. Always True () [Value of bit does not affect branch instruction] _This is a normal branch_
+ Never True (NVR) [Value of bit does not affect branch instruction]  _This is a NOP_
+ Equal (EQ)
+ Greater Than (GT)
+ Less Than (LT)
+ Greater Than or Equal To (GE)
+ Less Than or Equal To (LE)
+ Overflow (OVRF)
+ Underflow (UNDF)
+ Divide By Zero (DIVZ)
+ Parity / Is Even (EVEN)
+ Floating Point Infinity (FINF)
+ Floating Point Zero (FZ)
+ Floating Point Not-A-Number (FNAN)
+ Floating Point Positive (FPOS)
The rest of the bits are reserved for future use.


All 32 floating point registers are general purpose registers with no special meaning. \

There are 32 general purpose timers which are automatically decreased every clock cycle. Timers automatically restart at their previous set value after they are read to be zero. They do not fire interupts, they are just clock cycle counters that count down. \


#pagebreak()

= Instructions

- Ra defines an integer register
- [Ra + I << S] defines a register indirect into data memory at Ra + the value of I shifted by S ([Ra] is interpreted as [Ra + 0 << 0])
- [Ra + Rb << S] defines a double register indirect into data memory at Ra + the value of Rb shifted by S
- Fa defines an floating-point register
- Ta defines a timer
- _label_ is a label which represents an address
- d:0xYYYYYYYY is an auto generated label for 0xYYYYYYYY in the data memory
- p:0xYYYYYYYY is an auto generated label for 0xYYYYYYYY in the program memory

#include "instructions.typ"

#pagebreak()

= Memory
The plan is to have 2 64 line caches, one for memory and the other for data. Where each line is 4 words. This number might be changed depending on the efficiency of the simulator and therefore the maximum reasonable size of the benchmark. \
The memory is split into two distinct banks, where the program memory is what is loaded from the ASM / machine code and is in DRAM. The data memory bank will be in SRAM (though it might be clocked down for the purpose of the benchmarks). \
Each bank will store 65,536 words (2^16). \
The cache will be a write-though, no allocate direct cache. There will be a 6 bit index, a 2 bit offset, and a 8 bit tag. There will be a seperate cache for data and program memory.

The memory address is the lower 16-bits, any of the higher 16-bits are discarded.

#pagebreak()

= Simulator

To use the simulator write the assembly in the asm folder under the gui folder. The asm requires a `.prog` section and optionally a `.data` section

\
I will start with the simulator and a very simple assembler. Once when they are both in a usable state I will start on the GUI. After the GUI shows basic information I would need for the rest of my debugging I will switch back to the assembler and simulator. After that I will add more polish to the GUI.