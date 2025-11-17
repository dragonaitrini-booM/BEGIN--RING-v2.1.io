const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { PQCEngine } = require('../backend/utils/pqc');

async function setupPhiRing() {
    console.log('🔐 Setting up φ-RING Security Dashboard v2.0...\n');

    // Create directories
    const dirs = [
        'logs',
        'keys',
        'policies',
        'data',
        'tests/reports'
    ];

    dirs.forEach(dir => {
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
            console.log(`✅ Created directory: ${dir}`);
        }
    });

    // Generate PQC keys
    console.log('\n⚛️  Generating Post-Quantum key pairs...');
    const pqcEngine = new PQCEngine();
    await pqcEngine.initialize();
    
    const keyPair = await pqcEngine.generateKeyPair();
    fs.writeFileSync('./keys/pqc-public.key', keyPair.publicKey);
    fs.writeFileSync('./keys/pqc-private.key', keyPair.privateKey);
    console.log('✅ PQC keys generated and saved');

    // Create default ABAC policy
    console.log('\n🛡️  Creating default ABAC policy...');
    const defaultPolicy = {
        version: "1.0",
        policies: [
            {
                id: "default-access",
                description: "Default access policy for authenticated users",
                rules: [
                    {
                        effect: "PERMIT",
                        conditions: {
                            behavioralScore: { $gte: 0.3 },
                            deviceIntegrity: "PASS",
                            timeOfDay: { $gte: 6, $lte: 22 }
                        }
                    }
                ]
            },
            {
                id: "admin-access",
                description: "Admin access policy",
                rules: [
                    {
                        effect: "PERMIT",
                        conditions: {
                            role: "admin",
                            behavioralScore: { $gte: 0.5 },
                            deviceIntegrity: "PASS"
                        }
                    }
                ]
            }
        ]
    };

    fs.writeFileSync('./policies/security-policy.json', JSON.stringify(defaultPolicy, null, 2));
    console.log('✅ Default ABAC policy created');

    // Create environment file
    console.log('\n📋 Creating environment configuration...');
    if (!fs.existsSync('.env')) {
        fs.copyFileSync('.env.example', '.env');
        console.log('✅ .env file created from template');
        console.log('⚠️  Please update .env with your actual configuration');
    }

    // Create initial log file
    console.log('\n📝 Initializing secure logging...');
    const initialLog = {
        timestamp: new Date().toISOString(),
        event: 'SYSTEM_INITIALIZED',
        version: '2.0.0',
        securityLevel: 'HIGH'
    };
    
    fs.writeFileSync('./logs/phi-ring.log', JSON.stringify(initialLog) + '\n');
    console.log('✅ Secure logging initialized');

    console.log('\n🎉 φ-RING Security Dashboard setup complete!');
    console.log('\nNext steps:');
    console.log('1. Update .env with your configuration');
    console.log('2. Run: npm install');
    console.log('3. Run: npm test');
    console.log('4. Run: npm start');
    console.log('\nFrontend: Open frontend/index.html in your browser');
}

setupPhiRing().catch(console.error);
