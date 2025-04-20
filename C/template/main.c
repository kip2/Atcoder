#include <stdio.h>
#include <string.h>

#define MAX_LEN 100

char *get_input(char *buffer, int size)
{
  if (fgets(buffer, size, stdin))
  {
    buffer[strcspn(buffer, "\n")] = '\0';
  }
  return buffer;
}

void solve()
{
  return;
}

int main(void)
{
  // sample code
  // char input[MAX_LEN];
  // get_input(input, MAX_LEN);
  // printf("input string: %s\n", input);
  solve();
  return 0;
}
