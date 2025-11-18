const express = require('express');
const router = express.Router();

router.get('/current', (req, res) => {
  res.json({
    success: true,
    data: {
      supabase: {},
      drive: {},
      gas: {},
      github: {}
    }
  });
});

module.exports = router;
