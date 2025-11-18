#include <jni.h>
#include <string>
#include <vector>
#include <sstream>
#include <android/log.h>
#include <sys/random.h> // For getrandom(2)

// --- JNI LOGGING HELPER ---
#define LOG_TAG "PhiRingNDK"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO,  LOG_TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

// --- SECURE MEMORY ZEROIZATION ---
// Uses volatile to prevent the compiler from optimizing away the memory wipe.
// This is the fallback if explicit_bzero is not available.
static void secure_wipe(void* p, size_t len) {
    volatile unsigned char* v = static_cast<volatile unsigned char*>(p);
    while (len--) *v++ = 0;
}

// --- JNI: splitKeyNative ---
extern "C" JNIEXPORT jobjectArray JNICALL
Java_com_phiring_core_SssCore_splitKeyNative(
        JNIEnv* env, jobject thiz, jstring secret_hex, jint n, jint k) {

    // Acquire secret string from JNI (must be zeroized later)
    const char* sh = env->GetStringUTFChars(secret_hex, nullptr);
    std::string secret(sh);
    env->ReleaseStringUTFChars(secret_hex, sh);

    LOGI("Split: K=%d N=%d. Secret length: %zu", k, n, secret.size());
    
    // Hardened Validation
    if (k < 2 || n < k || n > 255) { 
        LOGE("Invalid K/N bounds: K=%d, N=%d.", k, n); 
        secure_wipe(&secret[0], secret.size());
        return nullptr; 
    }
    
    // ---- PLACEHOLDER: REPLACE WITH REAL GF(2^8) SSS SPLIT (Horner) ----
    std::vector<std::string> out;
    for (int x = 1; x <= n; ++x) {
        // TODO: Implement getrandom(2) for coefficients
        // TODO: Implement GF(256) Horner evaluation
        // Current: deterministically creating fake shares for architecture testing
        std::string y = secret.substr(0, 4) + std::to_string(x) + std::string(secret.size() - 4, '0');
        out.emplace_back(std::to_string(x) + ":" + y);
    }
    // -------------------------------------------------------------------

    // CRITICAL: Wipe the secret from memory immediately after use
    secure_wipe(&secret[0], secret.size());

    // Convert C++ vector of strings to a Java String array (jobjectArray)
    jclass cls = env->FindClass("java/lang/String");
    jobjectArray arr = env->NewObjectArray(out.size(), cls, nullptr);
    for (size_t i = 0; i < out.size(); ++i) {
        env->SetObjectArrayElement(arr, i, env->NewStringUTF(out[i].c_str()));
    }
    return arr;
}

// --- JNI: reconstructKeyNative ---
extern "C" JNIEXPORT jstring JNICALL
Java_com_phiring_core_SssCore_reconstructKeyNative(
        JNIEnv* env, jobject thiz, jobjectArray shares, jint k) {

    jsize len = env->GetArrayLength(shares);
    
    // CRITICAL: Validate against the passed K, not a hardcoded value.
    if (len < k) { 
        LOGE("Insufficient shares: %d provided, %d required.", len, k); 
        return nullptr; 
    }

    // ---- PLACEHOLDER: REPLACE WITH REAL GF(2^8) RECONSTRUCTION (Lagrange) ----
    // TODO: Parse "Index:Value" strings
    // TODO: Perform GF(256) Lagrange interpolation
    std::string secret = "1F3A2B4C5D6E7F809A1B2C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0C1D2E3F4A5B6C7D8E9F0A1B2C3D4E5F6A7B8C9D0E1F";
    // -------------------------------------------------------------------------
    
    return env->NewStringUTF(secret.c_str());
}

// --- JNI: zeroizeCriticalMemory ---
extern "C" JNIEXPORT void JNICALL
Java_com_phiring_core_SssCore_zeroizeCriticalMemory(JNIEnv* env, jobject thiz) {
    LOGI("Zeroize signal received: Running post-op cleanup.");
    // In the final audited version, this will wipe the global GF(256) lookup tables
    // if they are dynamically allocated, or any static buffers.
}
