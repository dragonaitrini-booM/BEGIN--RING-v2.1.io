// ┌───────────────────────────────────┐
// │     QUANTUMIX CORE JS LOGIC       │
// │   MOCK DATA. DO NOT SHIP THIS.    │
// └───────────────────────────────────┘

// ┌─────────────────┐
// │     UTILITY     │
// └─────────────────┘
const toast=({msg,dur=2600,icon='✓'})=>{
  const e=document.createElement('div');
  e.className='toast';
  e.style.setProperty('--dur',`${dur}ms`);
  e.innerHTML=`<div class=toast-icon>${icon}</div><div class=toast-body><div class=msg>${msg}</div><div class=bar></div></div>`;
  document.body.append(e);
  setTimeout(()=>e.remove(),dur);
};
const $ = s => document.querySelector(s)
const $$ = s => document.querySelectorAll(s)

// ┌────────────────────────┐
// │     STATE MGMT         │
// └────────────────────────┘
const defaults = {dark:1,lbp:0,ref:30,thr:80}
let tick = null

function load(){try{return JSON.parse(localStorage.getItem('qx'))||defaults}catch{return defaults}}
function save(o){localStorage.setItem('qx',JSON.stringify(o))}

// ┌────────────────────────┐
// │     UI RENDERING       │
// └────────────────────────┘
function apply(s){
  document.body.classList.toggle('dark',!!s.dark)
  clearInterval(tick)
  const base = s.lbp?90000:15000
  tick = setInterval(updateAll, Math.max(5000, s.ref*1000))
  window.CRIT = s.thr
  updateAll()
  toast({msg:'CONFIG LIVE ∞', icon:'⚡'})
}

function updateAll(){
  const crit = window.CRIT||80
  let totalUsage = 0, barCount = 0
  $$('.holo-bar').forEach(bar=>{
    const val = +bar.dataset.usage
    bar.style.setProperty('--val',val)
    bar.querySelector('.value').textContent = val+'%'
    bar.classList.toggle('critical',val>=crit)
    totalUsage += val
    barCount++
  })
  const avgUsage = totalUsage / barCount
  $('#orbVal').textContent = `${Math.round(avgUsage)}%`
  $('#orb').classList.toggle('critical', !!$('.holo-bar.critical'))
}

// ┌────────────────────────┐
// │     EVENT LISTENERS    │
// └────────────────────────┘
// Matrix panel
function toggleMatrix(){
  const d=$('#matrixDetails'), t=$('#matrixToggle')
  const open = d.classList.toggle('visible')
  t.textContent = open ? '🔼' : '🔽'
  if(open) fetchMatrixDetails()
}
async function fetchMatrixDetails(){
  // MOCK DATA: This is where a real fetch() would go for live data
  const mock = {ghMinutes:"1,971 / 2,000", ghFailures:3, cfR2:"1.1 GB / 10 GB", cfCPU:"6.8 s", cfKV:"92,104", cfBill:"2025-11-18"}
  $('#ghMinutes').textContent = mock.ghMinutes
  const f = $('#ghFailures'); f.textContent = mock.ghFailures; f.classList.toggle('alert-red',mock.ghFailures>5)
  $('#cfR2').textContent = mock.cfR2; $('#cfCPU').textContent = mock.cfCPU; $('#cfKV').textContent = mock.cfKV; $('#cfBill').textContent = mock.cfBill
  toast({msg:'Matrix Synced', icon:'📡'})
}

// Buttons
$('#save').onclick = _=>{
  const s = {
    dark: $('#dark').checked ? 1 : 0,
    lbp: $('#lbp').checked ? 1 : 0,
    ref: Math.max(5, Math.min(300, +$('#ref').value || 30)),
    thr: Math.max(50, Math.min(99, +$('#thr').value || 80))
  }
  save(s); apply(s)
}
$('#reset').onclick = _=>{
  confirm('Factory reset all settings?').then(confirmed=>{
    if (confirmed) {
      toast({msg:'Factory reset initiated', icon:'🗑️', dur:2000})
      setTimeout(_=>{save(defaults);location.reload()}, 2000)
    }
  })
}

// Modal
let res = null
function confirm(msg){
  $('#modalText').textContent = msg
  $('#modal').classList.add('visible')
  return new Promise(resolve => res = resolve)
}
$('#modalClose').onclick = _=>{ $('#modal').classList.remove('visible'); res(false) }
$('#modalCancel').onclick = _=>{ $('#modal').classList.remove('visible'); res(false) }
$('#modalConfirm').onclick = _=>{ $('#modal').classList.remove('visible'); res(true) }

// ┌───────────────────┐
// │     INIT          │
// └───────────────────┘
const init = load()
$('#dark').checked = !!init.dark
$('#lbp').checked = !!init.lbp
$('#ref').value = init.ref
$('#thr').value = init.thr
apply(init)

// ┌───────────────────┐
// │     MOCK DATA     │
// └───────────────────┘
// Fake live data (replace with real fetchMetrics() later)
setInterval(_=>{
  $$('.holo-bar').forEach(b=>{
    let v = +b.dataset.usage + (Math.random()-0.5)*5
    b.dataset.usage = Math.max(0, Math.min(99.9, v)).toFixed(1)
  })
  updateAll()
}, 10000)
