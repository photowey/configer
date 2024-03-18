# `configer`

A simple configuration management implemented in Rust.

[APIs Documents](https://docs.rs/configer)

[changelog](./CHANGELOG.md)

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
configer = "0.5"

# Or
# If necessary
configer = { version = "0.5", features = ["usetoml"] }
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



#### 2.2.3.`set_t`

- Update signature  `@since 0.4.3`

```rust
let mut configer = ConfigerEnvironment::new();
// set
// configer.set("io.github.photowey.string", String::from("Hello, Configer!").into()).unwrap();
// set_t
configer.set_t("io.github.photowey.string", String::from("Hello, Configer!")).unwrap();

// set
// configer.set("io.github.photowey.str", "Rust".into()).unwrap();
// set_t
configer.set_t("io.github.photowey.str", "Rust").unwrap();

let rvt = snowflake_dynamic!().unwrap() as i64;
configer.set_t("io.github.photowey.i32", 123_i32).unwrap();
configer.set_t("io.github.photowey.i64", rvt).unwrap();

let pi = PI as f64;
configer.set_t("io.github.photowey.configer.f32", 9527.8848_f32).unwrap();
configer.set_t("io.github.photowey.configer.f64", pi).unwrap();

let now = NaiveDateTime::parse_from_str("2024-03-11 22:50:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
configer.set_t("io.github.photowey.configer.Time", now).unwrap();
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

- `@since 0.3.0`

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



##### 3.1.2.2.`file content`

```rust
let toml_reader = TomlConfigReader::default();

let path = "resources/testdata/configer-dev.toml";
let content = fs::read_to_string(path).expect("Failed to read config file");
let toml_from_content_rvt = toml_reader.read_from_str(&content);
```



## 4.`ConfigerEnvironmentBuilder`

- `@since 0.4.0`

### 4.1.`With table`

```rust
let path = "resources/testdata/configer-dev.toml";

let toml_reader = TomlConfigReader::default();
let toml_rvt = toml_reader.read_from_path(path);

if let Ok(table) = toml_rvt {
    // With table
    let builder_rvt = ConfigerEnvironment::builder()
    .with_table(table)
    .build();

    if let Ok(configer) = builder_rvt {
        let rvt_database_servers = configer.get("database.servers");

        return assert_configer_array(rvt_database_servers, "database.servers");
    }

    panic!("Failed to build ConfigerEnvironment")
}

panic!("Failed to read configer-dev.toml file")
```

### 4.2.`With registry and path`

- Only support `toml` file now.

```rust
let path = "resources/testdata/configer-dev.toml";

let toml_reader = TomlConfigReader::default();
let mut registry = ConfigReaderRegistry::default();
registry.register(Box::new(toml_reader));

let builder_rvt = ConfigerEnvironment::builder()
.with_registry(Box::new(registry))
.with_path(path.to_string())
.build();

if let Ok(configer) = builder_rvt {
    let rvt_database_servers = configer.get("database.servers");
    return assert_configer_array(rvt_database_servers, "database.servers");
}

panic!("Failed to read configer-dev.toml file")
```



### 4.3.`With table,registry and path`

```rust
env::set_var("CONFIGER_TEST_VAR", "rust.configer");

let path = "resources/testdata/configer-dev.toml";

let toml_reader = TomlConfigReader::default();
let mut registry = ConfigReaderRegistry::default();
registry.register(Box::new(toml_reader));

let table = crate::env::try_load_env_variables();

let builder_rvt = ConfigerEnvironment::builder()
.with_table(table)
.with_registry(Box::new(registry))
.with_path(path.to_string())
.build();

if let Ok(configer) = builder_rvt {
    let rvt_database_servers = configer.get("database.servers");
    assert_configer_array(rvt_database_servers, "database.servers");

    let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
    assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));

    return ();
}

panic!("Failed to read configer-dev.toml file")
```



## 5.`Load Environment variables`

### 5.1.`default`

```rust
env::set_var("CONFIGER_TEST_VAR", "rust.configer");

let configer = ConfigerEnvironment::default();
let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));
```

### 5.2.`new`

```rust
env::set_var("CONFIGER_TEST_VAR", "rust.configer");

let configer = ConfigerEnvironment::new();
let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));
```

### 5.3.`mixed_with_env_variables`

```rust
env::set_var("CONFIGER_TEST_VAR", "rust.configer");

let configer = ConfigerEnvironment::mixed_with_env_variables(None, None);
let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));
```

### 5.4.`table`

```rust
env::set_var("CONFIGER_TEST_VAR", "rust.configer");

let path = "resources/testdata/configer-dev.toml";

let toml_reader = TomlConfigReader::default();
let toml_rvt = toml_reader.read_from_path(path);

if let Ok(table) = toml_rvt {
    let configer = ConfigerEnvironment::table(table);

    let rvt_database_servers = configer.get("database.servers");
    assert_configer_array(rvt_database_servers, "database.servers");

    let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
    assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));

    return ();
}

panic!("Failed to read configer-dev.toml file")
```

### 5.5.`builder`

```rust
env::set_var("CONFIGER_TEST_VAR", "rust.configer");

let path = "resources/testdata/configer-dev.toml";

let toml_reader = TomlConfigReader::default();
let mut registry = ConfigReaderRegistry::default();
registry.register(Box::new(toml_reader));

let builder_rvt = ConfigerEnvironment::builder()
.with_registry(Box::new(registry))
.with_path(path.to_string())
.build(); // load environment variables auto.

if let Ok(configer) = builder_rvt {
    let rvt_database_servers = configer.get("database.servers");
    assert_configer_array(rvt_database_servers, "database.servers");

    let env_var_rvt = configer.get("CONFIGER_TEST_VAR");
    assert_eq!(env_var_rvt, Ok(&Node::String(String::from("rust.configer"))));

    return ();
}

panic!("Failed to read configer-dev.toml file")
```





## 6.`Next`

- Support load `config` files (P 0).
    - [x] `configer.toml`
    - [ ] `configer-${profile}.toml`
    - …
    - [ ] `yaml` | `yml` ?
    - [ ] `properties` ?
    - [ ] `ini`?
    - [ ] `.env`?
    - [ ] `json`?
    - …
- [x] Auto. load environment variables (P 1)
- [x] Support merge exists `HashMap<String,Node>/Table`
- Support bind `struct`
- …



## 7.`Documents`

**Please wait a moment.**



## 8.`Test`

### 8.1.`cargo test`

```shell
$ cargo test --features "usetoml" -- --show-output
$ cargo test --features "usetoml"
```



## 9.`Docs`

### 9.1.`features`

- `usetoml`

```shell
$ cargo doc --open --features usetoml
```