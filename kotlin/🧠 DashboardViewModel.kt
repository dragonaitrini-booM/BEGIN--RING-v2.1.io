// app/src/main/java/com/your/app/pages/dashboard/DashboardViewModel.kt
package com.your.app.pages.dashboard

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.your.app.core.*
import com.google.gson.Gson
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

class DashboardViewModel(
    // Dependencies can be passed for testability
    private val sssEngine: SssDecryptionEngine = SssDecryptionEngine(),
    private val cryptoService: CryptoService = CryptoService(),
    private val gson: Gson = Gson()
) : ViewModel() {

    // --- State Management ---
    private val _state = MutableStateFlow<DecryptionState>(DecryptionState.Idle)
    val state: StateFlow<DecryptionState> = _state

    // Private fields to hold intermediate secrets
    private var loadedMessage: MessageData? = null
    private var reconstructedKeyHex: String? = null

    // --- STEP 1: Load Message from JSON Input ---
    fun loadMessage(jsonString: String) = viewModelScope.launch {
        _state.value = DecryptionState.LoadingMessage
        if (jsonString.isBlank()) {
            _state.value = DecryptionState.Error("JSON input cannot be empty.")
            return@launch
        }
        try {
            val message = gson.fromJson(jsonString, MessageData::class.java)
            if (message.ciphertext.isBlank() || message.iv.isBlank() || message.client_dh_pub.isBlank()) {
                throw IllegalArgumentException("Missing required fields (ciphertext, iv, client_dh_pub).")
            }
            // SUCCESS
            loadedMessage = message
            _state.value = DecryptionState.MessageLoaded(message)
        } catch (e: Exception) {
            _state.value = DecryptionState.Error("Invalid JSON structure or data. Check format.")
        }
    }

    // --- STEP 2: Reconstruct Private Key from Shares ---
    fun reconstructKey(shareStrings: List<String>) = viewModelScope.launch {
        if (_state.value !is DecryptionState.MessageLoaded) {
            _state.value = DecryptionState.Error("Protocol Sequence Error: Load message first.")
            return@launch
        }

        val validShares = shareStrings.filter { it.isNotBlank() }
        if (validShares.size < 3) {
            _state.value = DecryptionState.Error("SSS Quorum Failure: Minimum of 3 shares required.")
            return@launch
        }

        try {
            // Shares must be formatted as "X:Y_HEX" for the SssDecryptionEngine
            val shares = validShares.map { shareString ->
                val parts = shareString.split(':')
                if (parts.size != 2) throw IllegalArgumentException("Share format must be 'X:Y_HEX'.")
                Share(x = parts[0].trim().toInt(), y = parts[1].trim())
            }

            val keyHex = sssEngine.reconstruct(shares)
            
            // SUCCESS - Key material is now secured in memory
            reconstructedKeyHex = keyHex
            _state.value = DecryptionState.KeyReconstructed

        } catch (e: Exception) {
            _state.value = DecryptionState.Error("SSS Reconstruction Failed: ${e.message}")
        }
    }

    // --- STEP 3: Decrypt Message ---
    fun decryptMessage() = viewModelScope.launch {
        if (loadedMessage == null || reconstructedKeyHex == null) {
            _state.value = DecryptionState.Error("Protocol Sequence Error: Complete steps 1 and 2.")
            return@launch
        }
        
        _state.value = DecryptionState.Decrypting
        try {
            // Note: CryptoService.decrypt will handle the ECDH + AES-GCM logic.
            val plaintext = cryptoService.decrypt(loadedMessage!!, reconstructedKeyHex!!)
            
            // SUCCESS
            _state.value = DecryptionState.Decrypted(plaintext)
            
        } catch (e: Exception) {
            _state.value = DecryptionState.Error("Decryption Failed: JCA/Cipher Error. Key or data corruption.")
        }
    }

    // --- SECURITY PROTOCOL: Wipe Memory ---
    fun clearAllState() {
        loadedMessage = null
        reconstructedKeyHex = null // This is the crucial step: nullify the key
        _state.value = DecryptionState.Idle
    }
}
