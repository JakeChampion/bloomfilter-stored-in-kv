# Bloomfilter stored in Fastly KV

A bloom filter lets you insert items, and then check whether an item is contained within the filter. They are efficient in both space and time, at the cost of false positives, but not false negatives. This makes a bloom filter really good if what you are wanting to do is check is something is not contained in the filter.

This project is storing the bloom filter in Fastly KV, that allows the filter to be updated without having to redeploy the Fastly Compute Service, which is great if the filter contents needs to be updated frequently.

You can deploy this by running `fastly compute publish`.

You can check if a package exists by making a request like so:
```
curl "https://pypi.edgecompute.app/?name=<python package name here>"
```

# Real-world performance test results

## Package name in the index

<details>
<summary>
❯ oha --latency-correction --disable-keepalive 'https://pypi.edgecompute.app/?name=requests'
</summary>

<code><pre>
Summary:
  Success rate:	1.0000
  Total:	0.6317 secs
  Slowest:	0.2490 secs
  Fastest:	0.0795 secs
  Average:	0.1394 secs
  Requests/sec:	316.6078

  Total data:	800 B
  Size/request:	4 B
  Size/sec:	1.24 KiB

Response time histogram:
  0.079 [1]  |
  0.096 [9]  |■■■■
  0.113 [35] |■■■■■■■■■■■■■■■■■■
  0.130 [62] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.147 [35] |■■■■■■■■■■■■■■■■■■
  0.164 [11] |■■■■■
  0.181 [15] |■■■■■■■
  0.198 [13] |■■■■■■
  0.215 [12] |■■■■■■
  0.232 [3]  |■
  0.249 [4]  |■■

Latency distribution:
  10% in 0.1046 secs
  25% in 0.1154 secs
  50% in 0.1277 secs
  75% in 0.1559 secs
  90% in 0.1959 secs
  95% in 0.2078 secs
  99% in 0.2442 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.1071 secs, 0.0623 secs, 0.2170 secs
  DNS-lookup:	0.0154 secs, 0.0000 secs, 0.1025 secs

Status code distribution:
  [200] 200 responses
</pre></code></details>



## Package name not in the index

<details>
<summary>
❯ oha --latency-correction --disable-keepalive 'https://pypi.edgecompute.app/?name=request'
</summary>

<code><pre>
Summary:
  Success rate:	1.0000
  Total:	0.5977 secs
  Slowest:	0.2296 secs
  Fastest:	0.0875 secs
  Average:	0.1349 secs
  Requests/sec:	334.6323

  Total data:	1000 B
  Size/request:	5 B
  Size/sec:	1.63 KiB

Response time histogram:
  0.088 [1]  |
  0.102 [24] |■■■■■■■■■■■■■
  0.116 [59] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.130 [44] |■■■■■■■■■■■■■■■■■■■■■■■
  0.144 [18] |■■■■■■■■■
  0.159 [3]  |■
  0.173 [8]  |■■■■
  0.187 [6]  |■■■
  0.201 [22] |■■■■■■■■■■■
  0.215 [8]  |■■■■
  0.230 [7]  |■■■

Latency distribution:
  10% in 0.0996 secs
  25% in 0.1072 secs
  50% in 0.1195 secs
  75% in 0.1629 secs
  90% in 0.1951 secs
  95% in 0.2140 secs
  99% in 0.2187 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.1090 secs, 0.0654 secs, 0.1905 secs
  DNS-lookup:	0.0106 secs, 0.0000 secs, 0.0752 secs

Status code distribution:
  [200] 200 responses
</pre></code></details>