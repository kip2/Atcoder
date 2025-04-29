#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_INPUT 100
#define MAX_NUMS 20

int get_input_int() {
    int n;
    scanf("%d", &n);
    return n;
}

void get_int_array(int *arr, int max_size) {
    for (int i = 0; i < max_size; i++) {
        scanf("%d", &arr[i]);
    }
}

char *solve(int size, int *a_arr, int *b_arr) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += a_arr[i] * b_arr[i];
    }

    if (sum == 0) {
        return "Yes";
    }
    return "No";
}

int main(void) {
    int N = get_input_int();

    int A[N];
    int B[N];
    get_int_array(A, N);
    get_int_array(B, N);

    char *result = solve(N, A, B);
    printf("%s\n", result);

    return 0;
}