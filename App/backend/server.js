const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const rateLimit = require('express-rate-limit');
const { body, validationResult } = require('express-validator');
const { PQCEngine } = require('./utils/pqc');
const { ABACEngine } = require('./utils/abac');
const { SSSEngine } = require('./utils/sss');
const { securityMiddleware } = require('./middleware/security');
const { advancedRateLimit } = require('./middleware/rateLimit');

const app = express();
const PORT = process.env.PORT || 3000;

// Security Middleware
app.use(helmet({
    contentSecurityPolicy: {
        directives: {
            defaultSrc: ["'self'"],
            scriptSrc: ["'self'", "'unsafe-inline'"],
            styleSrc: ["'self'", "'unsafe-inline'"],
            imgSrc: ["'self'", "data:", "https:"],
        },
    },
    hsts: {
        maxAge: 31536000,
        includeSubDomains: true,
        preload: true
    }
}));

app.use(cors({
    origin: process.env.FRONTEND_URL || 'http://localhost:8080',
    credentials: true
}));

app.use(express.json({ limit: '10mb' }));
app.use(express.urlencoded({ extended: true }));

// Post-Quantum Security Engine
const pqcEngine = new PQCEngine();
const abacEngine = new ABACEngine();
const sssEngine = new SSSEngine();

// Advanced Rate Limiting
app.use('/api/', advancedRateLimit);

// Security Validation Middleware
app.use(securityMiddleware);

// Routes
const authRoutes = require('./routes/auth');
const metricsRoutes = require('./routes/metrics');
const logsRoutes = require('./routes/logs');

app.use('/api/auth', authRoutes);
app.use('/api/metrics', metricsRoutes);
app.use('/api/logs', logsRoutes);

// Health Check with PQC Status
app.get('/api/health', async (req, res) => {
    try {
        const pqcStatus = await pqcEngine.getStatus();
        const abacStatus = abacEngine.getPolicyStatus();
        
        res.json({
            status: 'healthy',
            timestamp: new Date().toISOString(),
            pqc: pqcStatus,
            abac: abacStatus,
            version: '2.0.0'
        });
    } catch (error) {
        res.status(500).json({
            status: 'error',
            message: 'Health check failed',
            error: error.message
        });
    }
});

// Global Error Handler
app.use((err, req, res, next) => {
    console.error('Global error:', err);
    
    // Zero-knowledge error responses
    res.status(err.status || 500).json({
        success: false,
        error: 'Internal security error',
        code: err.code || 'INTERNAL_ERROR',
        timestamp: new Date().toISOString()
    });
});

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('SIGTERM received, shutting down gracefully');
    server.close(() => {
        console.log('Process terminated');
    });
});

const server = app.listen(PORT, () => {
    console.log(`🔐 φ-RING Security Server v2.0 running on port ${PORT}`);
    console.log(`🛡️  PQC Status: ${pqcEngine.isReady() ? 'READY' : 'INITIALIZING'}`);
    console.log(`📊 ABAC Status: ${abacEngine.isReady() ? 'READY' : 'INITIALIZING'}`);
});

module.exports = app;
