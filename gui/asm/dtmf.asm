.prog

ldl r0 2000
ldh r0 0

SETT T0 r0

; number of total loops
ldl r10 341
ldh r10 0
; output address
lea r9 d:output

; last r11
ldl r11 0xFF
ldh r11 0

read_samples:
; Number of samples
ldl r0 2205

; data index
xor r1 r1 r1
; set all floats to 0
cst f2 r1
cst f3 r1
cst f4 r1
cst f5 r1
cst f6 r1
cst f7 r1
cst f8 r1
cst f9 r1
cst f10 r1
cst f11 r1
cst f12 r1
cst f13 r1
cst f14 r1
cst f15 r1
cst f16 r1
cst f17 r1

read_sample_loop:
; TRAP
STALL T0
CST f0 a1

; 697 f2 f3
ldl f1 0xba56
ldh f1 0x3ffe
; coeff *= s_prev
mul f1 f1 f2
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f3
; s_prev2 = s_prev
ldr f3 f2
; s_prev = coeff
ldr f2 f1

; 770 f4 f5
ldl f1 0x6bba
ldh f1 0x3ffe
; coeff *= s_prev
mul f1 f1 f4
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f5
; s_prev2 = s_prev
ldr f5 f4
; s_prev = coeff
ldr f4 f1

; 852 f6 f7
ldl f1 0x14a8
ldh f1 0x3ffe
; coeff *= s_prev
mul f1 f1 f6
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f7
; s_prev2 = s_prev
ldr f7 f6
; s_prev = coeff
ldr f6 f1

; 941 f8 f9
ldl f1 0xb522
ldh f1 0x3ffd
; coeff *= s_prev
mul f1 f1 f8
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f9
; s_prev2 = s_prev
ldr f9 f8
; s_prev = coeff
ldr f8 f1

; 1209 f10 f11
ldl f1 0x447d
ldh f1 0x3ffc
; coeff *= s_prev
mul f1 f1 f10
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f11
; s_prev2 = s_prev
ldr f11 f10
; s_prev = coeff
ldr f10 f1

; 1336 f12 f13
ldl f1 0x593f
ldh f1 0x3ffb
; coeff *= s_prev
mul f1 f1 f12
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f13
; s_prev2 = s_prev
ldr f13 f12
; s_prev = coeff
ldr f12 f1

; 1477 f14 f15
ldl f1 0x5467
ldh f1 0x3ffa
; coeff *= s_prev
mul f1 f1 f14
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f15
; s_prev2 = s_prev
ldr f15 f14
; s_prev = coeff
ldr f14 f1

; 1633 f16 f17
ldl f1 0x0b16
ldh f1 0x3ff9
; coeff *= s_prev
mul f1 f1 f16
; coeff += sample
add f1 f1 f0
; coeff -= s_prev2
sub f1 f1 f17
; s_prev2 = s_prev
ldr f17 f16
; s_prev = coeff
ldr f16 f1



inc r1
cmp r1 r0
blt p:read_sample_loop

decode_dtmf:
; 697 f2 f3
ldl f1 0xba56
ldh f1 0x3ffe
; coeff *= s_prev
mul f1 f1 f2
; coeff *= s_prev_2
mul f1 f1 f3
; s_prev *= s_prev
mul f2 f2 f2
; s_prev_2 *= s_prev_2
mul f3 f3 f3
; s_prev += s_prev_2
add f2 f2 f3
; s_prev -= coeff
sub f2 f2 f1

; 770 f4 f5
ldl f1 0x6bba
ldh f1 0x3ffe
; coeff *= s_prev
mul f1 f1 f4
; coeff *= s_prev_2
mul f1 f1 f5
; s_prev *= s_prev
mul f4 f4 f4
; s_prev_2 *= s_prev_2
mul f5 f5 f5
; s_prev += s_prev_2
add f4 f4 f5
; s_prev -= coeff
sub f4 f4 f1

; 852 f6 f7
ldl f1 0x14a8
ldh f1 0x3ffe
; coeff *= s_prev
mul f1 f1 f6
; coeff *= s_prev_2
mul f1 f1 f7
; s_prev *= s_prev
mul f6 f6 f6
; s_prev_2 *= s_prev_2
mul f7 f7 f7
; s_prev += s_prev_2
add f6 f6 f7
; s_prev -= coeff
sub f6 f6 f1

; 941 f8 f9
ldl f1 0xb522
ldh f1 0x3ffd
; coeff *= s_prev
mul f1 f1 f8
; coeff *= s_prev_2
mul f1 f1 f9
; s_prev *= s_prev
mul f8 f8 f8
; s_prev_2 *= s_prev_2
mul f9 f9 f9
; s_prev += s_prev_2
add f8 f8 f9
; s_prev -= coeff
sub f8 f8 f1

; 1209 f10 f11
ldl f1 0x447d
ldh f1 0x3ffc
; coeff *= s_prev
mul f1 f1 f10
; coeff *= s_prev_2
mul f1 f1 f11
; s_prev *= s_prev
mul f10 f10 f10
; s_prev_2 *= s_prev_2
mul f11 f11 f11
; s_prev += s_prev_2
add f10 f10 f11
; s_prev -= coeff
sub f10 f10 f1

; 1336 f12 f13
ldl f1 0x593f
ldh f1 0x3ffb
; coeff *= s_prev
mul f1 f1 f12
; coeff *= s_prev_2
mul f1 f1 f13
; s_prev *= s_prev
mul f12 f12 f12
; s_prev_2 *= s_prev_2
mul f13 f13 f13
; s_prev += s_prev_2
add f12 f12 f13
; s_prev -= coeff
sub f12 f12 f1

; 1477 f14 f15
ldl f1 0x5467
ldh f1 0x3ffa
; coeff *= s_prev
mul f1 f1 f14
; coeff *= s_prev_2
mul f1 f1 f15
; s_prev *= s_prev
mul f14 f14 f14
; s_prev_2 *= s_prev_2
mul f15 f15 f15
; s_prev += s_prev_2
add f14 f14 f15
; s_prev -= coeff
sub f14 f14 f1

; 1633 f16 f17
ldl f1 0x0b16
ldh f1 0x3ff9
; coeff *= s_prev
mul f1 f1 f16
; coeff *= s_prev_2
mul f1 f1 f17
; s_prev *= s_prev
mul f16 f16 f16
; s_prev_2 *= s_prev_2
mul f17 f17 f17
; s_prev += s_prev_2
add f16 f16 f17
; s_prev -= coeff
sub f16 f16 f1


; 697
xor r1 r1 r1
ldr f0 f2
cmp f4 f0
blt p:after_770
;770
ldl r1 1
ldr f0 f4

after_770:
cmp f6 f0
blt p:after_852
;852
ldl r1 2
ldr f0 f6

after_852:
cmp f8 f0
blt p:after_941
;941
ldl r1 3
ldr f0 f8

after_941:
;1209
xor r2 r2 r2
ldr f0 f10
cmp f12 f0
blt p:after_1336
;1336
ldl r2 1
ldr f0 f12

after_1336: 
cmp f14 f0
blt p:after_1477
;1477
ldl r2 2
ldr f0 f14

after_1477: 
cmp f16 f0
blt p:after_1633
;1633
ldl r2 3
ldr f0 f16
after_1633:

; f2 epsilon 1e-5
ldl f2 0xbf98
ldh f2 0x33d6

add f0 f0 f1
cmp f0 f2
blt p:no_value

ldr r1 d[r2 + r1 << 2]
; skip if value is a repeat with no silence
cmp r1 r11
beq p:dec_loop

ldr r11 r1
str d[r9] r1
inc r9

dec_loop:
dec r10
cmp r10
bgt p:read_samples
TRAP

no_value:
ldl r11 0xFF
b p:dec_loop

.data
; * = E, # = F
dtmf    0x1#1 0x2#1 0x3#1 0xA#1
        0x4#1 0x5#1 0x6#1 0xB#1
        0x7#1 0x8#1 0x9#1 0xC#1
        0xE#1 0x0#1 0xF#1 0xD#1
output  0#100

; samples 0#2205