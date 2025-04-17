.prog
ldr r1 d:v1
str r1 d:v2
add r1 r1 r1

ldl r3 1
ldh r3 0

loop:
ldr r2 d:v1
add r2 r2 r3
str r2 d:v1

b p:loop
str r4 d:v1

.data
v1 42#1
v2 0#1