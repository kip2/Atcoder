#include <stdio.h>

int main(void)
{
  int a, b, c;
  scanf("%d %d %d", &a, &b, &c);

  if (a * a + b * b < c * c)
  {
    printf("Yes\n");
  }
  else
  {
    printf("No\n");
  }

  return 0;
}
