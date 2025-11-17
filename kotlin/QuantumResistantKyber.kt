// Enhanced Kyber implementation with side-channel protection
class QuantumResistantKyber {
    companion object {
        private const val N = 256
        private const val Q = 3329
        private const val K = 4 // Kyber-1024 uses k=4
        private const val ETA1 = 2
        private const val ETA2 = 2
        private const val DU = 11
        private const val DV = 5
    }
    
    class KyberKeyPair(
        val publicKey: KyberPublicKey,
        val secretKey: KyberSecretKey
    ) : AutoCloseable {
        override fun close() {
            secretKey.zeroize()
        }
    }
    
    class KyberPublicKey(val data: ByteArray) {
        fun serialize(): ByteArray = data.copyOf()
    }
    
    class KyberSecretKey(private val data: ByteArray) : Zeroizable {
        fun getBytes(): ByteArray = data.copyOf()
        
        override fun zeroize() {
            data.fill(0)
        }
    }
    
    interface Zeroizable {
        fun zeroize()
    }
    
    fun generateKeyPair(): KyberKeyPair {
        val secureRandom = SecureRandom.getInstanceStrong()
        
        // Generate random seed
        val seed = ByteArray(32)
        secureRandom.nextBytes(seed)
        
        // Key generation with constant-time operations
        val (publicKeyData, secretKeyData) = generateKeyPairFromSeed(seed)
        
        return KyberKeyPair(
            KyberPublicKey(publicKeyData),
            KyberSecretKey(secretKeyData)
        )
    }
    
    fun encapsulate(publicKey: KyberPublicKey): Pair<ByteArray, ByteArray> {
        val secureRandom = SecureRandom.getInstanceStrong()
        val randomBytes = ByteArray(32)
        secureRandom.nextBytes(randomBytes)
        
        return encapsulateWithRandomness(publicKey, randomBytes)
    }
    
    fun decapsulate(ciphertext: ByteArray, secretKey: KyberSecretKey): ByteArray {
        return decapsulateConstantTime(ciphertext, secretKey)
    }
    
    // Constant-time key generation to prevent side-channel attacks
    private fun generateKeyPairFromSeed(seed: ByteArray): Pair<ByteArray, ByteArray> {
        val hash = sha3Hash(seed)
        val rho = hash.sliceArray(0..31)
        val sigma = hash.sliceArray(32..63)
        
        // Generate matrix A in NTT domain
        val matrixA = generateMatrixA(rho)
        
        // Generate secret vector s and error vector e
        val s = generateSecretVector(sigma, 0)
        val e = generateErrorVector(sigma, K)
        
        // Compute public key: t = A*s + e
        val sNTT = nttTransform(s)
        val t = computePublicKey(matrixA, sNTT, e)
        
        val publicKey = encodePublicKey(t, rho)
        val secretKey = encodeSecretKey(s)
        
        return Pair(publicKey, secretKey)
    }
    
    // Side-channel resistant encapsulation
    private fun encapsulateWithRandomness(
        publicKey: KyberPublicKey, 
        randomness: ByteArray
    ): Pair<ByteArray, ByteArray> {
        val (t, rho) = decodePublicKey(publicKey.data)
        val matrixA = generateMatrixA(rho)
        
        // Generate ephemeral keys with constant timing
        val r = generateSecretVector(randomness, 0)
        val e1 = generateErrorVector(randomness, K)
        val e2 = generateErrorVector(randomness, K + 1)
        
        val rNTT = nttTransform(r)
        
        // Compute ciphertext components
        val u = computeCiphertextU(matrixA, rNTT, e1)
        val v = computeCiphertextV(t, r, e2, randomness)
        
        val ciphertext = encodeCiphertext(u, v)
        val sharedSecret = sha3Hash(randomness + sha3Hash(publicKey.data))
        
        return Pair(ciphertext, sharedSecret)
    }
    
    // Constant-time decapsulation to prevent timing attacks
    private fun decapsulateConstantTime(
        ciphertext: ByteArray, 
        secretKey: KyberSecretKey
    ): ByteArray {
        val s = decodeSecretKey(secretKey.getBytes())
        val (u, v) = decodeCiphertext(ciphertext)
        
        // Decrypt with constant-time operations
        val sNTT = nttTransform(s)
        val message = decrypt(u, v, sNTT)
        
        // Constant-time comparison for security
        val isValid = constantTimeEquals(message, expectedMessage(ciphertext))
        
        return if (isValid) {
            sha3Hash(message)
        } else {
            // Return pseudorandom value on failure to prevent oracle attacks
            sha3Hash(secretKey.getBytes() + ciphertext)
        }
    }
    
    // NTT operations for polynomial arithmetic
    private fun nttTransform(poly: IntArray): IntArray {
        val result = poly.copyOf()
        nttInPlace(result)
        return result
    }
    
    private fun nttInPlace(poly: IntArray) {
        var j = N / 2
        while (j >= 1) {
            var k = 0
            while (k < N) {
                val zeta = getZeta(k / (2 * j))
                for (i in k until k + j) {
                    val t = montgomeryReduce(zeta.toLong() * poly[i + j])
                    poly[i + j] = poly[i] - t
                    poly[i] = poly[i] + t
                }
                k += 2 * j
            }
            j /= 2
        }
    }
    
    // Montgomery reduction for efficient modular arithmetic
    private fun montgomeryReduce(a: Long): Int {
        val m = ((a and 0xFFFF) * Q_INV) and 0xFFFF
        val t = (a + m * Q) shr 16
        return if (t >= Q) (t - Q).toInt() else t.toInt()
    }
    
    // Constant-time comparison to prevent timing attacks
    private fun constantTimeEquals(a: ByteArray, b: ByteArray): Boolean {
        if (a.size != b.size) return false
        
        var result = 0
        for (i in a.indices) {
            result = result or (a[i].toInt() xor b[i].toInt())
        }
        return result == 0
    }
    
    // SHA-3 hash function
    private fun sha3Hash(data: ByteArray): ByteArray {
        val digest = MessageDigest.getInstance("SHA3-256")
        return digest.digest(data)
    }
}
