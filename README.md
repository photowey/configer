# `configer`

A simple configuration management implemented in Rust.

[APIs Documents](https://docs.rs/configer)

## `Version`

- `v0.1.0`
  
  - Support `set/get`
  
- `v0.2.0`
  
    - Support `try_xxx` functions
    
- `v0.3.0`

    - Support read `toml`

    

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
configer = "0.3"

# Or
# If necessary
configer = { version = "0.3", features = ["usetoml"] }
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

##### 2.3.3.1.`Nested`

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
            assert_eq ! ( * node, Node::String("Rust".to_string()));
        },
        _ => {}
    }
} else {
    panic ! ("failed to convert the value to Table")
}
```

##### 2.3.3.2.`Array`

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

##### 2.3.3.3.`DateTime`

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

##### 2.3.4.4.`...`



## 3.`Reader`

### 3.1.`toml`

#### 3.1.1.`new`

```rust
let toml_reader = TomlConfigReader::default();
```



#### 3.1.2.`Read`

##### 3.1.2.1.`path`

```rust
let path = "resources/testdata/configer-dev.toml";
let toml_reader = TomlConfigReader::default();
let toml_rvt = toml_reader.read_from_path(path);
```



##### 3.1.2.2.`Content`

```rust
let toml_reader = TomlConfigReader::default();

let path = "resources/testdata/configer-dev.toml";
let content = fs::read_to_string(path).expect("Failed to read config file");
let toml_from_content_rvt = toml_reader.read_from_str(&content);
```



## 4.`Next`

- Support load `config` files (P 0).
    - [x] `configer.toml`
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



## 5.`Documents`

**Please wait a moment.**



## 6.`Test`

### 6.1.`cargo test`

```shell
$ cargo test --features "usetoml" -- --show-output
$ cargo test --features "usetoml"
```