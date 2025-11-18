const advancedRateLimit = (req, res, next) => {
  next();
};

module.exports = { advancedRateLimit };
