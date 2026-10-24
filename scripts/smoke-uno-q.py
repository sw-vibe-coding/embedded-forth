#!/usr/bin/env python3
"""Verify the deployed Forth REPL and baked-in demos over passwordless SSH."""
import subprocess
import sys


def check(destination, port, source, expected):
    result = subprocess.run(
        ["ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=10", "-p", port,
         destination, "./embedded-forth/embedded-forth"],
        input=source, text=True, capture_output=True, timeout=30,
    )
    if result.returncode or result.stderr:
        raise SystemExit(f"SSH/REPL failed: {result.stderr}")
    if not result.stdout.endswith(expected):
        raise SystemExit(f"Unexpected Forth output:\n{result.stdout}")
    for diagnostic in ["unknown token", "stack empty", "underflow", "not compiling"]:
        if diagnostic in result.stdout:
            raise SystemExit(f"Forth diagnostic: {result.stdout}")
    return result.stdout


def main():
    destination = sys.argv[1] if len(sys.argv) > 1 else "arduino@localhost"
    port = sys.argv[2] if len(sys.argv) > 2 else "2222"
    check(destination, port, "2 3 + .\n: SQUARE\nDUP * ;\n9 SQUARE .\n:quit\n", "81 \n")
    print("PASS arithmetic and multiline definitions")
    check(destination, port, "MENU\n1\n10 FIB .\n:quit\n", "55 \n")
    print("PASS MENU / fib.fth and retained FIB")
    check(destination, port, "menu\nfact.fth\n6 FACT .\n:quit\n", "720 \n")
    print("PASS MENU / fact.fth and retained FACT")
    text = check(destination, port, "MENU\n3\n:quit\n", "Buzz\n")
    if text.splitlines().count("FizzBuzz") != 6:
        raise SystemExit("FizzBuzz did not emit all six multiples of 15 through 100")
    print("PASS MENU / fizzbuzz.fth through 100")


if __name__ == "__main__":
    main()
