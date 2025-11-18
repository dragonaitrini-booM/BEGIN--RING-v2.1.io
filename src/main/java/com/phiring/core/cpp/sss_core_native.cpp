#include <jni.h>
#include <string>
#include <vector>
#include <android/log.h>
#include <sys/random.h>

#define LOG_TAG "PhiRingNDK"
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

// Secure wipe helper
static void secure_wipe(void* p, size_t len) {
    volatile unsigned char* v = static_cast<volatile unsigned char*>(p);
    while (len--) *v++ = 0;
}

extern "C" JNIEXPORT jobjectArray JNICALL
Java_com_phiring_core_SssCore_splitKeyNative(
        JNIEnv* env, jobject, jstring secret_hex, jint n, jint k) {

    const char* sh = env->GetStringUTFChars(secret_hex, nullptr);
    std::string secret(sh);
    env->ReleaseStringUTFChars(secret_hex, sh);

    if (k < 2 || n < k || n > 255) {
        LOGE("Invalid K/N");
        secure_wipe(&secret[0], secret.size());
        return nullptr;
    }

    // PLACEHOLDER LOGIC (Ready for Math)
    std::vector<std::string> out;
    for (int x = 1; x <= n; ++x) {
        std::string y = secret.substr(0, 4) + std::to_string(x) + "0000000000000000000000000000000000000000000000000000000000000000";
        out.emplace_back(std::to_string(x) + ":" + y);
    }

    secure_wipe(&secret[0], secret.size());

    jclass cls = env->FindClass("java/lang/String");
    jobjectArray arr = env->NewObjectArray(out.size(), cls, nullptr);
    for (size_t i = 0; i < out.size(); ++i) {
        env->SetObjectArrayElement(arr, i, env->NewStringUTF(out[i].c_str()));
    }
    return arr;
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_phiring_core_SssCore_reconstructKeyNative(
        JNIEnv* env, jobject, jobjectArray shares, jint k) {

    jsize len = env->GetArrayLength(shares);
    
    // FIX: Validate against 'k', not hardcoded 3
    if (len < k) {
        LOGE("Insufficient shares");
        return nullptr;
    }

    // PLACEHOLDER LOGIC (Ready for Math)
    std::string secret = "1F3A2B4C5D6E7F809A1B2C3D4E5F6A7B8C9D0E1F2A3B4C5D6E7F8A9B0C1D2E3F4A5B6C7D8E9F0A1B2C3D4E5F6A7B8C9D0E1F";

    return env->NewStringUTF(secret.c_str());
}

extern "C" JNIEXPORT void JNICALL
Java_com_phiring_core_SssCore_zeroizeCriticalMemory(JNIEnv*, jobject) {
    // Ready for cleanup logic
}
