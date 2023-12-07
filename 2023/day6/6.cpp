#include <bits/stdc++.h>
using namespace std;
using ll = long long;

ll p1(vector<ll> &time, vector<ll> &distance) {
	ll n = time.size();

	ll result = 1;
	
	for (ll i = 0; i < n; i++) {
		ll cur = 0;
		for (ll j = 1; j < time[i]; j++) {
			if ((time[i] - j) * j > distance[i])
				cur++;
		}
		result *= cur;
	}

	return result;
}

ll p2(vector<ll> &times, vector<ll> &distances) {
	ll n = times.size();

	ll time = 0;
	ll distance = 0;
	for (ll i = 0; i < n; i++) {
		for (ll j = 0; j < 1+floor(log10(times[i])); j++)
			time *= 10;
		for (ll j = 0; j < 1+floor(log10(distances[i])); j++)
			distance *= 10;
		time += times[i];
		distance += distances[i];
	}

	ll result = 0;
	for (ll j = 1; j < time; j++) {
		if ((time - j) * j > distance)
			result++;
	}
	
	return result;
}

int main() {
	ios_base::sync_with_stdio(0);
	cin.tie(0);
	
	vector<ll> time, distance;
	string inp;
	ll idx = 0;
	while (cin >> inp) {
		if (inp == "Distance:") idx++;
		else if (inp != "Time:") {
			(idx ? distance : time).push_back(stoi(inp));
		}
	}

	cout << p1(time, distance) << '\n';
	cout << p2(time, distance) << '\n';
	return 0;
}
