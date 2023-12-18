#[derive(Debug)]
pub enum EmailType {
    ConfirmEmail,
}

impl EmailType {
    pub fn get_all() -> Vec<EmailType> {
        let enums = vec![Self::ConfirmEmail];
        return enums;
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::ConfirmEmail => "Confirm",
        }
    }
}

impl ToString for EmailType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
