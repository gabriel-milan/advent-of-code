from sys import argv

if __name__ == "__main__":

    # There must be exactly one argument
    if len(argv) != 2:
        print("Usage: ./main.py <input_file>")
        exit(1)

    # File must exist
    try:
        with open(argv[1]) as f:
            pass
    except FileNotFoundError:
        print("File not found")
        exit(1)

    # Each line of the file contains
    # an instruction and a number
    # separated by a space.
    #
    # The instruction is either
    # "forward", "up" or "down".
    #
    # The number is the number of
    # units to move.
    #
    # "forward" adds to the horizontal position
    # "up" subtracts from the vertical position
    # "down" adds to the vertical position
    #
    # We need to keep track of
    # the current position
    horizontal = 0
    vertical = 0

    with open(argv[1]) as f:
        for line in f.readlines():
            instruction, number = line.split()
            number = int(number)

            if instruction == "forward":
                horizontal += number
            elif instruction == "up":
                vertical -= number
            elif instruction == "down":
                vertical += number

    print(f"The final position is {horizontal} {vertical}")

    # Print them multiplied
    print(f"The final position is {horizontal * vertical}")
