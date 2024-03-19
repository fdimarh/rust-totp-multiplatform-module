#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

char *get_pin_number(uintptr_t otp_digit,
                     uint8_t time_threshold,
                     uint64_t time_expiry,
                     const char *secret);
