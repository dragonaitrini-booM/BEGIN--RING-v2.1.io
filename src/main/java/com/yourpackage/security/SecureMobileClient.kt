// Mobile app security components
import javax.net.ssl.*
import java.security.KeyStore
import java.security.cert.X509Certificate
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyProperties
import androidx.biometric.BiometricPrompt
import androidx.biometric.BiometricManager

class SecureMobileClient {
    companion object {
        private const val ANDROID_KEYSTORE = "AndroidKeyStore"
        private const val KEY_ALIAS = "phi_ring_client_key"
        private const val BIOMETRIC_KEY_ALIAS = "phi_ring_biometric_key"
    }

    private lateinit var keyStore: KeyStore
    private lateinit var sslContext: SSLContext

    fun initializeSecureStorage() {
        keyStore = KeyStore.getInstance(ANDROID_KEYSTORE).apply {
            load(null)
        }
        
        generateBiometricProtectedKey()
        setupMutualTLS()
    }

    private fun generateBiometricProtectedKey() {
        val keyGenerator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, ANDROID_KEYSTORE)
        
        val keyGenParameterSpec = KeyGenParameterSpec.Builder(
            BIOMETRIC_KEY_ALIAS,
            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
        )
        .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
        .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
        .setUserAuthenticationRequired(true)
        .setInvalidatedByBiometricEnrollment(true)
        .setUserAuthenticationParameters(
            30, // 30 seconds timeout
            KeyProperties.AUTH_BIOMETRIC_STRONG or KeyProperties.AUTH_DEVICE_CREDENTIAL
        )
        .build()

        keyGenerator.init(keyGenParameterSpec)
        keyGenerator.generateKey()
    }

    private fun setupMutualTLS() {
        // Load client certificate from secure storage
        val clientKeyStore = KeyStore.getInstance("PKCS12")
        // Load your client.p12 file here
        
        val keyManagerFactory = KeyManagerFactory.getInstance(KeyManagerFactory.getDefaultAlgorithm())
        keyManagerFactory.init(clientKeyStore, "your_p12_password".toCharArray())
        
        val trustManagerFactory = TrustManagerFactory.getInstance(TrustManagerFactory.getDefaultAlgorithm())
        // Load your custom CA certificate
        trustManagerFactory.init(loadCustomTrustStore())
        
        sslContext = SSLContext.getInstance("TLSv1.3").apply {
            init(keyManagerFactory.keyManagers, trustManagerFactory.trustManagers, null)
        }
    }

    private fun loadCustomTrustStore(): KeyStore {
        val trustStore = KeyStore.getInstance(KeyStore.getDefaultType())
        trustStore.load(null)
        
        // Load your custom CA certificate
        val caInputStream = assets.open("client_ca.crt")
        val certificateFactory = CertificateFactory.getInstance("X.509")
        val caCert = certificateFactory.generateCertificate(caInputStream) as X509Certificate
        
        trustStore.setCertificateEntry("phi_ring_ca", caCert)
        return trustStore
    }

    fun encryptWithBiometric(data: ByteArray, callback: (ByteArray?) -> Unit) {
        try {
            val secretKey = keyStore.getKey(BIOMETRIC_KEY_ALIAS, null) as SecretKey
            val cipher = Cipher.getInstance("AES/GCM/NoPadding")
            cipher.init(Cipher.ENCRYPT_MODE, secretKey)
            
            val biometricPrompt = BiometricPrompt(
                this as FragmentActivity,
                ContextCompat.getMainExecutor(this),
                object : BiometricPrompt.AuthenticationCallback() {
                    override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                        super.onAuthenticationSucceeded(result)
                        val cryptoObject = result.cryptoObject
                        val cipher = cryptoObject?.cipher
                        if (cipher != null) {
                            val encryptedData = cipher.doFinal(data)
                            val iv = cipher.iv
                            
                            // Combine IV and encrypted data
                            val result = ByteArray(iv.size + encryptedData.size)
                            System.arraycopy(iv, 0, result, 0, iv.size)
                            System.arraycopy(encryptedData, 0, result, iv.size, encryptedData.size)
                            
                            callback(result)
                        }
                    }
                    
                    override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                        super.onAuthenticationError(errorCode, errString)
                        callback(null)
                    }
                }
            )
            
            val promptInfo = BiometricPrompt.PromptInfo.Builder()
                .setTitle("Authenticate to encrypt data")
                .setSubtitle("Use your biometric credential")
                .setNegativeButtonText("Cancel")
                .build()
                
            biometricPrompt.authenticate(
                promptInfo,
                BiometricPrompt.CryptoObject(cipher)
            )
            
        } catch (e: Exception) {
            callback(null)
        }
    }

    fun decryptWithBiometric(encryptedData: ByteArray, callback: (ByteArray?) -> Unit) {
        try {
            val secretKey = keyStore.getKey(BIOMETRIC_KEY_ALIAS, null) as SecretKey
            val cipher = Cipher.getInstance("AES/GCM/NoPadding")
            
            // Extract IV and encrypted data
            val iv = encryptedData.sliceArray(0..11)
            val cipherText = encryptedData.sliceArray(12 until encryptedData.size)
            
            val spec = GCMParameterSpec(128, iv)
            cipher.init(Cipher.DECRYPT_MODE, secretKey, spec)
            
            val biometricPrompt = BiometricPrompt(
                this as FragmentActivity,
                ContextCompat.getMainExecutor(this),
                object : BiometricPrompt.AuthenticationCallback() {
                    override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                        super.onAuthenticationSucceeded(result)
                        val cryptoObject = result.cryptoObject
                        val cipher = cryptoObject?.cipher
                        if (cipher != null) {
                            try {
                                val decryptedData = cipher.doFinal(cipherText)
                                callback(decryptedData)
                            } catch (e: Exception) {
                                callback(null)
                            }
                        }
                    }
                    
                    override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                        super.onAuthenticationError(errorCode, errString)
                        callback(null)
                    }
                }
            )
            
            val promptInfo = BiometricPrompt.PromptInfo.Builder()
                .setTitle("Authenticate to decrypt data")
                .setSubtitle("Use your biometric credential")
                .setNegativeButtonText("Cancel")
                .build()
                
            biometricPrompt.authenticate(
                promptInfo,
                BiometricPrompt.CryptoObject(cipher)
            )
            
        } catch (e: Exception) {
            callback(null)
        }
    }
}
