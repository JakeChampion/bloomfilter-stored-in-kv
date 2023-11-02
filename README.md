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
  Total:	0.9805 secs
  Slowest:	0.4689 secs
  Fastest:	0.1284 secs
  Average:	0.2127 secs
  Requests/sec:	203.9857

  Total data:	800 B
  Size/request:	4 B
  Size/sec:	815 B

Response time histogram:
  0.128 [1]  |
  0.162 [52] |■■■■■■■■■■■■■■■■■■■■■■
  0.196 [73] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.231 [23] |■■■■■■■■■■
  0.265 [9]  |■■■
  0.299 [10] |■■■■
  0.333 [0]  |
  0.367 [19] |■■■■■■■■
  0.401 [12] |■■■■■
  0.435 [0]  |
  0.469 [1]  |

Latency distribution:
  10% in 0.1465 secs
  25% in 0.1605 secs
  50% in 0.1832 secs
  75% in 0.2483 secs
  90% in 0.3631 secs
  95% in 0.3681 secs
  99% in 0.3789 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.1144 secs, 0.0627 secs, 0.2131 secs
  DNS-lookup:	0.0165 secs, 0.0000 secs, 0.0895 secs

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
  Total:	0.8519 secs
  Slowest:	0.4429 secs
  Fastest:	0.1152 secs
  Average:	0.1867 secs
  Requests/sec:	234.7626

  Total data:	1000 B
  Size/request:	5 B
  Size/sec:	1.15 KiB

Response time histogram:
  0.115 [1]  |
  0.148 [22] |■■■■■■■■
  0.181 [87] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.214 [47] |■■■■■■■■■■■■■■■■■
  0.246 [26] |■■■■■■■■■
  0.279 [14] |■■■■■
  0.312 [1]  |
  0.345 [1]  |
  0.377 [0]  |
  0.410 [0]  |
  0.443 [1]  |

Latency distribution:
  10% in 0.1473 secs
  25% in 0.1576 secs
  50% in 0.1777 secs
  75% in 0.2057 secs
  90% in 0.2413 secs
  95% in 0.2586 secs
  99% in 0.3441 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.1061 secs, 0.0571 secs, 0.1900 secs
  DNS-lookup:	0.0123 secs, 0.0000 secs, 0.1122 secs

Status code distribution:
  [200] 200 responses
</pre></code></details>