class PQCEngine {
  async initialize() {
    return;
  }

  async generateKeyPair() {
    return { publicKey: 'mockPublicKey', privateKey: 'mockPrivateKey' };
  }

  async encrypt(data) {
    return 'mockEncryptedData';
  }

  async verifySignature(apiKey, message, signature) {
    return true;
  }

  async getStatus() {
    return 'READY';
  }

  isReady() {
    return true;
  }
}

module.exports = { PQCEngine };
