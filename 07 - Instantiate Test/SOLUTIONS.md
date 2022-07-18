# Follow Up Exercises Solutions

So you attempted the follow up exercises, I'm going to give solutions in snippet format.

## Exercise 1

```rust
// Previous code omitted
#[test]
fn test_instantiate_with_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    // Send as ADDR1 to show admin is different
    let info = mock_info(ADDR1, &vec![]);

    // Create a message where ADDR2 will be an admin
    // Have to use .to_string() method
    let msg = InstantiateMsg { admin: Some(ADDR2.to_string()) };
    // Unwrap to assert success
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    // Assert admin is ADDR2 instead
    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR2),]
    );
}
// Following code omitted
```
