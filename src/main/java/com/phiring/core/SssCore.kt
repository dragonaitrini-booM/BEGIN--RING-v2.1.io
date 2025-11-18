package com.phiring.core

/**
 * The bridge class for the Shamir Secret Sharing Native C++ Core.
 * This class handles all high-security, constant-time, zeroised operations.
 *
 * ARCHITECTURE NOTE:
 * - Loads "sss_core" (C++20/NDK).
 * - No logic resides here; this is a pass-through to the hardened native layer.
 */
class SssCore {

    init {
        // Must match the library name in your CMakeLists.txt
        System.loadLibrary("sss_core")
    }

    // --- NATIVE SPLIT FUNCTION ---
    /**
     * Splits a master key (hex string) into N shares with threshold K.
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
     * Call this when the activity is destroyed or the session ends.
     */
    external fun zeroizeCriticalMemory()
}
