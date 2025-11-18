# φ-RING: Native SSS Module Implementation (v1.0)

**Date:** November 19, 2025  
**Module:** `app/src/main/cpp/` & `app/src/main/java/com/phiring/core/`  
**Status:** Audit-Ready Structure (Math Placeholders Active)  
**Compliance:** EU-CRA Ready, NDK r27+, C++20

---

## 1. The Kotlin Bridge
**Path:** `app/src/main/java/com/phiring/core/SssCore.kt`

This class defines the JNI contract. It is the only entry point for the UI to access the cryptographic core.

```kotlin
package com.phiring.core

/**
 * The bridge class for the Shamir Secret Sharing Native C++ Core.
 * This class handles all high-security, constant-time, zeroised operations.
 *
 * ARCHITECTURE NOTE:
 * - Loads "sss_core" (C++20/NDK).
 * - No logic resides here; this is a pass-through to the hardened native layer.
 * - All network access should be blocked while this class is in memory.
 */
class SssCore {

    init {
        // Must match the library name in your CMakeLists.txt
        System.loadLibrary("sss_core")
    }

    // --- NATIVE SPLIT FUNCTION ---
    /**
     * Splits a master key (hex string) into N shares with threshold K.
     * @param secretHex The secret to split (hex encoded).
     * @param N The total number of shares to generate (max 255).
     * @param K The reconstruction threshold.
     * @return An Array of String shares in "X:Y" format, or null on error.
     */
    external fun splitKeyNative(secretHex: String, N: Int, K: Int): Array<String>?

    // --- NATIVE RECONSTRUCT FUNCTION ---
    /**
     * Reconstructs the master key from an array of shares.
     * @param sharesArray The array of "X:Y" strings.
     * @param K The threshold required (must match the K used during split).
     * @return The reconstructed master key as a hex string, or null on failure.
     */
    external fun reconstructKeyNative(sharesArray: Array<String>, K: Int): String?

    // --- NATIVE SECURITY FUNCTION ---
    /**
     * Forces immediate memory zeroization on critical buffers in the C++ layer.
     * Call this when the activity is destroyed, the timer expires, or the session ends.
     */
    external fun zeroizeCriticalMemory()
}
