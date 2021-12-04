#include <stdio.h>

int main(int argc, char **argv)
{
    // There must be at least one argument,
    // which is the input file name.
    if (argc < 2)
    {
        printf("Usage: %s <input file>\n", argv[0]);
        return 1;
    }

    // Open the input file.
    FILE *input = fopen(argv[1], "r");

    // Check if the file was opened.
    if (input == NULL)
    {
        printf("Could not open file %s\n", argv[1]);
        return 1;
    }

    // Each line of the file contains a single integer.
    // We need to count how many times the next integer
    // is larger than the previous one.
    int previous = __INT_MAX__;
    int current = 0;
    int count = 0;
    while (fscanf(input, "%d", &current) == 1)
    {
        if (current > previous)
        {
            count++;
        }
        previous = current;
    }

    // Print the result.
    printf("%d\n", count);

    // Close the file.
    fclose(input);

    return 0;
}