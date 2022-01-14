#pragma once

/* Generated with cbindgen:0.20.0 */

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const uintptr_t GXT_KEY_LEN = 32;

static const uintptr_t GXT_KEY_LEN_BASE64 = 45;

static const uintptr_t GXT_KEY_LEN_HEX = 65;

enum class GixTunnelErrorKind : uint32_t {
  Ok = 0,
  Failed = 4096,
  NullInput = 4097,
  InvalidInput = 4098,
  InvalidInputLength = 4099,
};

extern "C" {

void curve25519_shared_secret(uint8_t sharedSecret[GXT_KEY_LEN],
                              const uint8_t privateKey[GXT_KEY_LEN],
                              const uint8_t publicKey[GXT_KEY_LEN]);

void curve25519_generate_private_key(uint8_t privateKey[GXT_KEY_LEN]);

bool curve25519_derive_public_key(uint8_t publicKey[GXT_KEY_LEN],
                                  const uint8_t privateKey[GXT_KEY_LEN]);

GixTunnelErrorKind key_from_base64(uint8_t dst[GXT_KEY_LEN], const char src[GXT_KEY_LEN_BASE64]);

GixTunnelErrorKind key_to_base64(char dst[GXT_KEY_LEN_BASE64], const uint8_t src[GXT_KEY_LEN]);

GixTunnelErrorKind hex_from_key(uint8_t dst[GXT_KEY_LEN], const char src[GXT_KEY_LEN_HEX]);

GixTunnelErrorKind key_to_hex(char dst[GXT_KEY_LEN_HEX], const uint8_t src[GXT_KEY_LEN]);

bool key_eq(const uint8_t lVal[GXT_KEY_LEN], const uint8_t rVal[GXT_KEY_LEN]);

} // extern "C"
