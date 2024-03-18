# `configer`

A simple configuration management implemented in Rust.

## `Changelog`

- `v0.1.0`

    - Support `set/get`.
- `v0.2.0`
    - Support `try_xxx` functions.
- `v0.3.0`

    - Support read `toml` file.
- `v0.4.0`
    - Support build `ConfigerEnvironment` by `ConfigerEnvironmentBuilder`.
      - `with_table`
      - `with registry` and `with path`
- `v0.5.0`
    - Support `Table`(`HashMap<String, Node>`) merge.
    - `ConfigerEnvironmentBuilder`
        - with `Table`
            - `@since 0.4.0`
        - with `registry` and `path`
            - `@since 0.4.0`
        - with `Table`,`registry` and `path`
            - `@since 0.5.0`
    - Support load `Environment` variables auto.
- `v0.6.0`
    - Support read `${config}-${profile}` files.
        - `with_profiles(vec![String::from("dev"), String::from("shared")])`
        - `config.toml`
            - `config-dev.toml`
            - `config-shared.toml`