import{y as f,q as g,B as E}from"./preact.module.551f0998.js";const w="modulepreload",S=function(e){return"/"+e},m={},p=function(l,s,i){if(!s||s.length===0)return l();const u=document.getElementsByTagName("link");return Promise.all(s.map(t=>{if(t=S(t),t in m)return;m[t]=!0;const a=t.endsWith(".css"),h=a?'[rel="stylesheet"]':"";if(!!i)for(let r=u.length-1;r>=0;r--){const o=u[r];if(o.href===t&&(!a||o.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${t}"]${h}`))return;const n=document.createElement("link");if(n.rel=a?"stylesheet":w,a||(n.as="script",n.crossOrigin=""),n.href=t,document.head.appendChild(n),a)return new Promise((r,o)=>{n.addEventListener("load",r),n.addEventListener("error",()=>o(new Error(`Unable to preload CSS for ${t}`)))})})).then(()=>l()).catch(t=>{const a=new Event("vite:preloadError",{cancelable:!0});if(a.payload=t,window.dispatchEvent(a),!a.defaultPrevented)throw t})},_=({value:e,name:l,hydrate:s=!0})=>e?f(s?"astro-slot":"astro-static-slot",{name:l,dangerouslySetInnerHTML:{__html:e}}):null;_.shouldComponentUpdate=()=>!1;var v=_;const d=new Map;var R=e=>async(l,s,{default:i,...u},{client:t})=>{if(!e.hasAttribute("ssr"))return;for(const[c,n]of Object.entries(u))s[c]=f(v,{value:n,name:c});if(e.dataset.preactSignals){const{signal:c}=await p(()=>import("./signals.module.7c68301c.js"),["_astro/signals.module.7c68301c.js","_astro/preact.module.551f0998.js","_astro/hooks.module.3a9cfe9f.js"]);let n=JSON.parse(e.dataset.preactSignals);for(const[r,o]of Object.entries(n)){if(!d.has(o)){const y=c(s[r]);d.set(o,y)}s[r]=d.get(o)}}(t!=="only"?E:g)(f(l,s,i!=null?f(v,{value:i}):i),e),e.addEventListener("astro:unmount",()=>g(null,e),{once:!0})};export{R as default};
