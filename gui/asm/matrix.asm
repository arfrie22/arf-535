.prog

ldl sp 0xF000
ldh sp 0

ldl r9 0xF000
ldh r9 0

; matA
ldl r1 0
ldh r1 0

; matB
ldl r2 16
ldh r2 0

; matC
ldl r3 32
ldh r3 0

; A = rows
ldl r4 4
ldh r4 0

; B
ldl r5 4
ldh r5 0

; C = cols
ldl r6 4
ldh r6 0

; multiply matrix r1 = &matA, r2 = &matB, r3 = &matC, r4 = A, r5 = B, r6 = C
matrix_multiply:
;i
push r7
;j
push r8
;k
push r9
;sum
push r10
;A_i,k
push r11
;B_k,j
push r12
;mult result
push r13
;&A_i,k
push r14
;&B_k,j
push r15
;&C_i,j
push r16

; AxB * BxC matrix multiplication

; matC_i,j = sum k=1 to m (matA_i,k * matB_k,j)

; i = 0
xor r7 r7 r7

;&C_i,j = matC
ldr r16 r3

row_loop:
; j = 0
xor r8 r8 r8

col_loop:
; sum = 0
xor r10 r10 r10
; k = 0
xor r9 r9 r9

;&A_i,k = (i * B) + k=0 + &matA
mul r14 r7 r5
add r14 r14 r1
;&B_k,j = (k=0 * C) + j + &matA => &matA + j
add r15 r8 r2

sum_loop:

;A_i,k = *&A_i,k
ldr r11 [r14]
;B_k,j = *&B_k,j
ldr r12 [r15]

;mult result = A_i,k * B_k,j
mul r13 r11 r12
;sum += mult result
add r10 r10 r13

;&A_i,k++
inc r14
;&B_k,j += C
add r15 r15 r6

;k++
inc r9
; if k < B
cmp r9 r5
blt p:sum_loop

;*&C_i,j = sum
str [r16] r10
;&C_i,j++
inc r16

;j++
inc r8
; if j < C
cmp r8 r6
blt p:col_loop

;i++
inc r7
; if i < A
cmp r7 r4
blt p:row_loop


;&B_k,j
pop r15
;&A_i,k
pop r14
;mult result
pop r13
;B_k,j
pop r12
;A_i,k
pop r11
;sum
pop r10
;k
pop r9
;j
pop r8
;i
pop r7

.data
amat1 2#1 0#1 0#1 0#1
amat2 0#1 1#1 0#1 0#1
amat3 0#1 0#1 1#1 0#1
amat4 0#1 0#1 0#1 1#1

bmat1 1#1 0#1 0#1 0#1
bmat2 0#1 1#1 0#1 0#1
bmat3 0#1 0#1 1#1 0#1
bmat4 0#1 0#1 0#1 1#1

cmat 1#16

; amat1  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; amat2  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; amat3  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; amat4  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; amat5  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; amat6  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; amat7  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; amat8  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; amat9  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; amat10 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1

; bmat1  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; bmat2  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; bmat3  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; bmat4  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; bmat5  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; bmat6  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; bmat7  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; bmat8  42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1
; bmat9  24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1
; bmat10 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1 42#1 24#1

; cmat 0#100