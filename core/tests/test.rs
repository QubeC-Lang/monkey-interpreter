use monkey_interpreter_core::nothing;

#[test]
fn it_works() {
    let result = nothing();
    assert_eq!(result, None);
}
