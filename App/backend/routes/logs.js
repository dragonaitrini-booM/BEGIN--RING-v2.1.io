const express = require('express');
const router = express.Router();

router.post('/secure', (req, res) => {
  res.json({
    success: true,
    data: {
      sssShare: 'mockSssShare',
      integrityHash: 'mockIntegrityHash'
    }
  });
});

module.exports = router;
