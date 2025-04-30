def solve(n, a, b):
    sum = 0
    for i in range(n):
        sum = sum + a[i] * b[i]
    if sum == 0:
        return "Yes"
    else:
        return "No"


def get_input_int():
    return int(input())


def get_int_list():
    return list(map(int, input().split()))


def main():
    n = get_input_int()
    a = get_int_list()
    b = get_int_list()
    print(solve(n, a, b))


if __name__ == "__main__":
    main()
