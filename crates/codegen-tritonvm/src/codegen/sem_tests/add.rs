use expect_test::expect;

use crate::codegen::sem_tests::check_wasm;

#[test]
fn test_add() {
    let input = vec![11, 7];
    let secret_input = vec![3];
    let expected_output = vec![21];
    let native_output = c2zk_rust_wasm_tests_helper::wrap_main_with_io(
        &c2zk_rust_wasm_tests_bundle1::add::main,
    )(input.clone(), secret_input.clone());
    assert_eq!(native_output, expected_output);
    let wasm_bytes = c2zk_rust_wasm_tests_helper::compile_rust_wasm_tests_bundle1_bin("add");
    check_wasm(
        &wasm_bytes,
        input,
        secret_input,
        expected_output,
        expect![[r#"
            (module
              (type (;0;) (func (result i64)))
              (type (;1;) (func (param i64)))
              (type (;2;) (func))
              (type (;3;) (func (param i64 i64) (result i64)))
              (import "env" "c2zk_stdlib_pub_input" (func $c2zk_stdlib_pub_input (;0;) (type 0)))
              (import "env" "c2zk_stdlib_pub_output" (func $c2zk_stdlib_pub_output (;1;) (type 1)))
              (import "env" "c2zk_stdlib_secret_input" (func $c2zk_stdlib_secret_input (;2;) (type 0)))
              (func $__main (;3;) (type 2)
                call $_ZN28c2zk_rust_wasm_tests_bundle13add4main17hb9ca2d9cba28e087E
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add3add17h1d8242aadf5ab846E (;4;) (type 3) (param i64 i64) (result i64)
                local.get 1
                local.get 0
                i64.add
              )
              (func $_ZN28c2zk_rust_wasm_tests_bundle13add4main17hb9ca2d9cba28e087E (;5;) (type 2)
                call $_ZN11c2zk_stdlib9pub_input17h0dee22ae2ec5e4e8E
                call $_ZN11c2zk_stdlib9pub_input17h0dee22ae2ec5e4e8E
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17h1d8242aadf5ab846E
                call $_ZN11c2zk_stdlib12secret_input17ha93a82c316e394d7E
                call $_ZN28c2zk_rust_wasm_tests_bundle13add3add17h1d8242aadf5ab846E
                call $_ZN11c2zk_stdlib10pub_output17h07b1e1ebe272f489E
              )
              (func $_ZN11c2zk_stdlib9pub_input17h0dee22ae2ec5e4e8E (;6;) (type 0) (result i64)
                call $c2zk_stdlib_pub_input
              )
              (func $_ZN11c2zk_stdlib10pub_output17h07b1e1ebe272f489E (;7;) (type 1) (param i64)
                local.get 0
                call $c2zk_stdlib_pub_output
              )
              (func $_ZN11c2zk_stdlib12secret_input17ha93a82c316e394d7E (;8;) (type 0) (result i64)
                call $c2zk_stdlib_secret_input
              )
              (memory (;0;) 16)
              (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
              (global (;1;) i32 i32.const 1048576)
              (global (;2;) i32 i32.const 1048576)
              (export "memory" (memory 0))
              (export "__main" (func $__main))
              (export "__data_end" (global 1))
              (export "__heap_base" (global 2))
            )"#]],
        expect![[r#"
            call f3
            halt
            f0:
            read_io
            return
            f1:
            write_io
            return
            f2:
            divine
            return
            f3:
            call f5
            return
            f4:
            add
            return
            f5:
            call f6
            call f6
            call f4
            call f8
            call f4
            call f7
            return
            f6:
            call f0
            return
            f7:
            call f1
            return
            f8:
            call f2
            return"#]],
    )
}
