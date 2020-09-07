fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[tokio::test]
    #[tracing::instrument]
    async fn my_test_function() {
        nonexistent_function()
        /*
        Compiler output:
        error[E0425]: cannot find function `nonexistent_function` in this scope
        --> src/main.rs:7:5
        |
        7 |     #[tokio::test]
        |       ^^^^^^^^^^^^^^ not found in this scope
        */
    }
}
