# read_access_bench

このリポジトリは、Rustでいくつかの種類のグローバル変数へのアクセス速度を比較します。

## 使い方

以下のコマンドを実行します。
```
cargo bench
```
標準出力に各ベンチマークの簡単な結果を表示すると共に、`target/criterion/report/index.html` にベンチマークのレポートが生成されます。

## 結果

手元のPC(CPU: AMD Ryzen 7 PRO 5850U)でのベンチマークで標準出力に表示された結果を以下に記載します。
```
Read global mut value   time:   [236.56 ps 237.97 ps 239.49 ps]
                        change: [+2.0461% +2.3907% +2.8107%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 32 outliers among 100 measurements (32.00%)
  7 (7.00%) low severe
  7 (7.00%) low mild
  6 (6.00%) high mild
  12 (12.00%) high severe

Read UnsafeCell value   time:   [234.24 ps 235.52 ps 237.03 ps]
                        change: [-4.7255% -3.8877% -2.9858%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

Read OnceCell value     time:   [512.78 ps 513.62 ps 514.57 ps]
                        change: [-1.3380% -1.1410% -0.9688%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe

Read OnceLock value     time:   [404.99 ps 405.72 ps 406.60 ps]
                        change: [-2.5062% -2.3225% -2.1239%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Read Mutex value        time:   [3.8040 ns 3.8146 ns 3.8279 ns]
                        change: [-5.8857% -5.6146% -5.3629%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
```

global mut value と UnsafeCell がほぼ同等かつ最速で、続いて OnceCell value と OnceLock value、最も遅かったのが Mutex value という結果となりました。

## License

MIT
