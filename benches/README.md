
## Bench Environment
```
Memory : 15.6 GiB
   CPU : Intel® Xeon(R) CPU E5-2670 0 @ 2.60GHz × 32 
    OS : Ubuntu 20.04.4 LTS (64-bit)
```

## The Report

* `blake2b_sip24` means firstly `blake2b` hash the data then `sip24` hash the data in `GCSFilterWriter`
* `blake2b` means `blake2b` hash the data in `GCSFilterWriter`
* `sip24` means `sip24` hash the data `GCSFilterWriter` (the bitcoin approach)
* `gcs/blake2b_sip24/1` the last number `1` here means only feed one data into `GCSFilterWriter`

```
$ cargo bench

gcs/blake2b_sip24/3     time:   [1.8478 µs 1.8490 µs 1.8504 µs]
                        change: [+0.2690% +0.4886% +0.6963%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  6 (6.00%) high mild
  10 (10.00%) high severe
gcs/blake2b/3           time:   [1.8329 µs 1.8345 µs 1.8365 µs]
                        change: [-8.7279% -7.7983% -7.0229%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  7 (7.00%) high severe
gcs/sip24/3             time:   [892.01 ns 892.48 ns 893.05 ns]
                        change: [+0.9293% +1.0754% +1.2454%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  7 (7.00%) high mild
  10 (10.00%) high severe
gcs/blake2b_sip24/4     time:   [2.7544 µs 2.7564 µs 2.7587 µs]
                        change: [-1.7389% -0.7813% -0.0050%] (p = 0.09 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  5 (5.00%) high severe
gcs/blake2b/4           time:   [2.6737 µs 2.6751 µs 2.6768 µs]
                        change: [-5.6785% -5.5545% -5.4191%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
gcs/sip24/4             time:   [1.3973 µs 1.3984 µs 1.3997 µs]
                        change: [+0.0541% +0.3401% +0.6651%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  15 (15.00%) high severe
gcs/blake2b_sip24/5     time:   [3.3206 µs 3.3236 µs 3.3271 µs]
                        change: [-1.0885% -0.1904% +0.5187%] (p = 0.68 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  10 (10.00%) high severe
gcs/blake2b/5           time:   [3.2118 µs 3.2145 µs 3.2178 µs]
                        change: [-5.8258% -5.5100% -5.2720%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
gcs/sip24/5             time:   [1.6544 µs 1.6557 µs 1.6572 µs]
                        change: [-0.2927% -0.1663% -0.0380%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  6 (6.00%) high severe
gcs/blake2b_sip24/6     time:   [3.7634 µs 3.7664 µs 3.7699 µs]
                        change: [-1.2383% -1.0807% -0.8793%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) low severe
  7 (7.00%) low mild
  2 (2.00%) high mild
  8 (8.00%) high severe
gcs/blake2b/6           time:   [3.7759 µs 3.7805 µs 3.7861 µs]
                        change: [-7.0041% -6.3019% -5.7737%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
gcs/sip24/6             time:   [1.8655 µs 1.8671 µs 1.8688 µs]
                        change: [-0.1258% +0.0827% +0.3025%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe
```
