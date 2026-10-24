\ Factorial: FACT ( n -- n! ). Keep inputs within 0..12 for 32-bit cells.
: FACT 1 SWAP 1 + 1 ?DO I * LOOP ;
: FACT-DEMO 11 0 DO I . I FACT . CR LOOP ;
." n and n! for 0 through 10:" CR
FACT-DEMO
