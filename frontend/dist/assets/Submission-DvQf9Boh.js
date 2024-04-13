import{_ as u,o as a,c as d,a as l,d as f}from"./index-C9FwR70A.js";const m={data(){return{isDragging:!1,hasDropped:!1,file:[]}},methods:{async onChange(){this.file.push(...this.$refs.file.files),this.hasDropped=!0,this.submission_content=await h(this.file[0]),this.$emit("file-dropped",this.submission_content)},dragover(r){r.preventDefault(),this.isDragging=!0},dragleave(){this.isDragging=!1},drop(r){r.preventDefault(),this.$refs.file.files=r.dataTransfer.files,this.onChange(),this.isDragging=!1,this.hasDropped=!0}}};function h(r){return new Promise(async(e,i)=>{var s=new FileReader;s.onload=()=>{e(s.result)},s.onerror=i,await s.readAsText(r)})}const g={class:"main"},_={for:"fileInput",class:"file-label"},w={key:0},y={key:1},b={key:2};function C(r,e,i,s,t,o){return a(),d("div",g,[l("div",{class:"dropzone-container",onDragover:e[1]||(e[1]=(...n)=>o.dragover&&o.dragover(...n)),onDragleave:e[2]||(e[2]=(...n)=>o.dragleave&&o.dragleave(...n)),onDrop:e[3]||(e[3]=(...n)=>o.drop&&o.drop(...n))},[l("input",{type:"file",multiple:"",name:"file",id:"fileInput",class:"hidden-input",onChange:e[0]||(e[0]=(...n)=>o.onChange&&o.onChange(...n)),ref:"file",accept:".txt"},null,544),l("label",_,[t.isDragging?(a(),d("div",w,"Release to drop submission file here!")):t.hasDropped?(a(),d("div",y,"Processing...")):(a(),d("div",b,"Drop submission text (txt) file here!"))])],32)])}const v=u(m,[["render",C],["__scopeId","data-v-63de05d5"]]),c=new Worker(new URL("https://shayandoust.github.io/PowersortCompetitionWebsite/assets/webworker-Zce6vEhX.js",import.meta.url)),p={};c.onmessage=r=>{const{id:e,...i}=r.data,s=p[e];delete p[e],s(i)};const k=(()=>{let r=0;return(e,i)=>(r=(r+1)%Number.MAX_SAFE_INTEGER,new Promise(s=>{p[r]=s,c.postMessage({...i,python:e,id:r})}))})(),D={class:"submission"},P=`
from pyodide.ffi import to_js
from pyodide.http import pyfetch

response = await pyfetch("https://corsproxy.io/?https%3A%2F%2Fgithub.com%2Fshayandoust%2FPowersortCompetitionWebsite%2Fraw%2Fmain%2Ffrontend%2Fpy_assets%2Fpowersort_timsort.tar.gz") # .zip, .whl, ...
await response.unpack_archive() # by default, unpacks to the current dir

print("Received and unpacked Python components successfully!")

import Powersort as Powersort
import Timsort as Timsort
import Counters as Counters

def cost(lst, sorter):
    wrapped = [Counters.ComparisonCounter(x) for x in lst]
    Counters.reset_counters()
    sorter.sort(wrapped)
    assert Counters.ComparisonCounter.EQ_COMPARISONS == 0

    return {
        'Algorithm': sorter.name(),
        'Comparisons': Counters.ComparisonCounter.COMPARISONS
    }

def compare_sorters(lst, sorters = [Powersort, Timsort]):
    sorters = sorted(sorters, key = lambda sorter: sorter.name())

    return [cost(lst, sorter) for sorter in sorters]

with open("./submission.txt", "r") as fh:
    data = fh.read()

print(compare_sorters(data))
`,x={__name:"Submission",setup(r){const e=async s=>{console.log("File dropped! Processing..."),await i(s)};async function i(s){console.log("Pyodide web worker initialising.");try{const{result:t,error:o}=await k(P,[s]);t?console.log("Pyodide web worker returned: ",t):o&&console.log("Pyodide web worker failed and returned: ",o)}catch(t){console.log(`Error with Pyodide web worker at ${t.filename}, ${t.lineno}, ${t.message}`)}}return(s,t)=>(a(),d("div",D,[f(v,{onFileDropped:e})]))}};export{x as default};
