#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::errors::ErrorKind;\

    #[test]
    fn createToken() {
        let user_id = "user_one";
        let token = create_token(user_id);

        assert!(!token.is_empty(). "Token should not be empty");
    }
}