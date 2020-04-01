# Golomb-Coded Set

An implementation of BIP158 Golomb-Coded Set data structure.

NOTE: The code is mainly copy from [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin/blob/a148e0673665a099d2771bf9c2dcf3402b75e7de/src/util/bip158.rs) project.

## How to choose parameters?

```
cd golomb-loss
cargo run --release
```

The output:
```
========
>> Reference: https://gist.github.com/sipa/576d5f09c3b86c3b1b75598d799fc845
--------
 P : the bit parameter of the Golomb-Rice coding
 M : 1.497137 * (2**P)
 N : how many items in filter
 bits/item : average bits cost for one item => log2(eM) - log2(2πN)/(2N)
 bits/item(real) : real(random items [u8;32]) average bits cost for one item
 false-positive-rate: 1.0/M
========

 # P=10, M=    1533, false-positive-rate=0.0006522867
N=10    , bits/item=11.7262, bits/item(real)=18.4000, build-filter-cost=6.926µs
N=100   , bits/item=11.9784, bits/item(real)=12.7200, build-filter-cost=24.154µs
N=1000  , bits/item=12.0186, bits/item(real)=12.1120, build-filter-cost=246.224µs
N=10000 , bits/item=12.0241, bits/item(real)=12.0600, build-filter-cost=2.978371ms
N=100000, bits/item=12.0248, bits/item(real)=12.0539, build-filter-cost=35.731131ms

 # P=15, M=   49058, false-positive-rate=0.0000203840
N=10    , bits/item=16.7262, bits/item(real)=23.2000, build-filter-cost=6.117µs
N=100   , bits/item=16.9784, bits/item(real)=17.7600, build-filter-cost=4.960362ms
N=1000  , bits/item=17.0186, bits/item(real)=17.1120, build-filter-cost=302.957µs
N=10000 , bits/item=17.0241, bits/item(real)=17.0600, build-filter-cost=2.626318ms
N=100000, bits/item=17.0248, bits/item(real)=17.0537, build-filter-cost=34.169061ms

 # P=19, M=  784931, false-positive-rate=0.0000012740
N=10    , bits/item=20.7262, bits/item(real)=27.2000, build-filter-cost=6.692µs
N=100   , bits/item=20.9784, bits/item(real)=21.7600, build-filter-cost=43.907µs
N=1000  , bits/item=21.0186, bits/item(real)=21.1120, build-filter-cost=310.448µs
N=10000 , bits/item=21.0241, bits/item(real)=21.0600, build-filter-cost=2.696983ms
N=100000, bits/item=21.0248, bits/item(real)=21.0537, build-filter-cost=34.340566ms

 # P=20, M= 1569862, false-positive-rate=0.0000006370
N=10    , bits/item=21.7262, bits/item(real)=28.8000, build-filter-cost=6.392µs
N=100   , bits/item=21.9784, bits/item(real)=22.7200, build-filter-cost=44.985µs
N=1000  , bits/item=22.0186, bits/item(real)=22.1120, build-filter-cost=322.643µs
N=10000 , bits/item=22.0241, bits/item(real)=22.0600, build-filter-cost=2.801545ms
N=100000, bits/item=22.0248, bits/item(real)=22.0537, build-filter-cost=32.687121ms

 # P=25, M=50235582, false-positive-rate=0.0000000199
N=10    , bits/item=26.7262, bits/item(real)=33.6000, build-filter-cost=6.894µs
N=100   , bits/item=26.9784, bits/item(real)=27.7600, build-filter-cost=46.113µs
N=1000  , bits/item=27.0186, bits/item(real)=27.1120, build-filter-cost=327.462µs
N=10000 , bits/item=27.0241, bits/item(real)=27.0600, build-filter-cost=2.868482ms
N=100000, bits/item=27.0248, bits/item(real)=27.0537, build-filter-cost=35.875129ms
```

