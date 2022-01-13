#include <stdio.h>

#include "WireGuardKitC/key.h"
#include "WireGuardKitC/x25519.h"

const char *wg_private_keys_arr[5]= {
        "ADdgjBTmzc7FCXBFxgD5Pz3UXal7TDqlE95IjJXs9kI=",
        "6L5h4vbCEVRLgxZ/znpNUD+0m/Fa+DtY4eV4DAh4pEA=",
        "2FzSWHf/Cib3Knl8VuIy5skedNVgXOsr/BKdW4gnul8=",
        "uBDEHfrtdLPUTDI4UH4Obl8znsAHEPxHl6OL2bE1wXg=",
        "6NPKNjP8o2TovHam9JMSIuuXoWyll88f4UMNgF37o1k="
};

static inline void print_val(const uint8_t *src, int len, const char * func_name, const char * label) {
    printf("\n ::%s:: %s: ", func_name, label);
    for(int i=0; i < len; i++) {
        printf("%0.2x", src[i]);
    }
    printf("\n");
}
static inline void print_bytes(const uint8_t *src, int len, const char * func_name, const char * label) {
    printf("::%s:: %s Bytes: [", func_name, label);
    for(int i=0; i < len; i++) {
        printf("%#0.2x,", src[i]);
    }
    printf("] \n");
}

static inline void print_chars(const uint8_t *src, int len, const char * func_name, const char * label) {
    printf("::%s:: %s Chars: [", func_name, label);
    for(int i=0; i < len; i++) {
        printf("'%c',", src[i]);
    }
    printf("] \n");
}

static inline void print_integer(const uint8_t *src, int len, const char * func_name, const char * label) {
    printf("\n ::%s:: %s Integer: [", func_name, label);
    for(int i=0; i < len; i++) {
        printf("'%d',", src[i]);
    }
    printf("] \n");
}

void make_key_to_hex_and_inverse_tests() {
    uint8_t input_key[WG_KEY_LEN];
    char hex_result_out[WG_KEY_LEN_HEX];
    uint8_t key_result_out[WG_KEY_LEN];
    for(int i=0; i < 1; i++) {
        if(key_from_base64(input_key, wg_private_keys_arr[i])) {
            key_to_hex(hex_result_out, input_key);
            print_bytes(input_key, WG_KEY_LEN, "key_to_hex", "input_key");
            //print_chars((uint8_t *)hex_result_out, WG_KEY_LEN_HEX, "key_to_hex", "hex_result_out");
            printf("HEX: %s\n", hex_result_out);
            bool ret = key_from_hex(key_result_out, hex_result_out);
            print_bytes(key_result_out, WG_KEY_LEN, "key_from_hex", "key_result_out");
            printf("\nret:%d\n", ret);
        }

    }
}

void make_key_from_base64_tests() {
    uint8_t key[WG_KEY_LEN];

    for(int i=0; i < 5; i++) {
        printf("base64 string: %s\n", wg_private_keys_arr[i]);
        print_bytes((uint8_t *)wg_private_keys_arr[i], WG_KEY_LEN_BASE64, "key_from_base64", "base64");
        if(key_from_base64(key, wg_private_keys_arr[i])) {
            print_bytes(key, WG_KEY_LEN, "key_from_base64", "key");
            printf("\n");
        } else {
            printf("Failed.\n");
        }
    }
}

extern uint8_t test_print1();

int main() {
    printf("\"Hello, World!\"\n");

//    make_key_from_base64_tests();
//
    printf("\n\nstarting next test data for key_to_hex :\n\n");
    make_key_to_hex_and_inverse_tests();

    //test_print1();
    return 0;
}
