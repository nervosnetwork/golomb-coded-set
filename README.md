# Golomb-Coded Set

An implementation of BIP158 Golomb-Coded Set data structure.

NOTE: The code is mainly copy from [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin/blob/a148e0673665a099d2771bf9c2dcf3402b75e7de/src/util/bip158.rs) project.

## How to choose parameters?

```
cd golomb-loss
cargo run
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
N=10    , bits/item=11.7262, bits/item(real)=18.4000, build-filter-cost=91.296µs
N=100   , bits/item=11.9784, bits/item(real)=12.7200, build-filter-cost=482.491µs
N=1000  , bits/item=12.0186, bits/item(real)=12.1280, build-filter-cost=4.775904ms
N=10000 , bits/item=12.0241, bits/item(real)=12.0632, build-filter-cost=42.614618ms
N=100000, bits/item=12.0248, bits/item(real)=12.0539, build-filter-cost=416.904422ms

 # P=15, M=   49058, false-positive-rate=0.0000203840
N=10    , bits/item=16.7262, bits/item(real)=24.0000, build-filter-cost=84.812µs
N=100   , bits/item=16.9784, bits/item(real)=17.7600, build-filter-cost=5.866147ms
N=1000  , bits/item=17.0186, bits/item(real)=17.1280, build-filter-cost=4.513592ms
N=10000 , bits/item=17.0241, bits/item(real)=17.0632, build-filter-cost=42.698718ms
N=100000, bits/item=17.0248, bits/item(real)=17.0536, build-filter-cost=418.770731ms

 # P=19, M=  784931, false-positive-rate=0.0000012740
N=10    , bits/item=20.7262, bits/item(real)=28.0000, build-filter-cost=70.931µs
N=100   , bits/item=20.9784, bits/item(real)=21.7600, build-filter-cost=450.961µs
N=1000  , bits/item=21.0186, bits/item(real)=21.1280, build-filter-cost=4.64919ms
N=10000 , bits/item=21.0241, bits/item(real)=21.0632, build-filter-cost=43.084025ms
N=100000, bits/item=21.0248, bits/item(real)=21.0536, build-filter-cost=427.381864ms

 # P=20, M= 1569862, false-positive-rate=0.0000006370
N=10    , bits/item=21.7262, bits/item(real)=28.8000, build-filter-cost=89.917µs
N=100   , bits/item=21.9784, bits/item(real)=22.7200, build-filter-cost=448.59µs
N=1000  , bits/item=22.0186, bits/item(real)=22.1280, build-filter-cost=4.671141ms
N=10000 , bits/item=22.0241, bits/item(real)=22.0632, build-filter-cost=43.219213ms
N=100000, bits/item=22.0248, bits/item(real)=22.0536, build-filter-cost=424.415087ms

 # P=25, M=50235582, false-positive-rate=0.0000000199
N=10    , bits/item=26.7262, bits/item(real)=33.6000, build-filter-cost=88.958µs
N=100   , bits/item=26.9784, bits/item(real)=27.7600, build-filter-cost=452.27µs
N=1000  , bits/item=27.0186, bits/item(real)=27.1280, build-filter-cost=4.766385ms
N=10000 , bits/item=27.0241, bits/item(real)=27.0632, build-filter-cost=43.95553ms
N=100000, bits/item=27.0248, bits/item(real)=27.0536, build-filter-cost=432.00147ms
```

