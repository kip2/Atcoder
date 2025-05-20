def solve(lst, x):
    return list(filter(lambda a: a != x, lst))


def get_input():
    return input()


def get_input_int():
    return int(input())


def get_int_list():
    return list(map(int, input().split()))


def join_space_int_list(lst):
    return " ".join(map(str, lst))


def main():
    list = get_int_list()
    _n = list[0]
    x = list[1]

    a = get_int_list()

    result = join_space_int_list(solve(a, x))
    print(result)


if __name__ == "__main__":
    main()
