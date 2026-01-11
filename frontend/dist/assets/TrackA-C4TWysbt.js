import{F as N,_ as z}from"./FileDropper-C6ZwNb_X.js";import{ax as E,x as _,ay as I,o as u,z as m,A as o,h as d,b as t,F as O,p as s,w as v,l as P,az as y,q as w,ab as R}from"./index-CZ5--WV8.js";import{g as W,a as j,b as A,p as h}from"./misc-C-tqdJMj.js";import{_ as B}from"./BAlert.vue_vue_type_script_setup_true_lang-Bq9xl30a-DQFECoNi.js";import"./BCloseButton.vue_vue_type_script_setup_true_lang-byZmQ_ot-DpvVQpY4.js";const M=new Worker(new URL("https://powersort-competition.github.io/PowersortCompetitionWebsite/assets/webworker-D48hWBOz.js",import.meta.url)),k={};M.onmessage=l=>{const{id:a,...p}=l.data,r=k[a];delete k[a],r(p)};const $=(()=>{let l=0;return(a,p)=>(l=(l+1)%Number.MAX_SAFE_INTEGER,new Promise(r=>{k[l]=r,M.postMessage({...p,python:a,id:l})}))})(),V={class:"container mt-4 main"},Y={key:0},L={key:0},q={key:1},G={key:2},U={class:"alert-heading"},Q={class:"mb-0"},X=`
from pyodide.ffi import to_js
from pyodide.http import pyfetch

response = await pyfetch("https://raw.githubusercontent.com/Powersort-Competition/PowersortCompetitionWebsite/main/frontend/py_assets/powersort_timsort.tar.gz") # .zip, .whl, ...
await response.unpack_archive() # by default, unpacks to the current dir

print("Received and unpacked Python components successfully!")

import Powersort as Powersort
import Timsort as Timsort
import Counters as Counters

import json

def cost(lst, sorter):
    lst = json.loads(lst)

    wrapped = [Counters.ComparisonCounter(x) for x in lst]

    Counters.reset_counters()
    sorter.sort(wrapped)
    assert Counters.ComparisonCounter.EQ_COMPARISONS == 0

    return {
        'Algorithm': sorter.name(),
        'Comparisons': Counters.ComparisonCounter.COMPARISONS,
        'MergeCost': Counters.MergeCosts.MERGECOST
    }

def compare_sorters(lst, sorters = [Powersort, Timsort]):
    sorters = sorted(sorters, key = lambda sorter: sorter.name())

    return [cost(lst, sorter) for sorter in sorters]

with open("./submission.txt", "r") as fh:
    data = fh.read()

comps = to_js(compare_sorters(data))
comps
`,oe={__name:"TrackA",setup(l){$cookies.get("pscomp_oauth")==null&&(console.log("Not logged in... routing to login page"),E.push({name:"login"}));let a=!1,p=!1,r,f,g,c;const C=_(!0),x=W();let F=j(x);const T=async()=>{C.value=!1,await R(),C.value=!0},S=async n=>{console.log("File dropped! Processing with length: ",n.length),_(null);var e=new FormData;if(e.append("file",n),A(n)<=5e6){let i=await D(n);r=i.results[0].get("Comparisons"),f=i.results[1].get("Comparisons"),g=i.results[0].get("MergeCost"),c=i.results[1].get("MergeCost")}else console.log("File too big for Pyodide, sending to server."),a=!0,y.post("/serverside_comp",e,{headers:{"user-id":"test","Access-Control-Allow-Origin":"*"}});let b={user_id:await F,powersort_comp:r,timsort_comp:f,mcost_diff:h(c,g),comp_diff:h(f,r),powersort_merge_cost:g,timsort_merge_cost:c,submission_size:A(n)};y.post("/new_submission_track_a",b,{headers:{"content-type":"application/json","Access-Control-Allow-Origin":"*"}}).then(i=>{y.post("/submission_input_save",e,{headers:{"file-name":i.data,track:"A","Access-Control-Allow-Origin":"*"}})}),p=!0,await T()};async function D(n){console.log("Pyodide web worker initialising.");try{let e=await $(X,[n]);return e?console.log("Pyodide web worker returned: ",e):console.log("Pyodide web worker failed and returned: ",error),e}catch(e){console.log(`Error with Pyodide web worker at ${e.filename}, ${e.lineno}, ${e.message}`)}}return(n,e)=>{const b=I("BLink"),i=z;return u(),m("div",V,[o("h1",null,[d(t(O),{icon:"puzzle-piece"}),e[0]||(e[0]=s(" Track A Submission",-1))]),e[18]||(e[18]=o("p",null,[s(" For Track A, a submission consists of a single text file containing a list of elements, separated by commas (a Python list expression), e.g., "),o("code",null,"[11, 12, 13, 14, 1, 2, 3]"),s(". ")],-1)),o("p",null,[e[2]||(e[2]=s(" (For more information, see the instructions on the ",-1)),d(b,{to:"/aboutA"},{default:v(()=>[...e[1]||(e[1]=[s("Track A",-1)])]),_:1}),e[3]||(e[3]=s(" page.) ",-1))]),C.value?(u(),m("div",Y,[t(p)==!1?(u(),m("div",L,[d(N,{onFileDropped:S})])):t(a)==!0?(u(),m("div",q," Your submission is too large for in-browser computation, so it has been sent to the server for processing. You will be notified by email the results. ")):t(p)==!0?(u(),m("div",G,[d(t(B),{"model-value":!0,variant:"success"},{default:v(()=>[o("h4",U,[d(i),e[4]||(e[4]=s(" Success! ",-1))]),e[16]||(e[16]=o("p",null,[s(" Your submission has been computed and recorded successfully! You should momentarily receive an email receipt outlining computation details for your convenience. "),o("br"),s(" It is now safe to close this page. ")],-1)),e[17]||(e[17]=o("hr",null,null,-1)),o("p",Q,[e[5]||(e[5]=o("b",null,"Number of Powersort Comparisons",-1)),e[6]||(e[6]=s(": ",-1)),o("code",null,w(t(r)),1),e[7]||(e[7]=o("br",null,null,-1)),e[8]||(e[8]=o("b",null,"Number of Timsort Comparisons",-1)),e[9]||(e[9]=s(": ",-1)),o("code",null,w(t(f)),1),e[10]||(e[10]=o("br",null,null,-1)),e[11]||(e[11]=o("b",null,"Powersort MergeCost",-1)),e[12]||(e[12]=s(": ",-1)),o("code",null,w(t(g)),1),e[13]||(e[13]=o("br",null,null,-1)),e[14]||(e[14]=o("b",null,"Timsort MergeCost",-1)),e[15]||(e[15]=s(": ",-1)),o("code",null,w(t(c)),1)])]),_:1})])):P("",!0)])):P("",!0)])}}};export{oe as default};
