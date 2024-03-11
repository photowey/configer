# configer
A simple configuration management implemented in Rust.

[APIs Documents](https://docs.rs/configer)

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
configer = "0.1"
```

## 2.`APPIs`

**Notes: It's not stable yet and like a toy. Please be careful when using it in a production environment.**

### 2.1.`new`

```rust
let mut configer = ConfigerEnvironment::new();
```

### 2.2.`set`

- Support nested key settings

#### 2.2.1.`Err`

```rust
let mut configer = ConfigerEnvironment::new();

let empty_rvt = configer.set("", "Rust".into());
assert_eq!(empty_rvt, Err(ConfigerError::EmptyKey));
```

#### 2.2.2.`Ok`

```rust
let mut configer = ConfigerEnvironment::new();

configer.set("io.github.photowey.string", String::from("Hello, Configer!").into()).unwrap();
configer.set("io.github.photowey.str", "Rust".into()).unwrap();

let rvt = snowflake_dynamic!().unwrap() as i64;
configer.set("io.github.photowey.i32", 123_i32.into()).unwrap();
configer.set("io.github.photowey.i64", rvt.into()).unwrap();

let pi = PI as f64;
configer.set("io.github.photowey.configer.f32", 9527.8848_f32.into()).unwrap();
configer.set("io.github.photowey.configer.f64", pi.into()).unwrap();

let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
configer.set("io.github.photowey.configer.Time", now.into()).unwrap();
```

### 2.3.`get`

#### 2.3.1.`Err`

```rust
let mut configer = ConfigerEnvironment::new();
configer.set("io.github.photowey.configer.ok", "Rust".into()).unwrap();

assert_eq!(configer.get("io.github.photowey.configer.not.found"), Err(ConfigerError::NotFound));
```

#### 2.3.2.`Ok`

```rust
let mut configer = ConfigerEnvironment::new();

configer.set("io.github.photowey.string", String::from("Hello, Configer!").into()).unwrap();
configer.set("io.github.photowey.str", "Rust".into()).unwrap();

let rvt = snowflake_dynamic!().unwrap() as i64;
configer.set("io.github.photowey.i32", 123_i32.into()).unwrap();
configer.set("io.github.photowey.i64", rvt.into()).unwrap();

let pi = PI as f64;
configer.set("io.github.photowey.configer.f32", 9527.8848_f32.into()).unwrap();
configer.set("io.github.photowey.configer.f64", pi.into()).unwrap();

let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
configer.set("io.github.photowey.configer.Time", now.into()).unwrap();

assert_eq!(configer.get("io.github.photowey.string"), Ok(&Node::String(String::from("Hello, Configer!").into())));
assert_eq!(configer.get("io.github.photowey.str"), Ok(&Node::String(String::from("Rust").into())));
assert_eq!(configer.get("io.github.photowey.i32"), Ok(&Node::Int32(123_i32)));
assert_eq!(configer.get("io.github.photowey.i64"), Ok(&Node::Int64(rvt)));
assert_eq!(configer.get("io.github.photowey.configer.f32"), Ok(&Node::Float32(9527.8848_f32)));
assert_eq!(configer.get("io.github.photowey.configer.f64"), Ok(&Node::Float64(pi)));
assert_eq!(configer.get("io.github.photowey.configer.Time"), Ok(&Node::DateTime(now)));
```

## 3.`Next`

- Support load `config` files (P 0).
  - `configer.toml`
  - `configer-${profile}.toml`
  - …
  - `yaml` | `yml` ?
  - `properties` ?
  - `init`?
  - `.env`?
  - …
- Auto. load environment variables (P 1)
- Support merge exists `Map`
- Support bind `struct`
- …

## 4.`Documents`

**Please wait a moment.**

