#include <bits/stdc++.h>
using namespace std;
using ll = long long;

ll p1(string &directions, map<string, pair<string, string>> &graph) {
	ll n = directions.size();

	string cur = "AAA";
	ll result = 0;
	for (; cur != "ZZZ"; result++)
		cur = (directions[result % n] == 'L' ? graph[cur].first : graph[cur].second);

	return result;
}

ll p2(string &directions, map<string, pair<string, string>> &graph) {
	ll n = directions.size();

	set<string> cur;
	set<string> end;
	for (auto [lft, rght]: graph) {
		if (lft[2] == 'A') cur.insert(lft);
		if (lft[2] == 'Z') end.insert(lft);
	}

	ll result = 1;
	for (string start: cur) {
		ll steps = 0;
		for (string s = start; end.count(s) == 0; steps++) {
			s = (directions[steps % n] == 'L' ? graph[s].first : graph[s].second);
		}
		result = result * steps / gcd(result, steps);
	}

	return result;
}


int main() {
	ios_base::sync_with_stdio(0);
	cin.tie(0);

	string directions;
	cin >> directions;

	string a;
	map<string, pair<string, string>> graph;
	while (cin >> a) {
		string eq, b, c;

		cin >> eq >> b >> c;
		b = b.substr(1, 3);
		c = c.substr(0, 3);

		graph[a] = make_pair(b, c);
	}

	cout << p1(directions, graph) << '\n';
	cout << p2(directions, graph) << '\n';

	return 0;
}
