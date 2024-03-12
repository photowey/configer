# `configer`
A simple configuration management implemented in Rust.

[APIs Documents](https://docs.rs/configer)

## `Version`

- `v0.1.0`
  - `set/get`

- `v0.2.0`
  - `try_xxx`

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
configer = "0.1"
```

## 2.`APIs`

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

#### 2.3.3.`Convert`

- Convert `&Node`

##### 2.3.4.1.`data-time`

```rust
let mut configer = ConfigerEnvironment::new();

let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
configer.set("io.github.photowey.configer.Time", now.into()).unwrap();

let rvt_time = configer.get("io.github.photowey.configer.Time");

// match
match rvt_time {
    Ok(node) => {
        match node {
            Node::DateTime(ref time) => {
                assert_eq!(*time, now);
            }
            _ => {}
        }
    }
    _ => {}
}

// converter
if let Some(into_value) = NodeConverter::try_datetime(rvt_time) {
    assert_eq!(*into_value, now);
} else {
    panic!("failed to convert the value to NaiveDateTime")
}
```

##### 2.3.4.2.`String`

```rust
let mut configer = ConfigerEnvironment::new();

configer
.set("io.github.photowey.str", String::from("Rust").into())
.unwrap();
let rvt_string = configer.get("io.github.photowey.str");

if let Some(into_value) = NodeConverter::try_string(rvt_string) {
    assert_eq!(*into_value, String::from("Rust"));
} else {
    panic!("failed to convert the value to String")
}
```

##### 2.3.4.3.`&str`

```rust
let mut configer = ConfigerEnvironment::new();

configer
.set("io.github.photowey.str", "Rust".into())
.unwrap();
let rvt_str = configer.get("io.github.photowey.str");

if let Some(into_value) = NodeConverter::try_str(rvt_str) {
    assert_eq!(into_value, "Rust");
} else {
    panic!("failed to convert the value to &str")
}
```

##### 2.3.4.4.`Nested`

```rust
let mut configer = ConfigerEnvironment::new();

let mut nested = Table::new();
nested.insert("Hello".to_string(), Node::String("Rust".to_string()));

configer
.set("io.github.photowey.nested", Node::Nested(nested))
.unwrap();

let rvt_nested = configer.get("io.github.photowey.nested");

if let Some(into_value) = NodeConverter::try_nested(rvt_nested) {
    match into_value.get("Hello") {
        Some(node) => {
            assert_eq!(*node, Node::String("Rust".to_string()));
        }
        _ => {}
    }
} else {
    panic!("failed to convert the value to Table")
}
```

##### 2.3.4.5.`Array`

```rust
let mut configer = ConfigerEnvironment::new();
let now = 1710265983u32;
let mut array = domain::Array::new();
array.push(Node::String("Rust".to_string()));
array.push(Node::IntU32(now));

configer
.set("io.github.photowey.array", Node::Array(array))
.unwrap();

let rvt_array = configer.get("io.github.photowey.array");

let mut image = domain::Array::new();
image.push(Node::String("Rust".to_string()));
image.push(Node::IntU32(now));

if let Some(into_value) = NodeConverter::try_array(rvt_array) {
    assert!(assert_array_equals(into_value, &image));
} else {
    panic!("failed to convert the value to Table")
}
```

##### 2.3.4.6.`U128`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = snowflake_dynamic!().unwrap() as u128;
configer.set("io.github.photowey.u128", rvt.into()).unwrap();
let rvt_u128 = configer.get("io.github.photowey.u128");

if let Some(into_value) = NodeConverter::try_int_u128(rvt_u128) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to u128")
}
```

##### 2.3.4.7.`U64`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = snowflake_dynamic!().unwrap();
configer.set("io.github.photowey.u64", rvt.into()).unwrap();
let rvt_u64 = configer.get("io.github.photowey.u64");

if let Some(into_value) = NodeConverter::try_int_u64(rvt_u64) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to u64")
}
```

##### 2.3.4.8.`U32`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = 1710265983u32;
configer.set("io.github.photowey.u32", rvt.into()).unwrap();
let rvt_u32 = configer.get("io.github.photowey.u32");

if let Some(into_value) = NodeConverter::try_int_u32(rvt_u32) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to u32")
}
```

##### 2.3.4.9.`I128`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = snowflake_dynamic!().unwrap() as i128;
configer.set("io.github.photowey.i128", rvt.into()).unwrap();
let rvt_i128 = configer.get("io.github.photowey.i128");

if let Some(into_value) = NodeConverter::try_int_i128(rvt_i128) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to i128")
}
```

##### 2.3.4.10.`I64`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = snowflake_dynamic!().unwrap() as i64;
configer.set("io.github.photowey.i64", rvt.into()).unwrap();
let rvt_i64 = configer.get("io.github.photowey.i64");

if let Some(into_value) = NodeConverter::try_int_i64(rvt_i64) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to i64")
}
```

##### 2.3.4.10.`I32`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = 1710265983i32;
configer.set("io.github.photowey.i32", rvt.into()).unwrap();
let rvt_i32 = configer.get("io.github.photowey.i32");

if let Some(into_value) = NodeConverter::try_int_i32(rvt_i32) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to i32")
}
```

##### 2.3.4.10.`F64`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = PI as f64;
configer.set("io.github.photowey.f64", rvt.into()).unwrap();
let rvt_f64 = configer.get("io.github.photowey.f64");

if let Some(into_value) = NodeConverter::try_float64(rvt_f64) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to f64")
}
```

##### 2.3.4.10.`F32`

```rust
let mut configer = ConfigerEnvironment::new();

let rvt = PI;
configer.set("io.github.photowey.f32", rvt.into()).unwrap();
let rvt_f32 = configer.get("io.github.photowey.f32");

if let Some(into_value) = NodeConverter::try_float32(rvt_f32) {
    assert_eq!(*into_value, rvt);
} else {
    panic!("failed to convert the value to f32")
}
```

##### 2.3.4.10.`None`

```rust
let mut configer = ConfigerEnvironment::new();

let none = Node::None;
configer.set("io.github.photowey.none", none).unwrap();
let rvt_none = configer.get("io.github.photowey.none");

if let Some(into_value) = NodeConverter::try_none(rvt_none) {
    assert_eq!(*into_value, ());
} else {
    panic!("failed to convert the value to none")
}
```

## 3.`Next`

- Support load `config` files (P 0).
  - `configer.toml`
  - `configer-${profile}.toml`
  - …
  - `yaml` | `yml` ?
  - `properties` ?
  - `ini`?
  - `.env`?
  - `json`?
  - …
- Auto. load environment variables (P 1)
- Support merge exists `Map`
- Support bind `struct`
- …

## 4.`Documents`

**Please wait a moment.**

