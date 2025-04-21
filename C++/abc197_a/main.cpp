#include <iostream>
using namespace std;

string solve(string s)
{
  char head = s[0];
  string tail = s.substr(1);
  return tail + head;
}

int main()
{
  string s;
  cin >> s;
  cout << solve(s) << endl;
  return 0;
}
