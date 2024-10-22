#[test]
fn greeting_test() {
    let want = String::from("Hello, Rusty!");
    let name = String::from("Rusty");
    let result = greeting(name);
    assert_eq!(want, result);
}
