#include <stdio.h>
#include <string.h>

#define MAX_LEN 100

int get_input_int() {
    int n;
    scanf("%d", &n);
    return n;
}

char *get_input(char *buffer, int size) {
    if (fgets(buffer, size, stdin)) {
        buffer[strcspn(buffer, "\n")] = '\0';
    }
    return buffer;
}

void get_int_array(int *arr, int max_size) {
    for (int i = 0; i < max_size; i++) {
        scanf("%d", &arr[i]);
    }
}

void solve() { return; }

int main(void) {
    // sample code
    // Get a array of int type.
    // int X[MAX_LEN];
    // get_int_array(X, MAX_LEN);

    // sample code2
    // Get a int value.
    // int N = get_input_int();

    // sample code 3
    // char input[MAX_LEN];
    // get_input(input, MAX_LEN);
    // printf("input string: %s\n", input);

    solve();
    return 0;
}
