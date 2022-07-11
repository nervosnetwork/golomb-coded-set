
## Bench Environment
```
Memory : 15.6 GiB
   CPU : Intel® Xeon(R) CPU E5-2670 0 @ 2.60GHz × 32 
    OS : Ubuntu 20.04.4 LTS (64-bit)
```

## The Report

* `blake2b_sip24` means `blake2b` hash the data then feed the hash as data to `GCSFilterWriter`
* `sip24` means directly feed the data `GCSFilterWriter`
* `gcs/blake2b_sip24/1` the last number `1` here means only feed one data into `GCSFilterWriter`

```
$ cargo bench

gcs/blake2b_sip24/1     time:   [747.98 ns 748.12 ns 748.28 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
gcs/sip24/1             time:   [406.08 ns 406.27 ns 406.54 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
gcs/blake2b_sip24/2     time:   [1.3206 µs 1.3252 µs 1.3304 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
gcs/sip24/2             time:   [656.30 ns 676.29 ns 696.92 ns]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) high mild
  10 (10.00%) high severe
gcs/blake2b_sip24/3     time:   [1.8490 µs 1.8549 µs 1.8640 µs]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
gcs/sip24/3             time:   [850.30 ns 850.76 ns 851.53 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
gcs/blake2b_sip24/4     time:   [2.7152 µs 2.7159 µs 2.7166 µs]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
gcs/sip24/4             time:   [1.3675 µs 1.3679 µs 1.3683 µs]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
gcs/blake2b_sip24/5     time:   [3.2665 µs 3.2670 µs 3.2676 µs]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
gcs/sip24/5             time:   [1.5914 µs 1.5917 µs 1.5921 µs]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  1 (1.00%) high severe
gcs/blake2b_sip24/6     time:   [3.8018 µs 3.8118 µs 3.8321 µs]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe
gcs/sip24/6             time:   [1.8235 µs 1.8249 µs 1.8269 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
gcs/blake2b_sip24/7     time:   [4.3739 µs 4.3759 µs 4.3785 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
gcs/sip24/7             time:   [2.0726 µs 2.0743 µs 2.0764 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
gcs/blake2b_sip24/8     time:   [5.3069 µs 5.3085 µs 5.3105 µs]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
gcs/sip24/8             time:   [2.7847 µs 2.7854 µs 2.7861 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
gcs/blake2b_sip24/9     time:   [5.8840 µs 5.8867 µs 5.8905 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
gcs/sip24/9             time:   [3.0367 µs 3.0378 µs 3.0392 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
```
