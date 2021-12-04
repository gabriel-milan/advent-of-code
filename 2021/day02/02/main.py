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
    # units.
    #
    # We need to keep track of three
    # variables:
    # - horizontal
    # - depth
    # - aim
    #
    # What commands do:
    # - "up" decreases the aim by X
    # - "down" increases the aim by X
    # - "forward":
    #   - increases the horizontal by X
    #   - increases depth by aim * X
    #
    horizontal = 0
    depth = 0
    aim = 0

    with open(argv[1]) as f:
        for line in f.readlines():
            instruction, number = line.split()
            number = int(number)

            if instruction == "up":
                aim -= number
            elif instruction == "down":
                aim += number
            elif instruction == "forward":
                horizontal += number
                depth += aim * number

    print(f"The final position is {horizontal} {depth}")

    # Print them multiplied
    print(f"The final position is {horizontal * depth}")
