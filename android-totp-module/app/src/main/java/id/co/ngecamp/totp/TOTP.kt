package id.co.ngecamp.totp

class TOTP {

    init {
        System.loadLibrary("totp_android")
    }

    /**
     * This is a native method declaration which is implemented in the native library `totp_android`.
     * It generates the OTP based on the provided parameters.
     *
     * @param otp_digit The number of digits in the OTP.
     * @param time_threshold The time threshold for the OTP.
     * @param time_expiry The expiry time for the OTP.
     * @param secret The secret key used for generating the OTP.
     * @return The generated OTP as a String. It can be null if the OTP generation fails.
     */
    private external fun getPinNumber(otp_digit:Int, time_threshold:Int, time_expiry:Int, secret:String): String?

    /**
     * This method is a wrapper for the native method `getPinNumber`.
     * It takes the same parameters as `getPinNumber` and returns the generated OTP.
     *
     * @param otp_digit The number of digits in the OTP.
     * @param time_threshold The time threshold for the OTP.
     * @param time_expiry The expiry time for the OTP.
     * @param secret The secret key used for generating the OTP.
     * @return The generated OTP as a String. It can be null if the OTP generation fails.
     */
    fun getPin(otp_digit:Int, time_threshold:Int, time_expiry:Int, secret:String): String? {
        return getPinNumber(otp_digit, time_threshold, time_expiry, secret)
    }

}