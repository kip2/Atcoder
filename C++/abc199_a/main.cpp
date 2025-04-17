#include <iostream>
using namespace std;

string solve(int a, int b, int c)
{
  return (a * a + b * b < c * c) ? "Yes" : "No";
}

int main()
{
  int a, b, c;
  cin >> a >> b >> c;
  cout << solve(a, b, c) << endl;
  return 0;
}
