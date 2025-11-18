/**
 * Interactive Command Deck — Resource Monitoring Frontend
 * Synapse Engine: Controls hyper-responsive animations and logic.
 */

// --- Global State and Initialization ---
const appSettings = {
    autoRefreshInterval: 5000,
    lowLatencyMode: false,
    historicalWindow: '24h',
    egressAlertThreshold: 80,
    fnCallLimit: 100000,
    sssShareCount: 3
};

function loadSettings() {
    const saved = localStorage.getItem('commandDeckSettings');
    if (saved) Object.assign(appSettings, JSON.parse(saved));
}

let autoRefreshTimer = null;
let logs = JSON.parse(localStorage.getItem('commandDeckLogs') || '[]');
const SUPABASE_ANON_KEY = 'YOUR_SUPABASE_ANON_KEY'; // Placeholder

// Mock data for standalone testing (replace with real fetch in production)
const mockData = {
    supabase: {
        dbSize: '330 MB / 500 MB',
        dbPercent: 66,
        egress: '2.1 GB / 25 GB',
        egressPercent: 8.4,
        fnCalls: '15,102',
        fnLatency: '38ms'
    },
    cloudflare: { usage: '68%' },
    github: { minutes: '1,840 / 2,000' },
    gas: { calls: '8,337' }
};

// --- CORE SYNAPSE FIRE & ANIMATION LOGIC ---

/**
 * Fires the ripple effect (Synapse Fire) on button press.
 */
function fireSynapse(event, button, isHeavyLift = true) {
    const ripple = document.createElement('span');
    ripple.className = 'ripple-effect';
    
    // Choose color based on lift weight
    const color = isHeavyLift ? 'rgba(0, 245, 255, 0.8)' : 'rgba(0, 255, 65, 0.8)'; 
    ripple.style.background = color;
    
    // Calculate size and position for the ripple
    const size = Math.max(button.clientWidth, button.clientHeight);
    const rect = button.getBoundingClientRect();
    
    // Position the ripple relative to the click point
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    ripple.style.width = ripple.style.height = size * 2 + 'px';
    ripple.style.left = x - size + 'px';
    ripple.style.top = y - size + 'px';
    
    button.appendChild(ripple);

    // Trigger the scale animation
    ripple.style.transition = 'transform 0.5s ease-out, opacity 0.5s';
    // Use a slight delay to ensure the element is attached
    setTimeout(() => {
        ripple.style.transform = 'scale(1)';
        ripple.style.opacity = '0';
    }, 10);

    // Cleanup after animation
    setTimeout(() => ripple.remove(), 600);
}

/**
 * Updates the System Info Panel with text warping effect on interval change.
 */
function updateSystemInfo(intervalChange = false) {
    const timeElement = document.getElementById('lastSyncTime');
    const intervalElement = document.getElementById('refreshIntervalDisplay');
    
    timeElement.textContent = new Date().toLocaleTimeString('en-GB');

    if (intervalChange) {
        const newText = appSettings.autoRefreshInterval === 0 ? 'OFF' : (appSettings.autoRefreshInterval / 1000) + 's';
        
        // Warp out animation
        intervalElement.style.transition = 'none';
        intervalElement.style.opacity = '0';
        intervalElement.style.transform = 'skewX(20deg) scale(1.1)';

        // Warp in animation after brief delay
        setTimeout(() => {
            intervalElement.textContent = newText;
            intervalElement.style.transition = 'opacity 0.3s, transform 0.3s';
            intervalElement.style.opacity = '1';
            intervalElement.style.transform = 'skewX(0deg) scale(1)';
        }, 300);
    } else {
        // Ensure default state if no warp
        intervalElement.textContent = appSettings.autoRefreshInterval === 0 ? 'OFF' : (appSettings.autoRefreshInterval / 1000) + 's';
        intervalElement.style.transition = 'none';
        intervalElement.style.transform = 'none';
        intervalElement.style.opacity = '1';
    }
}

// --- API & UI LOGIC (Updated to use event object for fireSynapse) ---

function updateUI(data = mockData) {
    document.getElementById('supabaseSummary').textContent = `${data.supabase.dbSize} • ${data.supabase.egress}`;
    document.getElementById('dbSize').textContent = data.supabase.dbSize;
    document.getElementById('dbBar').style.width = data.supabase.dbPercent + '%';
    document.getElementById('egress').textContent = data.supabase.egress;
    document.getElementById('egressBar').style.width = data.supabase.egressPercent + '%';
    document.getElementById('fnCalls').textContent = data.supabase.fnCalls;
    document.getElementById('fnLatency').textContent = data.supabase.fnLatency;

    const egressBar = document.getElementById('egressBar');
    egressBar.classList.remove('warning', 'critical');
    if (data.supabase.egressPercent >= appSettings.egressAlertThreshold) {
        egressBar.classList.add('warning');
        if (data.supabase.egressPercent >= 95) egressBar.classList.add('critical');
    }

    document.getElementById('cloudflareSummary').textContent = `Workers Usage: ${data.cloudflare.usage}`;
    document.getElementById('githubSummary').textContent = `Actions Minutes: ${data.github.minutes}`;
    document.getElementById('gasSummary').textContent = `Executions: ${data.gas.calls}`;
}

async function fetchLatestMetrics() {
    try {
        updateUI(); 
        document.getElementById('globalStatusBanner').textContent = '✅ ONLINE';
        document.getElementById('globalStatusBanner').className = 'status-banner status-online';
    } catch (err) {
        document.getElementById('globalStatusBanner').textContent = '⚠️ SYNC ERROR - FALLING BACK TO OFFLINE MODE';
        document.getElementById('globalStatusBanner').className = 'status-banner status-critical';
    }
    updateSystemInfo(); 
}

async function refreshDashboard() {
    const url = 'YOUR_SUPABASE_EDGE_FUNCTION_URL/metrics/refresh'; 
    try {
        document.getElementById('globalStatusBanner').textContent = '🛰️ QUANTUM SYNC INITIATED...';
        
        await new Promise(resolve => setTimeout(resolve, 800)); 
        const data = mockData; 
        
        updateUI(data);
        document.getElementById('globalStatusBanner').textContent = '✅ QUANTUM SYNC COMPLETE';
        document.getElementById('globalStatusBanner').className = 'status-banner status-online';
    } catch (error) {
        document.getElementById('globalStatusBanner').textContent = '🚨 CRITICAL OFFLINE MODE: API FAILED';
        document.getElementById('globalStatusBanner').className = 'status-banner status-critical';
        console.error("Critical API Failure:", error);
    }
    updateSystemInfo();
}

function startAutoRefresh() {
    if (autoRefreshTimer) clearInterval(autoRefreshTimer);
    let interval = appSettings.autoRefreshInterval;
    
    if (appSettings.lowLatencyMode) {
        interval = Math.max(interval, 30000); 
    }
    
    if (interval > 0) {
        autoRefreshTimer = setInterval(fetchLatestMetrics, interval);
    }
}

function drawSparklines() {
    const draw = (id, color) => {
        const canvas = document.getElementById(id);
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        const W = canvas.width;
        const H = canvas.height;
        ctx.clearRect(0, 0, W, H);
        
        ctx.strokeStyle = color;
        ctx.lineWidth = 1;
        
        const data = Array.from({length: 20}, () => Math.random() * (H * 0.7) + (H * 0.15));
        const spacing = W / (data.length - 1);

        ctx.beginPath();
        ctx.moveTo(0, data[0]);
        data.forEach((p, i) => ctx.lineTo(i * spacing, p));
        ctx.stroke();
    };
    
    draw('dbSizeChart', '#00ff41');
    draw('egressChart', '#00f5ff');
}

function syncLogToBackend(logEntry) {
    const url = 'YOUR_SUPABASE_EDGE_FUNCTION_URL/logs/share';
    try {
        // Mock success
        new Promise(resolve => setTimeout(resolve, 100));
        logEntry.synced = true; 
        console.log(`Log synced and SSS shared (k=${appSettings.sssShareCount})`);
    } catch (error) {
        console.warn('Network error during log sync. Will retry later.');
    }
}

// --- LOG SYSTEM & SETTINGS LOGIC ---

function saveLog() {
    const input = document.getElementById('logInput');
    const text = input.value.trim();
    if (!text) return;

    const entry = {
        timestamp: new Date().toISOString(),
        text,
        synced: false 
    };

    logs.unshift(entry);
    localStorage.setItem('commandDeckLogs', JSON.stringify(logs));
    
    // Visual shatter simulation
    input.style.opacity = 0;
    input.placeholder = 'LOG SHATTERED & SECURED';
    
    setTimeout(() => {
        input.value = ''; 
        input.style.opacity = 1;
        input.placeholder = 'Enter note or log entry...';
        document.getElementById('globalStatusBanner').textContent = '✅ LOG SHATTERED & SECURED';
        document.getElementById('globalStatusBanner').className = 'status-banner status-online';
    }, 600);

    syncLogToBackend(entry); 
    renderLogList();
}

function renderLogList() {
    const list = document.getElementById('logList');
    list.innerHTML = '';
    logs.slice(0, 50).forEach(entry => {
        const div = document.createElement('div');
        const status = entry.synced ? ' (SECURE)' : ' (PENDING)';
        div.textContent = `${new Date(entry.timestamp).toLocaleTimeString()} ${status} — ${entry.text}`;
        div.style.padding = '6px 0';
        div.style.color = entry.synced ? 'var(--text)' : 'var(--warning)';
        div.style.borderBottom = '1px dashed rgba(0,245,255,0.1)';
        list.appendChild(div);
    });
}

function toggleLogHistory() {
    const list = document.getElementById('logList');
    const isHidden = list.classList.contains('hidden');
    
    list.classList.toggle('hidden');
    
    if (!isHidden) {
        list.style.transform = 'translateY(0%)';
        renderLogList();
    } else {
        list.style.transform = 'translateY(100%)';
    }
}

function forceLogResync() {
    document.getElementById('globalStatusBanner').textContent = '🔄 FORCE RE-SYNC: RETRYING PENDING LOGS...';
    document.getElementById('globalStatusBanner').className = 'status-banner status-warning';
    
    logs.forEach(entry => {
        if (!entry.synced) syncLogToBackend(entry);
    });

    localStorage.setItem('commandDeckLogs', JSON.stringify(logs));
    setTimeout(() => fetchLatestMetrics(), 1000); 
}

function toggleSettingsPanel() {
    const panel = document.getElementById('settingsPanel');
    panel.classList.toggle('visible');

    if (panel.classList.contains('visible')) {
        document.getElementById('refreshIntervalSlider').value = appSettings.autoRefreshInterval / 1000;
        document.getElementById('refreshIntervalInput').value = appSettings.autoRefreshInterval / 1000;
        document.getElementById('lowLatencyToggle').checked = appSettings.lowLatencyMode;
        document.getElementById('historicalWindow').value = appSettings.historicalWindow;
        document.getElementById('egressWarningInput').value = appSettings.egressAlertThreshold;
        document.getElementById('fnLimitInput').value = appSettings.fnCallLimit;
        document.getElementById('sssShareSlider').value = appSettings.sssShareCount;
        document.getElementById('sssShareValue').textContent = appSettings.sssShareCount;
    }
}

function saveSettings() {
    const oldInterval = appSettings.autoRefreshInterval;

    appSettings.autoRefreshInterval = parseInt(document.getElementById('refreshIntervalInput').value) * 1000;
    appSettings.lowLatencyMode = document.getElementById('lowLatencyToggle').checked;
    appSettings.historicalWindow = document.getElementById('historicalWindow').value;
    appSettings.egressAlertThreshold = parseInt(document.getElementById('egressWarningInput').value);
    appSettings.fnCallLimit = parseInt(document.getElementById('fnLimitInput').value);
    appSettings.sssShareCount = parseInt(document.getElementById('sssShareSlider').value);

    localStorage.setItem('commandDeckSettings', JSON.stringify(appSettings));

    startAutoRefresh();
    toggleSettingsPanel();
    
    if (oldInterval !== appSettings.autoRefreshInterval) {
        updateSystemInfo(true); 
    }
}

// --- Event Bindings ---
document.getElementById('refreshButton').addEventListener('click', function(event) {
    fireSynapse(event, this, true);
    refreshDashboard();
});

document.getElementById('saveLogBtn').addEventListener('click', function(event) {
    fireSynapse(event, this, false);
    saveLog();
});

document.getElementById('viewHistoryBtn').addEventListener('click', function(event) {
    fireSynapse(event, this, false);
    toggleLogHistory();
});

document.getElementById('forceResyncBtn').addEventListener('click', function(event) {
    fireSynapse(event, this, false);
    forceLogResync();
});

document.getElementById('applyBtn').addEventListener('click', function(event) {
    fireSynapse(event, this, true);
    saveSettings();
});

document.getElementById('cancelBtn').addEventListener('click', function(event) {
    fireSynapse(event, this, false);
    toggleSettingsPanel();
});


document.getElementById('refreshIntervalSlider').addEventListener('input', e => {
    document.getElementById('refreshIntervalInput').value = e.target.value;
});
document.getElementById('refreshIntervalInput').addEventListener('change', e => {
    document.getElementById('refreshIntervalSlider').value = e.target.value;
});
document.getElementById('sssShareSlider').addEventListener('input', e => {
    document.getElementById('sssShareValue').textContent = e.target.value;
});

// --- Initializer Function ---
window.onload = () => {
    loadSettings();
    startAutoRefresh();
    refreshDashboard(); 
};

// --- Low-Energy Particle Field Implementation ---
const particleField = document.getElementById('particleField');
function createParticle() {
    const p = document.createElement('div');
    p.style.cssText = `
        position: absolute;
        width: 2px;
        height: 2px;
        background: var(--accent);
        border-radius: 50%;
        left: ${Math.random() * 100}vw;
        animation: rise ${8 + Math.random() * 8}s linear forwards;
        opacity: ${Math.random() * 0.6};
    `;
    particleField.appendChild(p);
    setTimeout(() => p.remove(), 16000);
}

let lastTime = 0;
const particleInterval = 300; 
function particleLoop(timestamp) {
    if (timestamp - lastTime > particleInterval) {
        createParticle();
        lastTime = timestamp;
    }
    requestAnimationFrame(particleLoop);
}
requestAnimationFrame(particleLoop);
