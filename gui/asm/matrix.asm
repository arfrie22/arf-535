.prog

ldl r1 1
ldh r1 0

ldl r2 2
ldh r2 0

ldr r3 r2

row_loop:
sub r3 r3 r1

ldr r4 r2

col_loop:
sub r4 r4 r1

cmp r4
bgt p:col_loop

cmp r3
bgt p:row_loop

; str r1 d:v2
; add r1 r1 r1

; ldl r3 1
; ldh r3 0

; loop:
; ldr r2 d:v1
; add r2 r2 r3
; str r2 d:v1

; b p:loop
; str r4 d:v1

.data
amat1  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat2  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat3  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat4  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat5  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat6  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat7  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat8  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat9  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat10 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1

amat1  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat2  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat3  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat4  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat5  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat6  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat7  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat8  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
amat9  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
amat10 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1

cmat 0#100