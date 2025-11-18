const request = require('supertest');
const app = require('../../server');
const { PQCEngine } = require('../../utils/pqc');

describe('φ-RING Security Integration Tests', () => {
    let apiKey = null;
    let sessionToken = null;
    let pqcEngine = null;

    beforeAll(async () => {
        pqcEngine = new PQCEngine();
        await pqcEngine.initialize();
    });

    describe('Post-Quantum Key Generation', () => {
        test('should generate secure API key with PQC', async () => {
            const response = await request(app)
                .post('/api/auth/generate-key')
                .send({
                    clientId: 'test-client-001',
                    permissions: ['READ_METRICS', 'WRITE_LOGS'],
                    attributes: {
                        role: 'admin',
                        department: 'security'
                    }
                });

            expect(response.status).toBe(200);
            expect(response.body.success).toBe(true);
            expect(response.body.data.apiKey).toHaveLength(64);
            expect(response.body.data.algorithm).toBe('KYBER_768');
            
            apiKey = response.body.data.apiKey;
        });

        test('should reject invalid client ID', async () => {
            const response = await request(app)
                .post('/api/auth/generate-key')
                .send({
                    clientId: 'short',
                    permissions: ['READ_METRICS']
                });

            expect(response.status).toBe(400);
            expect(response.body.success).toBe(false);
        });
    });

    describe('ABAC Authentication', () => {
        test('should authenticate with valid context', async () => {
            const timestamp = Date.now();
            const message = `${apiKey}:${timestamp}`;
            const signature = await pqcEngine.sign(message);

            const response = await request(app)
                .post('/api/auth/authenticate')
                .send({
                    apiKey,
                    timestamp,
                    signature,
                    deviceFingerprint: 'test-device-001',
                    context: {
                        location: 'Trinidad',
                        deviceIntegrity: 'PASS',
                        networkType: 'wifi',
                        deviceRooted: false
                    }
                });

            expect(response.status).toBe(200);
            expect(response.body.success).toBe(true);
            expect(response.body.data.sessionToken).toHaveLength(64);
            expect(response.body.data.abacStatus).toBe('PERMIT');
            
            sessionToken = response.body.data.sessionToken;
        });

        test('should deny access with suspicious behavior', async () => {
            const timestamp = Date.now();
            const message = `${apiKey}:${timestamp}`;
            const signature = await pqcEngine.sign(message);

            // Simulate impossible travel
            const response = await request(app)
                .post('/api/auth/authenticate')
                .send({
                    apiKey,
                    timestamp,
                    signature,
                    deviceFingerprint: 'suspicious-device',
                    context: {
                        location: 'London', // Impossible from previous request
                        deviceIntegrity: 'FAIL',
                        networkType: 'tor',
                        deviceRooted: true
                    }
                });

            expect(response.status).toBe(403);
            expect(response.body.success).toBe(false);
            expect(response.body.error).toContain('Access denied by security policy');
        });
    });

    describe('Security Metrics', () => {
        test('should fetch metrics with valid session', async () => {
            const response = await request(app)
                .get('/api/metrics/current')
                .set('Authorization', `Bearer ${sessionToken}`);

            expect(response.status).toBe(200);
            expect(response.body.success).toBe(true);
            expect(response.body.data).toHaveProperty('supabase');
            expect(response.body.data).toHaveProperty('drive');
            expect(response.body.data).toHaveProperty('gas');
            expect(response.body.data).toHaveProperty('github');
        });

        test('should deny metrics without session', async () => {
            const response = await request(app)
                .get('/api/metrics/current');

            expect(response.status).toBe(401);
            expect(response.body.success).toBe(false);
        });
    });

    describe('Secure Logging with SSS', () => {
        test('should save encrypted log with SSS', async () => {
            const logEntry = {
                event: 'SECURITY_CHECK',
                details: 'Routine scan completed',
                severity: 'INFO'
            };

            const response = await request(app)
                .post('/api/logs/secure')
                .set('Authorization', `Bearer ${sessionToken}`)
                .send({
                    logEntry,
                    encrypt: true
                });

            expect(response.status).toBe(200);
            expect(response.body.success).toBe(true);
            expect(response.body.data).toHaveProperty('sssShare');
            expect(response.body.data).toHaveProperty('integrityHash');
        });
    });

    describe('Rate Limiting & Surge Protection', () => {
        test('should enforce rate limits', async () => {
            const requests = [];
            
            // Send 101 requests (limit is 100 per 15 minutes)
            for (let i = 0; i < 101; i++) {
                requests.push(
                    request(app)
                        .get('/api/metrics/current')
                        .set('Authorization', `Bearer ${sessionToken}`)
                );
            }

            const responses = await Promise.all(requests);
            const rateLimitedResponse = responses[100];

            expect(rateLimitedResponse.status).toBe(429);
            expect(rateLimitedResponse.body.error).toContain('Too many requests');
        });
    });

    describe('Session Management', () => {
        test('should revoke session successfully', async () => {
            const response = await request(app)
                .post('/api/auth/revoke')
                .send({
                    sessionToken
                });

            expect(response.status).toBe(200);
            expect(response.body.success).toBe(true);
            expect(response.body).toHaveProperty('sssShare');
        });

        test('should deny access with revoked session', async () => {
            const response = await request(app)
                .get('/api/metrics/current')
                .set('Authorization', `Bearer ${sessionToken}`);

            expect(response.status).toBe(401);
            expect(response.body.success).toBe(false);
        });
    });
});

describe('Error Handling', () => {
    test('should handle malformed requests gracefully', async () => {
        const response = await request(app)
            .post('/api/auth/authenticate')
            .send({
                apiKey: 'invalid-key',
                timestamp: 'not-a-timestamp',
                signature: 'invalid-signature'
            });

        expect(response.status).toBe(400);
        expect(response.body.success).toBe(false);
        expect(response.body.error).toBe('Invalid authentication parameters');
    });

    test('should return zero-knowledge errors', async () => {
        const response = await request(app)
            .post('/api/auth/generate-key')
            .send({}); // Empty body

        expect(response.status).toBe(400);
        expect(response.body.error).toBe('Invalid request parameters');
        expect(response.body.details).toBeDefined();
    });
});
