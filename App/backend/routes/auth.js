const express = require('express');
const { body, validationResult } = require('express-validator');
const crypto = require('crypto');
const { PQCEngine } = require('../utils/pqc');
const { ABACEngine } = require('../utils/abac');
const { BehavioralAnalytics } = require('../utils/behavioral');

const router = express.Router();
const pqcEngine = new PQCEngine();
const abacEngine = new ABACEngine();
const behavioral = new BehavioralAnalytics();

// Post-Quantum Key Generation
router.post('/generate-key', 
    body('clientId').isLength({ min: 16, max: 64 }).isAlphanumeric(),
    body('permissions').isArray().custom((arr) => arr.length > 0),
    async (req, res) => {
        try {
            const errors = validationResult(req);
            if (!errors.isEmpty()) {
                return res.status(400).json({ 
                    success: false, 
                    error: 'Invalid request parameters',
                    details: errors.array() 
                });
            }

            const { clientId, permissions, attributes = {} } = req.body;

            // Behavioral analytics check
            const behavioralScore = await behavioral.analyzeRequest(req);
            if (behavioralScore < 0.3) {
                return res.status(403).json({
                    success: false,
                    error: 'Behavioral anomaly detected'
                });
            }

            // Generate post-quantum key pair
            const keyPair = await pqcEngine.generateKeyPair();
            
            // Create secure API key
            const apiKey = crypto.randomBytes(32).toString('hex');
            const keyId = crypto.randomBytes(16).toString('hex');
            
            // ABAC policy creation
            const policy = await abacEngine.createPolicy({
                keyId,
                permissions,
                attributes: {
                    ...attributes,
                    clientId,
                    createdAt: new Date(),
                    behavioralScore
                }
            });

            // Store in encrypted database (simulated)
            const encryptedData = await pqcEngine.encrypt({
                apiKey,
                keyId,
                permissions,
                policyId: policy.id,
                expiresAt: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000), // 30 days
                maxCalls: 10000
            });

            res.json({
                success: true,
                data: {
                    apiKey,
                    keyId,
                    publicKey: keyPair.publicKey,
                    algorithm: 'KYBER_768',
                    expiresAt: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000),
                    permissions
                }
            });

        } catch (error) {
            console.error('Key generation error:', error);
            res.status(500).json({
                success: false,
                error: 'Key generation failed',
                code: 'KEYGEN_FAILED'
            });
        }
    }
);

// Post-Quantum Authentication
router.post('/authenticate',
    body('apiKey').isLength({ min: 64, max: 64 }).isHexadecimal(),
    body('timestamp').isInt({ min: Date.now() - 300000, max: Date.now() + 300000 }),
    body('signature').isLength({ min: 128, max: 256 }),
    body('deviceFingerprint').isLength({ min: 32, max: 128 }),
    async (req, res) => {
        try {
            const errors = validationResult(req);
            if (!errors.isEmpty()) {
                return res.status(400).json({ 
                    success: false, 
                    error: 'Invalid authentication parameters',
                    details: errors.array() 
                });
            }

            const { apiKey, timestamp, signature, deviceFingerprint, context = {} } = req.body;

            // Anti-replay check
            if (Math.abs(Date.now() - timestamp) > 300000) { // 5 minute window
                return res.status(401).json({
                    success: false,
                    error: 'Request timestamp invalid'
                });
            }

            // Behavioral analytics
            const behavioralScore = await behavioral.analyzeAuthentication(req);
            if (behavioralScore < 0.2) {
                return res.status(403).json({
                    success: false,
                    error: 'Authentication anomaly detected'
                });
            }

            // ABAC evaluation
            const abacContext = {
                deviceFingerprint,
                timestamp: new Date(timestamp),
                behavioralScore,
                location: context.location,
                deviceIntegrity: context.deviceIntegrity,
                networkType: context.networkType,
                timeOfDay: new Date().getHours()
            };

            const abacDecision = await abacEngine.evaluate(abacContext);
            if (abacDecision !== 'PERMIT') {
                return res.status(403).json({
                    success: false,
                    error: 'Access denied by security policy',
                    policyViolation: abacDecision
                });
            }

            // Post-quantum signature verification
            const isValidSignature = await pqcEngine.verifySignature(
                apiKey,
                `${apiKey}:${timestamp}`,
                signature
            );

            if (!isValidSignature) {
                return res.status(401).json({
                    success: false,
                    error: 'Invalid signature'
                });
            }

            // Generate secure session
            const sessionToken = crypto.randomBytes(32).toString('hex');
            const sessionData = {
                apiKey,
                authenticatedAt: new Date(),
                expiresAt: new Date(Date.now() + 3600000), // 1 hour
                permissions: await getKeyPermissions(apiKey),
                behavioralScore,
                deviceFingerprint
            };

            res.json({
                success: true,
                data: {
                    sessionToken,
                    expiresAt: sessionData.expiresAt,
                    permissions: sessionData.permissions,
                    pqcStatus: 'VERIFIED',
                    abacStatus: 'PERMIT',
                    behavioralScore
                }
            });

        } catch (error) {
            console.error('Authentication error:', error);
            res.status(500).json({
                success: false,
                error: 'Authentication failed',
                code: 'AUTH_FAILED'
            });
        }
    }
);

// Session revocation
router.post('/revoke',
    body('sessionToken').isLength({ min: 64, max: 64 }).isHexadecimal(),
    async (req, res) => {
        try {
            const { sessionToken } = req.body;
            
            // Immediate revocation
            await revokeSession(sessionToken);
            
            // Log revocation event with SSS
            const revocationLog = {
                event: 'SESSION_REVOKED',
                sessionToken: sessionToken.substring(0, 8) + '...',
                timestamp: new Date(),
                reason: 'USER_REQUEST'
            };
            
            // SSS logging
            const sssShare = await sssEngine.createShare(JSON.stringify(revocationLog));
            
            res.json({
                success: true,
                message: 'Session revoked successfully',
                sssShare
            });
            
        } catch (error) {
            res.status(500).json({
                success: false,
                error: 'Revocation failed'
            });
        }
    }
);

// Helper functions
async function getKeyPermissions(apiKey) {
    // Simulated - would query encrypted database
    return ['READ_METRICS', 'WRITE_LOGS', 'TRIGGER_SECURITY_CHECK'];
}

async function revokeSession(sessionToken) {
    // Simulated - would update session store
    console.log(`Session ${sessionToken} revoked`);
}

module.exports = router;
