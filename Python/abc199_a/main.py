def solve(a, b, c):
    if a * a + b * b < c * c:
        return "Yes"
    else:
        return "No"

def main():
    a, b, c = map(int, input().split())
    print(solve(a, b, c))

if __name__ == "__main__":
    main()
