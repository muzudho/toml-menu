# Tomboy toml dom

For those who are struggling with Rust's cool syntax, our goal is to provide a TOML parser that's as easy as pointing to a menu and eating fast food.  
Rustのイケてる構文に難儀している人のために、メニューを指差してファーストフードを食べるぐらい簡単な操作のTOMLパーサーを提供することを目標とします。  

Unstable version. It's a tryal and error process. Specifications will change.  
不安定版。 試行錯誤中です。 仕様はコロコロ変わるでしょう。  

Tomboy is a pun.  
トムボーイ（おてんば娘）は語呂合わせです。  

References:  

* [Developer's blog(開発者ブログ)](https://crieit.net/drafts/5f8094a14a0cf)
* [TOML parsing（TOMLの構文解析）](https://crieit.net/posts/TOML-parsing-TOML)
* ABNF: [toml.abnf](https://github.com/toml-lang/toml/blob/1.0.0-rc.3/toml.abnf)

## Run (実行)

Take a look at the repository.  
リポジトリを見てください。  

```shell
cargo run --example advanced
cargo run --example comment
cargo run --example cover
cargo run --example deprecated
cargo run --example example
cargo run --example example-tail-comment
cargo run --example inline_table
cargo run --example main
cargo run --example mix_array
cargo run --example spot
cargo run --example table
cargo run --example toml-io-en-a-quick-tour-of-toml-v1-0-0rc3
cargo run --example toml-io-en-v1-0-0rc3-full-speck
```

## Specification (仕様)

The specifications will gradually solidify.  
仕様は少しずつ固めていきます。  

You can think that you can't do anything that isn't written here.  
ここに書かれていないことは何もできないと思ってもらって構いません。  

./resource/example.toml:  

```plain
age = 40
weight = 93.5

# Long. 32bit size.
i32_max = 2_147_483_647
i32_min = -2_147_483_648
u32_max = 4_294_967_295

# Long long. 64bit size.
i64_max = 9_223_372_036_854_775_807
i64_min = -9_223_372_036_854_775_808
u64_max = 18_446_744_073_709_551_615

# 128bit size.
i128_max = 170_141_183_460_469_231_731_687_303_715_884_105_727
i128_min = -170_141_183_460_469_231_731_687_303_715_884_105_728
u128_max = 340_282_366_920_938_463_463_374_607_431_768_211_455

# hexadecimal with prefix `0x`
hex1 = 0xDEADBEEF
hex2 = 0xdeadbeef
hex3 = 0xdead_beef

# octal with prefix `0o`
oct1 = 0o01234567
oct2 = 0o755

# binary with prefix `0b`
bin1 = 0b11010110

# fractional
float1 = +1.0
float2 = 3.1415
float3 = -0.01

# exponent
float4 = 5e+22
float5 = 1e06
float6 = -2E-2

# both
float7 = 6.626e-34

# separators
float8 = 224_617.445_991_228

# infinity
infinite1 = inf # positive infinity
infinite2 = +inf # positive infinity
infinite3 = -inf # negative infinity

# not a number
not1 = nan
not2 = +nan
not3 = -nan

# basic string
apple = "pie"
basic_string_empty = ""
basic_string_escape_backslash = "\\"
basic_string_escape_double_quotation = "\""
basic_string_letter = "Hello, world!!"
basic_string_punctuation = "., ={}[]'\"\\!?"
basic_string_tab = "a\tb"

multiline_basic_string_letter = """Hello,
world!!"""
multiline_basic_string_punctuation = """., ={}[]"'""\\
!?"""
multiline_basic_string_trim_start = """\
  The quick brown \
  fox jumps over \
  the lazy dog.\
  """
multiline_basic_string_escape_double_quotation = """
\\
"""
multiline_basic_string_tab = """
a\tb
"""

literal_string_empty = ''
literal_string_letter = 'Hello, world!!'
literal_string_punctuation = '., ={}[]"\!?'

multiline_literal_string_letter = '''Hello,
world!!'''
multiline_literal_string_punctuation = '''., ={}[]'"\
!?'''
multiline_literal_string_first_newline_is_trimmed = '''
The first newline is
trimmed in raw strings.
All other whitespace
is preserved.
'''

adult = true
student = false

dob = 1979-05-27T07:32:00-08:00

#offset datetime
odt1 = 1979-05-27T07:32:00Z
odt2 = 1979-05-27T00:32:00-07:00
odt3 = 1979-05-27T00:32:00.999999-07:00

# local datetime
ldt1 = 1979-05-27T07:32:00
ldt2 = 1979-05-27T00:32:00.999999

# local date
ld1 = 1979-05-27

# local time
lt1 = 07:32:00
lt2 = 00:32:00.999999

# Array
# int_array = [-1, 0, 1]
# float_array = [0.0, 0.5, 1.0]
string_array = ["a", 'b', '"c"']
```

examples/example.rs:  

```rust
//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use chrono::{
    prelude::{DateTime, Utc},
    FixedOffset, NaiveDate, NaiveDateTime, NaiveTime,
};
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/example.toml");

    // Read a number.
    // 数値読取。
    assert!(doc.contains_key("age"));
    assert_eq!(doc.get_i128_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_isize_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_u128_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_usize_by_key_v2("age"), Ok(Some(40)));
    assert_eq!(doc.get_f64_by_key_v2("weight"), Ok(Some(93.5)));

    assert_eq!(doc.get_i128_by_key_v2("i32_max"), Ok(Some(2147483647)));
    assert_eq!(doc.get_i128_by_key_v2("i32_min"), Ok(Some(-2147483648)));

    assert_eq!(
        doc.get_i128_by_key_v2("i128_max"),
        Ok(Some(170_141_183_460_469_231_731_687_303_715_884_105_727))
    );
    assert_eq!(
        doc.get_i128_by_key_v2("i128_min"),
        Ok(Some(-170_141_183_460_469_231_731_687_303_715_884_105_728))
    );
    assert_eq!(
        doc.get_u128_by_key_v2("u128_max"),
        Ok(Some(340_282_366_920_938_463_463_374_607_431_768_211_455))
    );

    assert_eq!(doc.get_i128_by_key_v2("hex1"), Ok(Some(0xDEADBEEF)));
    assert_eq!(doc.get_i128_by_key_v2("hex2"), Ok(Some(0xdeadbeef)));
    assert_eq!(doc.get_i128_by_key_v2("hex3"), Ok(Some(0xdead_beef)));
    assert_eq!(doc.get_i128_by_key_v2("oct1"), Ok(Some(0o01234567)));
    assert_eq!(doc.get_i128_by_key_v2("oct2"), Ok(Some(0o755)));
    assert_eq!(doc.get_i128_by_key_v2("bin1"), Ok(Some(0b11010110)));
    assert_eq!(doc.get_f64_by_key_v2("float1"), Ok(Some(1.0)));
    assert_eq!(doc.get_f64_by_key_v2("float2"), Ok(Some(3.1415)));
    assert_eq!(doc.get_f64_by_key_v2("float3"), Ok(Some(-0.01)));
    assert_eq!(doc.get_f64_by_key_v2("float4"), Ok(Some(5e+22)));
    assert_eq!(doc.get_f64_by_key_v2("float5"), Ok(Some(1e06)));
    assert_eq!(doc.get_f64_by_key_v2("float6"), Ok(Some(-2E-2)));
    assert_eq!(doc.get_f64_by_key_v2("float7"), Ok(Some(6.626e-34)));
    assert_eq!(
        doc.get_f64_by_key_v2("float8"),
        Ok(Some(224_617.445_991_228))
    );
    assert_eq!(doc.get_f64_by_key_v2("infinite1"), Ok(Some(f64::INFINITY)));
    assert_eq!(doc.get_f64_by_key_v2("infinite2"), Ok(Some(f64::INFINITY)));
    assert_eq!(doc.get_f64_by_key_v2("infinite3"), Ok(Some(-f64::INFINITY)));
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not1") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not2") {
        n.is_nan() && n.is_sign_positive()
    } else {
        false
    });
    assert!(if let Ok(Some(n)) = doc.get_f64_by_key_v2("not3") {
        n.is_nan() && n.is_sign_negative()
    } else {
        false
    });

    // WIP. Read a string.
    // 作業中。 文字列読取。
    assert_eq!(doc.get_string_by_key("apple"), Some("pie".to_string()));

    assert_eq!(
        doc.get_string_by_key("basic_string_letter"),
        Some("Hello, world!!".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_empty"),
        Some("".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_escape_backslash"),
        Some("\\".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_escape_double_quotation"),
        Some("\"".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_punctuation"),
        Some("., ={}[]'\"\\!?".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("basic_string_tab"),
        Some("a\tb".to_string())
    );

    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_letter"),
        Some(
            "Hello,
world!!"
                .to_string()
        )
    );

    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_punctuation"),
        Some(
            "., ={}[]\"'\"\"\\
!?"
            .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_trim_start"),
        Some("The quick brown fox jumps over the lazy dog.".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_escape_double_quotation"),
        Some(
            "
\\
"
            .to_string()
        )
    );
    /*
    // Fixed.
    println!(
        "debug|multiline_basic_string_tab|{}",
        doc.get_debug_string_by_key("multiline_basic_string_tab")
    );
    */
    assert_eq!(
        doc.get_string_by_key("multiline_basic_string_tab"),
        Some(
            "
a\tb
"
            .to_string()
        )
    );

    assert_eq!(
        doc.get_string_by_key("literal_string_empty"),
        Some("".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("literal_string_letter"),
        Some("Hello, world!!".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("literal_string_punctuation"),
        Some("., ={}[]\"\\!?".to_string())
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_letter"),
        Some(
            "Hello,
world!!"
                .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_punctuation"),
        Some(
            "., ={}[]'\"\\
!?"
            .to_string()
        )
    );
    assert_eq!(
        doc.get_string_by_key("multiline_literal_string_first_newline_is_trimmed"),
        Some(
            "The first newline is
trimmed in raw strings.
All other whitespace
is preserved.
"
            .to_string()
        )
    );

    // Read a boolean.
    // 論理値読取。
    assert_eq!(doc.get_bool_by_key("adult"), Some(true));
    assert_eq!(doc.get_bool_by_key("student"), Some(false));

    // DateTime.
    // 日付と時刻。
    assert_eq!(
        doc.get_datetime_utc_by_key("dob"),
        Some(
            "1979-05-27T07:32:00-08:00"
                .parse::<DateTime<Utc>>()
                .unwrap()
        )
    );

    assert_eq!(
        doc.get_datetime_utc_by_key("odt1"),
        Some("1979-05-27T07:32:00Z".parse::<DateTime<Utc>>().unwrap())
    );

    assert_eq!(
        doc.get_datetime_fixed_offset_by_key("odt2"),
        Some(
            "1979-05-27T00:32:00-07:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        )
    );

    assert_eq!(
        doc.get_datetime_fixed_offset_by_key("odt3"),
        Some(
            "1979-05-27T00:32:00.999999-07:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        )
    );

    // TODO Local datetime
    assert_eq!(
        // "1979-05-27T07:32:00". Toml の独自書式か。該当するフォーマット定義見つからず。
        doc.get_naive_datetime_by_key("ldt1"),
        Some(
            match NaiveDateTime::parse_from_str("1979-05-27T07:32:00", "%Y-%m-%dT%H:%M:%S") {
                Ok(n) => n,
                Err(why) => panic!("{}", why),
            }
        )
    );

    assert_eq!(
        // "1979-05-27T00:32:00.999999".
        doc.get_naive_datetime_by_key("ldt2"),
        Some(
            NaiveDateTime::parse_from_str("1979-05-27T00:32:00.999999", "%Y-%m-%dT%H:%M:%S%.6f")
                .unwrap()
        )
    );

    assert_eq!(
        // "1979-05-27".
        doc.get_naive_date_by_key("ld1"),
        Some(match NaiveDate::parse_from_str("1979-05-27", "%Y-%m-%d") {
            Ok(n) => n,
            Err(why) => panic!("{}", why),
        })
    );

    assert_eq!(
        doc.get_naive_time_by_key("lt1"),
        Some(NaiveTime::parse_from_str("07:32:00", "%H:%M:%S").unwrap())
    );

    assert_eq!(
        doc.get_naive_time_by_key("lt2"),
        Some(NaiveTime::parse_from_str("00:32:00.999999", "%H:%M:%S%.6f").unwrap())
    );

    // Read a array.
    // 配列読取。
    assert_eq!(
        doc.get_string_array_by_key("string_array"),
        Ok(Some(vec![
            "a".to_string(),
            "b".to_string(),
            "\"c\"".to_string()
        ]))
    );
}
```

## TODO

* [ ] Comment
  * [x] In empty line.
  * [x] After keyval.
  * [ ] After table.
* [ ] Literal
  * [ ] Literal numbers...
    * [ ] integer
      * [x] `0b` - binary.
      * [x] `0o` - octal.
      * [x] `0x` - hexadecimal.
      * [x] `_` - separators.
    * [ ] float
      * [x] `.` - point. Example: `3.14`.
      * [x] `_` - separators.
      * [x] `inf` - Positive infinity.
      * [x] `+inf` - Positive infinity.
      * [x] `-inf` - Negative infinity.
      * [x] `nan` - Not a number. Positive.
      * [x] `+nan` - Not a number. Positive.
      * [x] `-nan` - Not a number. Negative.
  * [ ] DateTime
    * [x] `1979-05-27` - Local date. (Naive date)
    * [x] `1979-05-27T07:32:00` - Local datetime. (Naive datetime)
    * [x] `1979-05-27T07:32:00Z` - UTC datetime. (Datetime Utc)
    * [x] `1979-05-27T00:32:00.999999` - Local datetime. (Naive datetime)
    * [x] `1979-05-27T00:32:00-07:00` - UTC datetime. (Datetime fixed offset)
    * [x] `1979-05-27T00:32:00.999999-07:00` - UTC datetime. (Datetime fixed offset)
    * [x] `07:32:00` - Local time. (Naive time)
    * [x] `00:32:00.999999` - Local time. (Naive time)
* [ ] String (Not str)
  * [x] `"abc"` - Basic string.
    * [x] Plain.
    * [ ] Escape sequence.
  * [ ] `"""abc"""` - Multi-line basic string.
    * [x] Plain.
    * [ ] Escape sequence.
    * [x] Ending backslash to automatically trim.
  * [ ] `'abc'` - Literal string.
    * [x] Plain.
  * [ ] `'''abc'''` - multi-line literal string.
    * [x] Plain.
    * [x] The first newline is trimmed in raw string.
  * [ ] Escape sequence.
    * [x] `\r` - caridge return.
    * [x] `\n` - line feed.
    * [x] `\t` - tab.
    * [x] `\\` - backslash.
    * [x] `\"` - double quotation.
    * [x] `\u0000` - Unicode.
    * [ ] `\U00000000` - Unicode.
* [ ] Array
  * [ ] `[-1, 0, 1]` - Int array.
  * [ ] `[0.1, 0.5, 1.0]` - Float array.
  * [x] `["a", 'b', '"c"']` - String array.
