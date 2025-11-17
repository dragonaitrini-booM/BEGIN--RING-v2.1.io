Import java.math.BigInteger
import java.security.SecureRandom

class ShamirSecretSharing {
    companion object {
        private const val FIELD_SIZE = 257L // Prime field
        private val PRIME = BigInteger.valueOf(FIELD_SIZE)
    }

    data class Share(val x: Int, val y: BigInteger)

    fun splitSecret(secret: ByteArray, threshold: Int, numShares: Int): List<Share> {
        require(threshold <= numShares) { "Threshold must be <= number of shares" }
        require(threshold >= 2) { "Threshold must be >= 2" }
        
        val shares = mutableListOf<Share>()
        val secureRandom = SecureRandom()
        
        // Convert secret to polynomial coefficients
        val coefficients = mutableListOf<BigInteger>()
        coefficients.add(BigInteger(1, secret).mod(PRIME))
        
        // Generate random coefficients for polynomial
        for (i in 1 until threshold) {
            val coeff = BigInteger(PRIME.bitLength(), secureRandom).mod(PRIME)
            coefficients.add(coeff)
        }
        
        // Generate shares
        for (i in 1..numShares) {
            val x = i
            val y = evaluatePolynomial(coefficients, x)
            shares.add(Share(x, y))
        }
        
        return shares
    }
    
    fun reconstructSecret(shares: List<Share>): ByteArray {
        require(shares.size >= 2) { "Need at least 2 shares to reconstruct" }
        
        // Lagrange interpolation to find f(0)
        var secret = BigInteger.ZERO
        
        for (i in shares.indices) {
            var numerator = BigInteger.ONE
            var denominator = BigInteger.ONE
            
            for (j in shares.indices) {
                if (i != j) {
                    numerator = numerator.multiply(BigInteger.valueOf(-shares[j].x.toLong())).mod(PRIME)
                    denominator = denominator.multiply(
                        BigInteger.valueOf((shares[i].x - shares[j].x).toLong())
                    ).mod(PRIME)
                }
            }
            
            val lagrangeCoeff = numerator.multiply(modInverse(denominator, PRIME)).mod(PRIME)
            secret = secret.add(shares[i].y.multiply(lagrangeCoeff)).mod(PRIME)
        }
        
        return secret.toByteArray()
    }
    
    private fun evaluatePolynomial(coefficients: List<BigInteger>, x: Int): BigInteger {
        var result = BigInteger.ZERO
        var xPower = BigInteger.ONE
        
        for (coeff in coefficients) {
            result = result.add(coeff.multiply(xPower)).mod(PRIME)
            xPower = xPower.multiply(BigInteger.valueOf(x.toLong())).mod(PRIME)
        }
        
        return result
    }
    
    private fun modInverse(a: BigInteger, m: BigInteger): BigInteger {
        return a.modInverse(m)
    }
}
