
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

gcs/blake2b_sip24/3     time:   [1.9332 µs 1.9391 µs 1.9459 µs]
                        change: [-0.3356% +0.0803% +0.5580%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
gcs/blake2b/3           time:   [1.7871 µs 1.7904 µs 1.7952 µs]
                        change: [+160.56% +161.17% +161.93%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
gcs/sip24/3             time:   [900.95 ns 902.02 ns 903.09 ns]
                        change: [+0.3053% +0.5926% +1.0487%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
gcs/blake2b_sip24/4     time:   [2.9067 µs 2.9106 µs 2.9146 µs]
                        change: [+1.4127% +1.7301% +2.0105%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
gcs/blake2b/4           time:   [2.6245 µs 2.6279 µs 2.6314 µs]
                        change: [+133.57% +133.93% +134.30%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
gcs/sip24/4             time:   [1.4421 µs 1.4454 µs 1.4495 µs]
                        change: [+1.1405% +1.4294% +1.8590%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  4 (4.00%) high severe
gcs/blake2b_sip24/5     time:   [3.5115 µs 3.5251 µs 3.5457 µs]
                        change: [+0.0689% +1.1681% +1.9184%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
gcs/blake2b/5           time:   [3.1172 µs 3.1223 µs 3.1277 µs]
                        change: [+141.09% +141.67% +142.18%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
gcs/sip24/5             time:   [1.6929 µs 1.6959 µs 1.6993 µs]
                        change: [-0.9356% -0.1276% +0.5502%] (p = 0.76 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
gcs/blake2b_sip24/6     time:   [4.0332 µs 4.0409 µs 4.0492 µs]
                        change: [+3.1389% +3.6812% +4.4487%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
gcs/blake2b/6           time:   [4.0350 µs 4.1485 µs 4.2773 µs]
                        change: [+195.99% +202.33% +209.36%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 26 outliers among 100 measurements (26.00%)
  19 (19.00%) low severe
  7 (7.00%) high severe
gcs/sip24/6             time:   [1.9077 µs 1.9109 µs 1.9141 µs]
                        change: [+1.3297% +1.5618% +1.8190%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```
