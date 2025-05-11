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
This architecture is specialized for DSP applications. It is a clean sheet architecture based on parts from ARM, RISC-V, and x86. The architecture is a 32 -it Harvard architecture with 32 32-bit integers registers and 32 single-precision floating-point registers. The registers are in little endian. \

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
+ Floating-Point Infinity (FINF)
+ Floating-Point Zero (FZ)
+ Floating-Point Not-A-Number (FNAN)
+ Floating-Point Positive (FPOS)
The rest of the bits are reserved for future use.


All 32 floating-point registers are general-purpose registers with no special meaning. \

There are 32 general-purpose timers, which are automatically decreased every clock cycle. Timers automatically restart at their previous set value after they are read to be zero. They do not fire interrupts; they are just clock-cycle counters that count down. \


#pagebreak()

= Instructions

- Ra defines an integer register
- [Ra + I << S] defines a register indirect into data memory at Ra + the value of I shifted by S ([Ra] is interpreted as [Ra + 0 << 0])
- [Ra + Rb << S] defines a double register indirect into data memory at Ra + the value of Rb shifted by S
- Fa defines a floating-point register
- Ta defines a timer
- _label_ is a label that represents an address
- d:0xYYYYYYYY is an auto-generated label for 0xYYYYYYYY in the data memory
- p:0xYYYYYYYY is an auto-generated label for 0xYYYYYYYY in the program memory
- Adding the optional l to the end of a branch instruction will update the link register to the next instruction.
- Adding the optional c to the end of a supported ALU operation will update the status register.


I removed the duplicate instructions that update the link register and turned it into a bit that needs to be set in the instruction. That removed the duplicate code. I ended up implementing all the instructions, but most of them were not used in my benchmark. I added a lot of floating-point operations based on the arm operations, but I ended up not needing them after reading more about the Goertzel algorithm, and I could precompute all trigonometric functions.

#include "instructions.typ"

#pagebreak()

= Memory
There are 2 64-line caches, one for memory and the other for data. Where each line is 4 words. The size of the benchmarks was made to be large enough so it will be bigger than the cache. \
The memory is split into two distinct banks, where the program memory is what is loaded from the ASM/machine code and is in DRAM (100 cycles). The data memory bank will be in SRAM (10 cycles). \
Each bank will store 65,536 words (2^16). \
The cache will be a write-though, no allocate direct cache. There will be a 6-bit index, a 2-bit offset, and an-8 bit tag. There will be a separate cache for data and program memory.

The memory address is the lower 16-bits; any of the higher 16-bits are discarded.

#pagebreak()

= Simulator GUI

To use the simulator, write the assembly in the `/asm` folder under the `GUI` folder. The assembly file requires a `.prog` section and optionally a `.data` section. If you need the ADCs, you can enable them to read from a `.wav` file placed in the `/wav` folder. The clock rate is set to 88,200,000 Hz to be a multiple of the normal audio sampling frequency of 44.1 kHz. Currently, the simulator has no support for writing the DAC to a `.wav` output. The cache and pipeline can be disabled (the pipeline does this by limiting the normal pipeline to allow for one instruction inside of it). The assembly file then must be assembled into a `.o` file, which is placed in the `compiled` folder. After it has been assembled, it can be loaded in.

#figure(
  image("images/main.png", width: 80%),
  caption: [
    The main view of my GUI after loading the `dtmf` program using the `dtmf` wav file on the first ADC register
  ],
)

\
The panes that contain the status can be moved around by dragging on the name in the tab or the header above the table. Any integer number is displayed in hexadecimal, except for floating-point numbers, which are displayed as the base-10 representation. The dividers between the tables can be dragged to be resized.

#figure(
  image("images/moved.png", width: 80%),
  caption: [
    Panes moved around
  ],
)

\
Once a program is loaded in, it can be single-stepped, which will run a single clock cycle, or it can be run until it hits a trap. When the `Run` button is pressed, the UI will update every second, and the button is replaced with a `Cancel` button to stop the simulator. This feature is there in case the assembly has an infinite loop, as well as for making sure the code is working properly while still letting it run.

#figure(
  image("images/ran.png", width: 80%),
  caption: [
    GUI after cycles
  ],
)


#pagebreak()

= Simulator Design

There are 3 parts to the simulator GUI: the UI code, the simulator code, and the assembler code. The UI uses `egui` to create all the widgets and many `egui_table`-based widgets for all the displays. The movable panes are done with `egui_pane` which handles the dragging and splitting.

\
The assembler is made using the `pest` library. The syntax is generated using a Python script that reads the instructions, which are stored in a JSON file to allow the base code for the instructions to be autogenerated and kept up-to-date with the instruction manual. It breaks up each instruction line into a whole instruction. It does not first parse the mnemonic, then find the arguments that would work for it. This was done to make the parsing simpler and easier to auto-gen. The downside is it can result in less than helpful assembler errors at times.

\
A big part of the design is focused around code-gen. The instruction JSON was made to be detailed enough so that all trivial parts of the instructions could be generated (every part except the execute pipeline stage). This made it very easy to add new instructions, change the order of them, or their requirements in the assembly language. I switched the write-ups from Google Docs to Typst to allow for easy generation of the instruction page while being easier to get a good-looking format than LaTeX.

\
The simulator code itself is the library that both the assembler library and the GUI use. This was done to keep the simulator as being the bare minimum, all-encompassing part that runs the simulation. I didn't want to split out common parts that both the simulator and assembler would use, as you can use the simulator without the assembler, and doing so would require fewer dependencies.

#pagebreak()

= Software Engineering

I chose rust for this project for a few reasons. I wanted the code to be fast, which ruled out a purely interpreted language. Furthermore, I also vastly prefer rust's semantics when it comes to errors and nullable values. I prefer those to be explicit rather than implicit, like C-like languages. I then chose to keep the whole project in rust, including the parser and GUI, as it allowed me to only code-gen the instructions once rather than having to keep the generations synced between different languages. It also made it easier to bounce between files and update a function if it breaks in code or change what a data type returns. The code-gen was done in Python to make it easy. If I were to work on it more, I would convert the Python over to rust build scripts to do everything at build time. This would ensure that every time I compile the code, it would be up-to-date without having to re-run the Python script.

\
The version control was done in git. I would push changes once when I had working code and made a significant change. This being one that is done, or at least I don't plan on immediately writing more for that change. The generated rust code was kept in; however, the assembled code was gitignored to prevent it from clogging the repo. That is because if the ISA was being changed, it would become out of date. So if someone wanted to download the code, they would be forced to generate up-to-date `.o` files.

\
To manage the project independently, I started by getting the bare minimum up and running---what I needed to get done for the demos. This left me with a simulator with no GUI or assembler. From there, I knew I just needed to sit down for a while and write all the tables for the GUI. I spent the time making a GUI that would work and wasn't the most pretty, fully form over function. The GUI ended up being the biggest struggle, as I spent a bit trying to get a different UI library to work before switching to egui. Working by myself made integration much easier. Since I spent most of the time writing code sequentially rather than in parallel, I never had to worry about integration details, as I would start by making sure it would integrate. However, it did introduce the issue that I was the only person who would check the code, and therefore, if I had a wrong idea, both my code and what I would do to check it would be wrong.

#pagebreak()

= Performance

There are 4 benchmarks:

The first one is `100m.asm`. This one uses a 100,000,00 cycle timer to benchmark how long 100,000,000 stalls take to execute. This benchmark completes in about 4.65 seconds.

\
The next benchmark (`matrix.asm`) is a 50x50 matrix multiplication, which takes 3,946,259 cycles (about 0.468 seconds) to execute with both the pipeline and cache. With no pipeline, it takes 6,032,027 cycles, which means the pipeline sped up the program by a factor of 1.529. Without the cache, it takes 12,002,351 cycles, which is a factor of 3.041. With neither, it takes 16,860,848 cycles, which means the cache speeds up by a factor of 2.79522 and the pipeline speeds up by a factor of 1.4048 when compared to no pipeline or cache.

\
The third benchmark (`sort.asm`) is a randomized 1,000 element list that is sorted using bubble sort. It takes 29,140,875 cycles to complete with both pipeline and cache. 39,487,342 cycles without pipeline. 80,921,777 cycles without cache. 111,415,039 with neither. Enabling the pipeline is a 1.37682 time improvement with no cache and 1.35505 times when there is already cache. Enabling the cache is a 2.82154 time improvement with no pipeline and 2.77692 times when there is already a pipeline. This means that the cache is more of a substantial improvement than, as it had a larger speed-up factor in both cases.

\
The last benchmark (`dtmf.asm`) is more of a test of the analog-specific and floating-point features. This benchmark samples the ADC to decode DTMF tones to get what number was being typed. DTMF is what phones use and what gives each key press its distinct sound. It is two overlapping frequencies where there is a 4x4 grid made up of the possible frequencies. The target sample frequency is 44.1 kHz, which matches the `dtmf.wav` file sample rate. The program takes 1,506,066,223 cycles to run and is able to complete each sample loop in about 260 cycles, which means about 90% of the time it spends idling. That is because it needs to sample every 2,000 cycles to reach its real-time target. This means that the clock rate of the CPU can be decreased as there is over 1,000 cycles of headroom. It takes 83.6628s to simulate the process of a 17s file. However, this overhead lets the pipeline and clock be disabled while still hitting its target. 

With no cache pipeline, it takes 1,506,066,562 cycles. There is very little improvement because most of the time is being stalled to read a new sample. The pipeline has a minimal effect on the program. It is only on the last iteration that makes the difference in speed. The cache was a different story. With no cache, the program was unable to meet its real-time target due to fetching the instructions from the slow DRAM and therefore could not parse the data.

#pagebreak()

= What I Learned

Since I did the project by myself, I was able to learn many facets of making a simulator. I got a lot more comfortable writing a parsing grammar, as well as learned how to avoid many pitfalls. The order you parse matters a lot if it is ambiguous. For my number, parsing the order as a digit then a hex digit; however, since hex digits start with `0x` it would fail to parse since it would start parsing the number `0` then read the `x` and throw the error. This was solved by making the order hex digit before digit. There were many small pitfalls like this I had to avoid when making the parser. Since most instructions' first argument is a register, it avoids any issues that would arise from parsing two instructions with the same mnemonic.

\
There was a lot of thinking originally about what instructions I wanted to do before I decided that I should spend the time writing code-gen. This was one of my favorite decisions. It took the pressure off instructions as I could change them around as needed, only having to update the JSON file.

\
I wish I had more time to write tests. A lot of my issues later on came from improperly implemented parts of the code or regression on previous issues. However, I didn't want to spend too much time on it, as making progress on the actual simulator was just as, if not more, important. The best testing came from the more throughout and planned out programs, where I would use a bunch of functions and realize they were bugged, rather than just testing a single instruction in a vacuum.

\
I got a lot more comfortable with parts of rust's typing system. I used to avoid traits (similar to interfaces in Java) as it can become a mess, with many ways to deal with it: generics, cells, or boxes. However, I feel like I have a better handle on when I would want to use one of them. Normally when I would do a project, I would just stick with all Rc-RefCells or Arc-Mutexes for async projects; however, with this, I used boxes, which just put the data on a heap like a refcell, but aren't designed for sharing data between structs.

\
I also learned a lot about what goes into making an architecture and the planning that needs to be done before a chipped was planned. While writing my simulator, I added instructions and changed major parts of how my ADC/DAC system would work. Even in its current form, I do not think it would be ready to be 
fabricated, it would require much more testing for both functionality and testing in analog programs.

\
Writing this simulator gave me the confidence to start a project that I had thought of a long time ago, which requires me to write a Game Boy emulator. I feel much more prepared to take on that task after finishing a simulator of my own. While there are differences between a simulator and an emulator, I have learned a lot of the jargon that comes from computer architecture.