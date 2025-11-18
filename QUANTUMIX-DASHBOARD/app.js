const toast=(m,t=1300)=>{const e=document.createElement('div');e.className='toast';e.textContent=m;document.body.append(e);setTimeout(()=>e.remove(),t)}
const $ = s => document.querySelector(s)
const $$ = s => document.querySelectorAll(s)

const defaults = {dark:1,lbp:0,ref:30,thr:80}
let tick = null

function load(){try{return JSON.parse(localStorage.getItem('qx'))||defaults}catch{return defaults}}
function save(o){localStorage.setItem('qx',JSON.stringify(o))}

function apply(s){
  document.body.classList.toggle('dark',!!s.dark)
  clearInterval(tick)
  const base = s.lbp?90000:15000 // Low Bandwidth Protocol logic (currently uses faster default if unchecked)
  tick = setInterval(updateAll, Math.max(5000, s.ref*1000))
  window.CRIT = s.thr
  updateAll()
  toast('CONFIG LIVE ∞')
}

function updateAll(){
  const crit = window.CRIT||80
  $$('.holo-bar').forEach(bar=>{
    const val = +bar.dataset.usage
    bar.style.setProperty('--val',val)
    bar.querySelector('.value').textContent = val+'%'
    bar.classList.toggle('critical',val>=crit)
  })
  $('#orb').classList.toggle('critical', !!$('.holo-bar.critical'))
}

// Matrix panel
function toggleMatrix(){
  const d=$('#matrixDetails'), t=$('#matrixToggle')
  const open = d.classList.toggle('visible')
  t.textContent = open ? 'Up Arrow' : 'Down Arrow'
  if(open) fetchMatrixDetails()
}
async function fetchMatrixDetails(){
  toast('SYNCING MATRIX...')
  // MOCK DATA: This is where a real fetch() would go for live data
  const mock = {ghMinutes:"1,971 / 2,000", ghFailures:3, cfR2:"1.1 GB / 10 GB", cfCPU:"6.8 s", cfKV:"92,104", cfBill:"2025-11-18"}
  $('#ghMinutes').textContent = mock.ghMinutes
  const f = $('#ghFailures'); f.textContent = mock.ghFailures; f.classList.toggle('alert-red',mock.ghFailures>5)
  $('#cfR2').textContent = mock.cfR2; $('#cfCPU').textContent = mock.cfCPU; $('#cfKV').textContent = mock.cfKV; $('#cfBill').textContent = mock.cfBill
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
$('#reset').onclick = _=>{if(confirm('Factory reset everything?')){save(defaults);location.reload()}}

// Init
const init = load()
$('#dark').checked = !!init.dark
$('#lbp').checked = !!init.lbp
$('#ref').value = init.ref
$('#thr').value = init.thr
apply(init)

// Fake live data (replace with real fetchMetrics() later)
setInterval(_=>{
  $$('.holo-bar').forEach(b=>{
    let v = +b.dataset.usage + (Math.random()-0.5)*5
    b.dataset.usage = Math.max(0, Math.min(99.9, v)).toFixed(1)
  })
  updateAll()
}, 10000)
