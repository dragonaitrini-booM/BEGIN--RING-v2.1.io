// Galois Field GF(2^8) operations for Reed-Solomon
pub struct GF256 {
    log_table: [u8; 256],
    exp_table: [u8; 256],
}

impl GF256 {
    pub fn new() -> Self {
        let mut log_table = [0u8; 256];
        let mut exp_table = [0u8; 256];
        
        // Generate logarithm and exponential tables
        let mut x = 1u16;
        for i in 0..255 {
            exp_table[i] = x as u8;
            log_table[x as usize] = i as u8;
            x = (x * 2) ^ if x >= 256 { 0x11d } else { 0 };
        }
        
        Self { log_table, exp_table }
    }

    pub fn multiply(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 { return 0; }
        let log_sum = (self.log_table[a as usize] as u16 + self.log_table[b as usize] as u16) % 255;
        self.exp_table[log_sum as usize]
    }
}
