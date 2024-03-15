pub mod totp {
    use totp_rs::{Algorithm, TOTP, Secret};

    /// The Time-Based One-Time Password (TOTP) algorithm is defined in the RFC 6238 standard.
    /// It's an extension of the HMAC-based One-Time Password (HOTP) algorithm, with the main difference being that it uses a timestamp rather than a counter.
    /// This makes TOTP a more suitable choice for short-lived OTPs which are used in two-factor authentication systems.
    /// Generates a One-Time Password (OTP) using the Time-Based One-Time Password (TOTP) algorithm.
    ///
    /// # Parameters
    ///
    /// * `otp_digit`: The number of digits in the OTP. Best practice recommendation 6-8 digits
    /// * `time_threshold`: The time threshold for the OTP. Best practice recommendation 1 second
    /// * `time_expiry`: The time expiry for the OTP. Best practice recommendation 30 seconds
    /// * `secret`: The secret key used to generate the OTP. The secret key must be a base32 encoded string.
    ///
    /// # Returns
    ///
    /// A `String` that represents the generated OTP.
    ///
    /// # Example
    ///
    /// ```
    /// use totp_multiplatform_module::totp::get_pin_number;
    /// let otp = get_pin_number(6, 3, 30, "12345678901234567890".to_string());
    /// assert_eq!(otp.len(), 6);
    /// ```
    pub fn get_pin_number(otp_digit: usize, time_threshold: u8, time_expiry: u64, secret: String) -> String {
        let totp = TOTP::new(
            Algorithm::SHA1,
            otp_digit,
            time_threshold,
            time_expiry,
            Secret::Raw(secret.as_bytes().to_vec()).to_bytes().unwrap(),
        ).unwrap();
        let token = totp.generate_current().unwrap();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use crate::totp::get_pin_number;

    #[test]
    fn otp_works() {
        let result = get_pin_number(6, 3, 30, "12345678901234567890".to_string());
        print!("OTP {}",result);
        assert_eq!(result.len(), 6);
    }
}
