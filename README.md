# Rules
- A code is an ordered sequence of 8 coloured pegs. There are 256 possible colours and a code may contain multiple pegs of the same colour.
- The goal is to guess the correct sequence in as few guesses as possible.
- After each guess, you are told: (1) the number of pegs in your guess that are of the correct colour and are in the correct position and (2) the number of pegs in your guess that are of the correct colour and are in the incorrect position.

# Format

- Pegs are represented as bytes (256 possible colours)
- Guesses are represented as a sequence of 8 bytes (pegs)
- A response is two bytes encoding two unsigned 8-bit integers. The first number is correct colour/correct position the second number is correct colour/incorrect position.

# Protocol
(1) Write an 8 byte sequence to stdout
(2) Read two bytes from stdin. If bytes correspond to the integers (8 0), then terminate. Otherwise goto (1)

# Example
- See sample_client.py
- Invoke with `cd checker; cargo run ../sample_client.py`
