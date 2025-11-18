package com.phiring.core

class SssCore {

    init {
        System.loadLibrary("sss_core")
    }

    /**
     * Splits a master key (hex string) into N shares with threshold K.
     */
    external fun splitKeyNative(secretHex: String, N: Int, K: Int): Array<String>?

    /**
     * Reconstructs the master key from an array of shares.
     * NOW FIX: Accepts 'K' explicitly to prevent hardcoding errors.
     */
    external fun reconstructKeyNative(sharesArray: Array<String>, K: Int): String?

    /**
     * Forces immediate memory zeroization on critical buffers.
     */
    external fun zeroizeCriticalMemory()
}
