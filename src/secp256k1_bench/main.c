/**********************************************************************
 * Copyright (c) 2014 Pieter Wuille                                   *
 * Distributed under the MIT software license, see the accompanying   *
 * file COPYING or http://www.opensource.org/licenses/mit-license.php.*
 **********************************************************************/
#define HAVE_CONFIG_H
#define SECP256K1_BUILD
// Do not define VERIFY for the benchmarks: it makes things run much slower

#if defined HAVE_CONFIG_H
#include "../secp256k1_tests/libsecp256k1-config.h"
#endif

#include <stdio.h>
#include <string.h>

#include "../secp256k1_tests/secp256k1_c.h"
#include "util.h"
#include "bench.h"

#include "sysctl.h"
#include "uarths.h"
#include "sleep.h"

#define V_RUNS (200)

typedef struct {
    secp256k1_context *ctx;
    unsigned char msg[32];
    unsigned char key[32];
    unsigned char sig[72];
    size_t siglen;
    unsigned char pubkey[33];
    size_t pubkeylen;
} benchmark_verify_t;

static void benchmark_verify(void* arg) {
    int i;
    benchmark_verify_t* data = (benchmark_verify_t*)arg;

    for (i = 0; i < V_RUNS; i++) {
        secp256k1_pubkey pubkey;
        secp256k1_ecdsa_signature sig;
        data->sig[data->siglen - 1] ^= (i & 0xFF);
        data->sig[data->siglen - 2] ^= ((i >> 8) & 0xFF);
        data->sig[data->siglen - 3] ^= ((i >> 16) & 0xFF);
        CHECK(secp256k1_ec_pubkey_parse(data->ctx, &pubkey, data->pubkey, data->pubkeylen) == 1);
        CHECK(secp256k1_ecdsa_signature_parse_der(data->ctx, &sig, data->sig, data->siglen) == 1);
        CHECK(secp256k1_ecdsa_verify(data->ctx, &sig, data->msg, &pubkey) == (i == 0));
        data->sig[data->siglen - 1] ^= (i & 0xFF);
        data->sig[data->siglen - 2] ^= ((i >> 8) & 0xFF);
        data->sig[data->siglen - 3] ^= ((i >> 16) & 0xFF);
    }
}

int bench_verify_main(void) {
    int i;
    secp256k1_pubkey pubkey;
    secp256k1_ecdsa_signature sig;
    benchmark_verify_t data;

    data.ctx = secp256k1_context_create(SECP256K1_CONTEXT_SIGN | SECP256K1_CONTEXT_VERIFY);

    for (i = 0; i < 32; i++) {
        data.msg[i] = 1 + i;
    }
    for (i = 0; i < 32; i++) {
        data.key[i] = 33 + i;
    }
    data.siglen = 72;
    CHECK(secp256k1_ecdsa_sign(data.ctx, &sig, data.msg, data.key, NULL, NULL));
    CHECK(secp256k1_ecdsa_signature_serialize_der(data.ctx, data.sig, &data.siglen, &sig));
    CHECK(secp256k1_ec_pubkey_create(data.ctx, &pubkey, data.key));
    data.pubkeylen = 33;
    CHECK(secp256k1_ec_pubkey_serialize(data.ctx, data.pubkey, &data.pubkeylen, &pubkey, SECP256K1_EC_COMPRESSED) == 1);

    run_benchmark("ecdsa_verify", benchmark_verify, NULL, NULL, &data, 10, V_RUNS);

    secp256k1_context_destroy(data.ctx);

    return 0;
}

typedef struct {
    secp256k1_context* ctx;
    unsigned char msg[32];
    unsigned char key[32];
} bench_sign;

static void bench_sign_setup(void* arg) {
    int i;
    bench_sign *data = (bench_sign*)arg;

    for (i = 0; i < 32; i++) {
        data->msg[i] = i + 1;
    }
    for (i = 0; i < 32; i++) {
        data->key[i] = i + 65;
    }
}

static void bench_sign_run(void* arg) {
    int i;
    bench_sign *data = (bench_sign*)arg;

    unsigned char sig[74];
    for (i = 0; i < V_RUNS; i++) {
        size_t siglen = 74;
        int j;
        secp256k1_ecdsa_signature signature;
        CHECK(secp256k1_ecdsa_sign(data->ctx, &signature, data->msg, data->key, NULL, NULL));
        CHECK(secp256k1_ecdsa_signature_serialize_der(data->ctx, sig, &siglen, &signature));
        for (j = 0; j < 32; j++) {
            data->msg[j] = sig[j];
            data->key[j] = sig[j + 32];
        }
    }
}

int bench_sign_main(void) {
    bench_sign data;

    data.ctx = secp256k1_context_create(SECP256K1_CONTEXT_SIGN);

    run_benchmark("ecdsa_sign", bench_sign_run, bench_sign_setup, NULL, &data, 10, 20000);

    secp256k1_context_destroy(data.ctx);

    return 0;
}

typedef struct {
    secp256k1_context *ctx;
    secp256k1_pubkey point;
    unsigned char scalar[32];
} bench_ecdh_data;

static void bench_ecdh_setup(void* arg) {
    int i;
    bench_ecdh_data *data = (bench_ecdh_data*)arg;
    const unsigned char point[] = {
        0x03,
        0x54, 0x94, 0xc1, 0x5d, 0x32, 0x09, 0x97, 0x06,
        0xc2, 0x39, 0x5f, 0x94, 0x34, 0x87, 0x45, 0xfd,
        0x75, 0x7c, 0xe3, 0x0e, 0x4e, 0x8c, 0x90, 0xfb,
        0xa2, 0xba, 0xd1, 0x84, 0xf8, 0x83, 0xc6, 0x9f
    };

    /* create a context with no capabilities */
    data->ctx = secp256k1_context_create(SECP256K1_FLAGS_TYPE_CONTEXT);
    for (i = 0; i < 32; i++) {
        data->scalar[i] = i + 1;
    }
    CHECK(secp256k1_ec_pubkey_parse(data->ctx, &data->point, point, sizeof(point)) == 1);
}

int main(void) {
    if (current_coreid() != 0) {
        while (1)
            asm volatile ("wfi");
    }

    sysctl_pll_set_freq(SYSCTL_PLL0, 800000000UL);

    uarths_init();
    usleep(100000);

    printf("Running secp256k1 benchamrks with CPU clock %u\n", sysctl_clock_get_freq(SYSCTL_CLOCK_CPU));
    printf("see add_compile_flags in cmake/compile-flags.cmake to tweak optimization flags\n");
    printf("  WINDOW_A is %d\n", WINDOW_A);
    printf("  WINDOW_G is %d\n", WINDOW_G);
    bench_verify_main();
    bench_sign_main();
    printf("[done]\n");

    while (1)
        asm volatile ("wfi");
    return 0;
}
