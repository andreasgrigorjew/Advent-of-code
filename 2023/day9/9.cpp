#include <bits/stdc++.h>
using namespace std;
using ll = long long;

deque<ll> to_diff(deque<ll> &a) {
	deque<ll> ret;
	for (ll i = 0; i < ((ll) a.size()) - 1; i++) ret.push_back(a[i+1] - a[i]);
	return ret;
}

ll p1(vector<deque<ll>> &input) {
	ll result = 0;

	for (deque<ll> a: input) {
		vector<deque<ll>> ds;
		for (deque<ll> d = a; !all_of(d.begin(), d.end(), [](ll k){ return k == 0; }); d = to_diff(d)) {
			ds.push_back(d);
		}
		
		ll n = ds.size();
		ds[n-1].push_back(ds[n-1].back());
		for (ll k = n - 2; k >= 0; k--) {
			ds[k].push_back(ds[k].back() + ds[k+1].back());
		}
		result += ds[0].back();
	}

	return result;
}

ll p2(vector<deque<ll>> &input) {
	ll result = 0;

	for (deque<ll> a: input) {
		vector<deque<ll>> ds;
		for (deque<ll> d = a; !all_of(d.begin(), d.end(), [](ll k){ return k == 0; }); d = to_diff(d)) {
			ds.push_back(d);
		}

		ll n = ds.size();
		ds[n-1].push_front(ds[n-1].front());
		for (ll k = n - 2; k >= 0; k--) {
			ds[k].push_front(ds[k].front() - ds[k+1].front());
		}
		result += ds[0].front();
	}

	return result;
}

int main() {
	ios_base::sync_with_stdio(0);
	cin.tie(0);

	string line;
	vector<deque<ll>> input;
	while (getline(cin, line)) {
		istringstream iss(line);
		input.push_back(deque<ll>());

		ll n;
		while (iss >> n) {
			input.back().push_back(n);
		}
	}

	cout << p1(input) << endl;
	cout << p2(input) << endl;
	
	return 0;
}
