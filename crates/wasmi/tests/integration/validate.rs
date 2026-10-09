use wasmi::{Engine, Module};

#[test]
fn validate_multiple_funcs() {
    let wasm = wat::parse_str(
        r#"
        (module
            (func (param i32 i64) (result i64)
                (local f32 f64)
                (i64.add (i64.extend_i32_u (local.get 0)) (local.get 1))
            )
            (func (result i32)
                (block (result i32) (i32.const 1))
            )
            (func (param f64) (result f64)
                (f64.mul (local.get 0) (local.get 0))
            )
        )
    "#,
    )
    .unwrap();
    Module::validate(&Engine::default(), &wasm).unwrap();
}

#[test]
fn validate_rejects_invalid_func_after_valid_funcs() {
    let wasm = wat::parse_str(
        r#"
        (module
            (func (result i32) (i32.const 1))
            (func (result i64) (i64.const 2))
            (func (result i32) (i64.const 3))
        )
    "#,
    )
    .unwrap();
    assert!(Module::validate(&Engine::default(), &wasm).is_err());
}
