def solve():
    # todo: implemented me!
    pass


def get_input():
    """
    Reads a single line of input from standard input and returns it as a string.

    Returns:
      str: The input line as a string.
    """
    return input()


def get_input_int():
    return int(input())


def get_int_list():
    """
    Reads a line of input, splits it by whitespace, converts each token to an integer,
    and returns the result as a list of integers.

    Returns:
      list[int]: A list of integers parsed from the input line.
    """
    return list(map(int, input().split))


def main():
    print(solve())


if __name__ == "__main__":
    main()
