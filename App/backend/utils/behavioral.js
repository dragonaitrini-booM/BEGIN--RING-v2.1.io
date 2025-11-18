class BehavioralAnalytics {
  async analyzeRequest(req) {
    if (req.body.clientId === 'suspicious-client') {
      return 0.2;
    }
    return 1;
  }

  async analyzeAuthentication(req) {
    if (req.body.deviceFingerprint === 'suspicious-device') {
      return 0.1;
    }
    return 1;
  }
}

module.exports = { BehavioralAnalytics };
