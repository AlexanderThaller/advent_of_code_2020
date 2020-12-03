use super::{
    Error,
    PasswordPolicySledRental,
    PasswordPolicyTobogganRental,
};
use std::collections::HashMap;

pub(super) struct SledTester<'a> {
    policies: HashMap<&'a str, PasswordPolicySledRental>,
}

impl<'a> SledTester<'a> {
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }

    pub fn test(&mut self, input: &'a str) -> Result<bool, Error> {
        let mut policy_password_split = input.split(':');
        let policy_raw = policy_password_split.next().ok_or(Error::MissingPolicy)?;

        let policy = self
            .policies
            .entry(policy_raw)
            .or_insert_with(|| policy_raw.parse().expect("bad input"));

        let password = policy_password_split
            .next()
            .ok_or(Error::MissingPassword)?
            .trim();

        let valid = policy.is_valid_password(password);

        Ok(valid)
    }
}

pub(super) struct TobogganTester<'a> {
    policies: HashMap<&'a str, PasswordPolicyTobogganRental>,
}

impl<'a> TobogganTester<'a> {
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }

    pub fn test(&mut self, input: &'a str) -> Result<bool, Error> {
        let mut policy_password_split = input.split(':');
        let policy_raw = policy_password_split.next().ok_or(Error::MissingPolicy)?;

        let policy = self
            .policies
            .entry(policy_raw)
            .or_insert_with(|| policy_raw.parse().expect("bad input"));

        let password = policy_password_split
            .next()
            .ok_or(Error::MissingPassword)?
            .trim();

        let valid = policy.is_valid_password(password);

        Ok(valid)
    }
}
