#include <bits/stdc++.h>
using namespace std;
using ll = long long;

ll part1() {
  ll sum = 0;
  for (string s; getline(cin, s);) {
    int l = s[s.find_first_of("123456789")]-'0';
    reverse(s.begin(), s.end());
    int r = s[s.find_first_of("123456789")]-'0';
    sum += l*10+r;
  }
  return sum;
}

ll part2() {
  ll sum = 0;
  for (string s; getline(cin, s);) {
	vector<string> to_find = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"};
	bool first = true;
	ll l = 0;
	ll r = 0;
	for (ll i = 0; i < (ll) s.size(); i++) {
		for (ll j = 0; j < 18; j++) {
			string t = to_find[j];
			if (i + (ll) t.size() > (ll) s.size()) continue;
			if (s.substr(i, t.size()) == t) {
				if (first) {
					first = false;
					l = (j%9)+1;
				}
				r = (j%9)+1;
			}
		}
	}
	sum += l*10+r;
  }
  return sum;
}

int main() {
  cin.tie(0);
  ios_base::sync_with_stdio(0);
  cout << part2() << '\n';
  return 0;
}
