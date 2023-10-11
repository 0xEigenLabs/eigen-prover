pub fn add(left: usize, right: usize) -> usize {
    left + right
}

async fn run() {
    // Goal:
    // pipeline deal the  stark_prove, aggre_setup/exec
    // might be parallel or distribute compute
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
