\ Fibonacci: FIB ( n -- fibonacci ), using an iterative loop.
: FIB 0 1 ROT 0 ?DO OVER + SWAP LOOP DROP ;
: FIB-DEMO 12 0 DO I FIB . LOOP CR ;
." First 12 Fibonacci numbers:" CR
FIB-DEMO
