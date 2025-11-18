// app/src/main/java/com/your/app/core/Models.kt

/**
 * Data received from the "encrypted message file" or paste input.
 */
data class MessageData(
    val ciphertext: String,
    val iv: String,
    val client_dh_pub: String,
    val created_at: String? = null
)

/**
 * Single share input for Shamir's Secret Sharing.
 */
data class Share(
    // The x-coordinate (must be an integer, 1, 2, 3, etc.)
    val x: Int,
    // The y-coordinate (BigInt value represented as a hex string)
    val y: String
)

/**
 * Represents the current, authoritative state of the Decryption Process.
 */
sealed class DecryptionState {
    data object Idle : DecryptionState()
    data object LoadingMessage : DecryptionState()
    data class MessageLoaded(val data: MessageData) : DecryptionState()
    data object KeyReconstructed : DecryptionState()
    data object Decrypting : DecryptionState()
    data class Decrypted(val plaintext: String) : DecryptionState()
    data class Error(val message: String) : DecryptionState()
}
