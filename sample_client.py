#!/usr/bin/env python3

from struct import pack
from random import randint
from sys import stderr

clues = open(0, "rb")
guess_output = open(1, "wb")


def submit_guess(pegs):
    guess_output.write(pack("B"*8, *pegs))
    guess_output.flush()
    cc, ci = clues.read(2)
    print(f"Guessed {cc} pegs with correct colour, correct position and {ci} pegs with correct colour and incorrect position", file=stderr)
    
guess = tuple(randint(0, 255) for _ in range(8))
submit_guess(guess)
submit_guess((62, 123, 78, 79, 0, 1, 255, 169))
