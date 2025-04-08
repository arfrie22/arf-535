== Type 000 (General) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Trap) \
TRAP \
Used in simulator to pause execution, and return control to the simulator's UI.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [0], [0], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 24)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Push Integer Register) \
PUSHRx \
Pushes the value of Rx onto the stack (SP) and increase SP by 1.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Push Floating-Point Register) \
PUSHFx \
Pushes the value of Fx onto the stack (SP) and increase SP by 1.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Fx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Pop Integer Register) \
POPRx \
Pops the top value from the stack (SP) into Rx and decrease SP by 1]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Pop Floating-Point Register) \
POPFx \
Pops the top value from the stack (SP) into Fx and decrease SP by 1]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Fx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Swap Register) \
SWPRxFy \
Swaps the integer register Rx with the value of the floating-point register Fy.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Integer Register], table.cell(colspan: 5)[Floating-Point Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Stall) \
STALLRx \
Stalls the pipeline at execute for Rx clock cycles.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [0], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
== Type 001 (Branch) \
The condition code for a branch is testing a bit inside the Status Register, consult the table to see the short name. In the ASM write the instruction as (B).(COND) where (B) is the branch instruction and (COND) is the short condition code.
#block(breakable: false,{text[=== OPCODE: 0x00 (Register Jump) \
BConditionRx \
Sets the program counter to Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Indirect Jump) \
BConditionRx \
Sets the program counter to the value stored at the memory location in Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Indirect with Register Offset Jump) \
BConditionRx \
Sets the program counter to the value stored at the memory location in Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Relative Jump) \
BRConditionRx \
Sets the program counter to (PC + Rx) where Rx is interpreted as a signed 32-bit integer.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Immediate Jump) \
BCondition_label_ \
Sets the program counter to the address of the label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Condition], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 16)[Destination Address], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Immediate Relative Jump) \
BOCondition_offset_ \
Adds the offset sign exteneded to 32-bits to the program counter (offset treated as signed 16-bit number).]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Condition], table.cell(colspan: 16)[_offset_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 16)[Signed Offset], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Register Jump with Link) \
BLConditionRx \
Sets the link register to (PC + 1). Sets the program counter to Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Indirect Jump with Link) \
BLConditionRx \
Sets the link register to (PC + 1). Sets the program counter to the value stored at the memory location in Rx offset by the I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Indirect with Register Offset Jump with Link) \
BLConditionRx \
Sets the link register to (PC + 1). Sets the program counter to the value stored at the memory location in Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [1], [0], [0], [0], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Relative Jump with Link) \
BRLConditionRx \
Sets the link register to (PC + 1). Sets the program counter to (PC + Rx) where Rx is interpreted as a signed 32-bit integer.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [1], [0], [0], [1], table.cell(colspan: 5)[Condition], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0a (Immediate Jump with Link) \
BLCondition_label_ \
Sets the link register to (PC + 1). Sets the program counter to label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [1], [0], [1], [0], table.cell(colspan: 5)[Condition], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 16)[Destination Address], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0b (Immediate Relative Jump with Link) \
BOLCondition_offset_ \
Sets the link register to (PC + 1). Adds the offset sign exteneded to 32-bits to the program counter (offset treated as signed 16-bit number that is sign extended).]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [0], [1], [0], [1], [0], [1], [1], table.cell(colspan: 5)[Condition], table.cell(colspan: 16)[_offset_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Condition code bit], table.cell(colspan: 16)[Signed Offset], table.cell(colspan: 3)[Unused]
)}) \
== Type 010 (Integer Register) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Integer Load Low) \
LDLRx_value_ \
Loads the 16-bit immediate value into the lower 16 bits of register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Integer Load High) \
LDHRx_value_ \
Loads the 16-bit immediate value into the higher 16 bits of register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Swap Integer Registers) \
SWPRxRy \
Swaps the values of Rx and Ry.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[First Register], table.cell(colspan: 5)[Second Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Copy Integer Register) \
LDRRxRy \
Sets the value of Rx to the value of Ry.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Load Integer Register Indirect) \
LDRRxRy \
Sets the value of Rx to the value of stored at the data memory location at Ry offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Load Integer Register Indirect with Register Offset) \
LDRRxRy \
Sets the value of Rx to the value of stored at the data memory location at Ry offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Load Integer Register Indirect Program) \
LDRRxRy \
Sets the value of Rx to the value of stored at the program memory location at Ry offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Load Integer Register Indirect with Register Offset Program) \
LDRRxRy \
Sets the value of Rx to the value of stored at the program memory location at Ry offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Store Integer Register Indirect) \
STRRxRy \
Stores the value of Ry into the data memory location at Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Store Integer Register Indirect with Register Offset Indirect) \
STRRxRy \
Stores the value of Ry into the data memory location at Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0a (Store Integer Register Indirect Program) \
STRRxRy \
Stores the value of Ry into the program memory location at Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0b (Store Integer Register Indirect with Register Offset Program) \
STRRxRy \
Stores the value of Ry into the program memory location at Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0c (Integer Load Data) \
LDRRx_label_ \
Loads the value at data memory address label into the register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Destination Address in data], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0d (Integer Load Program) \
LDRRx_label_ \
Loads the value at program memory address label into the register Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Destination Addres in programs], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0e (Integer Store Data) \
STRRx_label_ \
Stores the value in Rx into the data memory address label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 16)[Destination Address in data], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0f (Integer Store Program) \
STRRx_label_ \
Stores the value in Rx into the program memory address label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [0], [1], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 16)[Destination Address in program], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x10 (Unsigned Zero Extend) \
ZEXRxRy_count_ \
Stores the value of Ry into Rx where the top 31 - _count_ bits are zero.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_count_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Bit Count], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x11 (Sign Extend) \
SEXRxRy_count_ \
Stores the value of Ry into Rx where the top 31 - _count_ bits are sign extended.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [0], [1], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_count_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Bit Count], table.cell(colspan: 9)[Unused]
)}) \
== Type 011 (Floating-Point Register) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Floating-Point Load Low) \
LDLFx_value_ \
Loads the 16-bit immediate value into the lower 16 bits of register Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Floating-Point Load High) \
LDHFx_value_ \
Loads the 16-bit immediate value into the higher 16 bits of register Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_value_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Immediate Value], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Swap Floating-Point Registers) \
SWPFxFy \
Swaps the values of Fx and Fy.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[First Register], table.cell(colspan: 5)[Second Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Copy Floating-Point Register) \
LDRFxFy \
Sets the value of Fx to the value of Fy.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Load Floating-Point Register Indirect) \
LDRFxRy \
Sets the value of Fx to the value of stored at the data memory location at Ry offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Load Floating-Point Register Indirect with Register Offset) \
LDRFxRy \
Sets the value of Fx to the value of stored at the data memory location at Ry offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Store Floating-Point Register Indirect) \
STRRxFy \
Stores the value of Fy into the data memory location at Rx offset by I shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[I], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Store Floating-Point Register Indirect with Register Offset) \
STRRxFy \
Stores the value of Fy into the data memory location at Rx offset by the value of Ro shifted by S.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Ro], table.cell(colspan: 4)[S], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 5)[Offset Register], table.cell(colspan: 4)[Shift], table.cell(colspan: 5)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Floating-Point Load Data) \
LDRFx_label_ \
Loads the value at data memory address label into the register Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [1], [0], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 16)[Destination Address in data], table.cell(colspan: 3)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Floating-Point Store Data) \
STRFx_label_ \
Stores the value in Fx into the data memory address label.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[0], [1], [1], [0], [1], [0], [0], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 16)[_label_], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 16)[Destination Address in data], table.cell(colspan: 3)[Unused]
)}) \
== Type 100 (Integer ALU) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Integer Compare) \
CMPRxRy \
Stores the condition variables.
- Sets EQ if $"Rx" = "Ry"$
- Sets GT if $"Rx" > "Ry"$
- Sets LT if $"Rx" < "Ry"$
- Sets GE if $"Rx" >= "Ry"$
- Sets LE if $"Rx" <= "Ry"$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Integer Compare Single Against Zero) \
CMPRx \
Stores the condition variables.
- Sets EQ if $"Rx" = 0$
- Sets GT if $"Rx" > 0$
- Sets LT if $"Rx" < 0$
- Sets GE if $"Rx" >= 0$
- Sets LE if $"Rx" <= 0$
- Sets EVEN if $"Rx" % 2 = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Add Unsigned Integer) \
ADDcRxRyRz \
Sets Rx := Ry + Rz.\
Sets overflow bit if $"Ry" + "Rz" >= 2^32$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [1], [0], table.cell(colspan: 1)[c], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 1)[Condition Code Bit], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 8)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Subtract Unsigned Integer) \
SUBRxRyRz \
Sets Rx := Ry - Rz.\
Sets underflow bit if $"Ry" - "Rz" < 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Multiply Unsigned Integer) \
MULRxRyRz \
Sets Rx := Ry \* Rz.\
Sets overflow bit if $"Ry" times "Rz" >= 2^32$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Divide Unsigned Integer) \
DIVRxRyRz \
Sets Rx := Ry / Rz.\
Sets divide by zero bit if $"Rz" = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Modulo Unsigned Integer) \
MODRxRyRz \
Sets Rx := Ry % Rz.\
Sets divide by zero bit if $"Rz" = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Add Signed Integer) \
ADDSRxRyRz \
Sets Rx := Ry + Rz (treats all registers as signed).\
Sets overflow bit if $"Ry" + "Rz" >= 2^31 - 1$. Sets underflow bit if $"Ry" + "Rz" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x08 (Subtract Signed Integer) \
SUBSRxRyRz \
Sets Rx := Ry - Rz (treats all registers as signed).\
Sets overflow bit if $"Ry" - "Rz" >= 2^31 - 1$. Sets underflow bit if $"Ry" - "Rz" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x09 (Multiply Signed Integer) \
MULSRxRyRz \
Sets Rx := Ry \* Rz (treats all registers as signed).\
Sets overflow bit if $"Ry" times "Rz" >= 2^31 - 1$. Sets underflow bit if $"Ry" times "Rz" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0a (Divide Signed Integer) \
DIVSRxRyRz \
Sets Rx := Ry / Rz (treats all registers as signed).\
Sets divide by zero bit if $"Rz" = 0$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0b (Modulo Signed Integer) \
MODSRxRyRz \
Sets Rx := Ry % Rz (treats all registers as signed).\
Sets divide by zero bit if $"Rz" = 0$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0c (Bitwise AND) \
ANDRxRyRz \
Sets Rx := Ry & Rz.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0d (Bitwise OR) \
ORRxRyRz \
Sets Rx := Ry | Rz]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0e (Bitwise NOT) \
NOTRxRy \
Sets Rx := ~Ry]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x0f (Bitwise XOR) \
XORRxRyRz \
Sets Rx := Ry ^ Rz]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [0], [1], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x10 (Logical Shift Left) \
LSLRxRy_value_ \
Shifts Ry left by value bits, where 0 is the new value on the right and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x11 (Logical Shift Right) \
LSRRxRy_value_ \
Shifts Ry right by value bits, where 0 is the new value on the left and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x12 (Arithmetic Shift Left) \
ASLRxRy_value_ \
Shifts Ry left by value bits, where 0 is the new value on the right and only up to the 31st bit, the 32nd bit remains untouched. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x13 (Arithmetic Shift Right) \
ASRRxRy_value_ \
Shifts Ry right by value bits, where the 32nd bit is now the value of what it was previously and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x14 (Rotate Right) \
RTRRxRy_value_ \
Shifts Ry right by value bits, the bit that falls off gets shifted back in. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[_value_], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Shift Amount], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x15 (Logical Shift Left Register) \
LSLRxRyRz \
Shifts Ry left by value bits, where 0 is the new value on the right and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x16 (Logical Shift Right Register) \
LSRRxRyRz \
Shifts Ry right by value bits, where 0 is the new value on the left and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x17 (Arithmetic Shift Left Register) \
ASLRxRyRz \
Shifts Ry left by value bits, where 0 is the new value on the right and only up to the 31st bit, the 32nd bit remains untouched. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x18 (Arithmetic Shift Right Register) \
ASRRxRyRz \
Shifts Ry right by value bits, where the 32nd bit is now the value of what it was previously and the extra bit falls off. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [0], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x19 (Rotate Right Register) \
RTRRxRyRz \
Shifts Ry right by value bits, the bit that falls off gets shifted back in. Value stored in Rx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], table.cell(colspan: 5)[Rz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x1a (Map Unsigned To Signed) \
MUSRxRy \
Equivalent of storing (Ry-0x80000000) into Rx without loosing any bits due to casting due to signed numbers having one less bit.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [1], [0], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x1b (Map Signed To Unsigned) \
MSURxRy \
Equivalent of storing (Ry+0x80000000) into Rx without loosing any bits due to casting due to signed numbers having one less bit.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [0], [1], [1], [0], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
== Type 101 (Floating-Point ALU) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Floating-Point Compare) \
CMPFxFy \
Stores the condition variables.
- Sets EQ if $"Fx" = "Fy"$
- Sets GT if $"Fx" > "Fy"$
- Sets LT if $"Fx" < "Fy"$
- Sets GE if $"Fx" >= "Fy"$
- Sets LE if $"Fx" <= "Fy"$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Floating-Point Compare Single Against Zero) \
CMPFx \
Stores the condition variables.
- Sets EQ if $"Fx" = 0$
- Sets GT if $"Fx" > 0$
- Sets LT if $"Fx" < 0$
- Sets GE if $"Fx" >= 0$
- Sets LE if $"Fx" <= 0$
- Sets FINF if $"Fx" = plus.minus infinity$
- Sets FZ if $"Fx" = plus.minus 0$
- Sets FNAN if $"Fx" = "NAN"$]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Fx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Source Register], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Add Floating-Point) \
ADDFxFyFz \
Sets Fx := Fy + Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Subtract Floating-Point) \
SUBFxFyFz \
Sets Fx := Fy - Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x04 (Multiply Floating-Point) \
MULFxFyFz \
Sets Fx := Fy \* Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [0], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x05 (Divide Floating-Point) \
DIVFxFyFz \
Sets Fx := Fy / Fz.\
Sets overflow bit if result is outside of floating-point range. Sets underflow bit if result is subnormal. Sets FZ bit if result $= plus.minus 0$. Sets FINF bit if result $= plus.minus infinity$. Sets FNAN bit if result is not-a-number.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [0], [1], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Fy], table.cell(colspan: 5)[Fz], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register 1], table.cell(colspan: 5)[Source Register 2], table.cell(colspan: 9)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x06 (Cast to Float) \
CSTFxRy \
Converts the value of Ry into a float and stores it in Fx.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [1], [0], table.cell(colspan: 5)[Fx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x07 (Cast from Float) \
CSTRxFy \
Converts the value of Fy into a signed integer (rounded towards zero) and stores it in Rx.\
Sets OVRF if $"Fy" > 2^31 - 1$. Sets UNDF if $"Fy" < -2^31$.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [0], [1], [0], [0], [1], [1], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Fy], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
== Type 110 (Timers) \
#block(breakable: false,{text[=== OPCODE: 0x00 (Set Timer) \
SETTTxRy \
Sets the value of the timer (timer) to the value of Rx]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [0], [0], table.cell(colspan: 5)[Tx], table.cell(colspan: 5)[Ry], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Timer], table.cell(colspan: 5)[Source Register], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x01 (Get Current Timer) \
GETTRxTy \
Sets Rx to the value of the timer (timer)]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [0], [1], table.cell(colspan: 5)[Rx], table.cell(colspan: 5)[Ty], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Register], table.cell(colspan: 5)[Source Timer], table.cell(colspan: 14)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x02 (Check Timer) \
CHKTTx \
Sets the EQ flag if the current value of timer (timer) is zero and if it is restart the timer to the previously set value.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [1], [0], table.cell(colspan: 5)[Tx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Timer], table.cell(colspan: 19)[Unused]
)}) \
#block(breakable: false,{text[=== OPCODE: 0x03 (Clear Timer) \
CLRTTx \
Sets the timer (timer) to zero.]
table(
columns: 32,
align: center,

[31], [30], [29], [28], [27], [26], [25], [24], [23], [22], [21], [20], [19], [18], [17], [16], [15], [14], [13], [12], [11], [10], [9], [8], [7], [6], [5], [4], [3], [2], [1], [0],
[1], [1], [0], [0], [0], [0], [1], [1], table.cell(colspan: 5)[Tx], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], [\*], 
table.cell(colspan: 3)[Type], table.cell(colspan: 5)[Opcode], table.cell(colspan: 5)[Destination Timer], table.cell(colspan: 19)[Unused]
)}) \
== Type 111 (RFU) \
This section is intentionally left blank as it is reserved for future use.\
All opcodes of this type will be interpreted as a NOP.
