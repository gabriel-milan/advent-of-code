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
    // We need to sum three-measurement windows.
    // Every time the current window is larger than the previous one,
    // we increment the counter.
    int window_sum = 0;
    int window_sum_previous = 0;
    int window_size = 0;
    int window_d1 = 0;
    int window_d2 = 0;
    int window_d3 = 0;
    int current = 0;
    int count = 0;
    while (fscanf(input, "%d", &current) == 1)
    {
        // Subtract the oldest value from the sum.
        window_sum -= window_d3;

        // Delay values
        window_d3 = window_d2;
        window_d2 = window_d1;
        window_d1 = current;

        // Add current value to the window
        window_sum += current;

        // If window size is less than 3,
        // we need to wait until we have three values.
        if (window_size < 3)
        {
            window_size++;
            continue;
        }

        // If the current window is larger than the previous one,
        // we increment the counter.
        if (window_sum > window_sum_previous)
        {
            count++;
        }

        // Update the previous window sum.
        window_sum_previous = window_sum;
    }

    // Print the result.
    printf("%d\n", count);

    // Close the file.
    fclose(input);

    return 0;
}