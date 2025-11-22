const request = require('supertest');
const app = require('../backend/server'); // Import the actual app

jest.setTimeout(60000); // Set a global timeout of 60 seconds

describe('Security Stress Tests', () => {
  let originalConsoleError;
  let server;

  beforeAll((done) => {
    originalConsoleError = console.error;
    console.error = jest.fn(); // Suppress console.error
    server = app.listen(done); // Start the server
  });

  afterAll(done => {
    console.error = originalConsoleError; // Restore console.error
    server.close(done); // Close the server
  });

  describe('Rate Limiting', () => {
    it('should block requests after the limit is reached', async () => {
      const agent = request.agent(server);
      for (let i = 0; i < 100; i++) {
        await agent.get('/api/health').expect(200);
      }
      await agent.get('/api/health').expect(429);
    });
  });

  describe('Input Validation', () => {
    it('should handle large payloads gracefully', async () => {
        const largePayload = {
            "clientId": "test-client",
            "permissions": ["all"],
            "data": "a".repeat(11 * 1024 * 1024) // 11mb
        }
        await request(server)
            .post('/api/auth/generate-key')
            .send(largePayload)
            .expect(413);
    });

    it('should handle invalid JSON gracefully', async () => {
        await request(server)
            .post('/api/auth/generate-key')
            .set('Content-Type', 'application/json')
            .send('{ "clientId": "test", "permissions": ["all"], }')
            .expect(400);
    });
  });

  describe('Concurrent Requests', () => {
    it('should handle 50 concurrent requests', async () => {
        const promises = Array(50).fill(null).map(() => request(server).get('/api/health'));
        const responses = await Promise.all(promises);
        responses.forEach(response => {
            expect(response.status).toBe(200);
        });
    });
  });
});
