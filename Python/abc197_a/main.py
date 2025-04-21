def solve(s):
    head = s[0:1]
    tail = s[1:]
    return tail + head


def get_input():
    return input()


def main():
    # case: get input to int nums
    # a, b, c = map(int, input().split())
    inpt = get_input()
    print(solve(inpt))


if __name__ == "__main__":
    main()
