#include <bits/stdc++.h>
using namespace std;
using ll = long long;

ll p1(vector<pair<set<ll>, set<ll>>> &inp) {
	ll sum = 0;
	for (ll i = 0; i < (ll) inp.size(); i++) {
		ll matches = 0;
		for (ll k: inp[i].second) if (inp[i].first.count(k)) matches++;
		if (matches)
			sum += (1LL << (matches-1));
	}
	return sum;
}

ll p2(vector<pair<set<ll>, set<ll>>> &inp) {
	ll n = inp.size();

	vector<ll> copies(n, 1);
	for (ll i = 0; i < n; i++) {
		ll matches = 0;
		for (ll k: inp[i].second) if (inp[i].first.count(k)) matches++;

		for (ll j = i+1; j < min(n, i + matches + 1); j++) copies[j] += copies[i];
	}
	return accumulate(copies.begin(), copies.end(), 0LL);
}

int main() {
	ios_base::sync_with_stdio(0);
	cin.tie(0);

	vector<pair<set<ll>, set<ll>>> inp;
	string line;
	while (getline(cin, line)) {
		inp.push_back(make_pair(set<ll>(), set<ll>()));
		bool found_delim = false;
		size_t start = line.find(":");

		ll num = 0;
		for (ll i = start + 1; i < (ll) line.size(); i++) {
			if (line[i] == '|') {
				found_delim = true;
				continue;
			}

			if (line[i] >= '0' && line[i] <= '9') {
				num *= 10;
				num += line[i] - '0';
			}
			if (line[i] == ' ' || i + 1 >= (ll) line.size()) {
				(found_delim ? inp.back().second : inp.back().first).insert(num);
				num = 0;
			}
		}

		inp.back().first.erase(0);
		inp.back().second.erase(0);
	}
	cout << endl;

	cout << p1(inp) << '\n';
	cout << p2(inp) << '\n';

	return 0;
}
