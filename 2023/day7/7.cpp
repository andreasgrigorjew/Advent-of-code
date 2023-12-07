#include <bits/stdc++.h>
using namespace std;
using ll = long long;

ll type(const string &a) {
	map<char, ll> count;
	for (char c: a) count[c]++;

	vector<ll> count_counts(6);
	for (auto [c, n]: count) {
		count_counts[n]++;
	}

	if (count_counts[5] == 1)
		return 7;
	if (count_counts[4] == 1)
		return 6;
	if (count_counts[3] == 1 && count_counts[2] == 1)
		return 5;
	if (count_counts[3] == 1 && count_counts[1] == 2)
		return 4;
	if (count_counts[2] == 2)
		return 3;
	if (count_counts[2] == 1 && count_counts[1] == 3)
		return 2;
	return 1;
}

ll type_maximize(const string &a) {
	map<char, ll> count;
	for (char c: a) count[c]++;

	char most = 'A';
	ll occurs = 0;
	for (auto [c, n]: count) if (c != 'J') {
		if (n > occurs) {
			most = c;
			occurs = n;
		}
	}
	
	string b = a;
	for (ll i = 0; i < 5; i++) if (b[i] == 'J') b[i] = most;
	return type(b);
}

ll p1(const vector<string> &hands, const map<string, ll> &bids) {
	const string cards = "AKQJT98765432";
	auto cmp = [&](const string &a, const string &b) {
		if (type(a) < type(b)) return true;
		if (type(a) > type(b)) return false;

		for (ll i = 0; i < 5; i++) {
			if (cards.find(a[i]) > cards.find(b[i])) return true;
			if (cards.find(a[i]) < cards.find(b[i])) return false;
		}
		return false;
	};

	vector<string> sorted_hands = hands;
	sort(sorted_hands.begin(), sorted_hands.end(), cmp);

	ll result = 0;
	for (ll i = 0; i < (ll) hands.size(); i++) {
		result += (i+1) * bids.at(sorted_hands[i]);
	}

	return result;
}

ll p2(const vector<string> &hands, const map<string, ll> &bids) {
	const string cards = "AKQT98765432J";
	auto cmp = [&](const string &a, const string &b) {
		if (type_maximize(a) < type_maximize(b)) return true;
		if (type_maximize(a) > type_maximize(b)) return false;

		for (ll i = 0; i < 5; i++) {
			if (cards.find(a[i]) > cards.find(b[i])) return true;
			if (cards.find(a[i]) < cards.find(b[i])) return false;
		}
		return false;
	};

	vector<string> sorted_hands = hands;
	sort(sorted_hands.begin(), sorted_hands.end(), cmp);

	ll result = 0;
	for (ll i = 0; i < (ll) hands.size(); i++) {
		result += (i+1) * bids.at(sorted_hands[i]);
	}

	return result;
}

int main() {
	ios_base::sync_with_stdio(0);
	cin.tie(0);
	
	vector<string> hands;
	map<string, ll> bids;

	string hand;
	while (cin >> hand) {
		hands.push_back(hand);
		ll b; cin >> b;
		bids[hand] = b;
	}

	cout << p1(hands, bids) << '\n';
	cout << p2(hands, bids) << '\n';

	return 0;
}
