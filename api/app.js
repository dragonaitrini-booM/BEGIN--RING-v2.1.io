async function fetchDrive() {
    try {
        const res = await fetch('http://localhost:8080/api/drive')
        const text = await res.text()
        
        // Check if the response is the expected usage string
        if (text.includes('GB')) {
            const [usedGB, limitGB] = text.split(' / ').map(s => parseFloat(s));
            const pct = Math.round(usedGB / limitGB * 100);
            
            // Update the display text and the holo-bar's width
            $('#driveBar .value').textContent = text
            $('#driveBar').dataset.usage = pct
        } else {
            // Display error or status (NO_KEY, API_ERROR, OFFLINE)
            $('#driveBar .value').textContent = text
        }
    } catch {
        $('#driveBar .value').textContent = 'OFFLINE'
    }
}async function fetchDrive() {
    try {
        const res = await fetch('http://localhost:8080/api/drive')
        const text = await res.text()
        
        // Check if the response is the expected usage string
        if (text.includes('GB')) {
            const [usedGB, limitGB] = text.split(' / ').map(s => parseFloat(s));
            const pct = Math.round(usedGB / limitGB * 100);
            
            // Update the display text and the holo-bar's width
            $('#driveBar .value').textContent = text
            $('#driveBar').dataset.usage = pct
        } else {
            // Display error or status (NO_KEY, API_ERROR, OFFLINE)
            $('#driveBar .value').textContent = text
        }
    } catch {
        $('#driveBar .value').textContent = 'OFFLINE'
    }
}
