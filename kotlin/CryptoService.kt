// app/src/main/java/com/your/app/core/CryptoService.kt

import android.util.Base64
import java.math.BigInteger
import java.security.*
import java.security.spec.*
import javax.crypto.*
import javax.crypto.spec.*

class CryptoService {

    private val KEY_ALGORITHM = "AES/GCM/NoPadding"
    // P-256 is the standard curve used by Android and many protocols
    private val CURVE_NAME = "secp256r1" 

    // ... (Your public decrypt method remains the same)

    fun decrypt(messageData: MessageData, serverPrivateKeyHex: String): String {
        // 1. Import Keys
        val serverPrivateKey = importServerPrivateKey(serverPrivateKeyHex)
        val clientPublicKey = importClientPublicKey(messageData.client_dh_pub)

        // 2. Derive Shared Secret (ECDH)
        val sharedSecretKey = deriveSharedSecret(serverPrivateKey, clientPublicKey)
        
        // 3. Decrypt Ciphertext (AES-GCM)
        val iv = Base64.decode(messageData.iv, Base64.NO_WRAP)
        val ciphertext = Base64.decode(messageData.ciphertext, Base64.NO_WRAP)
        
        val cipher = Cipher.getInstance(KEY_ALGORITHM)
        // 128 bit tag length is standard for AES-GCM
        val gcmSpec = GCMParameterSpec(128, iv) 
        
        cipher.init(Cipher.DECRYPT_MODE, sharedSecretKey, gcmSpec)
        
        val decryptedBytes = cipher.doFinal(ciphertext)
        
        return String(decryptedBytes, Charsets.UTF_8)
    }

    // --- KEY IMPORT LOGIC ---

    /**
     * IMPORTS RAW PRIVATE SCALAR (s) FROM SSS OUTPUT.
     * This is the critical fix for JCA compliance without PKCS#8 or Bouncy Castle.
     */
    private fun importServerPrivateKey(keyHex: String): PrivateKey {
        // 1. Get the raw scalar 's' (BigInteger) from SSS output
        val s = BigInteger(keyHex, 16)
        
        // 2. Get the EC Domain Parameters for P-256 (secp256r1)
        val ecParams = getEcParamSpec(CURVE_NAME)
        
        // 3. Create the ECPrivateKeySpec from the scalar 's' and the domain parameters
        val ecSpec = ECPrivateKeySpec(s, ecParams)
        
        // 4. Generate the PrivateKey object
        return KeyFactory.getInstance("EC").generatePrivate(ecSpec)
    }
    
    // ... (Your importClientPublicKey and deriveSharedSecret methods remain the same)
    
    private fun importClientPublicKey(base64Key: String): PublicKey {
        val keyBytes = Base64.decode(base64Key, Base64.NO_WRAP)
        val keySpec = X509EncodedKeySpec(keyBytes)
        return KeyFactory.getInstance("EC").generatePublic(keySpec)
    }

    private fun deriveSharedSecret(privateKey: PrivateKey, publicKey: PublicKey): SecretKey {
        val keyAgreement = KeyAgreement.getInstance("ECDH")
        keyAgreement.init(privateKey)
        keyAgreement.doPhase(publicKey, true)
        
        // Use the key agreement result to generate the AES-256 key
        return keyAgreement.generateSecret("AES") as SecretKey 
    }

    // --- EC PARAMETER HELPER (CRITICAL FOR AIR-GAPPED JCA) ---

    /**
     * Generates the standard ECParameterSpec for a named curve using JCA.
     * This avoids external library dependencies by relying on the OS's provider.
     * @param curveName The name of the curve (e.g., "secp256r1").
     * @return The ECParameterSpec object.
     */
    private fun getEcParamSpec(curveName: String): ECParameterSpec {
        // We use a temporary KeyPairGenerator to initialize with the named curve,
        // then extract the parameters, which is a common JCA trick.
        val kpg = KeyPairGenerator.getInstance("EC")
        
        // ECGenParameterSpec requests the curve by name from the provider
        val ecGenSpec = ECGenParameterSpec(curveName)
        kpg.initialize(ecGenSpec, SecureRandom())
        
        // Generate the key pair, then immediately discard the keys and extract the specs
        val tempKeyPair = kpg.genKeyPair()
        val ecPrivateKey = tempKeyPair.private as java.security.interfaces.ECPrivateKey
        
        return ecPrivateKey.params
    }
}
