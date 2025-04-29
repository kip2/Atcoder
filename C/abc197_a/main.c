#include <stdio.h>
#include <string.h>

void solve(const char *input, char *answer) {
    char head = input[0];
    char tail[3];

    strncpy(tail, &input[1], 2);
    tail[2] = '\0';

    strncpy(answer, &tail[0], 2);
    answer[2] = head;
    answer[3] = '\0';
}

int main(void) {
    char input[4];
    char answer[4];

    scanf("%3s", input);

    solve(input, answer);

    printf("%s\n", answer);
    return 0;
}
