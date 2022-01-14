#pragma once

/* Generated with cbindgen:0.20.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define GXT_KEY_LEN 32

#define GXT_KEY_LEN_BASE64 45

#define GXT_KEY_LEN_HEX 65

enum GixTunnelErrorKind {
  Ok = 0,
  Failed = 4096,
  NullInput = 4097,
  InvalidInput = 4098,
  InvalidInputLength = 4099,
};
typedef uint32_t GixTunnelErrorKind;

void curve25519_shared_secret(uint8_t sharedSecret[GXT_KEY_LEN],
                              const uint8_t privateKey[GXT_KEY_LEN],
                              const uint8_t publicKey[GXT_KEY_LEN]);

void curve25519_generate_private_key(uint8_t privateKey[GXT_KEY_LEN]);

bool curve25519_derive_public_key(uint8_t publicKey[GXT_KEY_LEN],
                                  const uint8_t privateKey[GXT_KEY_LEN]);

GixTunnelErrorKind key_from_base64(uint8_t dst[GXT_KEY_LEN], const char src[GXT_KEY_LEN_BASE64]);

GixTunnelErrorKind key_to_base64(char dst[GXT_KEY_LEN_BASE64], const uint8_t src[GXT_KEY_LEN]);

GixTunnelErrorKind key_from_hex(uint8_t dst[GXT_KEY_LEN], const char src[GXT_KEY_LEN_HEX]);

GixTunnelErrorKind key_to_hex(char dst[GXT_KEY_LEN_HEX], const uint8_t src[GXT_KEY_LEN]);

bool key_eq(const uint8_t lVal[GXT_KEY_LEN], const uint8_t rVal[GXT_KEY_LEN]);
