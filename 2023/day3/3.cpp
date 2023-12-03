#include <bits/stdc++.h>
using namespace std;
using ll = long long;

ll dx[3] = { -1, 0, 1 };

ll p1(vector<string> &inp) {
	ll n = inp.size();
	ll m = inp[0].size();

	ll sum = 0;

	auto outside = [&](ll x, ll y) {
		return x < 0 || x >= n || y < 0 || y >= m;
	};
	auto is_digit = [&](ll x, ll y) {
		return inp[x][y] >= '0' && inp[x][y] <= '9';
	};

	vector<vector<bool>> vis(n, vector<bool>(m, false));

	auto find_integer = [&](ll x, ll y) {
		// go left
		while (!outside(x, y-1) && is_digit(x, y-1)) y -= 1;

		ll sum = 0;
		for (; !outside(x, y) && is_digit(x, y); y++) {
			sum *= 10;
			sum += (inp[x][y] - '0');
			vis[x][y] = true;
		}

		return sum;
	};

	for (ll i = 0; i < n; i++) for (ll j = 0; j < m; j++) {
		if (is_digit(i, j)) continue;
		if (inp[i][j] == '.') continue;

		// inp[i][j] is a symbol
		for (ll x = 0; x < 3; x++) for (ll y = 0; y < 3; y++) {
			ll ni = i + dx[x];
			ll nj = j + dx[y];
			if (outside(ni, nj)) continue;
			if (is_digit(ni, nj) && !vis[ni][nj]) {
				sum += find_integer(ni, nj);
			}
		}
	}

	return sum;
}

ll p2(vector<string> &inp) {
	ll n = inp.size();
	ll m = inp[0].size();

	ll sum = 0;

	auto outside = [&](ll x, ll y) {
		return x < 0 || x >= n || y < 0 || y >= m;
	};
	auto is_digit = [&](ll x, ll y) {
		return inp[x][y] >= '0' && inp[x][y] <= '9';
	};

	vector<vector<bool>> vis(n, vector<bool>(m, false));

	auto find_integer = [&](ll x, ll y) {
		// go left
		while (!outside(x, y-1) && is_digit(x, y-1)) y -= 1;

		ll sum = 0;
		for (; !outside(x, y) && is_digit(x, y); y++) {
			sum *= 10;
			sum += (inp[x][y] - '0');
			vis[x][y] = true;
		}

		return sum;
	};

	for (ll i = 0; i < n; i++) for (ll j = 0; j < m; j++) {
		if (inp[i][j] != '*') continue;

		vis.clear();
		vis.assign(n, vector<bool>(m, false));

		ll ratio = 1;
		ll count = 0;
		for (ll x = 0; x < 3; x++) for (ll y = 0; y < 3; y++) {
			ll ni = i + dx[x];
			ll nj = j + dx[y];
			if (outside(ni, nj)) continue;
			if (is_digit(ni, nj) && !vis[ni][nj] && count < 2) {
				count++;
				ratio *= find_integer(ni, nj);
			}
		}
		if (count == 2) sum += ratio;
	}

	return sum;
}


int main() {
	ios_base::sync_with_stdio(0);
	cin.tie(0);

	string line;
	vector<string> inp;
	while (getline(cin, line)) {
		inp.push_back(line);
	}

	cout << p1(inp) << '\n';
	cout << p2(inp) << '\n';
	
	return 0;
}
