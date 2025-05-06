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

// To debug print
void debug_print_int_array(int *arr, int length, char *name) {
    printf("[DEBUG] %s :", name);
    for (int i = 0; i < length; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

// To debug print
void debug_print_int_arr_pointer(int *arr) {
    printf("[DEBUG] arr first pointer address: %p", (void *)arr);
}

// To debug print
void debug_print_int(int n, char *name) { printf("[DEBUG] %s: %d\n", name, n); }

void join_int_array(const int *arr, int length, char *out, const char *sep) {
    out[0] = '\0';

    for (int i = 0; i < length; i++) {
        char buffer[32];
        if (i > 0) {
            strcat(out, sep);
        }
        sprintf(buffer, "%d", arr[i]);
        strcat(out, buffer);
    }
}

void solve(int *arr, int x, int length, int *out, int *out_len) {
    int cnt = 0;
    for (int i = 0; i < length; i++) {
        if (arr[i] != x) {
            out[cnt] = arr[i];
            cnt++;
        }
    }
    *out_len = cnt;
}

int main(void) {
    int LINE = 2;
    int line[LINE];
    get_int_array(line, LINE);

    int N = line[0];
    int X = line[1];

    int A[N];
    get_int_array(A, N);

    int result[N];
    int result_len;

    solve(A, X, N, result, &result_len);

    char output[N * 2];
    join_int_array(result, result_len, output, " ");
    printf("%s\n", output);

    return 0;
}
