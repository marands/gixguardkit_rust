#include <iostream>

namespace gix_guard_kit {
    uint8_t test_print1();
}

//#include <gix_guard_kit_rust.h>
#include <include/gix_guard_kit_rust.h>

int main() {
    std::cout << "Hello, World!" << std::endl;

    const uint8_t raw_key[GXT_KEY_LEN] = {
            0x0, 0x37, 0x60, 0x8c, 0x14, 0xe6, 0xcd, 0xce, 0xc5, 0x09, 0x70, 0x45, 0xc6, 00, 0xf9, 0x3f, 0x3d, 0xd4, 0x5d, 0xa9, 0x7b, 0x4c, 0x3a, 0xa5, 0x13, 0xde, 0x48, 0x8c, 0x95, 0xec, 0xf6, 0x42
    };
    //auto *raw_hex2 = (char *) malloc(GXT_KEY_LEN_HEX * sizeof(char) );
    //memset(raw_hex, '\0', GXT_KEY_LEN_HEX * sizeof(char));
    char raw_hex[GXT_KEY_LEN_HEX];
    GixTunnelErrorKind res = key_to_hex( raw_hex, raw_key);

    printf("raw_hex as string: %s\n", raw_hex);
    printf("HEX:\n");
    for(int i=0; i < GXT_KEY_LEN_HEX; i++) {
        printf("%c:", raw_hex[i]);
    }
    printf("\nHEX:%s\n", raw_hex);
    //free(raw_hex);

    printf("res:%d\n", (int) res);

    uint8_t raw_key_result[GXT_KEY_LEN];
    res = hex_from_key(raw_key_result, raw_hex);
    printf("raw_key_result:\n");
    for(int i=0; i < GXT_KEY_LEN; i++) {
        printf("%02X:", raw_key_result[i]);
    }
    printf("\nres: %d\n", (int) res);

    uint8_t key[GXT_KEY_LEN] = {00, 0x37, 0x60, 0x8c, 0x14, 0xe6, 0xcd, 0xce, 0xc5, 0x09, 0x70, 0x45, 0xc6, 00,
                0xf9, 0x3f, 0x3d, 0xd4, 0x5d, 0xa9, 0x7b, 0x4c, 0x3a, 0xa5, 0x13, 0xde, 0x48, 0x8c,
                0x95, 0xec, 0xf6, 0x42};
    char base64Str[GXT_KEY_LEN_BASE64];
    res = key_to_base64(base64Str, key);
    printf("res:%d - base64: %s", (int) res, base64Str);

    uint8_t raw_key_out[GXT_KEY_LEN];
    res = key_from_base64(raw_key_out, base64Str);
    printf("key_from_base64:\n");
    for(int i=0; i < GXT_KEY_LEN; i++) {
        printf("%02X:", raw_key_out[i]);
    }
    printf("\nres: %d\n", (int) res);

    bool eq = key_eq(raw_key_out, raw_key_out);
    printf("Key equality: %d\n", eq);

    return 0;
}
