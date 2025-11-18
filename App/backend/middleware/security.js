const securityMiddleware = (req, res, next) => {
  next();
};

module.exports = { securityMiddleware };
