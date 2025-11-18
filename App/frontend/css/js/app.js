// ═══════════════════════════════════════════════════════════
// φ-RING MOBILE DATA - 3 ROTATING STATES
// ═══════════════════════════════════════════════════════════
const ROTATING_DATA = [
    {
        status: 'OK',
        message: '✅ ALL SYSTEMS OPERATIONAL',
        metrics: [
            { metric: 'Supabase Storage', current: '45 MB', limit: '500 MB', percent: 9, status: '🟢 OK' },
            { metric: 'Drive Storage', current: '18 GB', limit: '105 GB', percent: 17, status: '🟢 OK' },
            { metric: 'GAS Daily Calls', current: '234', limit: '20,000', percent: 1, status: '🟢 OK' },
            { metric: 'GitHub Actions', current: '12 min', limit: '2,000 min', percent: 0.6, status: '🟢 OK' }
        ]
    },
    {
        status: 'WARNING',
        message: '⚠️ WARNING: 2 quota(s) approaching limit',
        metrics: [
            { metric: 'Supabase Storage', current: '325 MB', limit: '500 MB', percent: 65, status: '🟡 WARNING: Plan archival' },
            { metric: 'Drive Storage', current: '73 GB', limit: '105 GB', percent: 70, status: '🟡 WARNING: Monitor closely' },
            { metric: 'GAS Daily Calls', current: '1,240', limit: '20,000', percent: 6, status: '🟢 OK' },
            { metric: 'GitHub Actions', current: '89 min', limit: '2,000 min', percent: 4, status: '🟢 OK' }
        ]
    },
    {
        status: 'ERROR',
        message: '🚨 CRITICAL: 1 quota(s) exceeded',
        metrics: [
            { metric: 'Supabase Storage', current: '425 MB', limit: '500 MB', percent: 85, status: '🔴 CRITICAL: Archive now!' },
            { metric: 'Drive Storage', current: '87 GB', limit: '105 GB', percent: 83, status: '🔴 CRITICAL: Add account!' },
            { metric: 'GAS Daily Calls', current: '16,500', limit: '20,000', percent: 82, status: '🟡 WARNING: High usage' },
            { metric: 'GitHub Actions', current: '234 min', limit: '2,000 min', percent: 12, status: '🟢 OK' }
        ]
    }
];

let dataIndex = 0;
let autoRefreshInterval = null;
let logs = JSON.parse(localStorage.getItem('phiRingLogs')) || [];

// ═══════════════════════════════════════════════════════════
// DASHBOARD FUNCTIONS (The core logic for your buttons)
// ═══════════════════════════════════════════════════════════
function refreshDashboard() {
    const data = ROTATING_DATA[dataIndex];
    updateStatusBanner(data);
    renderMetrics(data);
    updateSystemInfo();
    dataIndex = (dataIndex + 1) % ROTATING_DATA.length;
}

function updateStatusBanner(data) {
    const banner = document.getElementById('statusBanner');
    banner.textContent = data.message;
    banner.className = 'status-banner ' + 
        (data.status === 'ERROR' ? 'error' : 
         data.status === 'WARNING' ? 'warning' : '');
}

function renderMetrics(data) {
    const grid = document.getElementById('metricsGrid');
    grid.innerHTML = data.metrics.map(m => {
        const statusClass = getStatusClass(m.status);
        const barClass = m.percent > 80 ? 'critical' : m.percent > 60 ? 'warning' : '';
        return `
            <div class="metric-card ${statusClass}">
                <h3><span class="icon">${getMetricIcon(m.metric)}</span>${m.metric}</h3>
                <div class="metric-label">Current Usage</div>
                <div class="metric-value">${m.current} / ${m.limit}</div>
                <div class="metric-bar">
                    <div class="metric-bar-fill ${barClass}" style="width: ${m.percent}%"></div>
                </div>
                <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 10px;">
                    <span style="color: #888; font-size: 0.85em;">${m.percent.toFixed(1)}% used</span>
                    <span class="metric-status ${statusClass}">${getStatusText(m.status)}</span>
                </div>
            </div>`;
    }).join('');
}

function getStatusClass(status) {
    if (status.includes('🔴') || status.includes('CRITICAL')) return 'critical';
    if (status.includes('🟡') || status.includes('WARNING')) return 'warning';
    return 'ok';
}

function getStatusText(status) {
    if (status.includes('🔴') || status.includes('CRITICAL')) return '🔴 Critical';
    if (status.includes('🟡') || status.includes('WARNING')) return '🟡 Warning';
    return '🟢 OK';
}

function getMetricIcon(metricName) {
    if (metricName.includes('Supabase')) return '💾';
    if (metricName.includes('Drive')) return '☁️';
    if (metricName.includes('GAS')) return '⚡';
    if (metricName.includes('GitHub')) return '🔧';
    return '📊';
}

function updateSystemInfo() {
    const states = ['1 of 3', '2 of 3', '3 of 3'];
    document.getElementById('currentState').textContent = states[dataIndex];
    document.getElementById('lastUpdated').textContent = new Date().toLocaleTimeString();
    document.getElementById('logCount').textContent = logs.length;
}

function resetDemo() {
    dataIndex = 0;
    refreshDashboard();
    alert('📱 Demo reset to State 1 (ALL OK)');
}

function toggleAutoRefresh() {
    if (autoRefreshInterval) {
        clearInterval(autoRefreshInterval);
        autoRefreshInterval = null;
        document.getElementById('autoRefresh').textContent = 'OFF';
        alert('⏸️ Auto-refresh disabled');
    } else {
        autoRefreshInterval = setInterval(refreshDashboard, 5000);
        document.getElementById('autoRefresh').textContent = 'ON (5s)';
        alert('▶️ Auto-refresh enabled (5 seconds)');
    }
}

// ═══════════════════════════════════════════════════════════
// OFFLINE LOGGING SYSTEM (For your 'Log' button)
// ═══════════════════════════════════════════════════════════
function openLogModal() {
    document.getElementById('logModal').style.display = 'block';
    renderLogList();
}

function closeLogModal() {
    document.getElementById('logModal').style.display = 'none';
    document.getElementById('logInput').value = '';
}

function saveLog() {
    const input = document.getElementById('logInput');
    const text = input.value.trim();
    if (!text) return alert('⚠️ Cannot save empty log.');

    const entry = {
        id: Date.now(),
        timestamp: new Date().toLocaleString('en-TT', { 
            timeZone: 'America/Port_of_Spain',
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit'
        }),
        text: text
    };

    logs.unshift(entry);
    localStorage.setItem('phiRingLogs', JSON.stringify(logs));
    alert('✅ Log saved to device storage (SSS share simulated)');

    input.value = '';
    renderLogList();
    updateSystemInfo();
}

function renderLogList() {
    const list = document.getElementById('logList');
    if (logs.length === 0) {
        list.innerHTML = '<p style="color:#888; text-align:center; padding:20px;">No logs yet. Start logging.</p>';
        return;
    }

    list.innerHTML = logs.map(log => `
        <div class="log-entry">
            <div class="log-timestamp">${escapeHtml(log.timestamp)} AST</div>
            <div class="log-text">${escapeHtml(log.text)}</div>
            <button class="log-delete" onclick="deleteLog(${log.id})">Delete</button>
        </div>
    `).join('');
}

function deleteLog(id) {
    if (!confirm('Delete this log entry?')) return;
    logs = logs.filter(l => l.id !== id);
    localStorage.setItem('phiRingLogs', JSON.stringify(logs));
    renderLogList();
    updateSystemInfo();
}

function clearAllLogs() {
    if (!confirm('⚠️ DELETE ALL LOGS? This cannot be undone.')) return;
    logs = [];
    localStorage.setItem('phiRingLogs', JSON.stringify(logs));
    renderLogList();
    updateSystemInfo();
    alert('🗑️ All logs deleted');
}

function downloadLogs() {
    if (logs.length === 0) return alert('No logs to export.');

    const content = logs.map(l => `[${l.timestamp} AST]\n${l.text}\n\n`).join('');
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);

    const a = document.createElement('a');
    a.href = url;
    a.download = `phi-ring-log-${new Date().toISOString().slice(0,10)}.txt`;
    a.click();

    URL.revokeObjectURL(url);
    alert('📄 Log exported as .txt to Downloads');
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// ═══════════════════════════════════════════════════════════
// INITIALIZATION
// ═══════════════════════════════════════════════════════════
window.onload = function () {
    console.log('φ-RING Mobile Console v0.9.5 loaded');
    refreshDashboard();
    updateSystemInfo();
};

// Prevent double-tap zoom for mobile-friendly feeling
let lastTouchEnd = 0;
document.addEventListener('touchend', function (event) {
    const now = Date.now();
    if (now - lastTouchEnd <= 300) {
        event.preventDefault();
    }
    lastTouchEnd = now;
}, false);
