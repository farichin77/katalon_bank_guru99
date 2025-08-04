<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Book Your Free Demo Now - Phptravels  _c29661</name>
   <tag></tag>
   <elementGuidId>9b9ef340-6d62-4656-8121-e396c5ef3b4c</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>html</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
      <webElementGuid>7663156a-4981-4f8a-b487-5ba89f9cdb2b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
      <webElementGuid>86910d27-1fbd-40f2-9757-8747c376b34a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
Book Your Free Demo Now - Phptravels






































  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'G-6TWLEQHRC1');




   {
   &quot;@context&quot; : &quot;http://schema.org&quot;,
   &quot;@type&quot; : &quot;Corporation&quot;,
   &quot;brand&quot;: &quot;PHPTRAVELS&quot;,
   &quot;description&quot;: &quot;Targeting an incredible small niche, PHPtravels is a script designed for travel agencies that want to move their operations online, helping them setup a fully-working hotel room booking system.\n\nPHPtravels lets users search hotels, check room availability, book a room for their desired time period, and then pay for it, all via a simple interface, that not only looks good, but is also very manageable and customizable via a specially crafted administration panel.\n\nVarious accounts are available, for customers, for travel agents, and above all, for the site webmaster (admin).\n\nAnything on the PHPtravels website is customizable, from the frontend skin, to the active languages, available currencies, and not last, the hotels and rooms.\n\nIf you can get past the fact you need a commercial license to run PHPtravels, the system can be of incredible help if you plan to sell vacations and trips online as well.&quot;,
   &quot;name&quot; : &quot;PHPTRAVELS&quot;,
   &quot;founders&quot;: [
      &quot;Qasim Hussain&quot;
   ],
   &quot;foundingDate&quot;: &quot;2014-05&quot;,
   &quot;foundingLocation&quot;: &quot;Lahore&quot;,
   &quot;knowsAbout&quot;: &quot;Create online travel agency using PHPTRAVELS products&quot;,
   &quot;legalName&quot;: &quot;PHPTRAVELS&quot;,
   &quot;logo&quot; : &quot;https://phptravels.com/assets/img/pages/media/icon-primary.png&quot;,
   &quot;numberOfEmployees&quot;: &quot;15&quot;,
   &quot;ownershipFundingInfo&quot;: &quot;https://phptravels.com/about-us/&quot;,
   &quot;url&quot; : &quot;https://phptravels.com/demo&quot;,
   &quot;sameAs&quot; : [
      &quot;https://www.facebook.com/phptravels&quot;,
      &quot;https://www.twitter.com/phptravels&quot;,
      &quot;https://snapchat.com/add/phptravels&quot;,
      &quot;https://instagram.com/phptravels_/&quot;,
      &quot;https://www.youtube.com/user/phptravels&quot;,
      &quot;https://www.linkedin.com/company/phptravels&quot;,
      &quot;https://www.pinterest.com/phptravels_pin/&quot;
   ],
   &quot;slogan&quot;: &quot;Travel technology partner&quot;,
   &quot;tickerSymbol&quot;: [
      &quot;NYSE:SHOP&quot;,
      &quot;TSX:SHOP&quot;
   ],
   &quot;awards&quot;: &quot;https://www.ivisa.com/visa-blog/php-travels&quot;
      }






  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'AW-419817699');




// SAVE TRAFFIC TO DATABASE ON PAGE LOAD
setTimeout(function() {
var requestUrl = &quot;https://phptravels.com/visitor_details&quot;;
      fetch(requestUrl)
      .then(function(response) {
         return response.json();
      })
      .then(function(c) {
            if (typeof c.country_code === &quot;undefined&quot;) {
               // If country_code is undefined, log and return or handle appropriately
               console.log(&quot;Localhost traffic not counted&quot;);
            } else {
               console.log(c);
               var country = c.country_code.toUpperCase();

                const formdata = new FormData();
                formdata.append(&quot;country_code&quot;, country);
                formdata.append(&quot;user_ip&quot;, c.user_ip);
                formdata.append(&quot;page_link&quot;, window.location.href);
                formdata.append(&quot;user_agent&quot;, c.user_agent);
                formdata.append(&quot;http_referer&quot;, &quot;Direct Traffic&quot;);

                const requestOptions = {
                method: &quot;POST&quot;,
                body: formdata,
                redirect: &quot;follow&quot;
                };

                fetch(&quot;https://app.phptravels.com/api_website_traffic?&quot;, requestOptions)
                .then((response) => response.text())
                // .then((result) => console.log(result))
                .catch((error) => console.error(error));

            }
      });
  }, 2500);

  (function () {
      window.usermaven = window.usermaven || (function () { (window.usermavenQ = window.usermavenQ || []).push(arguments); })
      var t = document.createElement('script'),
          s = document.getElementsByTagName('script')[0];
      t.defer = true;
      t.id = 'um-tracker';
      t.setAttribute('data-tracking-host', &quot;https://events.usermaven.com&quot;)
      t.setAttribute('data-key', 'UMlduvNwrU');
      t.setAttribute('data-autocapture', 'true');
      t.src = 'https://t.usermaven.com/lib.js';
      s.parentNode.insertBefore(t, s);
  })();



#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}





  
  
      
        
          We stand with Palestine
          
            
              .cls-1{fill:#fff;}
              .cls-2{fill:#007a3d;}
              .cls-3{fill:#ce1126;}
            
          Palestine
          
          
          
          
          Because Humanity Matters
        
      
  

    
      
        
          

          
               
                  
                  
               
            

          
          
            PHPTRAVELS  
            Travel Tech Partner
            
          
        

        
        
        

        
          
            
              
                Product
                
                  
                
              
              
              
                    Themes
                    
                  
                
                
                    Integrations
                    
                  
                
                
                  Customization

                    
                  
                
                
                    Technology

                  
                
                    Requirements

                    
                  
              
            
            
              
                Features
                
                  
                
              
              
                
                    Flights Modules

                  
                
                
                    Hotels Modules

                  
                
                    Tours Modules

                  
                
                    Cars Modules

                  
                
                    Blogs Modules

                  
                
                    CMS Modules

                  
                
                    Offers Modules

                  
                
                    Newsletter Modules

                  
              
            
            
              
                Company
                
                  
                
              
              
                
                    Clients

                  
                
                    Blogs

                  
                
                    Docs

                  
                
                    Contact Us

                  
                
                    About Us

                  
                
                    The Team

                  
                
                    Jobs

                  
                
                    Partners

                  
                
                    Media Kit
                    
                  
              
            
            
              Demo
            
            
              Pricing
            
          
          
            
              
              Talk to Sales
            Login
            Sign Up

          
        
      
    
  
  
/*! jQuery v3.6.0 | (c) OpenJS Foundation and other contributors | jquery.org/license */
!function(e,t){&quot;use strict&quot;;&quot;object&quot;==typeof module&amp;&amp;&quot;object&quot;==typeof module.exports?module.exports=e.document?t(e,!0):function(e){if(!e.document)throw new Error(&quot;jQuery requires a window with a document&quot;);return t(e)}:t(e)}(&quot;undefined&quot;!=typeof window?window:this,function(C,e){&quot;use strict&quot;;var t=[],r=Object.getPrototypeOf,s=t.slice,g=t.flat?function(e){return t.flat.call(e)}:function(e){return t.concat.apply([],e)},u=t.push,i=t.indexOf,n={},o=n.toString,v=n.hasOwnProperty,a=v.toString,l=a.call(Object),y={},m=function(e){return&quot;function&quot;==typeof e&amp;&amp;&quot;number&quot;!=typeof e.nodeType&amp;&amp;&quot;function&quot;!=typeof e.item},x=function(e){return null!=e&amp;&amp;e===e.window},E=C.document,c={type:!0,src:!0,nonce:!0,noModule:!0};function b(e,t,n){var r,i,o=(n=n||E).createElement(&quot;script&quot;);if(o.text=e,t)for(r in c)(i=t[r]||t.getAttribute&amp;&amp;t.getAttribute(r))&amp;&amp;o.setAttribute(r,i);n.head.appendChild(o).parentNode.removeChild(o)}function w(e){return null==e?e+&quot;&quot;:&quot;object&quot;==typeof e||&quot;function&quot;==typeof e?n[o.call(e)]||&quot;object&quot;:typeof e}var f=&quot;3.6.0&quot;,S=function(e,t){return new S.fn.init(e,t)};function p(e){var t=!!e&amp;&amp;&quot;length&quot;in e&amp;&amp;e.length,n=w(e);return!m(e)&amp;&amp;!x(e)&amp;&amp;(&quot;array&quot;===n||0===t||&quot;number&quot;==typeof t&amp;&amp;0&lt;t&amp;&amp;t-1 in e)}S.fn=S.prototype={jquery:f,constructor:S,length:0,toArray:function(){return s.call(this)},get:function(e){return null==e?s.call(this):e&lt;0?this[e+this.length]:this[e]},pushStack:function(e){var t=S.merge(this.constructor(),e);return t.prevObject=this,t},each:function(e){return S.each(this,e)},map:function(n){return this.pushStack(S.map(this,function(e,t){return n.call(e,t,e)}))},slice:function(){return this.pushStack(s.apply(this,arguments))},first:function(){return this.eq(0)},last:function(){return this.eq(-1)},even:function(){return this.pushStack(S.grep(this,function(e,t){return(t+1)%2}))},odd:function(){return this.pushStack(S.grep(this,function(e,t){return t%2}))},eq:function(e){var t=this.length,n=+e+(e&lt;0?t:0);return this.pushStack(0&lt;=n&amp;&amp;n&lt;t?[this[n]]:[])},end:function(){return this.prevObject||this.constructor()},push:u,sort:t.sort,splice:t.splice},S.extend=S.fn.extend=function(){var e,t,n,r,i,o,a=arguments[0]||{},s=1,u=arguments.length,l=!1;for(&quot;boolean&quot;==typeof a&amp;&amp;(l=a,a=arguments[s]||{},s++),&quot;object&quot;==typeof a||m(a)||(a={}),s===u&amp;&amp;(a=this,s--);s&lt;u;s++)if(null!=(e=arguments[s]))for(t in e)r=e[t],&quot;__proto__&quot;!==t&amp;&amp;a!==r&amp;&amp;(l&amp;&amp;r&amp;&amp;(S.isPlainObject(r)||(i=Array.isArray(r)))?(n=a[t],o=i&amp;&amp;!Array.isArray(n)?[]:i||S.isPlainObject(n)?n:{},i=!1,a[t]=S.extend(l,o,r)):void 0!==r&amp;&amp;(a[t]=r));return a},S.extend({expando:&quot;jQuery&quot;+(f+Math.random()).replace(/\D/g,&quot;&quot;),isReady:!0,error:function(e){throw new Error(e)},noop:function(){},isPlainObject:function(e){var t,n;return!(!e||&quot;[object Object]&quot;!==o.call(e))&amp;&amp;(!(t=r(e))||&quot;function&quot;==typeof(n=v.call(t,&quot;constructor&quot;)&amp;&amp;t.constructor)&amp;&amp;a.call(n)===l)},isEmptyObject:function(e){var t;for(t in e)return!1;return!0},globalEval:function(e,t,n){b(e,{nonce:t&amp;&amp;t.nonce},n)},each:function(e,t){var n,r=0;if(p(e)){for(n=e.length;r&lt;n;r++)if(!1===t.call(e[r],r,e[r]))break}else for(r in e)if(!1===t.call(e[r],r,e[r]))break;return e},makeArray:function(e,t){var n=t||[];return null!=e&amp;&amp;(p(Object(e))?S.merge(n,&quot;string&quot;==typeof e?[e]:e):u.call(n,e)),n},inArray:function(e,t,n){return null==t?-1:i.call(t,e,n)},merge:function(e,t){for(var n=+t.length,r=0,i=e.length;r&lt;n;r++)e[i++]=t[r];return e.length=i,e},grep:function(e,t,n){for(var r=[],i=0,o=e.length,a=!n;i&lt;o;i++)!t(e[i],i)!==a&amp;&amp;r.push(e[i]);return r},map:function(e,t,n){var r,i,o=0,a=[];if(p(e))for(r=e.length;o&lt;r;o++)null!=(i=t(e[o],o,n))&amp;&amp;a.push(i);else for(o in e)null!=(i=t(e[o],o,n))&amp;&amp;a.push(i);return g(a)},guid:1,support:y}),&quot;function&quot;==typeof Symbol&amp;&amp;(S.fn[Symbol.iterator]=t[Symbol.iterator]),S.each(&quot;Boolean Number String Function Array Date RegExp Object Error Symbol&quot;.split(&quot; &quot;),function(e,t){n[&quot;[object &quot;+t+&quot;]&quot;]=t.toLowerCase()});var d=function(n){var e,d,b,o,i,h,f,g,w,u,l,T,C,a,E,v,s,c,y,S=&quot;sizzle&quot;+1*new Date,p=n.document,k=0,r=0,m=ue(),x=ue(),A=ue(),N=ue(),j=function(e,t){return e===t&amp;&amp;(l=!0),0},D={}.hasOwnProperty,t=[],q=t.pop,L=t.push,H=t.push,O=t.slice,P=function(e,t){for(var n=0,r=e.length;n&lt;r;n++)if(e[n]===t)return n;return-1},R=&quot;checked|selected|async|autofocus|autoplay|controls|defer|disabled|hidden|ismap|loop|multiple|open|readonly|required|scoped&quot;,M=&quot;[\\x20\\t\\r\\n\\f]&quot;,I=&quot;(?:\\\\[\\da-fA-F]{1,6}&quot;+M+&quot;?|\\\\[^\\r\\n\\f]|[\\w-]|[^\0-\\x7f])+&quot;,W=&quot;\\[&quot;+M+&quot;*(&quot;+I+&quot;)(?:&quot;+M+&quot;*([*^$|!~]?=)&quot;+M+&quot;*(?:'((?:\\\\.|[^\\\\'])*)'|\&quot;((?:\\\\.|[^\\\\\&quot;])*)\&quot;|(&quot;+I+&quot;))|)&quot;+M+&quot;*\\]&quot;,F=&quot;:(&quot;+I+&quot;)(?:\\((('((?:\\\\.|[^\\\\'])*)'|\&quot;((?:\\\\.|[^\\\\\&quot;])*)\&quot;)|((?:\\\\.|[^\\\\()[\\]]|&quot;+W+&quot;)*)|.*)\\)|)&quot;,B=new RegExp(M+&quot;+&quot;,&quot;g&quot;),$=new RegExp(&quot;^&quot;+M+&quot;+|((?:^|[^\\\\])(?:\\\\.)*)&quot;+M+&quot;+$&quot;,&quot;g&quot;),_=new RegExp(&quot;^&quot;+M+&quot;*,&quot;+M+&quot;*&quot;),z=new RegExp(&quot;^&quot;+M+&quot;*([>+~]|&quot;+M+&quot;)&quot;+M+&quot;*&quot;),U=new RegExp(M+&quot;|>&quot;),X=new RegExp(F),V=new RegExp(&quot;^&quot;+I+&quot;$&quot;),G={ID:new RegExp(&quot;^#(&quot;+I+&quot;)&quot;),CLASS:new RegExp(&quot;^\\.(&quot;+I+&quot;)&quot;),TAG:new RegExp(&quot;^(&quot;+I+&quot;|[*])&quot;),ATTR:new RegExp(&quot;^&quot;+W),PSEUDO:new RegExp(&quot;^&quot;+F),CHILD:new RegExp(&quot;^:(only|first|last|nth|nth-last)-(child|of-type)(?:\\(&quot;+M+&quot;*(even|odd|(([+-]|)(\\d*)n|)&quot;+M+&quot;*(?:([+-]|)&quot;+M+&quot;*(\\d+)|))&quot;+M+&quot;*\\)|)&quot;,&quot;i&quot;),bool:new RegExp(&quot;^(?:&quot;+R+&quot;)$&quot;,&quot;i&quot;),needsContext:new RegExp(&quot;^&quot;+M+&quot;*[>+~]|:(even|odd|eq|gt|lt|nth|first|last)(?:\\(&quot;+M+&quot;*((?:-\\d)?\\d*)&quot;+M+&quot;*\\)|)(?=[^-]|$)&quot;,&quot;i&quot;)},Y=/HTML$/i,Q=/^(?:input|select|textarea|button)$/i,J=/^h\d$/i,K=/^[^{]+\{\s*\[native \w/,Z=/^(?:#([\w-]+)|(\w+)|\.([\w-]+))$/,ee=/[+~]/,te=new RegExp(&quot;\\\\[\\da-fA-F]{1,6}&quot;+M+&quot;?|\\\\([^\\r\\n\\f])&quot;,&quot;g&quot;),ne=function(e,t){var n=&quot;0x&quot;+e.slice(1)-65536;return t||(n&lt;0?String.fromCharCode(n+65536):String.fromCharCode(n>>10|55296,1023&amp;n|56320))},re=/([\0-\x1f\x7f]|^-?\d)|^-$|[^\0-\x1f\x7f-\uFFFF\w-]/g,ie=function(e,t){return t?&quot;\0&quot;===e?&quot;\ufffd&quot;:e.slice(0,-1)+&quot;\\&quot;+e.charCodeAt(e.length-1).toString(16)+&quot; &quot;:&quot;\\&quot;+e},oe=function(){T()},ae=be(function(e){return!0===e.disabled&amp;&amp;&quot;fieldset&quot;===e.nodeName.toLowerCase()},{dir:&quot;parentNode&quot;,next:&quot;legend&quot;});try{H.apply(t=O.call(p.childNodes),p.childNodes),t[p.childNodes.length].nodeType}catch(e){H={apply:t.length?function(e,t){L.apply(e,O.call(t))}:function(e,t){var n=e.length,r=0;while(e[n++]=t[r++]);e.length=n-1}}}function se(t,e,n,r){var i,o,a,s,u,l,c,f=e&amp;&amp;e.ownerDocument,p=e?e.nodeType:9;if(n=n||[],&quot;string&quot;!=typeof t||!t||1!==p&amp;&amp;9!==p&amp;&amp;11!==p)return n;if(!r&amp;&amp;(T(e),e=e||C,E)){if(11!==p&amp;&amp;(u=Z.exec(t)))if(i=u[1]){if(9===p){if(!(a=e.getElementById(i)))return n;if(a.id===i)return n.push(a),n}else if(f&amp;&amp;(a=f.getElementById(i))&amp;&amp;y(e,a)&amp;&amp;a.id===i)return n.push(a),n}else{if(u[2])return H.apply(n,e.getElementsByTagName(t)),n;if((i=u[3])&amp;&amp;d.getElementsByClassName&amp;&amp;e.getElementsByClassName)return H.apply(n,e.getElementsByClassName(i)),n}if(d.qsa&amp;&amp;!N[t+&quot; &quot;]&amp;&amp;(!v||!v.test(t))&amp;&amp;(1!==p||&quot;object&quot;!==e.nodeName.toLowerCase())){if(c=t,f=e,1===p&amp;&amp;(U.test(t)||z.test(t))){(f=ee.test(t)&amp;&amp;ye(e.parentNode)||e)===e&amp;&amp;d.scope||((s=e.getAttribute(&quot;id&quot;))?s=s.replace(re,ie):e.setAttribute(&quot;id&quot;,s=S)),o=(l=h(t)).length;while(o--)l[o]=(s?&quot;#&quot;+s:&quot;:scope&quot;)+&quot; &quot;+xe(l[o]);c=l.join(&quot;,&quot;)}try{return H.apply(n,f.querySelectorAll(c)),n}catch(e){N(t,!0)}finally{s===S&amp;&amp;e.removeAttribute(&quot;id&quot;)}}}return g(t.replace($,&quot;$1&quot;),e,n,r)}function ue(){var r=[];return function e(t,n){return r.push(t+&quot; &quot;)>b.cacheLength&amp;&amp;delete e[r.shift()],e[t+&quot; &quot;]=n}}function le(e){return e[S]=!0,e}function ce(e){var t=C.createElement(&quot;fieldset&quot;);try{return!!e(t)}catch(e){return!1}finally{t.parentNode&amp;&amp;t.parentNode.removeChild(t),t=null}}function fe(e,t){var n=e.split(&quot;|&quot;),r=n.length;while(r--)b.attrHandle[n[r]]=t}function pe(e,t){var n=t&amp;&amp;e,r=n&amp;&amp;1===e.nodeType&amp;&amp;1===t.nodeType&amp;&amp;e.sourceIndex-t.sourceIndex;if(r)return r;if(n)while(n=n.nextSibling)if(n===t)return-1;return e?1:-1}function de(t){return function(e){return&quot;input&quot;===e.nodeName.toLowerCase()&amp;&amp;e.type===t}}function he(n){return function(e){var t=e.nodeName.toLowerCase();return(&quot;input&quot;===t||&quot;button&quot;===t)&amp;&amp;e.type===n}}function ge(t){return function(e){return&quot;form&quot;in e?e.parentNode&amp;&amp;!1===e.disabled?&quot;label&quot;in e?&quot;label&quot;in e.parentNode?e.parentNode.disabled===t:e.disabled===t:e.isDisabled===t||e.isDisabled!==!t&amp;&amp;ae(e)===t:e.disabled===t:&quot;label&quot;in e&amp;&amp;e.disabled===t}}function ve(a){return le(function(o){return o=+o,le(function(e,t){var n,r=a([],e.length,o),i=r.length;while(i--)e[n=r[i]]&amp;&amp;(e[n]=!(t[n]=e[n]))})})}function ye(e){return e&amp;&amp;&quot;undefined&quot;!=typeof e.getElementsByTagName&amp;&amp;e}for(e in d=se.support={},i=se.isXML=function(e){var t=e&amp;&amp;e.namespaceURI,n=e&amp;&amp;(e.ownerDocument||e).documentElement;return!Y.test(t||n&amp;&amp;n.nodeName||&quot;HTML&quot;)},T=se.setDocument=function(e){var t,n,r=e?e.ownerDocument||e:p;return r!=C&amp;&amp;9===r.nodeType&amp;&amp;r.documentElement&amp;&amp;(a=(C=r).documentElement,E=!i(C),p!=C&amp;&amp;(n=C.defaultView)&amp;&amp;n.top!==n&amp;&amp;(n.addEventListener?n.addEventListener(&quot;unload&quot;,oe,!1):n.attachEvent&amp;&amp;n.attachEvent(&quot;onunload&quot;,oe)),d.scope=ce(function(e){return a.appendChild(e).appendChild(C.createElement(&quot;div&quot;)),&quot;undefined&quot;!=typeof e.querySelectorAll&amp;&amp;!e.querySelectorAll(&quot;:scope fieldset div&quot;).length}),d.attributes=ce(function(e){return e.className=&quot;i&quot;,!e.getAttribute(&quot;className&quot;)}),d.getElementsByTagName=ce(function(e){return e.appendChild(C.createComment(&quot;&quot;)),!e.getElementsByTagName(&quot;*&quot;).length}),d.getElementsByClassName=K.test(C.getElementsByClassName),d.getById=ce(function(e){return a.appendChild(e).id=S,!C.getElementsByName||!C.getElementsByName(S).length}),d.getById?(b.filter.ID=function(e){var t=e.replace(te,ne);return function(e){return e.getAttribute(&quot;id&quot;)===t}},b.find.ID=function(e,t){if(&quot;undefined&quot;!=typeof t.getElementById&amp;&amp;E){var n=t.getElementById(e);return n?[n]:[]}}):(b.filter.ID=function(e){var n=e.replace(te,ne);return function(e){var t=&quot;undefined&quot;!=typeof e.getAttributeNode&amp;&amp;e.getAttributeNode(&quot;id&quot;);return t&amp;&amp;t.value===n}},b.find.ID=function(e,t){if(&quot;undefined&quot;!=typeof t.getElementById&amp;&amp;E){var n,r,i,o=t.getElementById(e);if(o){if((n=o.getAttributeNode(&quot;id&quot;))&amp;&amp;n.value===e)return[o];i=t.getElementsByName(e),r=0;while(o=i[r++])if((n=o.getAttributeNode(&quot;id&quot;))&amp;&amp;n.value===e)return[o]}return[]}}),b.find.TAG=d.getElementsByTagName?function(e,t){return&quot;undefined&quot;!=typeof t.getElementsByTagName?t.getElementsByTagName(e):d.qsa?t.querySelectorAll(e):void 0}:function(e,t){var n,r=[],i=0,o=t.getElementsByTagName(e);if(&quot;*&quot;===e){while(n=o[i++])1===n.nodeType&amp;&amp;r.push(n);return r}return o},b.find.CLASS=d.getElementsByClassName&amp;&amp;function(e,t){if(&quot;undefined&quot;!=typeof t.getElementsByClassName&amp;&amp;E)return t.getElementsByClassName(e)},s=[],v=[],(d.qsa=K.test(C.querySelectorAll))&amp;&amp;(ce(function(e){var t;a.appendChild(e).innerHTML=&quot;&lt;a id='&quot;+S+&quot;'>&lt;/a>&lt;select id='&quot;+S+&quot;-\r\\' msallowcapture=''>&lt;option selected=''>&lt;/option>&lt;/select>&quot;,e.querySelectorAll(&quot;[msallowcapture^='']&quot;).length&amp;&amp;v.push(&quot;[*^$]=&quot;+M+&quot;*(?:''|\&quot;\&quot;)&quot;),e.querySelectorAll(&quot;[selected]&quot;).length||v.push(&quot;\\[&quot;+M+&quot;*(?:value|&quot;+R+&quot;)&quot;),e.querySelectorAll(&quot;[id~=&quot;+S+&quot;-]&quot;).length||v.push(&quot;~=&quot;),(t=C.createElement(&quot;input&quot;)).setAttribute(&quot;name&quot;,&quot;&quot;),e.appendChild(t),e.querySelectorAll(&quot;[name='']&quot;).length||v.push(&quot;\\[&quot;+M+&quot;*name&quot;+M+&quot;*=&quot;+M+&quot;*(?:''|\&quot;\&quot;)&quot;),e.querySelectorAll(&quot;:checked&quot;).length||v.push(&quot;:checked&quot;),e.querySelectorAll(&quot;a#&quot;+S+&quot;+*&quot;).length||v.push(&quot;.#.+[+~]&quot;),e.querySelectorAll(&quot;\\\f&quot;),v.push(&quot;[\\r\\n\\f]&quot;)}),ce(function(e){e.innerHTML=&quot;&lt;a href='' disabled='disabled'>&lt;/a>&lt;select disabled='disabled'>&lt;option/>&lt;/select>&quot;;var t=C.createElement(&quot;input&quot;);t.setAttribute(&quot;type&quot;,&quot;hidden&quot;),e.appendChild(t).setAttribute(&quot;name&quot;,&quot;D&quot;),e.querySelectorAll(&quot;[name=d]&quot;).length&amp;&amp;v.push(&quot;name&quot;+M+&quot;*[*^$|!~]?=&quot;),2!==e.querySelectorAll(&quot;:enabled&quot;).length&amp;&amp;v.push(&quot;:enabled&quot;,&quot;:disabled&quot;),a.appendChild(e).disabled=!0,2!==e.querySelectorAll(&quot;:disabled&quot;).length&amp;&amp;v.push(&quot;:enabled&quot;,&quot;:disabled&quot;),e.querySelectorAll(&quot;*,:x&quot;),v.push(&quot;,.*:&quot;)})),(d.matchesSelector=K.test(c=a.matches||a.webkitMatchesSelector||a.mozMatchesSelector||a.oMatchesSelector||a.msMatchesSelector))&amp;&amp;ce(function(e){d.disconnectedMatch=c.call(e,&quot;*&quot;),c.call(e,&quot;[s!='']:x&quot;),s.push(&quot;!=&quot;,F)}),v=v.length&amp;&amp;new RegExp(v.join(&quot;|&quot;)),s=s.length&amp;&amp;new RegExp(s.join(&quot;|&quot;)),t=K.test(a.compareDocumentPosition),y=t||K.test(a.contains)?function(e,t){var n=9===e.nodeType?e.documentElement:e,r=t&amp;&amp;t.parentNode;return e===r||!(!r||1!==r.nodeType||!(n.contains?n.contains(r):e.compareDocumentPosition&amp;&amp;16&amp;e.compareDocumentPosition(r)))}:function(e,t){if(t)while(t=t.parentNode)if(t===e)return!0;return!1},j=t?function(e,t){if(e===t)return l=!0,0;var n=!e.compareDocumentPosition-!t.compareDocumentPosition;return n||(1&amp;(n=(e.ownerDocument||e)==(t.ownerDocument||t)?e.compareDocumentPosition(t):1)||!d.sortDetached&amp;&amp;t.compareDocumentPosition(e)===n?e==C||e.ownerDocument==p&amp;&amp;y(p,e)?-1:t==C||t.ownerDocument==p&amp;&amp;y(p,t)?1:u?P(u,e)-P(u,t):0:4&amp;n?-1:1)}:function(e,t){if(e===t)return l=!0,0;var n,r=0,i=e.parentNode,o=t.parentNode,a=[e],s=[t];if(!i||!o)return e==C?-1:t==C?1:i?-1:o?1:u?P(u,e)-P(u,t):0;if(i===o)return pe(e,t);n=e;while(n=n.parentNode)a.unshift(n);n=t;while(n=n.parentNode)s.unshift(n);while(a[r]===s[r])r++;return r?pe(a[r],s[r]):a[r]==p?-1:s[r]==p?1:0}),C},se.matches=function(e,t){return se(e,null,null,t)},se.matchesSelector=function(e,t){if(T(e),d.matchesSelector&amp;&amp;E&amp;&amp;!N[t+&quot; &quot;]&amp;&amp;(!s||!s.test(t))&amp;&amp;(!v||!v.test(t)))try{var n=c.call(e,t);if(n||d.disconnectedMatch||e.document&amp;&amp;11!==e.document.nodeType)return n}catch(e){N(t,!0)}return 0&lt;se(t,C,null,[e]).length},se.contains=function(e,t){return(e.ownerDocument||e)!=C&amp;&amp;T(e),y(e,t)},se.attr=function(e,t){(e.ownerDocument||e)!=C&amp;&amp;T(e);var n=b.attrHandle[t.toLowerCase()],r=n&amp;&amp;D.call(b.attrHandle,t.toLowerCase())?n(e,t,!E):void 0;return void 0!==r?r:d.attributes||!E?e.getAttribute(t):(r=e.getAttributeNode(t))&amp;&amp;r.specified?r.value:null},se.escape=function(e){return(e+&quot;&quot;).replace(re,ie)},se.error=function(e){throw new Error(&quot;Syntax error, unrecognized expression: &quot;+e)},se.uniqueSort=function(e){var t,n=[],r=0,i=0;if(l=!d.detectDuplicates,u=!d.sortStable&amp;&amp;e.slice(0),e.sort(j),l){while(t=e[i++])t===e[i]&amp;&amp;(r=n.push(i));while(r--)e.splice(n[r],1)}return u=null,e},o=se.getText=function(e){var t,n=&quot;&quot;,r=0,i=e.nodeType;if(i){if(1===i||9===i||11===i){if(&quot;string&quot;==typeof e.textContent)return e.textContent;for(e=e.firstChild;e;e=e.nextSibling)n+=o(e)}else if(3===i||4===i)return e.nodeValue}else while(t=e[r++])n+=o(t);return n},(b=se.selectors={cacheLength:50,createPseudo:le,match:G,attrHandle:{},find:{},relative:{&quot;>&quot;:{dir:&quot;parentNode&quot;,first:!0},&quot; &quot;:{dir:&quot;parentNode&quot;},&quot;+&quot;:{dir:&quot;previousSibling&quot;,first:!0},&quot;~&quot;:{dir:&quot;previousSibling&quot;}},preFilter:{ATTR:function(e){return e[1]=e[1].replace(te,ne),e[3]=(e[3]||e[4]||e[5]||&quot;&quot;).replace(te,ne),&quot;~=&quot;===e[2]&amp;&amp;(e[3]=&quot; &quot;+e[3]+&quot; &quot;),e.slice(0,4)},CHILD:function(e){return e[1]=e[1].toLowerCase(),&quot;nth&quot;===e[1].slice(0,3)?(e[3]||se.error(e[0]),e[4]=+(e[4]?e[5]+(e[6]||1):2*(&quot;even&quot;===e[3]||&quot;odd&quot;===e[3])),e[5]=+(e[7]+e[8]||&quot;odd&quot;===e[3])):e[3]&amp;&amp;se.error(e[0]),e},PSEUDO:function(e){var t,n=!e[6]&amp;&amp;e[2];return G.CHILD.test(e[0])?null:(e[3]?e[2]=e[4]||e[5]||&quot;&quot;:n&amp;&amp;X.test(n)&amp;&amp;(t=h(n,!0))&amp;&amp;(t=n.indexOf(&quot;)&quot;,n.length-t)-n.length)&amp;&amp;(e[0]=e[0].slice(0,t),e[2]=n.slice(0,t)),e.slice(0,3))}},filter:{TAG:function(e){var t=e.replace(te,ne).toLowerCase();return&quot;*&quot;===e?function(){return!0}:function(e){return e.nodeName&amp;&amp;e.nodeName.toLowerCase()===t}},CLASS:function(e){var t=m[e+&quot; &quot;];return t||(t=new RegExp(&quot;(^|&quot;+M+&quot;)&quot;+e+&quot;(&quot;+M+&quot;|$)&quot;))&amp;&amp;m(e,function(e){return t.test(&quot;string&quot;==typeof e.className&amp;&amp;e.className||&quot;undefined&quot;!=typeof e.getAttribute&amp;&amp;e.getAttribute(&quot;class&quot;)||&quot;&quot;)})},ATTR:function(n,r,i){return function(e){var t=se.attr(e,n);return null==t?&quot;!=&quot;===r:!r||(t+=&quot;&quot;,&quot;=&quot;===r?t===i:&quot;!=&quot;===r?t!==i:&quot;^=&quot;===r?i&amp;&amp;0===t.indexOf(i):&quot;*=&quot;===r?i&amp;&amp;-1&lt;t.indexOf(i):&quot;$=&quot;===r?i&amp;&amp;t.slice(-i.length)===i:&quot;~=&quot;===r?-1&lt;(&quot; &quot;+t.replace(B,&quot; &quot;)+&quot; &quot;).indexOf(i):&quot;|=&quot;===r&amp;&amp;(t===i||t.slice(0,i.length+1)===i+&quot;-&quot;))}},CHILD:function(h,e,t,g,v){var y=&quot;nth&quot;!==h.slice(0,3),m=&quot;last&quot;!==h.slice(-4),x=&quot;of-type&quot;===e;return 1===g&amp;&amp;0===v?function(e){return!!e.parentNode}:function(e,t,n){var r,i,o,a,s,u,l=y!==m?&quot;nextSibling&quot;:&quot;previousSibling&quot;,c=e.parentNode,f=x&amp;&amp;e.nodeName.toLowerCase(),p=!n&amp;&amp;!x,d=!1;if(c){if(y){while(l){a=e;while(a=a[l])if(x?a.nodeName.toLowerCase()===f:1===a.nodeType)return!1;u=l=&quot;only&quot;===h&amp;&amp;!u&amp;&amp;&quot;nextSibling&quot;}return!0}if(u=[m?c.firstChild:c.lastChild],m&amp;&amp;p){d=(s=(r=(i=(o=(a=c)[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]||[])[0]===k&amp;&amp;r[1])&amp;&amp;r[2],a=s&amp;&amp;c.childNodes[s];while(a=++s&amp;&amp;a&amp;&amp;a[l]||(d=s=0)||u.pop())if(1===a.nodeType&amp;&amp;++d&amp;&amp;a===e){i[h]=[k,s,d];break}}else if(p&amp;&amp;(d=s=(r=(i=(o=(a=e)[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]||[])[0]===k&amp;&amp;r[1]),!1===d)while(a=++s&amp;&amp;a&amp;&amp;a[l]||(d=s=0)||u.pop())if((x?a.nodeName.toLowerCase()===f:1===a.nodeType)&amp;&amp;++d&amp;&amp;(p&amp;&amp;((i=(o=a[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]=[k,d]),a===e))break;return(d-=v)===g||d%g==0&amp;&amp;0&lt;=d/g}}},PSEUDO:function(e,o){var t,a=b.pseudos[e]||b.setFilters[e.toLowerCase()]||se.error(&quot;unsupported pseudo: &quot;+e);return a[S]?a(o):1&lt;a.length?(t=[e,e,&quot;&quot;,o],b.setFilters.hasOwnProperty(e.toLowerCase())?le(function(e,t){var n,r=a(e,o),i=r.length;while(i--)e[n=P(e,r[i])]=!(t[n]=r[i])}):function(e){return a(e,0,t)}):a}},pseudos:{not:le(function(e){var r=[],i=[],s=f(e.replace($,&quot;$1&quot;));return s[S]?le(function(e,t,n,r){var i,o=s(e,null,r,[]),a=e.length;while(a--)(i=o[a])&amp;&amp;(e[a]=!(t[a]=i))}):function(e,t,n){return r[0]=e,s(r,null,n,i),r[0]=null,!i.pop()}}),has:le(function(t){return function(e){return 0&lt;se(t,e).length}}),contains:le(function(t){return t=t.replace(te,ne),function(e){return-1&lt;(e.textContent||o(e)).indexOf(t)}}),lang:le(function(n){return V.test(n||&quot;&quot;)||se.error(&quot;unsupported lang: &quot;+n),n=n.replace(te,ne).toLowerCase(),function(e){var t;do{if(t=E?e.lang:e.getAttribute(&quot;xml:lang&quot;)||e.getAttribute(&quot;lang&quot;))return(t=t.toLowerCase())===n||0===t.indexOf(n+&quot;-&quot;)}while((e=e.parentNode)&amp;&amp;1===e.nodeType);return!1}}),target:function(e){var t=n.location&amp;&amp;n.location.hash;return t&amp;&amp;t.slice(1)===e.id},root:function(e){return e===a},focus:function(e){return e===C.activeElement&amp;&amp;(!C.hasFocus||C.hasFocus())&amp;&amp;!!(e.type||e.href||~e.tabIndex)},enabled:ge(!1),disabled:ge(!0),checked:function(e){var t=e.nodeName.toLowerCase();return&quot;input&quot;===t&amp;&amp;!!e.checked||&quot;option&quot;===t&amp;&amp;!!e.selected},selected:function(e){return e.parentNode&amp;&amp;e.parentNode.selectedIndex,!0===e.selected},empty:function(e){for(e=e.firstChild;e;e=e.nextSibling)if(e.nodeType&lt;6)return!1;return!0},parent:function(e){return!b.pseudos.empty(e)},header:function(e){return J.test(e.nodeName)},input:function(e){return Q.test(e.nodeName)},button:function(e){var t=e.nodeName.toLowerCase();return&quot;input&quot;===t&amp;&amp;&quot;button&quot;===e.type||&quot;button&quot;===t},text:function(e){var t;return&quot;input&quot;===e.nodeName.toLowerCase()&amp;&amp;&quot;text&quot;===e.type&amp;&amp;(null==(t=e.getAttribute(&quot;type&quot;))||&quot;text&quot;===t.toLowerCase())},first:ve(function(){return[0]}),last:ve(function(e,t){return[t-1]}),eq:ve(function(e,t,n){return[n&lt;0?n+t:n]}),even:ve(function(e,t){for(var n=0;n&lt;t;n+=2)e.push(n);return e}),odd:ve(function(e,t){for(var n=1;n&lt;t;n+=2)e.push(n);return e}),lt:ve(function(e,t,n){for(var r=n&lt;0?n+t:t&lt;n?t:n;0&lt;=--r;)e.push(r);return e}),gt:ve(function(e,t,n){for(var r=n&lt;0?n+t:n;++r&lt;t;)e.push(r);return e})}}).pseudos.nth=b.pseudos.eq,{radio:!0,checkbox:!0,file:!0,password:!0,image:!0})b.pseudos[e]=de(e);for(e in{submit:!0,reset:!0})b.pseudos[e]=he(e);function me(){}function xe(e){for(var t=0,n=e.length,r=&quot;&quot;;t&lt;n;t++)r+=e[t].value;return r}function be(s,e,t){var u=e.dir,l=e.next,c=l||u,f=t&amp;&amp;&quot;parentNode&quot;===c,p=r++;return e.first?function(e,t,n){while(e=e[u])if(1===e.nodeType||f)return s(e,t,n);return!1}:function(e,t,n){var r,i,o,a=[k,p];if(n){while(e=e[u])if((1===e.nodeType||f)&amp;&amp;s(e,t,n))return!0}else while(e=e[u])if(1===e.nodeType||f)if(i=(o=e[S]||(e[S]={}))[e.uniqueID]||(o[e.uniqueID]={}),l&amp;&amp;l===e.nodeName.toLowerCase())e=e[u]||e;else{if((r=i[c])&amp;&amp;r[0]===k&amp;&amp;r[1]===p)return a[2]=r[2];if((i[c]=a)[2]=s(e,t,n))return!0}return!1}}function we(i){return 1&lt;i.length?function(e,t,n){var r=i.length;while(r--)if(!i[r](e,t,n))return!1;return!0}:i[0]}function Te(e,t,n,r,i){for(var o,a=[],s=0,u=e.length,l=null!=t;s&lt;u;s++)(o=e[s])&amp;&amp;(n&amp;&amp;!n(o,r,i)||(a.push(o),l&amp;&amp;t.push(s)));return a}function Ce(d,h,g,v,y,e){return v&amp;&amp;!v[S]&amp;&amp;(v=Ce(v)),y&amp;&amp;!y[S]&amp;&amp;(y=Ce(y,e)),le(function(e,t,n,r){var i,o,a,s=[],u=[],l=t.length,c=e||function(e,t,n){for(var r=0,i=t.length;r&lt;i;r++)se(e,t[r],n);return n}(h||&quot;*&quot;,n.nodeType?[n]:n,[]),f=!d||!e&amp;&amp;h?c:Te(c,s,d,n,r),p=g?y||(e?d:l||v)?[]:t:f;if(g&amp;&amp;g(f,p,n,r),v){i=Te(p,u),v(i,[],n,r),o=i.length;while(o--)(a=i[o])&amp;&amp;(p[u[o]]=!(f[u[o]]=a))}if(e){if(y||d){if(y){i=[],o=p.length;while(o--)(a=p[o])&amp;&amp;i.push(f[o]=a);y(null,p=[],i,r)}o=p.length;while(o--)(a=p[o])&amp;&amp;-1&lt;(i=y?P(e,a):s[o])&amp;&amp;(e[i]=!(t[i]=a))}}else p=Te(p===t?p.splice(l,p.length):p),y?y(null,t,p,r):H.apply(t,p)})}function Ee(e){for(var i,t,n,r=e.length,o=b.relative[e[0].type],a=o||b.relative[&quot; &quot;],s=o?1:0,u=be(function(e){return e===i},a,!0),l=be(function(e){return-1&lt;P(i,e)},a,!0),c=[function(e,t,n){var r=!o&amp;&amp;(n||t!==w)||((i=t).nodeType?u(e,t,n):l(e,t,n));return i=null,r}];s&lt;r;s++)if(t=b.relative[e[s].type])c=[be(we(c),t)];else{if((t=b.filter[e[s].type].apply(null,e[s].matches))[S]){for(n=++s;n&lt;r;n++)if(b.relative[e[n].type])break;return Ce(1&lt;s&amp;&amp;we(c),1&lt;s&amp;&amp;xe(e.slice(0,s-1).concat({value:&quot; &quot;===e[s-2].type?&quot;*&quot;:&quot;&quot;})).replace($,&quot;$1&quot;),t,s&lt;n&amp;&amp;Ee(e.slice(s,n)),n&lt;r&amp;&amp;Ee(e=e.slice(n)),n&lt;r&amp;&amp;xe(e))}c.push(t)}return we(c)}return me.prototype=b.filters=b.pseudos,b.setFilters=new me,h=se.tokenize=function(e,t){var n,r,i,o,a,s,u,l=x[e+&quot; &quot;];if(l)return t?0:l.slice(0);a=e,s=[],u=b.preFilter;while(a){for(o in n&amp;&amp;!(r=_.exec(a))||(r&amp;&amp;(a=a.slice(r[0].length)||a),s.push(i=[])),n=!1,(r=z.exec(a))&amp;&amp;(n=r.shift(),i.push({value:n,type:r[0].replace($,&quot; &quot;)}),a=a.slice(n.length)),b.filter)!(r=G[o].exec(a))||u[o]&amp;&amp;!(r=u[o](r))||(n=r.shift(),i.push({value:n,type:o,matches:r}),a=a.slice(n.length));if(!n)break}return t?a.length:a?se.error(e):x(e,s).slice(0)},f=se.compile=function(e,t){var n,v,y,m,x,r,i=[],o=[],a=A[e+&quot; &quot;];if(!a){t||(t=h(e)),n=t.length;while(n--)(a=Ee(t[n]))[S]?i.push(a):o.push(a);(a=A(e,(v=o,m=0&lt;(y=i).length,x=0&lt;v.length,r=function(e,t,n,r,i){var o,a,s,u=0,l=&quot;0&quot;,c=e&amp;&amp;[],f=[],p=w,d=e||x&amp;&amp;b.find.TAG(&quot;*&quot;,i),h=k+=null==p?1:Math.random()||.1,g=d.length;for(i&amp;&amp;(w=t==C||t||i);l!==g&amp;&amp;null!=(o=d[l]);l++){if(x&amp;&amp;o){a=0,t||o.ownerDocument==C||(T(o),n=!E);while(s=v[a++])if(s(o,t||C,n)){r.push(o);break}i&amp;&amp;(k=h)}m&amp;&amp;((o=!s&amp;&amp;o)&amp;&amp;u--,e&amp;&amp;c.push(o))}if(u+=l,m&amp;&amp;l!==u){a=0;while(s=y[a++])s(c,f,t,n);if(e){if(0&lt;u)while(l--)c[l]||f[l]||(f[l]=q.call(r));f=Te(f)}H.apply(r,f),i&amp;&amp;!e&amp;&amp;0&lt;f.length&amp;&amp;1&lt;u+y.length&amp;&amp;se.uniqueSort(r)}return i&amp;&amp;(k=h,w=p),c},m?le(r):r))).selector=e}return a},g=se.select=function(e,t,n,r){var i,o,a,s,u,l=&quot;function&quot;==typeof e&amp;&amp;e,c=!r&amp;&amp;h(e=l.selector||e);if(n=n||[],1===c.length){if(2&lt;(o=c[0]=c[0].slice(0)).length&amp;&amp;&quot;ID&quot;===(a=o[0]).type&amp;&amp;9===t.nodeType&amp;&amp;E&amp;&amp;b.relative[o[1].type]){if(!(t=(b.find.ID(a.matches[0].replace(te,ne),t)||[])[0]))return n;l&amp;&amp;(t=t.parentNode),e=e.slice(o.shift().value.length)}i=G.needsContext.test(e)?0:o.length;while(i--){if(a=o[i],b.relative[s=a.type])break;if((u=b.find[s])&amp;&amp;(r=u(a.matches[0].replace(te,ne),ee.test(o[0].type)&amp;&amp;ye(t.parentNode)||t))){if(o.splice(i,1),!(e=r.length&amp;&amp;xe(o)))return H.apply(n,r),n;break}}}return(l||f(e,c))(r,t,!E,n,!t||ee.test(e)&amp;&amp;ye(t.parentNode)||t),n},d.sortStable=S.split(&quot;&quot;).sort(j).join(&quot;&quot;)===S,d.detectDuplicates=!!l,T(),d.sortDetached=ce(function(e){return 1&amp;e.compareDocumentPosition(C.createElement(&quot;fieldset&quot;))}),ce(function(e){return e.innerHTML=&quot;&lt;a href='#'>&lt;/a>&quot;,&quot;#&quot;===e.firstChild.getAttribute(&quot;href&quot;)})||fe(&quot;type|href|height|width&quot;,function(e,t,n){if(!n)return e.getAttribute(t,&quot;type&quot;===t.toLowerCase()?1:2)}),d.attributes&amp;&amp;ce(function(e){return e.innerHTML=&quot;&lt;input/>&quot;,e.firstChild.setAttribute(&quot;value&quot;,&quot;&quot;),&quot;&quot;===e.firstChild.getAttribute(&quot;value&quot;)})||fe(&quot;value&quot;,function(e,t,n){if(!n&amp;&amp;&quot;input&quot;===e.nodeName.toLowerCase())return e.defaultValue}),ce(function(e){return null==e.getAttribute(&quot;disabled&quot;)})||fe(R,function(e,t,n){var r;if(!n)return!0===e[t]?t.toLowerCase():(r=e.getAttributeNode(t))&amp;&amp;r.specified?r.value:null}),se}(C);S.find=d,S.expr=d.selectors,S.expr[&quot;:&quot;]=S.expr.pseudos,S.uniqueSort=S.unique=d.uniqueSort,S.text=d.getText,S.isXMLDoc=d.isXML,S.contains=d.contains,S.escapeSelector=d.escape;var h=function(e,t,n){var r=[],i=void 0!==n;while((e=e[t])&amp;&amp;9!==e.nodeType)if(1===e.nodeType){if(i&amp;&amp;S(e).is(n))break;r.push(e)}return r},T=function(e,t){for(var n=[];e;e=e.nextSibling)1===e.nodeType&amp;&amp;e!==t&amp;&amp;n.push(e);return n},k=S.expr.match.needsContext;function A(e,t){return e.nodeName&amp;&amp;e.nodeName.toLowerCase()===t.toLowerCase()}var N=/^&lt;([a-z][^\/\0>:\x20\t\r\n\f]*)[\x20\t\r\n\f]*\/?>(?:&lt;\/\1>|)$/i;function j(e,n,r){return m(n)?S.grep(e,function(e,t){return!!n.call(e,t,e)!==r}):n.nodeType?S.grep(e,function(e){return e===n!==r}):&quot;string&quot;!=typeof n?S.grep(e,function(e){return-1&lt;i.call(n,e)!==r}):S.filter(n,e,r)}S.filter=function(e,t,n){var r=t[0];return n&amp;&amp;(e=&quot;:not(&quot;+e+&quot;)&quot;),1===t.length&amp;&amp;1===r.nodeType?S.find.matchesSelector(r,e)?[r]:[]:S.find.matches(e,S.grep(t,function(e){return 1===e.nodeType}))},S.fn.extend({find:function(e){var t,n,r=this.length,i=this;if(&quot;string&quot;!=typeof e)return this.pushStack(S(e).filter(function(){for(t=0;t&lt;r;t++)if(S.contains(i[t],this))return!0}));for(n=this.pushStack([]),t=0;t&lt;r;t++)S.find(e,i[t],n);return 1&lt;r?S.uniqueSort(n):n},filter:function(e){return this.pushStack(j(this,e||[],!1))},not:function(e){return this.pushStack(j(this,e||[],!0))},is:function(e){return!!j(this,&quot;string&quot;==typeof e&amp;&amp;k.test(e)?S(e):e||[],!1).length}});var D,q=/^(?:\s*(&lt;[\w\W]+>)[^>]*|#([\w-]+))$/;(S.fn.init=function(e,t,n){var r,i;if(!e)return this;if(n=n||D,&quot;string&quot;==typeof e){if(!(r=&quot;&lt;&quot;===e[0]&amp;&amp;&quot;>&quot;===e[e.length-1]&amp;&amp;3&lt;=e.length?[null,e,null]:q.exec(e))||!r[1]&amp;&amp;t)return!t||t.jquery?(t||n).find(e):this.constructor(t).find(e);if(r[1]){if(t=t instanceof S?t[0]:t,S.merge(this,S.parseHTML(r[1],t&amp;&amp;t.nodeType?t.ownerDocument||t:E,!0)),N.test(r[1])&amp;&amp;S.isPlainObject(t))for(r in t)m(this[r])?this[r](t[r]):this.attr(r,t[r]);return this}return(i=E.getElementById(r[2]))&amp;&amp;(this[0]=i,this.length=1),this}return e.nodeType?(this[0]=e,this.length=1,this):m(e)?void 0!==n.ready?n.ready(e):e(S):S.makeArray(e,this)}).prototype=S.fn,D=S(E);var L=/^(?:parents|prev(?:Until|All))/,H={children:!0,contents:!0,next:!0,prev:!0};function O(e,t){while((e=e[t])&amp;&amp;1!==e.nodeType);return e}S.fn.extend({has:function(e){var t=S(e,this),n=t.length;return this.filter(function(){for(var e=0;e&lt;n;e++)if(S.contains(this,t[e]))return!0})},closest:function(e,t){var n,r=0,i=this.length,o=[],a=&quot;string&quot;!=typeof e&amp;&amp;S(e);if(!k.test(e))for(;r&lt;i;r++)for(n=this[r];n&amp;&amp;n!==t;n=n.parentNode)if(n.nodeType&lt;11&amp;&amp;(a?-1&lt;a.index(n):1===n.nodeType&amp;&amp;S.find.matchesSelector(n,e))){o.push(n);break}return this.pushStack(1&lt;o.length?S.uniqueSort(o):o)},index:function(e){return e?&quot;string&quot;==typeof e?i.call(S(e),this[0]):i.call(this,e.jquery?e[0]:e):this[0]&amp;&amp;this[0].parentNode?this.first().prevAll().length:-1},add:function(e,t){return this.pushStack(S.uniqueSort(S.merge(this.get(),S(e,t))))},addBack:function(e){return this.add(null==e?this.prevObject:this.prevObject.filter(e))}}),S.each({parent:function(e){var t=e.parentNode;return t&amp;&amp;11!==t.nodeType?t:null},parents:function(e){return h(e,&quot;parentNode&quot;)},parentsUntil:function(e,t,n){return h(e,&quot;parentNode&quot;,n)},next:function(e){return O(e,&quot;nextSibling&quot;)},prev:function(e){return O(e,&quot;previousSibling&quot;)},nextAll:function(e){return h(e,&quot;nextSibling&quot;)},prevAll:function(e){return h(e,&quot;previousSibling&quot;)},nextUntil:function(e,t,n){return h(e,&quot;nextSibling&quot;,n)},prevUntil:function(e,t,n){return h(e,&quot;previousSibling&quot;,n)},siblings:function(e){return T((e.parentNode||{}).firstChild,e)},children:function(e){return T(e.firstChild)},contents:function(e){return null!=e.contentDocument&amp;&amp;r(e.contentDocument)?e.contentDocument:(A(e,&quot;template&quot;)&amp;&amp;(e=e.content||e),S.merge([],e.childNodes))}},function(r,i){S.fn[r]=function(e,t){var n=S.map(this,i,e);return&quot;Until&quot;!==r.slice(-5)&amp;&amp;(t=e),t&amp;&amp;&quot;string&quot;==typeof t&amp;&amp;(n=S.filter(t,n)),1&lt;this.length&amp;&amp;(H[r]||S.uniqueSort(n),L.test(r)&amp;&amp;n.reverse()),this.pushStack(n)}});var P=/[^\x20\t\r\n\f]+/g;function R(e){return e}function M(e){throw e}function I(e,t,n,r){var i;try{e&amp;&amp;m(i=e.promise)?i.call(e).done(t).fail(n):e&amp;&amp;m(i=e.then)?i.call(e,t,n):t.apply(void 0,[e].slice(r))}catch(e){n.apply(void 0,[e])}}S.Callbacks=function(r){var e,n;r=&quot;string&quot;==typeof r?(e=r,n={},S.each(e.match(P)||[],function(e,t){n[t]=!0}),n):S.extend({},r);var i,t,o,a,s=[],u=[],l=-1,c=function(){for(a=a||r.once,o=i=!0;u.length;l=-1){t=u.shift();while(++l&lt;s.length)!1===s[l].apply(t[0],t[1])&amp;&amp;r.stopOnFalse&amp;&amp;(l=s.length,t=!1)}r.memory||(t=!1),i=!1,a&amp;&amp;(s=t?[]:&quot;&quot;)},f={add:function(){return s&amp;&amp;(t&amp;&amp;!i&amp;&amp;(l=s.length-1,u.push(t)),function n(e){S.each(e,function(e,t){m(t)?r.unique&amp;&amp;f.has(t)||s.push(t):t&amp;&amp;t.length&amp;&amp;&quot;string&quot;!==w(t)&amp;&amp;n(t)})}(arguments),t&amp;&amp;!i&amp;&amp;c()),this},remove:function(){return S.each(arguments,function(e,t){var n;while(-1&lt;(n=S.inArray(t,s,n)))s.splice(n,1),n&lt;=l&amp;&amp;l--}),this},has:function(e){return e?-1&lt;S.inArray(e,s):0&lt;s.length},empty:function(){return s&amp;&amp;(s=[]),this},disable:function(){return a=u=[],s=t=&quot;&quot;,this},disabled:function(){return!s},lock:function(){return a=u=[],t||i||(s=t=&quot;&quot;),this},locked:function(){return!!a},fireWith:function(e,t){return a||(t=[e,(t=t||[]).slice?t.slice():t],u.push(t),i||c()),this},fire:function(){return f.fireWith(this,arguments),this},fired:function(){return!!o}};return f},S.extend({Deferred:function(e){var o=[[&quot;notify&quot;,&quot;progress&quot;,S.Callbacks(&quot;memory&quot;),S.Callbacks(&quot;memory&quot;),2],[&quot;resolve&quot;,&quot;done&quot;,S.Callbacks(&quot;once memory&quot;),S.Callbacks(&quot;once memory&quot;),0,&quot;resolved&quot;],[&quot;reject&quot;,&quot;fail&quot;,S.Callbacks(&quot;once memory&quot;),S.Callbacks(&quot;once memory&quot;),1,&quot;rejected&quot;]],i=&quot;pending&quot;,a={state:function(){return i},always:function(){return s.done(arguments).fail(arguments),this},&quot;catch&quot;:function(e){return a.then(null,e)},pipe:function(){var i=arguments;return S.Deferred(function(r){S.each(o,function(e,t){var n=m(i[t[4]])&amp;&amp;i[t[4]];s[t[1]](function(){var e=n&amp;&amp;n.apply(this,arguments);e&amp;&amp;m(e.promise)?e.promise().progress(r.notify).done(r.resolve).fail(r.reject):r[t[0]+&quot;With&quot;](this,n?[e]:arguments)})}),i=null}).promise()},then:function(t,n,r){var u=0;function l(i,o,a,s){return function(){var n=this,r=arguments,e=function(){var e,t;if(!(i&lt;u)){if((e=a.apply(n,r))===o.promise())throw new TypeError(&quot;Thenable self-resolution&quot;);t=e&amp;&amp;(&quot;object&quot;==typeof e||&quot;function&quot;==typeof e)&amp;&amp;e.then,m(t)?s?t.call(e,l(u,o,R,s),l(u,o,M,s)):(u++,t.call(e,l(u,o,R,s),l(u,o,M,s),l(u,o,R,o.notifyWith))):(a!==R&amp;&amp;(n=void 0,r=[e]),(s||o.resolveWith)(n,r))}},t=s?e:function(){try{e()}catch(e){S.Deferred.exceptionHook&amp;&amp;S.Deferred.exceptionHook(e,t.stackTrace),u&lt;=i+1&amp;&amp;(a!==M&amp;&amp;(n=void 0,r=[e]),o.rejectWith(n,r))}};i?t():(S.Deferred.getStackHook&amp;&amp;(t.stackTrace=S.Deferred.getStackHook()),C.setTimeout(t))}}return S.Deferred(function(e){o[0][3].add(l(0,e,m(r)?r:R,e.notifyWith)),o[1][3].add(l(0,e,m(t)?t:R)),o[2][3].add(l(0,e,m(n)?n:M))}).promise()},promise:function(e){return null!=e?S.extend(e,a):a}},s={};return S.each(o,function(e,t){var n=t[2],r=t[5];a[t[1]]=n.add,r&amp;&amp;n.add(function(){i=r},o[3-e][2].disable,o[3-e][3].disable,o[0][2].lock,o[0][3].lock),n.add(t[3].fire),s[t[0]]=function(){return s[t[0]+&quot;With&quot;](this===s?void 0:this,arguments),this},s[t[0]+&quot;With&quot;]=n.fireWith}),a.promise(s),e&amp;&amp;e.call(s,s),s},when:function(e){var n=arguments.length,t=n,r=Array(t),i=s.call(arguments),o=S.Deferred(),a=function(t){return function(e){r[t]=this,i[t]=1&lt;arguments.length?s.call(arguments):e,--n||o.resolveWith(r,i)}};if(n&lt;=1&amp;&amp;(I(e,o.done(a(t)).resolve,o.reject,!n),&quot;pending&quot;===o.state()||m(i[t]&amp;&amp;i[t].then)))return o.then();while(t--)I(i[t],a(t),o.reject);return o.promise()}});var W=/^(Eval|Internal|Range|Reference|Syntax|Type|URI)Error$/;S.Deferred.exceptionHook=function(e,t){C.console&amp;&amp;C.console.warn&amp;&amp;e&amp;&amp;W.test(e.name)&amp;&amp;C.console.warn(&quot;jQuery.Deferred exception: &quot;+e.message,e.stack,t)},S.readyException=function(e){C.setTimeout(function(){throw e})};var F=S.Deferred();function B(){E.removeEventListener(&quot;DOMContentLoaded&quot;,B),C.removeEventListener(&quot;load&quot;,B),S.ready()}S.fn.ready=function(e){return F.then(e)[&quot;catch&quot;](function(e){S.readyException(e)}),this},S.extend({isReady:!1,readyWait:1,ready:function(e){(!0===e?--S.readyWait:S.isReady)||(S.isReady=!0)!==e&amp;&amp;0&lt;--S.readyWait||F.resolveWith(E,[S])}}),S.ready.then=F.then,&quot;complete&quot;===E.readyState||&quot;loading&quot;!==E.readyState&amp;&amp;!E.documentElement.doScroll?C.setTimeout(S.ready):(E.addEventListener(&quot;DOMContentLoaded&quot;,B),C.addEventListener(&quot;load&quot;,B));var $=function(e,t,n,r,i,o,a){var s=0,u=e.length,l=null==n;if(&quot;object&quot;===w(n))for(s in i=!0,n)$(e,t,s,n[s],!0,o,a);else if(void 0!==r&amp;&amp;(i=!0,m(r)||(a=!0),l&amp;&amp;(a?(t.call(e,r),t=null):(l=t,t=function(e,t,n){return l.call(S(e),n)})),t))for(;s&lt;u;s++)t(e[s],n,a?r:r.call(e[s],s,t(e[s],n)));return i?e:l?t.call(e):u?t(e[0],n):o},_=/^-ms-/,z=/-([a-z])/g;function U(e,t){return t.toUpperCase()}function X(e){return e.replace(_,&quot;ms-&quot;).replace(z,U)}var V=function(e){return 1===e.nodeType||9===e.nodeType||!+e.nodeType};function G(){this.expando=S.expando+G.uid++}G.uid=1,G.prototype={cache:function(e){var t=e[this.expando];return t||(t={},V(e)&amp;&amp;(e.nodeType?e[this.expando]=t:Object.defineProperty(e,this.expando,{value:t,configurable:!0}))),t},set:function(e,t,n){var r,i=this.cache(e);if(&quot;string&quot;==typeof t)i[X(t)]=n;else for(r in t)i[X(r)]=t[r];return i},get:function(e,t){return void 0===t?this.cache(e):e[this.expando]&amp;&amp;e[this.expando][X(t)]},access:function(e,t,n){return void 0===t||t&amp;&amp;&quot;string&quot;==typeof t&amp;&amp;void 0===n?this.get(e,t):(this.set(e,t,n),void 0!==n?n:t)},remove:function(e,t){var n,r=e[this.expando];if(void 0!==r){if(void 0!==t){n=(t=Array.isArray(t)?t.map(X):(t=X(t))in r?[t]:t.match(P)||[]).length;while(n--)delete r[t[n]]}(void 0===t||S.isEmptyObject(r))&amp;&amp;(e.nodeType?e[this.expando]=void 0:delete e[this.expando])}},hasData:function(e){var t=e[this.expando];return void 0!==t&amp;&amp;!S.isEmptyObject(t)}};var Y=new G,Q=new G,J=/^(?:\{[\w\W]*\}|\[[\w\W]*\])$/,K=/[A-Z]/g;function Z(e,t,n){var r,i;if(void 0===n&amp;&amp;1===e.nodeType)if(r=&quot;data-&quot;+t.replace(K,&quot;-$&amp;&quot;).toLowerCase(),&quot;string&quot;==typeof(n=e.getAttribute(r))){try{n=&quot;true&quot;===(i=n)||&quot;false&quot;!==i&amp;&amp;(&quot;null&quot;===i?null:i===+i+&quot;&quot;?+i:J.test(i)?JSON.parse(i):i)}catch(e){}Q.set(e,t,n)}else n=void 0;return n}S.extend({hasData:function(e){return Q.hasData(e)||Y.hasData(e)},data:function(e,t,n){return Q.access(e,t,n)},removeData:function(e,t){Q.remove(e,t)},_data:function(e,t,n){return Y.access(e,t,n)},_removeData:function(e,t){Y.remove(e,t)}}),S.fn.extend({data:function(n,e){var t,r,i,o=this[0],a=o&amp;&amp;o.attributes;if(void 0===n){if(this.length&amp;&amp;(i=Q.get(o),1===o.nodeType&amp;&amp;!Y.get(o,&quot;hasDataAttrs&quot;))){t=a.length;while(t--)a[t]&amp;&amp;0===(r=a[t].name).indexOf(&quot;data-&quot;)&amp;&amp;(r=X(r.slice(5)),Z(o,r,i[r]));Y.set(o,&quot;hasDataAttrs&quot;,!0)}return i}return&quot;object&quot;==typeof n?this.each(function(){Q.set(this,n)}):$(this,function(e){var t;if(o&amp;&amp;void 0===e)return void 0!==(t=Q.get(o,n))?t:void 0!==(t=Z(o,n))?t:void 0;this.each(function(){Q.set(this,n,e)})},null,e,1&lt;arguments.length,null,!0)},removeData:function(e){return this.each(function(){Q.remove(this,e)})}}),S.extend({queue:function(e,t,n){var r;if(e)return t=(t||&quot;fx&quot;)+&quot;queue&quot;,r=Y.get(e,t),n&amp;&amp;(!r||Array.isArray(n)?r=Y.access(e,t,S.makeArray(n)):r.push(n)),r||[]},dequeue:function(e,t){t=t||&quot;fx&quot;;var n=S.queue(e,t),r=n.length,i=n.shift(),o=S._queueHooks(e,t);&quot;inprogress&quot;===i&amp;&amp;(i=n.shift(),r--),i&amp;&amp;(&quot;fx&quot;===t&amp;&amp;n.unshift(&quot;inprogress&quot;),delete o.stop,i.call(e,function(){S.dequeue(e,t)},o)),!r&amp;&amp;o&amp;&amp;o.empty.fire()},_queueHooks:function(e,t){var n=t+&quot;queueHooks&quot;;return Y.get(e,n)||Y.access(e,n,{empty:S.Callbacks(&quot;once memory&quot;).add(function(){Y.remove(e,[t+&quot;queue&quot;,n])})})}}),S.fn.extend({queue:function(t,n){var e=2;return&quot;string&quot;!=typeof t&amp;&amp;(n=t,t=&quot;fx&quot;,e--),arguments.length&lt;e?S.queue(this[0],t):void 0===n?this:this.each(function(){var e=S.queue(this,t,n);S._queueHooks(this,t),&quot;fx&quot;===t&amp;&amp;&quot;inprogress&quot;!==e[0]&amp;&amp;S.dequeue(this,t)})},dequeue:function(e){return this.each(function(){S.dequeue(this,e)})},clearQueue:function(e){return this.queue(e||&quot;fx&quot;,[])},promise:function(e,t){var n,r=1,i=S.Deferred(),o=this,a=this.length,s=function(){--r||i.resolveWith(o,[o])};&quot;string&quot;!=typeof e&amp;&amp;(t=e,e=void 0),e=e||&quot;fx&quot;;while(a--)(n=Y.get(o[a],e+&quot;queueHooks&quot;))&amp;&amp;n.empty&amp;&amp;(r++,n.empty.add(s));return s(),i.promise(t)}});var ee=/[+-]?(?:\d*\.|)\d+(?:[eE][+-]?\d+|)/.source,te=new RegExp(&quot;^(?:([+-])=|)(&quot;+ee+&quot;)([a-z%]*)$&quot;,&quot;i&quot;),ne=[&quot;Top&quot;,&quot;Right&quot;,&quot;Bottom&quot;,&quot;Left&quot;],re=E.documentElement,ie=function(e){return S.contains(e.ownerDocument,e)},oe={composed:!0};re.getRootNode&amp;&amp;(ie=function(e){return S.contains(e.ownerDocument,e)||e.getRootNode(oe)===e.ownerDocument});var ae=function(e,t){return&quot;none&quot;===(e=t||e).style.display||&quot;&quot;===e.style.display&amp;&amp;ie(e)&amp;&amp;&quot;none&quot;===S.css(e,&quot;display&quot;)};function se(e,t,n,r){var i,o,a=20,s=r?function(){return r.cur()}:function(){return S.css(e,t,&quot;&quot;)},u=s(),l=n&amp;&amp;n[3]||(S.cssNumber[t]?&quot;&quot;:&quot;px&quot;),c=e.nodeType&amp;&amp;(S.cssNumber[t]||&quot;px&quot;!==l&amp;&amp;+u)&amp;&amp;te.exec(S.css(e,t));if(c&amp;&amp;c[3]!==l){u/=2,l=l||c[3],c=+u||1;while(a--)S.style(e,t,c+l),(1-o)*(1-(o=s()/u||.5))&lt;=0&amp;&amp;(a=0),c/=o;c*=2,S.style(e,t,c+l),n=n||[]}return n&amp;&amp;(c=+c||+u||0,i=n[1]?c+(n[1]+1)*n[2]:+n[2],r&amp;&amp;(r.unit=l,r.start=c,r.end=i)),i}var ue={};function le(e,t){for(var n,r,i,o,a,s,u,l=[],c=0,f=e.length;c&lt;f;c++)(r=e[c]).style&amp;&amp;(n=r.style.display,t?(&quot;none&quot;===n&amp;&amp;(l[c]=Y.get(r,&quot;display&quot;)||null,l[c]||(r.style.display=&quot;&quot;)),&quot;&quot;===r.style.display&amp;&amp;ae(r)&amp;&amp;(l[c]=(u=a=o=void 0,a=(i=r).ownerDocument,s=i.nodeName,(u=ue[s])||(o=a.body.appendChild(a.createElement(s)),u=S.css(o,&quot;display&quot;),o.parentNode.removeChild(o),&quot;none&quot;===u&amp;&amp;(u=&quot;block&quot;),ue[s]=u)))):&quot;none&quot;!==n&amp;&amp;(l[c]=&quot;none&quot;,Y.set(r,&quot;display&quot;,n)));for(c=0;c&lt;f;c++)null!=l[c]&amp;&amp;(e[c].style.display=l[c]);return e}S.fn.extend({show:function(){return le(this,!0)},hide:function(){return le(this)},toggle:function(e){return&quot;boolean&quot;==typeof e?e?this.show():this.hide():this.each(function(){ae(this)?S(this).show():S(this).hide()})}});var ce,fe,pe=/^(?:checkbox|radio)$/i,de=/&lt;([a-z][^\/\0>\x20\t\r\n\f]*)/i,he=/^$|^module$|\/(?:java|ecma)script/i;ce=E.createDocumentFragment().appendChild(E.createElement(&quot;div&quot;)),(fe=E.createElement(&quot;input&quot;)).setAttribute(&quot;type&quot;,&quot;radio&quot;),fe.setAttribute(&quot;checked&quot;,&quot;checked&quot;),fe.setAttribute(&quot;name&quot;,&quot;t&quot;),ce.appendChild(fe),y.checkClone=ce.cloneNode(!0).cloneNode(!0).lastChild.checked,ce.innerHTML=&quot;&lt;textarea>x&lt;/textarea>&quot;,y.noCloneChecked=!!ce.cloneNode(!0).lastChild.defaultValue,ce.innerHTML=&quot;&lt;option>&lt;/option>&quot;,y.option=!!ce.lastChild;var ge={thead:[1,&quot;&lt;table>&quot;,&quot;&lt;/table>&quot;],col:[2,&quot;&lt;table>&lt;colgroup>&quot;,&quot;&lt;/colgroup>&lt;/table>&quot;],tr:[2,&quot;&lt;table>&lt;tbody>&quot;,&quot;&lt;/tbody>&lt;/table>&quot;],td:[3,&quot;&lt;table>&lt;tbody>&lt;tr>&quot;,&quot;&lt;/tr>&lt;/tbody>&lt;/table>&quot;],_default:[0,&quot;&quot;,&quot;&quot;]};function ve(e,t){var n;return n=&quot;undefined&quot;!=typeof e.getElementsByTagName?e.getElementsByTagName(t||&quot;*&quot;):&quot;undefined&quot;!=typeof e.querySelectorAll?e.querySelectorAll(t||&quot;*&quot;):[],void 0===t||t&amp;&amp;A(e,t)?S.merge([e],n):n}function ye(e,t){for(var n=0,r=e.length;n&lt;r;n++)Y.set(e[n],&quot;globalEval&quot;,!t||Y.get(t[n],&quot;globalEval&quot;))}ge.tbody=ge.tfoot=ge.colgroup=ge.caption=ge.thead,ge.th=ge.td,y.option||(ge.optgroup=ge.option=[1,&quot;&lt;select multiple='multiple'>&quot;,&quot;&lt;/select>&quot;]);var me=/&lt;|&amp;#?\w+;/;function xe(e,t,n,r,i){for(var o,a,s,u,l,c,f=t.createDocumentFragment(),p=[],d=0,h=e.length;d&lt;h;d++)if((o=e[d])||0===o)if(&quot;object&quot;===w(o))S.merge(p,o.nodeType?[o]:o);else if(me.test(o)){a=a||f.appendChild(t.createElement(&quot;div&quot;)),s=(de.exec(o)||[&quot;&quot;,&quot;&quot;])[1].toLowerCase(),u=ge[s]||ge._default,a.innerHTML=u[1]+S.htmlPrefilter(o)+u[2],c=u[0];while(c--)a=a.lastChild;S.merge(p,a.childNodes),(a=f.firstChild).textContent=&quot;&quot;}else p.push(t.createTextNode(o));f.textContent=&quot;&quot;,d=0;while(o=p[d++])if(r&amp;&amp;-1&lt;S.inArray(o,r))i&amp;&amp;i.push(o);else if(l=ie(o),a=ve(f.appendChild(o),&quot;script&quot;),l&amp;&amp;ye(a),n){c=0;while(o=a[c++])he.test(o.type||&quot;&quot;)&amp;&amp;n.push(o)}return f}var be=/^([^.]*)(?:\.(.+)|)/;function we(){return!0}function Te(){return!1}function Ce(e,t){return e===function(){try{return E.activeElement}catch(e){}}()==(&quot;focus&quot;===t)}function Ee(e,t,n,r,i,o){var a,s;if(&quot;object&quot;==typeof t){for(s in&quot;string&quot;!=typeof n&amp;&amp;(r=r||n,n=void 0),t)Ee(e,s,n,r,t[s],o);return e}if(null==r&amp;&amp;null==i?(i=n,r=n=void 0):null==i&amp;&amp;(&quot;string&quot;==typeof n?(i=r,r=void 0):(i=r,r=n,n=void 0)),!1===i)i=Te;else if(!i)return e;return 1===o&amp;&amp;(a=i,(i=function(e){return S().off(e),a.apply(this,arguments)}).guid=a.guid||(a.guid=S.guid++)),e.each(function(){S.event.add(this,t,i,r,n)})}function Se(e,i,o){o?(Y.set(e,i,!1),S.event.add(e,i,{namespace:!1,handler:function(e){var t,n,r=Y.get(this,i);if(1&amp;e.isTrigger&amp;&amp;this[i]){if(r.length)(S.event.special[i]||{}).delegateType&amp;&amp;e.stopPropagation();else if(r=s.call(arguments),Y.set(this,i,r),t=o(this,i),this[i](),r!==(n=Y.get(this,i))||t?Y.set(this,i,!1):n={},r!==n)return e.stopImmediatePropagation(),e.preventDefault(),n&amp;&amp;n.value}else r.length&amp;&amp;(Y.set(this,i,{value:S.event.trigger(S.extend(r[0],S.Event.prototype),r.slice(1),this)}),e.stopImmediatePropagation())}})):void 0===Y.get(e,i)&amp;&amp;S.event.add(e,i,we)}S.event={global:{},add:function(t,e,n,r,i){var o,a,s,u,l,c,f,p,d,h,g,v=Y.get(t);if(V(t)){n.handler&amp;&amp;(n=(o=n).handler,i=o.selector),i&amp;&amp;S.find.matchesSelector(re,i),n.guid||(n.guid=S.guid++),(u=v.events)||(u=v.events=Object.create(null)),(a=v.handle)||(a=v.handle=function(e){return&quot;undefined&quot;!=typeof S&amp;&amp;S.event.triggered!==e.type?S.event.dispatch.apply(t,arguments):void 0}),l=(e=(e||&quot;&quot;).match(P)||[&quot;&quot;]).length;while(l--)d=g=(s=be.exec(e[l])||[])[1],h=(s[2]||&quot;&quot;).split(&quot;.&quot;).sort(),d&amp;&amp;(f=S.event.special[d]||{},d=(i?f.delegateType:f.bindType)||d,f=S.event.special[d]||{},c=S.extend({type:d,origType:g,data:r,handler:n,guid:n.guid,selector:i,needsContext:i&amp;&amp;S.expr.match.needsContext.test(i),namespace:h.join(&quot;.&quot;)},o),(p=u[d])||((p=u[d]=[]).delegateCount=0,f.setup&amp;&amp;!1!==f.setup.call(t,r,h,a)||t.addEventListener&amp;&amp;t.addEventListener(d,a)),f.add&amp;&amp;(f.add.call(t,c),c.handler.guid||(c.handler.guid=n.guid)),i?p.splice(p.delegateCount++,0,c):p.push(c),S.event.global[d]=!0)}},remove:function(e,t,n,r,i){var o,a,s,u,l,c,f,p,d,h,g,v=Y.hasData(e)&amp;&amp;Y.get(e);if(v&amp;&amp;(u=v.events)){l=(t=(t||&quot;&quot;).match(P)||[&quot;&quot;]).length;while(l--)if(d=g=(s=be.exec(t[l])||[])[1],h=(s[2]||&quot;&quot;).split(&quot;.&quot;).sort(),d){f=S.event.special[d]||{},p=u[d=(r?f.delegateType:f.bindType)||d]||[],s=s[2]&amp;&amp;new RegExp(&quot;(^|\\.)&quot;+h.join(&quot;\\.(?:.*\\.|)&quot;)+&quot;(\\.|$)&quot;),a=o=p.length;while(o--)c=p[o],!i&amp;&amp;g!==c.origType||n&amp;&amp;n.guid!==c.guid||s&amp;&amp;!s.test(c.namespace)||r&amp;&amp;r!==c.selector&amp;&amp;(&quot;**&quot;!==r||!c.selector)||(p.splice(o,1),c.selector&amp;&amp;p.delegateCount--,f.remove&amp;&amp;f.remove.call(e,c));a&amp;&amp;!p.length&amp;&amp;(f.teardown&amp;&amp;!1!==f.teardown.call(e,h,v.handle)||S.removeEvent(e,d,v.handle),delete u[d])}else for(d in u)S.event.remove(e,d+t[l],n,r,!0);S.isEmptyObject(u)&amp;&amp;Y.remove(e,&quot;handle events&quot;)}},dispatch:function(e){var t,n,r,i,o,a,s=new Array(arguments.length),u=S.event.fix(e),l=(Y.get(this,&quot;events&quot;)||Object.create(null))[u.type]||[],c=S.event.special[u.type]||{};for(s[0]=u,t=1;t&lt;arguments.length;t++)s[t]=arguments[t];if(u.delegateTarget=this,!c.preDispatch||!1!==c.preDispatch.call(this,u)){a=S.event.handlers.call(this,u,l),t=0;while((i=a[t++])&amp;&amp;!u.isPropagationStopped()){u.currentTarget=i.elem,n=0;while((o=i.handlers[n++])&amp;&amp;!u.isImmediatePropagationStopped())u.rnamespace&amp;&amp;!1!==o.namespace&amp;&amp;!u.rnamespace.test(o.namespace)||(u.handleObj=o,u.data=o.data,void 0!==(r=((S.event.special[o.origType]||{}).handle||o.handler).apply(i.elem,s))&amp;&amp;!1===(u.result=r)&amp;&amp;(u.preventDefault(),u.stopPropagation()))}return c.postDispatch&amp;&amp;c.postDispatch.call(this,u),u.result}},handlers:function(e,t){var n,r,i,o,a,s=[],u=t.delegateCount,l=e.target;if(u&amp;&amp;l.nodeType&amp;&amp;!(&quot;click&quot;===e.type&amp;&amp;1&lt;=e.button))for(;l!==this;l=l.parentNode||this)if(1===l.nodeType&amp;&amp;(&quot;click&quot;!==e.type||!0!==l.disabled)){for(o=[],a={},n=0;n&lt;u;n++)void 0===a[i=(r=t[n]).selector+&quot; &quot;]&amp;&amp;(a[i]=r.needsContext?-1&lt;S(i,this).index(l):S.find(i,this,null,[l]).length),a[i]&amp;&amp;o.push(r);o.length&amp;&amp;s.push({elem:l,handlers:o})}return l=this,u&lt;t.length&amp;&amp;s.push({elem:l,handlers:t.slice(u)}),s},addProp:function(t,e){Object.defineProperty(S.Event.prototype,t,{enumerable:!0,configurable:!0,get:m(e)?function(){if(this.originalEvent)return e(this.originalEvent)}:function(){if(this.originalEvent)return this.originalEvent[t]},set:function(e){Object.defineProperty(this,t,{enumerable:!0,configurable:!0,writable:!0,value:e})}})},fix:function(e){return e[S.expando]?e:new S.Event(e)},special:{load:{noBubble:!0},click:{setup:function(e){var t=this||e;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Se(t,&quot;click&quot;,we),!1},trigger:function(e){var t=this||e;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Se(t,&quot;click&quot;),!0},_default:function(e){var t=e.target;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Y.get(t,&quot;click&quot;)||A(t,&quot;a&quot;)}},beforeunload:{postDispatch:function(e){void 0!==e.result&amp;&amp;e.originalEvent&amp;&amp;(e.originalEvent.returnValue=e.result)}}}},S.removeEvent=function(e,t,n){e.removeEventListener&amp;&amp;e.removeEventListener(t,n)},S.Event=function(e,t){if(!(this instanceof S.Event))return new S.Event(e,t);e&amp;&amp;e.type?(this.originalEvent=e,this.type=e.type,this.isDefaultPrevented=e.defaultPrevented||void 0===e.defaultPrevented&amp;&amp;!1===e.returnValue?we:Te,this.target=e.target&amp;&amp;3===e.target.nodeType?e.target.parentNode:e.target,this.currentTarget=e.currentTarget,this.relatedTarget=e.relatedTarget):this.type=e,t&amp;&amp;S.extend(this,t),this.timeStamp=e&amp;&amp;e.timeStamp||Date.now(),this[S.expando]=!0},S.Event.prototype={constructor:S.Event,isDefaultPrevented:Te,isPropagationStopped:Te,isImmediatePropagationStopped:Te,isSimulated:!1,preventDefault:function(){var e=this.originalEvent;this.isDefaultPrevented=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.preventDefault()},stopPropagation:function(){var e=this.originalEvent;this.isPropagationStopped=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.stopPropagation()},stopImmediatePropagation:function(){var e=this.originalEvent;this.isImmediatePropagationStopped=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.stopImmediatePropagation(),this.stopPropagation()}},S.each({altKey:!0,bubbles:!0,cancelable:!0,changedTouches:!0,ctrlKey:!0,detail:!0,eventPhase:!0,metaKey:!0,pageX:!0,pageY:!0,shiftKey:!0,view:!0,&quot;char&quot;:!0,code:!0,charCode:!0,key:!0,keyCode:!0,button:!0,buttons:!0,clientX:!0,clientY:!0,offsetX:!0,offsetY:!0,pointerId:!0,pointerType:!0,screenX:!0,screenY:!0,targetTouches:!0,toElement:!0,touches:!0,which:!0},S.event.addProp),S.each({focus:&quot;focusin&quot;,blur:&quot;focusout&quot;},function(e,t){S.event.special[e]={setup:function(){return Se(this,e,Ce),!1},trigger:function(){return Se(this,e),!0},_default:function(){return!0},delegateType:t}}),S.each({mouseenter:&quot;mouseover&quot;,mouseleave:&quot;mouseout&quot;,pointerenter:&quot;pointerover&quot;,pointerleave:&quot;pointerout&quot;},function(e,i){S.event.special[e]={delegateType:i,bindType:i,handle:function(e){var t,n=e.relatedTarget,r=e.handleObj;return n&amp;&amp;(n===this||S.contains(this,n))||(e.type=r.origType,t=r.handler.apply(this,arguments),e.type=i),t}}}),S.fn.extend({on:function(e,t,n,r){return Ee(this,e,t,n,r)},one:function(e,t,n,r){return Ee(this,e,t,n,r,1)},off:function(e,t,n){var r,i;if(e&amp;&amp;e.preventDefault&amp;&amp;e.handleObj)return r=e.handleObj,S(e.delegateTarget).off(r.namespace?r.origType+&quot;.&quot;+r.namespace:r.origType,r.selector,r.handler),this;if(&quot;object&quot;==typeof e){for(i in e)this.off(i,t,e[i]);return this}return!1!==t&amp;&amp;&quot;function&quot;!=typeof t||(n=t,t=void 0),!1===n&amp;&amp;(n=Te),this.each(function(){S.event.remove(this,e,n,t)})}});var ke=/&lt;script|&lt;style|&lt;link/i,Ae=/checked\s*(?:[^=]|=\s*.checked.)/i,Ne=/^\s*&lt;!(?:\[CDATA\[|--)|(?:\]\]|--)>\s*$/g;function je(e,t){return A(e,&quot;table&quot;)&amp;&amp;A(11!==t.nodeType?t:t.firstChild,&quot;tr&quot;)&amp;&amp;S(e).children(&quot;tbody&quot;)[0]||e}function De(e){return e.type=(null!==e.getAttribute(&quot;type&quot;))+&quot;/&quot;+e.type,e}function qe(e){return&quot;true/&quot;===(e.type||&quot;&quot;).slice(0,5)?e.type=e.type.slice(5):e.removeAttribute(&quot;type&quot;),e}function Le(e,t){var n,r,i,o,a,s;if(1===t.nodeType){if(Y.hasData(e)&amp;&amp;(s=Y.get(e).events))for(i in Y.remove(t,&quot;handle events&quot;),s)for(n=0,r=s[i].length;n&lt;r;n++)S.event.add(t,i,s[i][n]);Q.hasData(e)&amp;&amp;(o=Q.access(e),a=S.extend({},o),Q.set(t,a))}}function He(n,r,i,o){r=g(r);var e,t,a,s,u,l,c=0,f=n.length,p=f-1,d=r[0],h=m(d);if(h||1&lt;f&amp;&amp;&quot;string&quot;==typeof d&amp;&amp;!y.checkClone&amp;&amp;Ae.test(d))return n.each(function(e){var t=n.eq(e);h&amp;&amp;(r[0]=d.call(this,e,t.html())),He(t,r,i,o)});if(f&amp;&amp;(t=(e=xe(r,n[0].ownerDocument,!1,n,o)).firstChild,1===e.childNodes.length&amp;&amp;(e=t),t||o)){for(s=(a=S.map(ve(e,&quot;script&quot;),De)).length;c&lt;f;c++)u=e,c!==p&amp;&amp;(u=S.clone(u,!0,!0),s&amp;&amp;S.merge(a,ve(u,&quot;script&quot;))),i.call(n[c],u,c);if(s)for(l=a[a.length-1].ownerDocument,S.map(a,qe),c=0;c&lt;s;c++)u=a[c],he.test(u.type||&quot;&quot;)&amp;&amp;!Y.access(u,&quot;globalEval&quot;)&amp;&amp;S.contains(l,u)&amp;&amp;(u.src&amp;&amp;&quot;module&quot;!==(u.type||&quot;&quot;).toLowerCase()?S._evalUrl&amp;&amp;!u.noModule&amp;&amp;S._evalUrl(u.src,{nonce:u.nonce||u.getAttribute(&quot;nonce&quot;)},l):b(u.textContent.replace(Ne,&quot;&quot;),u,l))}return n}function Oe(e,t,n){for(var r,i=t?S.filter(t,e):e,o=0;null!=(r=i[o]);o++)n||1!==r.nodeType||S.cleanData(ve(r)),r.parentNode&amp;&amp;(n&amp;&amp;ie(r)&amp;&amp;ye(ve(r,&quot;script&quot;)),r.parentNode.removeChild(r));return e}S.extend({htmlPrefilter:function(e){return e},clone:function(e,t,n){var r,i,o,a,s,u,l,c=e.cloneNode(!0),f=ie(e);if(!(y.noCloneChecked||1!==e.nodeType&amp;&amp;11!==e.nodeType||S.isXMLDoc(e)))for(a=ve(c),r=0,i=(o=ve(e)).length;r&lt;i;r++)s=o[r],u=a[r],void 0,&quot;input&quot;===(l=u.nodeName.toLowerCase())&amp;&amp;pe.test(s.type)?u.checked=s.checked:&quot;input&quot;!==l&amp;&amp;&quot;textarea&quot;!==l||(u.defaultValue=s.defaultValue);if(t)if(n)for(o=o||ve(e),a=a||ve(c),r=0,i=o.length;r&lt;i;r++)Le(o[r],a[r]);else Le(e,c);return 0&lt;(a=ve(c,&quot;script&quot;)).length&amp;&amp;ye(a,!f&amp;&amp;ve(e,&quot;script&quot;)),c},cleanData:function(e){for(var t,n,r,i=S.event.special,o=0;void 0!==(n=e[o]);o++)if(V(n)){if(t=n[Y.expando]){if(t.events)for(r in t.events)i[r]?S.event.remove(n,r):S.removeEvent(n,r,t.handle);n[Y.expando]=void 0}n[Q.expando]&amp;&amp;(n[Q.expando]=void 0)}}}),S.fn.extend({detach:function(e){return Oe(this,e,!0)},remove:function(e){return Oe(this,e)},text:function(e){return $(this,function(e){return void 0===e?S.text(this):this.empty().each(function(){1!==this.nodeType&amp;&amp;11!==this.nodeType&amp;&amp;9!==this.nodeType||(this.textContent=e)})},null,e,arguments.length)},append:function(){return He(this,arguments,function(e){1!==this.nodeType&amp;&amp;11!==this.nodeType&amp;&amp;9!==this.nodeType||je(this,e).appendChild(e)})},prepend:function(){return He(this,arguments,function(e){if(1===this.nodeType||11===this.nodeType||9===this.nodeType){var t=je(this,e);t.insertBefore(e,t.firstChild)}})},before:function(){return He(this,arguments,function(e){this.parentNode&amp;&amp;this.parentNode.insertBefore(e,this)})},after:function(){return He(this,arguments,function(e){this.parentNode&amp;&amp;this.parentNode.insertBefore(e,this.nextSibling)})},empty:function(){for(var e,t=0;null!=(e=this[t]);t++)1===e.nodeType&amp;&amp;(S.cleanData(ve(e,!1)),e.textContent=&quot;&quot;);return this},clone:function(e,t){return e=null!=e&amp;&amp;e,t=null==t?e:t,this.map(function(){return S.clone(this,e,t)})},html:function(e){return $(this,function(e){var t=this[0]||{},n=0,r=this.length;if(void 0===e&amp;&amp;1===t.nodeType)return t.innerHTML;if(&quot;string&quot;==typeof e&amp;&amp;!ke.test(e)&amp;&amp;!ge[(de.exec(e)||[&quot;&quot;,&quot;&quot;])[1].toLowerCase()]){e=S.htmlPrefilter(e);try{for(;n&lt;r;n++)1===(t=this[n]||{}).nodeType&amp;&amp;(S.cleanData(ve(t,!1)),t.innerHTML=e);t=0}catch(e){}}t&amp;&amp;this.empty().append(e)},null,e,arguments.length)},replaceWith:function(){var n=[];return He(this,arguments,function(e){var t=this.parentNode;S.inArray(this,n)&lt;0&amp;&amp;(S.cleanData(ve(this)),t&amp;&amp;t.replaceChild(e,this))},n)}}),S.each({appendTo:&quot;append&quot;,prependTo:&quot;prepend&quot;,insertBefore:&quot;before&quot;,insertAfter:&quot;after&quot;,replaceAll:&quot;replaceWith&quot;},function(e,a){S.fn[e]=function(e){for(var t,n=[],r=S(e),i=r.length-1,o=0;o&lt;=i;o++)t=o===i?this:this.clone(!0),S(r[o])[a](t),u.apply(n,t.get());return this.pushStack(n)}});var Pe=new RegExp(&quot;^(&quot;+ee+&quot;)(?!px)[a-z%]+$&quot;,&quot;i&quot;),Re=function(e){var t=e.ownerDocument.defaultView;return t&amp;&amp;t.opener||(t=C),t.getComputedStyle(e)},Me=function(e,t,n){var r,i,o={};for(i in t)o[i]=e.style[i],e.style[i]=t[i];for(i in r=n.call(e),t)e.style[i]=o[i];return r},Ie=new RegExp(ne.join(&quot;|&quot;),&quot;i&quot;);function We(e,t,n){var r,i,o,a,s=e.style;return(n=n||Re(e))&amp;&amp;(&quot;&quot;!==(a=n.getPropertyValue(t)||n[t])||ie(e)||(a=S.style(e,t)),!y.pixelBoxStyles()&amp;&amp;Pe.test(a)&amp;&amp;Ie.test(t)&amp;&amp;(r=s.width,i=s.minWidth,o=s.maxWidth,s.minWidth=s.maxWidth=s.width=a,a=n.width,s.width=r,s.minWidth=i,s.maxWidth=o)),void 0!==a?a+&quot;&quot;:a}function Fe(e,t){return{get:function(){if(!e())return(this.get=t).apply(this,arguments);delete this.get}}}!function(){function e(){if(l){u.style.cssText=&quot;position:absolute;left:-11111px;width:60px;margin-top:1px;padding:0;border:0&quot;,l.style.cssText=&quot;position:relative;display:block;box-sizing:border-box;overflow:scroll;margin:auto;border:1px;padding:1px;width:60%;top:1%&quot;,re.appendChild(u).appendChild(l);var e=C.getComputedStyle(l);n=&quot;1%&quot;!==e.top,s=12===t(e.marginLeft),l.style.right=&quot;60%&quot;,o=36===t(e.right),r=36===t(e.width),l.style.position=&quot;absolute&quot;,i=12===t(l.offsetWidth/3),re.removeChild(u),l=null}}function t(e){return Math.round(parseFloat(e))}var n,r,i,o,a,s,u=E.createElement(&quot;div&quot;),l=E.createElement(&quot;div&quot;);l.style&amp;&amp;(l.style.backgroundClip=&quot;content-box&quot;,l.cloneNode(!0).style.backgroundClip=&quot;&quot;,y.clearCloneStyle=&quot;content-box&quot;===l.style.backgroundClip,S.extend(y,{boxSizingReliable:function(){return e(),r},pixelBoxStyles:function(){return e(),o},pixelPosition:function(){return e(),n},reliableMarginLeft:function(){return e(),s},scrollboxSize:function(){return e(),i},reliableTrDimensions:function(){var e,t,n,r;return null==a&amp;&amp;(e=E.createElement(&quot;table&quot;),t=E.createElement(&quot;tr&quot;),n=E.createElement(&quot;div&quot;),e.style.cssText=&quot;position:absolute;left:-11111px;border-collapse:separate&quot;,t.style.cssText=&quot;border:1px solid&quot;,t.style.height=&quot;1px&quot;,n.style.height=&quot;9px&quot;,n.style.display=&quot;block&quot;,re.appendChild(e).appendChild(t).appendChild(n),r=C.getComputedStyle(t),a=parseInt(r.height,10)+parseInt(r.borderTopWidth,10)+parseInt(r.borderBottomWidth,10)===t.offsetHeight,re.removeChild(e)),a}}))}();var Be=[&quot;Webkit&quot;,&quot;Moz&quot;,&quot;ms&quot;],$e=E.createElement(&quot;div&quot;).style,_e={};function ze(e){var t=S.cssProps[e]||_e[e];return t||(e in $e?e:_e[e]=function(e){var t=e[0].toUpperCase()+e.slice(1),n=Be.length;while(n--)if((e=Be[n]+t)in $e)return e}(e)||e)}var Ue=/^(none|table(?!-c[ea]).+)/,Xe=/^--/,Ve={position:&quot;absolute&quot;,visibility:&quot;hidden&quot;,display:&quot;block&quot;},Ge={letterSpacing:&quot;0&quot;,fontWeight:&quot;400&quot;};function Ye(e,t,n){var r=te.exec(t);return r?Math.max(0,r[2]-(n||0))+(r[3]||&quot;px&quot;):t}function Qe(e,t,n,r,i,o){var a=&quot;width&quot;===t?1:0,s=0,u=0;if(n===(r?&quot;border&quot;:&quot;content&quot;))return 0;for(;a&lt;4;a+=2)&quot;margin&quot;===n&amp;&amp;(u+=S.css(e,n+ne[a],!0,i)),r?(&quot;content&quot;===n&amp;&amp;(u-=S.css(e,&quot;padding&quot;+ne[a],!0,i)),&quot;margin&quot;!==n&amp;&amp;(u-=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i))):(u+=S.css(e,&quot;padding&quot;+ne[a],!0,i),&quot;padding&quot;!==n?u+=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i):s+=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i));return!r&amp;&amp;0&lt;=o&amp;&amp;(u+=Math.max(0,Math.ceil(e[&quot;offset&quot;+t[0].toUpperCase()+t.slice(1)]-o-u-s-.5))||0),u}function Je(e,t,n){var r=Re(e),i=(!y.boxSizingReliable()||n)&amp;&amp;&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,r),o=i,a=We(e,t,r),s=&quot;offset&quot;+t[0].toUpperCase()+t.slice(1);if(Pe.test(a)){if(!n)return a;a=&quot;auto&quot;}return(!y.boxSizingReliable()&amp;&amp;i||!y.reliableTrDimensions()&amp;&amp;A(e,&quot;tr&quot;)||&quot;auto&quot;===a||!parseFloat(a)&amp;&amp;&quot;inline&quot;===S.css(e,&quot;display&quot;,!1,r))&amp;&amp;e.getClientRects().length&amp;&amp;(i=&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,r),(o=s in e)&amp;&amp;(a=e[s])),(a=parseFloat(a)||0)+Qe(e,t,n||(i?&quot;border&quot;:&quot;content&quot;),o,r,a)+&quot;px&quot;}function Ke(e,t,n,r,i){return new Ke.prototype.init(e,t,n,r,i)}S.extend({cssHooks:{opacity:{get:function(e,t){if(t){var n=We(e,&quot;opacity&quot;);return&quot;&quot;===n?&quot;1&quot;:n}}}},cssNumber:{animationIterationCount:!0,columnCount:!0,fillOpacity:!0,flexGrow:!0,flexShrink:!0,fontWeight:!0,gridArea:!0,gridColumn:!0,gridColumnEnd:!0,gridColumnStart:!0,gridRow:!0,gridRowEnd:!0,gridRowStart:!0,lineHeight:!0,opacity:!0,order:!0,orphans:!0,widows:!0,zIndex:!0,zoom:!0},cssProps:{},style:function(e,t,n,r){if(e&amp;&amp;3!==e.nodeType&amp;&amp;8!==e.nodeType&amp;&amp;e.style){var i,o,a,s=X(t),u=Xe.test(t),l=e.style;if(u||(t=ze(s)),a=S.cssHooks[t]||S.cssHooks[s],void 0===n)return a&amp;&amp;&quot;get&quot;in a&amp;&amp;void 0!==(i=a.get(e,!1,r))?i:l[t];&quot;string&quot;===(o=typeof n)&amp;&amp;(i=te.exec(n))&amp;&amp;i[1]&amp;&amp;(n=se(e,t,i),o=&quot;number&quot;),null!=n&amp;&amp;n==n&amp;&amp;(&quot;number&quot;!==o||u||(n+=i&amp;&amp;i[3]||(S.cssNumber[s]?&quot;&quot;:&quot;px&quot;)),y.clearCloneStyle||&quot;&quot;!==n||0!==t.indexOf(&quot;background&quot;)||(l[t]=&quot;inherit&quot;),a&amp;&amp;&quot;set&quot;in a&amp;&amp;void 0===(n=a.set(e,n,r))||(u?l.setProperty(t,n):l[t]=n))}},css:function(e,t,n,r){var i,o,a,s=X(t);return Xe.test(t)||(t=ze(s)),(a=S.cssHooks[t]||S.cssHooks[s])&amp;&amp;&quot;get&quot;in a&amp;&amp;(i=a.get(e,!0,n)),void 0===i&amp;&amp;(i=We(e,t,r)),&quot;normal&quot;===i&amp;&amp;t in Ge&amp;&amp;(i=Ge[t]),&quot;&quot;===n||n?(o=parseFloat(i),!0===n||isFinite(o)?o||0:i):i}}),S.each([&quot;height&quot;,&quot;width&quot;],function(e,u){S.cssHooks[u]={get:function(e,t,n){if(t)return!Ue.test(S.css(e,&quot;display&quot;))||e.getClientRects().length&amp;&amp;e.getBoundingClientRect().width?Je(e,u,n):Me(e,Ve,function(){return Je(e,u,n)})},set:function(e,t,n){var r,i=Re(e),o=!y.scrollboxSize()&amp;&amp;&quot;absolute&quot;===i.position,a=(o||n)&amp;&amp;&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,i),s=n?Qe(e,u,n,a,i):0;return a&amp;&amp;o&amp;&amp;(s-=Math.ceil(e[&quot;offset&quot;+u[0].toUpperCase()+u.slice(1)]-parseFloat(i[u])-Qe(e,u,&quot;border&quot;,!1,i)-.5)),s&amp;&amp;(r=te.exec(t))&amp;&amp;&quot;px&quot;!==(r[3]||&quot;px&quot;)&amp;&amp;(e.style[u]=t,t=S.css(e,u)),Ye(0,t,s)}}}),S.cssHooks.marginLeft=Fe(y.reliableMarginLeft,function(e,t){if(t)return(parseFloat(We(e,&quot;marginLeft&quot;))||e.getBoundingClientRect().left-Me(e,{marginLeft:0},function(){return e.getBoundingClientRect().left}))+&quot;px&quot;}),S.each({margin:&quot;&quot;,padding:&quot;&quot;,border:&quot;Width&quot;},function(i,o){S.cssHooks[i+o]={expand:function(e){for(var t=0,n={},r=&quot;string&quot;==typeof e?e.split(&quot; &quot;):[e];t&lt;4;t++)n[i+ne[t]+o]=r[t]||r[t-2]||r[0];return n}},&quot;margin&quot;!==i&amp;&amp;(S.cssHooks[i+o].set=Ye)}),S.fn.extend({css:function(e,t){return $(this,function(e,t,n){var r,i,o={},a=0;if(Array.isArray(t)){for(r=Re(e),i=t.length;a&lt;i;a++)o[t[a]]=S.css(e,t[a],!1,r);return o}return void 0!==n?S.style(e,t,n):S.css(e,t)},e,t,1&lt;arguments.length)}}),((S.Tween=Ke).prototype={constructor:Ke,init:function(e,t,n,r,i,o){this.elem=e,this.prop=n,this.easing=i||S.easing._default,this.options=t,this.start=this.now=this.cur(),this.end=r,this.unit=o||(S.cssNumber[n]?&quot;&quot;:&quot;px&quot;)},cur:function(){var e=Ke.propHooks[this.prop];return e&amp;&amp;e.get?e.get(this):Ke.propHooks._default.get(this)},run:function(e){var t,n=Ke.propHooks[this.prop];return this.options.duration?this.pos=t=S.easing[this.easing](e,this.options.duration*e,0,1,this.options.duration):this.pos=t=e,this.now=(this.end-this.start)*t+this.start,this.options.step&amp;&amp;this.options.step.call(this.elem,this.now,this),n&amp;&amp;n.set?n.set(this):Ke.propHooks._default.set(this),this}}).init.prototype=Ke.prototype,(Ke.propHooks={_default:{get:function(e){var t;return 1!==e.elem.nodeType||null!=e.elem[e.prop]&amp;&amp;null==e.elem.style[e.prop]?e.elem[e.prop]:(t=S.css(e.elem,e.prop,&quot;&quot;))&amp;&amp;&quot;auto&quot;!==t?t:0},set:function(e){S.fx.step[e.prop]?S.fx.step[e.prop](e):1!==e.elem.nodeType||!S.cssHooks[e.prop]&amp;&amp;null==e.elem.style[ze(e.prop)]?e.elem[e.prop]=e.now:S.style(e.elem,e.prop,e.now+e.unit)}}}).scrollTop=Ke.propHooks.scrollLeft={set:function(e){e.elem.nodeType&amp;&amp;e.elem.parentNode&amp;&amp;(e.elem[e.prop]=e.now)}},S.easing={linear:function(e){return e},swing:function(e){return.5-Math.cos(e*Math.PI)/2},_default:&quot;swing&quot;},S.fx=Ke.prototype.init,S.fx.step={};var Ze,et,tt,nt,rt=/^(?:toggle|show|hide)$/,it=/queueHooks$/;function ot(){et&amp;&amp;(!1===E.hidden&amp;&amp;C.requestAnimationFrame?C.requestAnimationFrame(ot):C.setTimeout(ot,S.fx.interval),S.fx.tick())}function at(){return C.setTimeout(function(){Ze=void 0}),Ze=Date.now()}function st(e,t){var n,r=0,i={height:e};for(t=t?1:0;r&lt;4;r+=2-t)i[&quot;margin&quot;+(n=ne[r])]=i[&quot;padding&quot;+n]=e;return t&amp;&amp;(i.opacity=i.width=e),i}function ut(e,t,n){for(var r,i=(lt.tweeners[t]||[]).concat(lt.tweeners[&quot;*&quot;]),o=0,a=i.length;o&lt;a;o++)if(r=i[o].call(n,t,e))return r}function lt(o,e,t){var n,a,r=0,i=lt.prefilters.length,s=S.Deferred().always(function(){delete u.elem}),u=function(){if(a)return!1;for(var e=Ze||at(),t=Math.max(0,l.startTime+l.duration-e),n=1-(t/l.duration||0),r=0,i=l.tweens.length;r&lt;i;r++)l.tweens[r].run(n);return s.notifyWith(o,[l,n,t]),n&lt;1&amp;&amp;i?t:(i||s.notifyWith(o,[l,1,0]),s.resolveWith(o,[l]),!1)},l=s.promise({elem:o,props:S.extend({},e),opts:S.extend(!0,{specialEasing:{},easing:S.easing._default},t),originalProperties:e,originalOptions:t,startTime:Ze||at(),duration:t.duration,tweens:[],createTween:function(e,t){var n=S.Tween(o,l.opts,e,t,l.opts.specialEasing[e]||l.opts.easing);return l.tweens.push(n),n},stop:function(e){var t=0,n=e?l.tweens.length:0;if(a)return this;for(a=!0;t&lt;n;t++)l.tweens[t].run(1);return e?(s.notifyWith(o,[l,1,0]),s.resolveWith(o,[l,e])):s.rejectWith(o,[l,e]),this}}),c=l.props;for(!function(e,t){var n,r,i,o,a;for(n in e)if(i=t[r=X(n)],o=e[n],Array.isArray(o)&amp;&amp;(i=o[1],o=e[n]=o[0]),n!==r&amp;&amp;(e[r]=o,delete e[n]),(a=S.cssHooks[r])&amp;&amp;&quot;expand&quot;in a)for(n in o=a.expand(o),delete e[r],o)n in e||(e[n]=o[n],t[n]=i);else t[r]=i}(c,l.opts.specialEasing);r&lt;i;r++)if(n=lt.prefilters[r].call(l,o,c,l.opts))return m(n.stop)&amp;&amp;(S._queueHooks(l.elem,l.opts.queue).stop=n.stop.bind(n)),n;return S.map(c,ut,l),m(l.opts.start)&amp;&amp;l.opts.start.call(o,l),l.progress(l.opts.progress).done(l.opts.done,l.opts.complete).fail(l.opts.fail).always(l.opts.always),S.fx.timer(S.extend(u,{elem:o,anim:l,queue:l.opts.queue})),l}S.Animation=S.extend(lt,{tweeners:{&quot;*&quot;:[function(e,t){var n=this.createTween(e,t);return se(n.elem,e,te.exec(t),n),n}]},tweener:function(e,t){m(e)?(t=e,e=[&quot;*&quot;]):e=e.match(P);for(var n,r=0,i=e.length;r&lt;i;r++)n=e[r],lt.tweeners[n]=lt.tweeners[n]||[],lt.tweeners[n].unshift(t)},prefilters:[function(e,t,n){var r,i,o,a,s,u,l,c,f=&quot;width&quot;in t||&quot;height&quot;in t,p=this,d={},h=e.style,g=e.nodeType&amp;&amp;ae(e),v=Y.get(e,&quot;fxshow&quot;);for(r in n.queue||(null==(a=S._queueHooks(e,&quot;fx&quot;)).unqueued&amp;&amp;(a.unqueued=0,s=a.empty.fire,a.empty.fire=function(){a.unqueued||s()}),a.unqueued++,p.always(function(){p.always(function(){a.unqueued--,S.queue(e,&quot;fx&quot;).length||a.empty.fire()})})),t)if(i=t[r],rt.test(i)){if(delete t[r],o=o||&quot;toggle&quot;===i,i===(g?&quot;hide&quot;:&quot;show&quot;)){if(&quot;show&quot;!==i||!v||void 0===v[r])continue;g=!0}d[r]=v&amp;&amp;v[r]||S.style(e,r)}if((u=!S.isEmptyObject(t))||!S.isEmptyObject(d))for(r in f&amp;&amp;1===e.nodeType&amp;&amp;(n.overflow=[h.overflow,h.overflowX,h.overflowY],null==(l=v&amp;&amp;v.display)&amp;&amp;(l=Y.get(e,&quot;display&quot;)),&quot;none&quot;===(c=S.css(e,&quot;display&quot;))&amp;&amp;(l?c=l:(le([e],!0),l=e.style.display||l,c=S.css(e,&quot;display&quot;),le([e]))),(&quot;inline&quot;===c||&quot;inline-block&quot;===c&amp;&amp;null!=l)&amp;&amp;&quot;none&quot;===S.css(e,&quot;float&quot;)&amp;&amp;(u||(p.done(function(){h.display=l}),null==l&amp;&amp;(c=h.display,l=&quot;none&quot;===c?&quot;&quot;:c)),h.display=&quot;inline-block&quot;)),n.overflow&amp;&amp;(h.overflow=&quot;hidden&quot;,p.always(function(){h.overflow=n.overflow[0],h.overflowX=n.overflow[1],h.overflowY=n.overflow[2]})),u=!1,d)u||(v?&quot;hidden&quot;in v&amp;&amp;(g=v.hidden):v=Y.access(e,&quot;fxshow&quot;,{display:l}),o&amp;&amp;(v.hidden=!g),g&amp;&amp;le([e],!0),p.done(function(){for(r in g||le([e]),Y.remove(e,&quot;fxshow&quot;),d)S.style(e,r,d[r])})),u=ut(g?v[r]:0,r,p),r in v||(v[r]=u.start,g&amp;&amp;(u.end=u.start,u.start=0))}],prefilter:function(e,t){t?lt.prefilters.unshift(e):lt.prefilters.push(e)}}),S.speed=function(e,t,n){var r=e&amp;&amp;&quot;object&quot;==typeof e?S.extend({},e):{complete:n||!n&amp;&amp;t||m(e)&amp;&amp;e,duration:e,easing:n&amp;&amp;t||t&amp;&amp;!m(t)&amp;&amp;t};return S.fx.off?r.duration=0:&quot;number&quot;!=typeof r.duration&amp;&amp;(r.duration in S.fx.speeds?r.duration=S.fx.speeds[r.duration]:r.duration=S.fx.speeds._default),null!=r.queue&amp;&amp;!0!==r.queue||(r.queue=&quot;fx&quot;),r.old=r.complete,r.complete=function(){m(r.old)&amp;&amp;r.old.call(this),r.queue&amp;&amp;S.dequeue(this,r.queue)},r},S.fn.extend({fadeTo:function(e,t,n,r){return this.filter(ae).css(&quot;opacity&quot;,0).show().end().animate({opacity:t},e,n,r)},animate:function(t,e,n,r){var i=S.isEmptyObject(t),o=S.speed(e,n,r),a=function(){var e=lt(this,S.extend({},t),o);(i||Y.get(this,&quot;finish&quot;))&amp;&amp;e.stop(!0)};return a.finish=a,i||!1===o.queue?this.each(a):this.queue(o.queue,a)},stop:function(i,e,o){var a=function(e){var t=e.stop;delete e.stop,t(o)};return&quot;string&quot;!=typeof i&amp;&amp;(o=e,e=i,i=void 0),e&amp;&amp;this.queue(i||&quot;fx&quot;,[]),this.each(function(){var e=!0,t=null!=i&amp;&amp;i+&quot;queueHooks&quot;,n=S.timers,r=Y.get(this);if(t)r[t]&amp;&amp;r[t].stop&amp;&amp;a(r[t]);else for(t in r)r[t]&amp;&amp;r[t].stop&amp;&amp;it.test(t)&amp;&amp;a(r[t]);for(t=n.length;t--;)n[t].elem!==this||null!=i&amp;&amp;n[t].queue!==i||(n[t].anim.stop(o),e=!1,n.splice(t,1));!e&amp;&amp;o||S.dequeue(this,i)})},finish:function(a){return!1!==a&amp;&amp;(a=a||&quot;fx&quot;),this.each(function(){var e,t=Y.get(this),n=t[a+&quot;queue&quot;],r=t[a+&quot;queueHooks&quot;],i=S.timers,o=n?n.length:0;for(t.finish=!0,S.queue(this,a,[]),r&amp;&amp;r.stop&amp;&amp;r.stop.call(this,!0),e=i.length;e--;)i[e].elem===this&amp;&amp;i[e].queue===a&amp;&amp;(i[e].anim.stop(!0),i.splice(e,1));for(e=0;e&lt;o;e++)n[e]&amp;&amp;n[e].finish&amp;&amp;n[e].finish.call(this);delete t.finish})}}),S.each([&quot;toggle&quot;,&quot;show&quot;,&quot;hide&quot;],function(e,r){var i=S.fn[r];S.fn[r]=function(e,t,n){return null==e||&quot;boolean&quot;==typeof e?i.apply(this,arguments):this.animate(st(r,!0),e,t,n)}}),S.each({slideDown:st(&quot;show&quot;),slideUp:st(&quot;hide&quot;),slideToggle:st(&quot;toggle&quot;),fadeIn:{opacity:&quot;show&quot;},fadeOut:{opacity:&quot;hide&quot;},fadeToggle:{opacity:&quot;toggle&quot;}},function(e,r){S.fn[e]=function(e,t,n){return this.animate(r,e,t,n)}}),S.timers=[],S.fx.tick=function(){var e,t=0,n=S.timers;for(Ze=Date.now();t&lt;n.length;t++)(e=n[t])()||n[t]!==e||n.splice(t--,1);n.length||S.fx.stop(),Ze=void 0},S.fx.timer=function(e){S.timers.push(e),S.fx.start()},S.fx.interval=13,S.fx.start=function(){et||(et=!0,ot())},S.fx.stop=function(){et=null},S.fx.speeds={slow:600,fast:200,_default:400},S.fn.delay=function(r,e){return r=S.fx&amp;&amp;S.fx.speeds[r]||r,e=e||&quot;fx&quot;,this.queue(e,function(e,t){var n=C.setTimeout(e,r);t.stop=function(){C.clearTimeout(n)}})},tt=E.createElement(&quot;input&quot;),nt=E.createElement(&quot;select&quot;).appendChild(E.createElement(&quot;option&quot;)),tt.type=&quot;checkbox&quot;,y.checkOn=&quot;&quot;!==tt.value,y.optSelected=nt.selected,(tt=E.createElement(&quot;input&quot;)).value=&quot;t&quot;,tt.type=&quot;radio&quot;,y.radioValue=&quot;t&quot;===tt.value;var ct,ft=S.expr.attrHandle;S.fn.extend({attr:function(e,t){return $(this,S.attr,e,t,1&lt;arguments.length)},removeAttr:function(e){return this.each(function(){S.removeAttr(this,e)})}}),S.extend({attr:function(e,t,n){var r,i,o=e.nodeType;if(3!==o&amp;&amp;8!==o&amp;&amp;2!==o)return&quot;undefined&quot;==typeof e.getAttribute?S.prop(e,t,n):(1===o&amp;&amp;S.isXMLDoc(e)||(i=S.attrHooks[t.toLowerCase()]||(S.expr.match.bool.test(t)?ct:void 0)),void 0!==n?null===n?void S.removeAttr(e,t):i&amp;&amp;&quot;set&quot;in i&amp;&amp;void 0!==(r=i.set(e,n,t))?r:(e.setAttribute(t,n+&quot;&quot;),n):i&amp;&amp;&quot;get&quot;in i&amp;&amp;null!==(r=i.get(e,t))?r:null==(r=S.find.attr(e,t))?void 0:r)},attrHooks:{type:{set:function(e,t){if(!y.radioValue&amp;&amp;&quot;radio&quot;===t&amp;&amp;A(e,&quot;input&quot;)){var n=e.value;return e.setAttribute(&quot;type&quot;,t),n&amp;&amp;(e.value=n),t}}}},removeAttr:function(e,t){var n,r=0,i=t&amp;&amp;t.match(P);if(i&amp;&amp;1===e.nodeType)while(n=i[r++])e.removeAttribute(n)}}),ct={set:function(e,t,n){return!1===t?S.removeAttr(e,n):e.setAttribute(n,n),n}},S.each(S.expr.match.bool.source.match(/\w+/g),function(e,t){var a=ft[t]||S.find.attr;ft[t]=function(e,t,n){var r,i,o=t.toLowerCase();return n||(i=ft[o],ft[o]=r,r=null!=a(e,t,n)?o:null,ft[o]=i),r}});var pt=/^(?:input|select|textarea|button)$/i,dt=/^(?:a|area)$/i;function ht(e){return(e.match(P)||[]).join(&quot; &quot;)}function gt(e){return e.getAttribute&amp;&amp;e.getAttribute(&quot;class&quot;)||&quot;&quot;}function vt(e){return Array.isArray(e)?e:&quot;string&quot;==typeof e&amp;&amp;e.match(P)||[]}S.fn.extend({prop:function(e,t){return $(this,S.prop,e,t,1&lt;arguments.length)},removeProp:function(e){return this.each(function(){delete this[S.propFix[e]||e]})}}),S.extend({prop:function(e,t,n){var r,i,o=e.nodeType;if(3!==o&amp;&amp;8!==o&amp;&amp;2!==o)return 1===o&amp;&amp;S.isXMLDoc(e)||(t=S.propFix[t]||t,i=S.propHooks[t]),void 0!==n?i&amp;&amp;&quot;set&quot;in i&amp;&amp;void 0!==(r=i.set(e,n,t))?r:e[t]=n:i&amp;&amp;&quot;get&quot;in i&amp;&amp;null!==(r=i.get(e,t))?r:e[t]},propHooks:{tabIndex:{get:function(e){var t=S.find.attr(e,&quot;tabindex&quot;);return t?parseInt(t,10):pt.test(e.nodeName)||dt.test(e.nodeName)&amp;&amp;e.href?0:-1}}},propFix:{&quot;for&quot;:&quot;htmlFor&quot;,&quot;class&quot;:&quot;className&quot;}}),y.optSelected||(S.propHooks.selected={get:function(e){var t=e.parentNode;return t&amp;&amp;t.parentNode&amp;&amp;t.parentNode.selectedIndex,null},set:function(e){var t=e.parentNode;t&amp;&amp;(t.selectedIndex,t.parentNode&amp;&amp;t.parentNode.selectedIndex)}}),S.each([&quot;tabIndex&quot;,&quot;readOnly&quot;,&quot;maxLength&quot;,&quot;cellSpacing&quot;,&quot;cellPadding&quot;,&quot;rowSpan&quot;,&quot;colSpan&quot;,&quot;useMap&quot;,&quot;frameBorder&quot;,&quot;contentEditable&quot;],function(){S.propFix[this.toLowerCase()]=this}),S.fn.extend({addClass:function(t){var e,n,r,i,o,a,s,u=0;if(m(t))return this.each(function(e){S(this).addClass(t.call(this,e,gt(this)))});if((e=vt(t)).length)while(n=this[u++])if(i=gt(n),r=1===n.nodeType&amp;&amp;&quot; &quot;+ht(i)+&quot; &quot;){a=0;while(o=e[a++])r.indexOf(&quot; &quot;+o+&quot; &quot;)&lt;0&amp;&amp;(r+=o+&quot; &quot;);i!==(s=ht(r))&amp;&amp;n.setAttribute(&quot;class&quot;,s)}return this},removeClass:function(t){var e,n,r,i,o,a,s,u=0;if(m(t))return this.each(function(e){S(this).removeClass(t.call(this,e,gt(this)))});if(!arguments.length)return this.attr(&quot;class&quot;,&quot;&quot;);if((e=vt(t)).length)while(n=this[u++])if(i=gt(n),r=1===n.nodeType&amp;&amp;&quot; &quot;+ht(i)+&quot; &quot;){a=0;while(o=e[a++])while(-1&lt;r.indexOf(&quot; &quot;+o+&quot; &quot;))r=r.replace(&quot; &quot;+o+&quot; &quot;,&quot; &quot;);i!==(s=ht(r))&amp;&amp;n.setAttribute(&quot;class&quot;,s)}return this},toggleClass:function(i,t){var o=typeof i,a=&quot;string&quot;===o||Array.isArray(i);return&quot;boolean&quot;==typeof t&amp;&amp;a?t?this.addClass(i):this.removeClass(i):m(i)?this.each(function(e){S(this).toggleClass(i.call(this,e,gt(this),t),t)}):this.each(function(){var e,t,n,r;if(a){t=0,n=S(this),r=vt(i);while(e=r[t++])n.hasClass(e)?n.removeClass(e):n.addClass(e)}else void 0!==i&amp;&amp;&quot;boolean&quot;!==o||((e=gt(this))&amp;&amp;Y.set(this,&quot;__className__&quot;,e),this.setAttribute&amp;&amp;this.setAttribute(&quot;class&quot;,e||!1===i?&quot;&quot;:Y.get(this,&quot;__className__&quot;)||&quot;&quot;))})},hasClass:function(e){var t,n,r=0;t=&quot; &quot;+e+&quot; &quot;;while(n=this[r++])if(1===n.nodeType&amp;&amp;-1&lt;(&quot; &quot;+ht(gt(n))+&quot; &quot;).indexOf(t))return!0;return!1}});var yt=/\r/g;S.fn.extend({val:function(n){var r,e,i,t=this[0];return arguments.length?(i=m(n),this.each(function(e){var t;1===this.nodeType&amp;&amp;(null==(t=i?n.call(this,e,S(this).val()):n)?t=&quot;&quot;:&quot;number&quot;==typeof t?t+=&quot;&quot;:Array.isArray(t)&amp;&amp;(t=S.map(t,function(e){return null==e?&quot;&quot;:e+&quot;&quot;})),(r=S.valHooks[this.type]||S.valHooks[this.nodeName.toLowerCase()])&amp;&amp;&quot;set&quot;in r&amp;&amp;void 0!==r.set(this,t,&quot;value&quot;)||(this.value=t))})):t?(r=S.valHooks[t.type]||S.valHooks[t.nodeName.toLowerCase()])&amp;&amp;&quot;get&quot;in r&amp;&amp;void 0!==(e=r.get(t,&quot;value&quot;))?e:&quot;string&quot;==typeof(e=t.value)?e.replace(yt,&quot;&quot;):null==e?&quot;&quot;:e:void 0}}),S.extend({valHooks:{option:{get:function(e){var t=S.find.attr(e,&quot;value&quot;);return null!=t?t:ht(S.text(e))}},select:{get:function(e){var t,n,r,i=e.options,o=e.selectedIndex,a=&quot;select-one&quot;===e.type,s=a?null:[],u=a?o+1:i.length;for(r=o&lt;0?u:a?o:0;r&lt;u;r++)if(((n=i[r]).selected||r===o)&amp;&amp;!n.disabled&amp;&amp;(!n.parentNode.disabled||!A(n.parentNode,&quot;optgroup&quot;))){if(t=S(n).val(),a)return t;s.push(t)}return s},set:function(e,t){var n,r,i=e.options,o=S.makeArray(t),a=i.length;while(a--)((r=i[a]).selected=-1&lt;S.inArray(S.valHooks.option.get(r),o))&amp;&amp;(n=!0);return n||(e.selectedIndex=-1),o}}}}),S.each([&quot;radio&quot;,&quot;checkbox&quot;],function(){S.valHooks[this]={set:function(e,t){if(Array.isArray(t))return e.checked=-1&lt;S.inArray(S(e).val(),t)}},y.checkOn||(S.valHooks[this].get=function(e){return null===e.getAttribute(&quot;value&quot;)?&quot;on&quot;:e.value})}),y.focusin=&quot;onfocusin&quot;in C;var mt=/^(?:focusinfocus|focusoutblur)$/,xt=function(e){e.stopPropagation()};S.extend(S.event,{trigger:function(e,t,n,r){var i,o,a,s,u,l,c,f,p=[n||E],d=v.call(e,&quot;type&quot;)?e.type:e,h=v.call(e,&quot;namespace&quot;)?e.namespace.split(&quot;.&quot;):[];if(o=f=a=n=n||E,3!==n.nodeType&amp;&amp;8!==n.nodeType&amp;&amp;!mt.test(d+S.event.triggered)&amp;&amp;(-1&lt;d.indexOf(&quot;.&quot;)&amp;&amp;(d=(h=d.split(&quot;.&quot;)).shift(),h.sort()),u=d.indexOf(&quot;:&quot;)&lt;0&amp;&amp;&quot;on&quot;+d,(e=e[S.expando]?e:new S.Event(d,&quot;object&quot;==typeof e&amp;&amp;e)).isTrigger=r?2:3,e.namespace=h.join(&quot;.&quot;),e.rnamespace=e.namespace?new RegExp(&quot;(^|\\.)&quot;+h.join(&quot;\\.(?:.*\\.|)&quot;)+&quot;(\\.|$)&quot;):null,e.result=void 0,e.target||(e.target=n),t=null==t?[e]:S.makeArray(t,[e]),c=S.event.special[d]||{},r||!c.trigger||!1!==c.trigger.apply(n,t))){if(!r&amp;&amp;!c.noBubble&amp;&amp;!x(n)){for(s=c.delegateType||d,mt.test(s+d)||(o=o.parentNode);o;o=o.parentNode)p.push(o),a=o;a===(n.ownerDocument||E)&amp;&amp;p.push(a.defaultView||a.parentWindow||C)}i=0;while((o=p[i++])&amp;&amp;!e.isPropagationStopped())f=o,e.type=1&lt;i?s:c.bindType||d,(l=(Y.get(o,&quot;events&quot;)||Object.create(null))[e.type]&amp;&amp;Y.get(o,&quot;handle&quot;))&amp;&amp;l.apply(o,t),(l=u&amp;&amp;o[u])&amp;&amp;l.apply&amp;&amp;V(o)&amp;&amp;(e.result=l.apply(o,t),!1===e.result&amp;&amp;e.preventDefault());return e.type=d,r||e.isDefaultPrevented()||c._default&amp;&amp;!1!==c._default.apply(p.pop(),t)||!V(n)||u&amp;&amp;m(n[d])&amp;&amp;!x(n)&amp;&amp;((a=n[u])&amp;&amp;(n[u]=null),S.event.triggered=d,e.isPropagationStopped()&amp;&amp;f.addEventListener(d,xt),n[d](),e.isPropagationStopped()&amp;&amp;f.removeEventListener(d,xt),S.event.triggered=void 0,a&amp;&amp;(n[u]=a)),e.result}},simulate:function(e,t,n){var r=S.extend(new S.Event,n,{type:e,isSimulated:!0});S.event.trigger(r,null,t)}}),S.fn.extend({trigger:function(e,t){return this.each(function(){S.event.trigger(e,t,this)})},triggerHandler:function(e,t){var n=this[0];if(n)return S.event.trigger(e,t,n,!0)}}),y.focusin||S.each({focus:&quot;focusin&quot;,blur:&quot;focusout&quot;},function(n,r){var i=function(e){S.event.simulate(r,e.target,S.event.fix(e))};S.event.special[r]={setup:function(){var e=this.ownerDocument||this.document||this,t=Y.access(e,r);t||e.addEventListener(n,i,!0),Y.access(e,r,(t||0)+1)},teardown:function(){var e=this.ownerDocument||this.document||this,t=Y.access(e,r)-1;t?Y.access(e,r,t):(e.removeEventListener(n,i,!0),Y.remove(e,r))}}});var bt=C.location,wt={guid:Date.now()},Tt=/\?/;S.parseXML=function(e){var t,n;if(!e||&quot;string&quot;!=typeof e)return null;try{t=(new C.DOMParser).parseFromString(e,&quot;text/xml&quot;)}catch(e){}return n=t&amp;&amp;t.getElementsByTagName(&quot;parsererror&quot;)[0],t&amp;&amp;!n||S.error(&quot;Invalid XML: &quot;+(n?S.map(n.childNodes,function(e){return e.textContent}).join(&quot;\n&quot;):e)),t};var Ct=/\[\]$/,Et=/\r?\n/g,St=/^(?:submit|button|image|reset|file)$/i,kt=/^(?:input|select|textarea|keygen)/i;function At(n,e,r,i){var t;if(Array.isArray(e))S.each(e,function(e,t){r||Ct.test(n)?i(n,t):At(n+&quot;[&quot;+(&quot;object&quot;==typeof t&amp;&amp;null!=t?e:&quot;&quot;)+&quot;]&quot;,t,r,i)});else if(r||&quot;object&quot;!==w(e))i(n,e);else for(t in e)At(n+&quot;[&quot;+t+&quot;]&quot;,e[t],r,i)}S.param=function(e,t){var n,r=[],i=function(e,t){var n=m(t)?t():t;r[r.length]=encodeURIComponent(e)+&quot;=&quot;+encodeURIComponent(null==n?&quot;&quot;:n)};if(null==e)return&quot;&quot;;if(Array.isArray(e)||e.jquery&amp;&amp;!S.isPlainObject(e))S.each(e,function(){i(this.name,this.value)});else for(n in e)At(n,e[n],t,i);return r.join(&quot;&amp;&quot;)},S.fn.extend({serialize:function(){return S.param(this.serializeArray())},serializeArray:function(){return this.map(function(){var e=S.prop(this,&quot;elements&quot;);return e?S.makeArray(e):this}).filter(function(){var e=this.type;return this.name&amp;&amp;!S(this).is(&quot;:disabled&quot;)&amp;&amp;kt.test(this.nodeName)&amp;&amp;!St.test(e)&amp;&amp;(this.checked||!pe.test(e))}).map(function(e,t){var n=S(this).val();return null==n?null:Array.isArray(n)?S.map(n,function(e){return{name:t.name,value:e.replace(Et,&quot;\r\n&quot;)}}):{name:t.name,value:n.replace(Et,&quot;\r\n&quot;)}}).get()}});var Nt=/%20/g,jt=/#.*$/,Dt=/([?&amp;])_=[^&amp;]*/,qt=/^(.*?):[ \t]*([^\r\n]*)$/gm,Lt=/^(?:GET|HEAD)$/,Ht=/^\/\//,Ot={},Pt={},Rt=&quot;*/&quot;.concat(&quot;*&quot;),Mt=E.createElement(&quot;a&quot;);function It(o){return function(e,t){&quot;string&quot;!=typeof e&amp;&amp;(t=e,e=&quot;*&quot;);var n,r=0,i=e.toLowerCase().match(P)||[];if(m(t))while(n=i[r++])&quot;+&quot;===n[0]?(n=n.slice(1)||&quot;*&quot;,(o[n]=o[n]||[]).unshift(t)):(o[n]=o[n]||[]).push(t)}}function Wt(t,i,o,a){var s={},u=t===Pt;function l(e){var r;return s[e]=!0,S.each(t[e]||[],function(e,t){var n=t(i,o,a);return&quot;string&quot;!=typeof n||u||s[n]?u?!(r=n):void 0:(i.dataTypes.unshift(n),l(n),!1)}),r}return l(i.dataTypes[0])||!s[&quot;*&quot;]&amp;&amp;l(&quot;*&quot;)}function Ft(e,t){var n,r,i=S.ajaxSettings.flatOptions||{};for(n in t)void 0!==t[n]&amp;&amp;((i[n]?e:r||(r={}))[n]=t[n]);return r&amp;&amp;S.extend(!0,e,r),e}Mt.href=bt.href,S.extend({active:0,lastModified:{},etag:{},ajaxSettings:{url:bt.href,type:&quot;GET&quot;,isLocal:/^(?:about|app|app-storage|.+-extension|file|res|widget):$/.test(bt.protocol),global:!0,processData:!0,async:!0,contentType:&quot;application/x-www-form-urlencoded; charset=UTF-8&quot;,accepts:{&quot;*&quot;:Rt,text:&quot;text/plain&quot;,html:&quot;text/html&quot;,xml:&quot;application/xml, text/xml&quot;,json:&quot;application/json, text/javascript&quot;},contents:{xml:/\bxml\b/,html:/\bhtml/,json:/\bjson\b/},responseFields:{xml:&quot;responseXML&quot;,text:&quot;responseText&quot;,json:&quot;responseJSON&quot;},converters:{&quot;* text&quot;:String,&quot;text html&quot;:!0,&quot;text json&quot;:JSON.parse,&quot;text xml&quot;:S.parseXML},flatOptions:{url:!0,context:!0}},ajaxSetup:function(e,t){return t?Ft(Ft(e,S.ajaxSettings),t):Ft(S.ajaxSettings,e)},ajaxPrefilter:It(Ot),ajaxTransport:It(Pt),ajax:function(e,t){&quot;object&quot;==typeof e&amp;&amp;(t=e,e=void 0),t=t||{};var c,f,p,n,d,r,h,g,i,o,v=S.ajaxSetup({},t),y=v.context||v,m=v.context&amp;&amp;(y.nodeType||y.jquery)?S(y):S.event,x=S.Deferred(),b=S.Callbacks(&quot;once memory&quot;),w=v.statusCode||{},a={},s={},u=&quot;canceled&quot;,T={readyState:0,getResponseHeader:function(e){var t;if(h){if(!n){n={};while(t=qt.exec(p))n[t[1].toLowerCase()+&quot; &quot;]=(n[t[1].toLowerCase()+&quot; &quot;]||[]).concat(t[2])}t=n[e.toLowerCase()+&quot; &quot;]}return null==t?null:t.join(&quot;, &quot;)},getAllResponseHeaders:function(){return h?p:null},setRequestHeader:function(e,t){return null==h&amp;&amp;(e=s[e.toLowerCase()]=s[e.toLowerCase()]||e,a[e]=t),this},overrideMimeType:function(e){return null==h&amp;&amp;(v.mimeType=e),this},statusCode:function(e){var t;if(e)if(h)T.always(e[T.status]);else for(t in e)w[t]=[w[t],e[t]];return this},abort:function(e){var t=e||u;return c&amp;&amp;c.abort(t),l(0,t),this}};if(x.promise(T),v.url=((e||v.url||bt.href)+&quot;&quot;).replace(Ht,bt.protocol+&quot;//&quot;),v.type=t.method||t.type||v.method||v.type,v.dataTypes=(v.dataType||&quot;*&quot;).toLowerCase().match(P)||[&quot;&quot;],null==v.crossDomain){r=E.createElement(&quot;a&quot;);try{r.href=v.url,r.href=r.href,v.crossDomain=Mt.protocol+&quot;//&quot;+Mt.host!=r.protocol+&quot;//&quot;+r.host}catch(e){v.crossDomain=!0}}if(v.data&amp;&amp;v.processData&amp;&amp;&quot;string&quot;!=typeof v.data&amp;&amp;(v.data=S.param(v.data,v.traditional)),Wt(Ot,v,t,T),h)return T;for(i in(g=S.event&amp;&amp;v.global)&amp;&amp;0==S.active++&amp;&amp;S.event.trigger(&quot;ajaxStart&quot;),v.type=v.type.toUpperCase(),v.hasContent=!Lt.test(v.type),f=v.url.replace(jt,&quot;&quot;),v.hasContent?v.data&amp;&amp;v.processData&amp;&amp;0===(v.contentType||&quot;&quot;).indexOf(&quot;application/x-www-form-urlencoded&quot;)&amp;&amp;(v.data=v.data.replace(Nt,&quot;+&quot;)):(o=v.url.slice(f.length),v.data&amp;&amp;(v.processData||&quot;string&quot;==typeof v.data)&amp;&amp;(f+=(Tt.test(f)?&quot;&amp;&quot;:&quot;?&quot;)+v.data,delete v.data),!1===v.cache&amp;&amp;(f=f.replace(Dt,&quot;$1&quot;),o=(Tt.test(f)?&quot;&amp;&quot;:&quot;?&quot;)+&quot;_=&quot;+wt.guid+++o),v.url=f+o),v.ifModified&amp;&amp;(S.lastModified[f]&amp;&amp;T.setRequestHeader(&quot;If-Modified-Since&quot;,S.lastModified[f]),S.etag[f]&amp;&amp;T.setRequestHeader(&quot;If-None-Match&quot;,S.etag[f])),(v.data&amp;&amp;v.hasContent&amp;&amp;!1!==v.contentType||t.contentType)&amp;&amp;T.setRequestHeader(&quot;Content-Type&quot;,v.contentType),T.setRequestHeader(&quot;Accept&quot;,v.dataTypes[0]&amp;&amp;v.accepts[v.dataTypes[0]]?v.accepts[v.dataTypes[0]]+(&quot;*&quot;!==v.dataTypes[0]?&quot;, &quot;+Rt+&quot;; q=0.01&quot;:&quot;&quot;):v.accepts[&quot;*&quot;]),v.headers)T.setRequestHeader(i,v.headers[i]);if(v.beforeSend&amp;&amp;(!1===v.beforeSend.call(y,T,v)||h))return T.abort();if(u=&quot;abort&quot;,b.add(v.complete),T.done(v.success),T.fail(v.error),c=Wt(Pt,v,t,T)){if(T.readyState=1,g&amp;&amp;m.trigger(&quot;ajaxSend&quot;,[T,v]),h)return T;v.async&amp;&amp;0&lt;v.timeout&amp;&amp;(d=C.setTimeout(function(){T.abort(&quot;timeout&quot;)},v.timeout));try{h=!1,c.send(a,l)}catch(e){if(h)throw e;l(-1,e)}}else l(-1,&quot;No Transport&quot;);function l(e,t,n,r){var i,o,a,s,u,l=t;h||(h=!0,d&amp;&amp;C.clearTimeout(d),c=void 0,p=r||&quot;&quot;,T.readyState=0&lt;e?4:0,i=200&lt;=e&amp;&amp;e&lt;300||304===e,n&amp;&amp;(s=function(e,t,n){var r,i,o,a,s=e.contents,u=e.dataTypes;while(&quot;*&quot;===u[0])u.shift(),void 0===r&amp;&amp;(r=e.mimeType||t.getResponseHeader(&quot;Content-Type&quot;));if(r)for(i in s)if(s[i]&amp;&amp;s[i].test(r)){u.unshift(i);break}if(u[0]in n)o=u[0];else{for(i in n){if(!u[0]||e.converters[i+&quot; &quot;+u[0]]){o=i;break}a||(a=i)}o=o||a}if(o)return o!==u[0]&amp;&amp;u.unshift(o),n[o]}(v,T,n)),!i&amp;&amp;-1&lt;S.inArray(&quot;script&quot;,v.dataTypes)&amp;&amp;S.inArray(&quot;json&quot;,v.dataTypes)&lt;0&amp;&amp;(v.converters[&quot;text script&quot;]=function(){}),s=function(e,t,n,r){var i,o,a,s,u,l={},c=e.dataTypes.slice();if(c[1])for(a in e.converters)l[a.toLowerCase()]=e.converters[a];o=c.shift();while(o)if(e.responseFields[o]&amp;&amp;(n[e.responseFields[o]]=t),!u&amp;&amp;r&amp;&amp;e.dataFilter&amp;&amp;(t=e.dataFilter(t,e.dataType)),u=o,o=c.shift())if(&quot;*&quot;===o)o=u;else if(&quot;*&quot;!==u&amp;&amp;u!==o){if(!(a=l[u+&quot; &quot;+o]||l[&quot;* &quot;+o]))for(i in l)if((s=i.split(&quot; &quot;))[1]===o&amp;&amp;(a=l[u+&quot; &quot;+s[0]]||l[&quot;* &quot;+s[0]])){!0===a?a=l[i]:!0!==l[i]&amp;&amp;(o=s[0],c.unshift(s[1]));break}if(!0!==a)if(a&amp;&amp;e[&quot;throws&quot;])t=a(t);else try{t=a(t)}catch(e){return{state:&quot;parsererror&quot;,error:a?e:&quot;No conversion from &quot;+u+&quot; to &quot;+o}}}return{state:&quot;success&quot;,data:t}}(v,s,T,i),i?(v.ifModified&amp;&amp;((u=T.getResponseHeader(&quot;Last-Modified&quot;))&amp;&amp;(S.lastModified[f]=u),(u=T.getResponseHeader(&quot;etag&quot;))&amp;&amp;(S.etag[f]=u)),204===e||&quot;HEAD&quot;===v.type?l=&quot;nocontent&quot;:304===e?l=&quot;notmodified&quot;:(l=s.state,o=s.data,i=!(a=s.error))):(a=l,!e&amp;&amp;l||(l=&quot;error&quot;,e&lt;0&amp;&amp;(e=0))),T.status=e,T.statusText=(t||l)+&quot;&quot;,i?x.resolveWith(y,[o,l,T]):x.rejectWith(y,[T,l,a]),T.statusCode(w),w=void 0,g&amp;&amp;m.trigger(i?&quot;ajaxSuccess&quot;:&quot;ajaxError&quot;,[T,v,i?o:a]),b.fireWith(y,[T,l]),g&amp;&amp;(m.trigger(&quot;ajaxComplete&quot;,[T,v]),--S.active||S.event.trigger(&quot;ajaxStop&quot;)))}return T},getJSON:function(e,t,n){return S.get(e,t,n,&quot;json&quot;)},getScript:function(e,t){return S.get(e,void 0,t,&quot;script&quot;)}}),S.each([&quot;get&quot;,&quot;post&quot;],function(e,i){S[i]=function(e,t,n,r){return m(t)&amp;&amp;(r=r||n,n=t,t=void 0),S.ajax(S.extend({url:e,type:i,dataType:r,data:t,success:n},S.isPlainObject(e)&amp;&amp;e))}}),S.ajaxPrefilter(function(e){var t;for(t in e.headers)&quot;content-type&quot;===t.toLowerCase()&amp;&amp;(e.contentType=e.headers[t]||&quot;&quot;)}),S._evalUrl=function(e,t,n){return S.ajax({url:e,type:&quot;GET&quot;,dataType:&quot;script&quot;,cache:!0,async:!1,global:!1,converters:{&quot;text script&quot;:function(){}},dataFilter:function(e){S.globalEval(e,t,n)}})},S.fn.extend({wrapAll:function(e){var t;return this[0]&amp;&amp;(m(e)&amp;&amp;(e=e.call(this[0])),t=S(e,this[0].ownerDocument).eq(0).clone(!0),this[0].parentNode&amp;&amp;t.insertBefore(this[0]),t.map(function(){var e=this;while(e.firstElementChild)e=e.firstElementChild;return e}).append(this)),this},wrapInner:function(n){return m(n)?this.each(function(e){S(this).wrapInner(n.call(this,e))}):this.each(function(){var e=S(this),t=e.contents();t.length?t.wrapAll(n):e.append(n)})},wrap:function(t){var n=m(t);return this.each(function(e){S(this).wrapAll(n?t.call(this,e):t)})},unwrap:function(e){return this.parent(e).not(&quot;body&quot;).each(function(){S(this).replaceWith(this.childNodes)}),this}}),S.expr.pseudos.hidden=function(e){return!S.expr.pseudos.visible(e)},S.expr.pseudos.visible=function(e){return!!(e.offsetWidth||e.offsetHeight||e.getClientRects().length)},S.ajaxSettings.xhr=function(){try{return new C.XMLHttpRequest}catch(e){}};var Bt={0:200,1223:204},$t=S.ajaxSettings.xhr();y.cors=!!$t&amp;&amp;&quot;withCredentials&quot;in $t,y.ajax=$t=!!$t,S.ajaxTransport(function(i){var o,a;if(y.cors||$t&amp;&amp;!i.crossDomain)return{send:function(e,t){var n,r=i.xhr();if(r.open(i.type,i.url,i.async,i.username,i.password),i.xhrFields)for(n in i.xhrFields)r[n]=i.xhrFields[n];for(n in i.mimeType&amp;&amp;r.overrideMimeType&amp;&amp;r.overrideMimeType(i.mimeType),i.crossDomain||e[&quot;X-Requested-With&quot;]||(e[&quot;X-Requested-With&quot;]=&quot;XMLHttpRequest&quot;),e)r.setRequestHeader(n,e[n]);o=function(e){return function(){o&amp;&amp;(o=a=r.onload=r.onerror=r.onabort=r.ontimeout=r.onreadystatechange=null,&quot;abort&quot;===e?r.abort():&quot;error&quot;===e?&quot;number&quot;!=typeof r.status?t(0,&quot;error&quot;):t(r.status,r.statusText):t(Bt[r.status]||r.status,r.statusText,&quot;text&quot;!==(r.responseType||&quot;text&quot;)||&quot;string&quot;!=typeof r.responseText?{binary:r.response}:{text:r.responseText},r.getAllResponseHeaders()))}},r.onload=o(),a=r.onerror=r.ontimeout=o(&quot;error&quot;),void 0!==r.onabort?r.onabort=a:r.onreadystatechange=function(){4===r.readyState&amp;&amp;C.setTimeout(function(){o&amp;&amp;a()})},o=o(&quot;abort&quot;);try{r.send(i.hasContent&amp;&amp;i.data||null)}catch(e){if(o)throw e}},abort:function(){o&amp;&amp;o()}}}),S.ajaxPrefilter(function(e){e.crossDomain&amp;&amp;(e.contents.script=!1)}),S.ajaxSetup({accepts:{script:&quot;text/javascript, application/javascript, application/ecmascript, application/x-ecmascript&quot;},contents:{script:/\b(?:java|ecma)script\b/},converters:{&quot;text script&quot;:function(e){return S.globalEval(e),e}}}),S.ajaxPrefilter(&quot;script&quot;,function(e){void 0===e.cache&amp;&amp;(e.cache=!1),e.crossDomain&amp;&amp;(e.type=&quot;GET&quot;)}),S.ajaxTransport(&quot;script&quot;,function(n){var r,i;if(n.crossDomain||n.scriptAttrs)return{send:function(e,t){r=S(&quot;&lt;script>&quot;).attr(n.scriptAttrs||{}).prop({charset:n.scriptCharset,src:n.url}).on(&quot;load error&quot;,i=function(e){r.remove(),i=null,e&amp;&amp;t(&quot;error&quot;===e.type?404:200,e.type)}),E.head.appendChild(r[0])},abort:function(){i&amp;&amp;i()}}});var _t,zt=[],Ut=/(=)\?(?=&amp;|$)|\?\?/;S.ajaxSetup({jsonp:&quot;callback&quot;,jsonpCallback:function(){var e=zt.pop()||S.expando+&quot;_&quot;+wt.guid++;return this[e]=!0,e}}),S.ajaxPrefilter(&quot;json jsonp&quot;,function(e,t,n){var r,i,o,a=!1!==e.jsonp&amp;&amp;(Ut.test(e.url)?&quot;url&quot;:&quot;string&quot;==typeof e.data&amp;&amp;0===(e.contentType||&quot;&quot;).indexOf(&quot;application/x-www-form-urlencoded&quot;)&amp;&amp;Ut.test(e.data)&amp;&amp;&quot;data&quot;);if(a||&quot;jsonp&quot;===e.dataTypes[0])return r=e.jsonpCallback=m(e.jsonpCallback)?e.jsonpCallback():e.jsonpCallback,a?e[a]=e[a].replace(Ut,&quot;$1&quot;+r):!1!==e.jsonp&amp;&amp;(e.url+=(Tt.test(e.url)?&quot;&amp;&quot;:&quot;?&quot;)+e.jsonp+&quot;=&quot;+r),e.converters[&quot;script json&quot;]=function(){return o||S.error(r+&quot; was not called&quot;),o[0]},e.dataTypes[0]=&quot;json&quot;,i=C[r],C[r]=function(){o=arguments},n.always(function(){void 0===i?S(C).removeProp(r):C[r]=i,e[r]&amp;&amp;(e.jsonpCallback=t.jsonpCallback,zt.push(r)),o&amp;&amp;m(i)&amp;&amp;i(o[0]),o=i=void 0}),&quot;script&quot;}),y.createHTMLDocument=((_t=E.implementation.createHTMLDocument(&quot;&quot;).body).innerHTML=&quot;&lt;form>&lt;/form>&lt;form>&lt;/form>&quot;,2===_t.childNodes.length),S.parseHTML=function(e,t,n){return&quot;string&quot;!=typeof e?[]:(&quot;boolean&quot;==typeof t&amp;&amp;(n=t,t=!1),t||(y.createHTMLDocument?((r=(t=E.implementation.createHTMLDocument(&quot;&quot;)).createElement(&quot;base&quot;)).href=E.location.href,t.head.appendChild(r)):t=E),o=!n&amp;&amp;[],(i=N.exec(e))?[t.createElement(i[1])]:(i=xe([e],t,o),o&amp;&amp;o.length&amp;&amp;S(o).remove(),S.merge([],i.childNodes)));var r,i,o},S.fn.load=function(e,t,n){var r,i,o,a=this,s=e.indexOf(&quot; &quot;);return-1&lt;s&amp;&amp;(r=ht(e.slice(s)),e=e.slice(0,s)),m(t)?(n=t,t=void 0):t&amp;&amp;&quot;object&quot;==typeof t&amp;&amp;(i=&quot;POST&quot;),0&lt;a.length&amp;&amp;S.ajax({url:e,type:i||&quot;GET&quot;,dataType:&quot;html&quot;,data:t}).done(function(e){o=arguments,a.html(r?S(&quot;&lt;div>&quot;).append(S.parseHTML(e)).find(r):e)}).always(n&amp;&amp;function(e,t){a.each(function(){n.apply(this,o||[e.responseText,t,e])})}),this},S.expr.pseudos.animated=function(t){return S.grep(S.timers,function(e){return t===e.elem}).length},S.offset={setOffset:function(e,t,n){var r,i,o,a,s,u,l=S.css(e,&quot;position&quot;),c=S(e),f={};&quot;static&quot;===l&amp;&amp;(e.style.position=&quot;relative&quot;),s=c.offset(),o=S.css(e,&quot;top&quot;),u=S.css(e,&quot;left&quot;),(&quot;absolute&quot;===l||&quot;fixed&quot;===l)&amp;&amp;-1&lt;(o+u).indexOf(&quot;auto&quot;)?(a=(r=c.position()).top,i=r.left):(a=parseFloat(o)||0,i=parseFloat(u)||0),m(t)&amp;&amp;(t=t.call(e,n,S.extend({},s))),null!=t.top&amp;&amp;(f.top=t.top-s.top+a),null!=t.left&amp;&amp;(f.left=t.left-s.left+i),&quot;using&quot;in t?t.using.call(e,f):c.css(f)}},S.fn.extend({offset:function(t){if(arguments.length)return void 0===t?this:this.each(function(e){S.offset.setOffset(this,t,e)});var e,n,r=this[0];return r?r.getClientRects().length?(e=r.getBoundingClientRect(),n=r.ownerDocument.defaultView,{top:e.top+n.pageYOffset,left:e.left+n.pageXOffset}):{top:0,left:0}:void 0},position:function(){if(this[0]){var e,t,n,r=this[0],i={top:0,left:0};if(&quot;fixed&quot;===S.css(r,&quot;position&quot;))t=r.getBoundingClientRect();else{t=this.offset(),n=r.ownerDocument,e=r.offsetParent||n.documentElement;while(e&amp;&amp;(e===n.body||e===n.documentElement)&amp;&amp;&quot;static&quot;===S.css(e,&quot;position&quot;))e=e.parentNode;e&amp;&amp;e!==r&amp;&amp;1===e.nodeType&amp;&amp;((i=S(e).offset()).top+=S.css(e,&quot;borderTopWidth&quot;,!0),i.left+=S.css(e,&quot;borderLeftWidth&quot;,!0))}return{top:t.top-i.top-S.css(r,&quot;marginTop&quot;,!0),left:t.left-i.left-S.css(r,&quot;marginLeft&quot;,!0)}}},offsetParent:function(){return this.map(function(){var e=this.offsetParent;while(e&amp;&amp;&quot;static&quot;===S.css(e,&quot;position&quot;))e=e.offsetParent;return e||re})}}),S.each({scrollLeft:&quot;pageXOffset&quot;,scrollTop:&quot;pageYOffset&quot;},function(t,i){var o=&quot;pageYOffset&quot;===i;S.fn[t]=function(e){return $(this,function(e,t,n){var r;if(x(e)?r=e:9===e.nodeType&amp;&amp;(r=e.defaultView),void 0===n)return r?r[i]:e[t];r?r.scrollTo(o?r.pageXOffset:n,o?n:r.pageYOffset):e[t]=n},t,e,arguments.length)}}),S.each([&quot;top&quot;,&quot;left&quot;],function(e,n){S.cssHooks[n]=Fe(y.pixelPosition,function(e,t){if(t)return t=We(e,n),Pe.test(t)?S(e).position()[n]+&quot;px&quot;:t})}),S.each({Height:&quot;height&quot;,Width:&quot;width&quot;},function(a,s){S.each({padding:&quot;inner&quot;+a,content:s,&quot;&quot;:&quot;outer&quot;+a},function(r,o){S.fn[o]=function(e,t){var n=arguments.length&amp;&amp;(r||&quot;boolean&quot;!=typeof e),i=r||(!0===e||!0===t?&quot;margin&quot;:&quot;border&quot;);return $(this,function(e,t,n){var r;return x(e)?0===o.indexOf(&quot;outer&quot;)?e[&quot;inner&quot;+a]:e.document.documentElement[&quot;client&quot;+a]:9===e.nodeType?(r=e.documentElement,Math.max(e.body[&quot;scroll&quot;+a],r[&quot;scroll&quot;+a],e.body[&quot;offset&quot;+a],r[&quot;offset&quot;+a],r[&quot;client&quot;+a])):void 0===n?S.css(e,t,i):S.style(e,t,n,i)},s,n?e:void 0,n)}})}),S.each([&quot;ajaxStart&quot;,&quot;ajaxStop&quot;,&quot;ajaxComplete&quot;,&quot;ajaxError&quot;,&quot;ajaxSuccess&quot;,&quot;ajaxSend&quot;],function(e,t){S.fn[t]=function(e){return this.on(t,e)}}),S.fn.extend({bind:function(e,t,n){return this.on(e,null,t,n)},unbind:function(e,t){return this.off(e,null,t)},delegate:function(e,t,n,r){return this.on(t,e,n,r)},undelegate:function(e,t,n){return 1===arguments.length?this.off(e,&quot;**&quot;):this.off(t,e||&quot;**&quot;,n)},hover:function(e,t){return this.mouseenter(e).mouseleave(t||e)}}),S.each(&quot;blur focus focusin focusout resize scroll click dblclick mousedown mouseup mousemove mouseover mouseout mouseenter mouseleave change select submit keydown keypress keyup contextmenu&quot;.split(&quot; &quot;),function(e,n){S.fn[n]=function(e,t){return 0&lt;arguments.length?this.on(n,null,e,t):this.trigger(n)}});var Xt=/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g;S.proxy=function(e,t){var n,r,i;if(&quot;string&quot;==typeof t&amp;&amp;(n=e[t],t=e,e=n),m(e))return r=s.call(arguments,2),(i=function(){return e.apply(t||this,r.concat(s.call(arguments)))}).guid=e.guid=e.guid||S.guid++,i},S.holdReady=function(e){e?S.readyWait++:S.ready(!0)},S.isArray=Array.isArray,S.parseJSON=JSON.parse,S.nodeName=A,S.isFunction=m,S.isWindow=x,S.camelCase=X,S.type=w,S.now=Date.now,S.isNumeric=function(e){var t=S.type(e);return(&quot;number&quot;===t||&quot;string&quot;===t)&amp;&amp;!isNaN(e-parseFloat(e))},S.trim=function(e){return null==e?&quot;&quot;:(e+&quot;&quot;).replace(Xt,&quot;&quot;)},&quot;function&quot;==typeof define&amp;&amp;define.amd&amp;&amp;define(&quot;jquery&quot;,[],function(){return S});var Vt=C.jQuery,Gt=C.$;return S.noConflict=function(e){return C.$===S&amp;&amp;(C.$=Gt),e&amp;&amp;C.jQuery===S&amp;&amp;(C.jQuery=Vt),S},&quot;undefined&quot;==typeof e&amp;&amp;(C.jQuery=C.$=S),S});


/* Demo page specific styles */
.demo-headline {
    background-color: #110D95;
    background-image: url('https://phptravels.com/assets/img/head.webp');
    background-position: bottom right;
    background-repeat: no-repeat;
    background-size: contain;
    color: white;
    padding: 5rem 0;
    margin-bottom: 3rem;
    position: relative;
    overflow: hidden;
}
.demo-headline-content {
    position: relative;
    z-index: 2;
    max-width: 60%;
}
.demo-headline-small {
    font-size: 0.9rem;
    font-weight: 500;
    letter-spacing: 2px;
    text-transform: uppercase;
    margin-bottom: 1rem;
    opacity: 0.9;
}
.demo-headline h1 {
    font-size: 3.5rem;
    font-weight: 700;
    letter-spacing: -2px;
    margin-bottom: 1.5rem;
    line-height: 1.1;
}
.demo-headline p {
    font-size: 1.3rem;
    opacity: 0.9;
    font-weight: 300;
    margin-top: 0;
    line-height: 1.5;
    margin-bottom: 2rem;
}
.demo-section {
    padding: 4rem 0;
    background: #f8f9fa;
}
.demo-form-container {
    background: white;
    border-radius: 16px;
    border: 1px solid #e8e8e8;
    padding: 2rem;
    height: 100%;
    /* box-shadow removed */
}
.demo-form-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 1.5rem;
}
.demo-form-divider {
    border-color: #e8e8e8;
    margin: 1.5rem 0;
}
.demo-form-group {
    margin-bottom: 1rem;
}
.demo-form-control {
    border: 1px solid #e8e8e8;
    border-radius: 8px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
}
.demo-form-control:focus {
    border-color: #110D95;
    box-shadow: none;
}
.demo-submit-btn {
    background: #007bff;
    border: none;
    border-radius: 8px;
    padding: 1rem 2rem;
    font-weight: 600;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
}
.demo-submit-btn:hover {
    background: #0056b3;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,123,255,0.3);
}
.demo-submit-btn:disabled {
    background: #6c757d;
    transform: none;
    box-shadow: none;
}
.demo-captcha-container {
    background: #f8f9fa;
    border-radius: 8px;
    padding: 1rem;
    text-align: center;
    margin-bottom: 1rem;
}
.demo-captcha-result {
    border: 1px solid #e8e8e8;
    border-radius: 8px;
    height: 58px;
    font-size: 1.1rem;
    text-align: center;
}
.demo-image-container {
    background: rgba(124, 145, 161, 0.11);
    border-radius: 16px;
    padding: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
}
.demo-image-container img {
    max-width: 100%;
    height: auto;
    object-fit: cover;
    border-radius: 8px;
}
.demo-success-container {
    text-align: center;
    padding: 2rem 0;
}
.demo-success-icon {
    margin-bottom: 1.5rem;
}
.demo-success-title {
    font-size: 2rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 1rem;
}
.demo-success-text {
    color: #666;
    line-height: 1.6;
    font-size: 1rem;
}
.demo-faq-section {
    padding: 4rem 0;
    background: white;
}
.demo-faq-title {
    font-size: 2rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 2rem;
}
.demo-faq-subtitle {
    font-size: 1rem;
    color: #666;
    margin-bottom: 3rem;
}
.demo-accordion-item {
    border: 1px solid #e8e8e8;
    border-radius: 12px !important;
    margin-bottom: 1rem;
    overflow: hidden;
}
.demo-accordion-button {
    background: white;
    border: none;
    padding: 1.25rem 1.5rem;
    font-weight: 600;
    color: #1a1a1a;
    font-size: 1rem;
}
.demo-accordion-button:not(.collapsed) {
    background: #f8f9fa;
    color: #110D95;
    box-shadow: none;
}
.demo-accordion-button:focus {
    box-shadow: 0 0 0 0.2rem rgba(0,123,255,0.25);
}
.demo-accordion-body {
    padding: 1.5rem;
    background: white;
    color: #666;
    line-height: 1.6;
}
.demo-loading-container {
    text-align: center;
    padding: 3rem 0;
}
.demo-alert {
    border-radius: 8px;
    padding: 0.75rem 1rem;
    margin-top: 1rem;
    border: none;
    background: #f8d7da;
    color: #721c24;
}
/* Responsive adjustments */
@media (max-width: 768px) {
    .demo-headline-content {
        max-width: 100%;
        text-align: center;
    }
    .demo-headline {
        padding: 2.5rem 0;
    }
    .demo-headline h1 {
        font-size: 2.2rem;
    }
    .demo-headline p {
        font-size: 1rem;
    }
    .demo-section {
        padding: 2rem 0;
    }
    .demo-form-container {
        padding: 1.5rem;
        margin-bottom: 2rem;
    }
}
@media (max-width: 576px) {
    .demo-headline h1 {
        font-size: 1.8rem;
    }
    .demo-form-container {
        padding: 1rem;
    }
    .demo-faq-section {
        padding: 2rem 0;
    }
}
/* Animation styles for success checkmark */
#check-group {
    animation: 0.32s ease-in-out 1.03s check-group;
    transform-origin: center;
}
#check-group #check {
    animation: 0.34s cubic-bezier(0.65, 0, 1, 1) 0.8s forwards check;
    stroke-dasharray: 0, 75px;
    stroke-linecap: round;
    stroke-linejoin: round;
}
#check-group #outline {
    animation: 0.38s ease-in outline;
    transform: rotate(0deg);
    transform-origin: center;
}
#check-group #white-circle {
    animation: 0.35s ease-in 0.35s forwards circle;
    transform: none;
    transform-origin: center;
}
@keyframes outline {
    from { stroke-dasharray: 0, 345.576px; }
    to { stroke-dasharray: 345.576px, 345.576px; }
}
@keyframes circle {
    from { transform: scale(1); }
    to { transform: scale(0); }
}
@keyframes check {
    from { stroke-dasharray: 0, 75px; }
    to { stroke-dasharray: 75px, 75px; }
}
@keyframes check-group {
    from { transform: scale(1); }
    50% { transform: scale(1.09); }
    to { transform: scale(1); }
}
/* Utility classes */
.no-spin-buttons {
    -webkit-appearance: none;
    -moz-appearance: textfield;
}
input[type=&quot;number&quot;]::-webkit-inner-spin-button,
input[type=&quot;number&quot;]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
}



    
        
            Demo
            Live Demonstration
            Test drive our complete travel booking platform with full features and functionality
        
    



    
        
            
                
                    Request Demo Access
                    

                    
                        
                            
                                
                                    
                                    First Name
                                
                            
                            
                                
                                    
                                    Last Name
                                
                            
                        

                        
                            
                                
                                    
                                            Country
    Afghanistan +93
    Albania +355
    Algeria +213
    American Samoa +1684
    Andorra +376
    Angola +244
    Anguilla +1264
    Antarctica +0
    Antigua and Barbuda +1268
    Argentina +54
    Armenia +374
    Aruba +297
    Australia +61
    Austria +43
    Azerbaijan +994
    Bahamas +1242
    Bahrain +973
    Bangladesh +880
    Barbados +1246
    Belarus +375
    Belgium +32
    Belize +501
    Benin +229
    Bermuda +1441
    Bhutan +975
    Bolivia +591
    Bosnia and Herzegovina +387
    Botswana +267
    Brazil +55
    British Indian Ocean Territory +246
    Brunei Darussalam +673
    Bulgaria +359
    Burkina Faso +226
    Burundi +257
    Cambodia +855
    Cameroon +237
    Canada +1
    Cape Verde +238
    Cayman Islands +1345
    Central African Republic +236
    Chad +235
    Chile +56
    China +86
    Christmas Island +61
    Cocos (Keeling) Islands +672
    Colombia +57
    Comoros +269
    Congo +243
    Congo, the Democratic Republic of the +242
    Cook Islands +682
    Costa Rica +506
    Cote D'Ivoire +225
    Croatia +385
    Cuba +53
    Cyprus +357
    Czech Republic +420
    Denmark +45
    Djibouti +253
    Dominica +1767
    Dominican Republic +1809
    Ecuador +593
    Egypt +20
    El Salvador +503
    Equatorial Guinea +240
    Eritrea +291
    Estonia +372
    Ethiopia +251
    Falkland Islands (Malvinas) +500
    Faroe Islands +298
    Fiji +679
    Finland +358
    France +33
    French Guiana +594
    French Polynesia +689
    French Southern Territories +0
    Gabon +241
    Gambia +220
    Georgia +995
    Germany +49
    Ghana +233
    Gibraltar +350
    Greece +30
    Greenland +299
    Grenada +1473
    Guadeloupe +590
    Guam +1671
    Guatemala +502
    Guinea +224
    Guinea-Bissau +245
    Guyana +592
    Haiti +509
    Holy See (Vatican City State) +39
    Honduras +504
    Hong Kong +852
    Hungary +36
    Iceland +354
    India +91
    Indonesia +62
    Iran, Islamic Republic of +98
    Iraq +964
    Ireland +353
    Israel +972
    Italy +39
    Jamaica +1876
    Japan +81
    Jordan +962
    Kazakhstan +7
    Kenya +254
    Kiribati +686
    Korea, Democratic People's Republic of +850
    Korea, Republic of +82
    Kuwait +965
    Kyrgyzstan +996
    Lao People's Democratic Republic +856
    Latvia +371
    Lebanon +961
    Lesotho +266
    Liberia +231
    Libyan Arab Jamahiriya +218
    Liechtenstein +423
    Lithuania +370
    Luxembourg +352
    Macao +853
    Macedonia, the Former Yugoslav Republic of +389
    Madagascar +261
    Malawi +265
    Malaysia +60
    Maldives +960
    Mali +223
    Malta +356
    Marshall Islands +692
    Martinique +596
    Mauritania +222
    Mauritius +230
    Mayotte +269
    Mexico +52
    Micronesia, Federated States of +691
    Moldova, Republic of +373
    Monaco +377
    Mongolia +976
    Montserrat +1664
    Morocco +212
    Mozambique +258
    Myanmar +95
    Namibia +264
    Nauru +674
    Nepal +977
    Netherlands +31
    Netherlands Antilles +599
    New Caledonia +687
    New Zealand +64
    Nicaragua +505
    Niger +227
    Nigeria +234
    Niue +683
    Norfolk Island +672
    Northern Mariana Islands +1670
    Norway +47
    Oman +968
    Pakistan +92
    Palau +680
    Palestinian Territory, Occupied +970
    Panama +507
    Papua New Guinea +675
    Paraguay +595
    Peru +51
    Philippines +63
    Pitcairn +0
    Poland +48
    Portugal +351
    Puerto Rico +1787
    Qatar +974
    Reunion +262
    Romania +40
    Russia +70
    Rwanda +250
    Saint Kitts and Nevis +1869
    Saint Lucia +1758
    Saint Pierre and Miquelon +508
    Saint Vincent and the Grenadines +1784
    Samoa +684
    San Marino +378
    Sao Tome and Principe +239
    Saudi Arabia +966
    Senegal +221
    Serbia and Montenegro +381
    Seychelles +248
    Sierra Leone +232
    Singapore +65
    Slovakia +421
    Slovenia +386
    Solomon Islands +677
    Somalia +252
    South Africa +27
    South Georgia and the South Sandwich Islands +0
    Spain +34
    Sri Lanka +94
    Sudan +249
    Swaziland +268
    Sweden +46
    Switzerland +41
    Syrian Arab Republic +963
    Taiwan, Province of China +886
    Tajikistan +992
    Tanzania, United Republic of +255
    Thailand +66
    Timor-Leste +670
    Togo +228
    Tokelau +690
    Tonga +676
    Trinidad and Tobago +1868
    Tunisia +216
    Turkey +90
    Turkmenistan +7370
    Turks and Caicos Islands +1649
    Tuvalu +688
    Uganda +256
    Ukraine +380
    United Arab Emirates +971
    United Kingdom +44
    United States +1
    Uruguay +598
    Uzbekistan +998
    Vanuatu +678
    Venezuela +58
    Viet Nam +84
    Virgin Islands, British +1284
    Virgin Islands, U.s. +1340
    Wallis and Futuna +681
    Western Sahara +212
    Yemen +967
    Zambia +260
    Zimbabwe +263
    Serbia +381
    Asia / Pacific Region +0
    Montenegro +382
    Aland Islands +358
    Curacao +599
    Guernsey +44
    Isle of Man +44
    Jersey +44
    Kosovo +381
    Saint Barthelemy +590
    Saint Martin +590
    Sint Maarten +1
    South Sudan +211



var requestUrl = &quot;https://ipwhois.app/json/&quot;;
fetch(requestUrl)
.then(function(response) { return response.json(); })
.then(function(c) {
var user_country = c['country_phone'];
user_country = user_country.replace('+', '');
$(&quot;[data-country-phonecode='&quot; + user_country + &quot;']&quot;).prop(&quot;selected&quot;, true);
// $(&quot;[name='country_id']&quot;).val(user_country)
console.log(user_country);
});
                                    
                                    Select Country
                                
                            
                            
                                
                                    
                                    WhatsApp Number
                                
                            
                        

                        
                            
                            Business Name
                        

                        
                            
                            Email Address
                        

                        
                            
                                
                                    Submit
                                    
                                        
                                    
                                

                                
                                    
                                
                            
                            
                                
                                    
                                        
                                            
                                                4 + 10 = ?
                                            
                                        
                                    
                                    
                                
                            
                        

                        
                            The whatsapp number is not valid. avoid adding country number, Zero or + signs before the number
                        

                        
                            
                                
                                    
                                
                                
                                    
                                
                                
                                    
                                
                            
                        
                    
                    
                    
                        
                        
                            
                                
                                    
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                         Thank you!
                        We have sent your demo credentials to your email please check your mailbox. if demo credentials not found inbox please check spam folder
                    
                
            
            
                
                    
                
            
        
    



    
        Frequently Asked Questions
        Find answers to common questions about our demo platform and testing process.
        
                            
                    
                        
                            How can I test the PHPTRAVELS demo?                        
                    
                    
                        
                            Submit the demo request form to receive complete access credentials. You'll be able to test everything from the admin panel to the client interface on our live demo platform.                        
                    
                
                            
                    
                        
                            Can I save content during testing?                        
                    
                    
                        
                            Yes, but please note that the database content resets every few minutes to maintain demo integrity. However, all content can be fully customized in your purchased version.                        
                    
                
                            
                    
                        
                            What features are included in the demo?                        
                    
                    
                        
                            Our demo includes all features needed to run your travel business. Test the complete application from backend administration to frontend booking experience at no cost.                        
                    
                
                            
                    
                        
                            Do you charge for demo testing?                        
                    
                    
                        
                            No, we offer completely free application testing. Access our demo anytime by submitting the request form - no hidden charges or commitments.                        
                    
                
                            
                    
                        
                            Can I test the application on my own hosting?                        
                    
                    
                        
                            Demo testing is available exclusively on our servers for security reasons. Once satisfied with the demo, you can purchase and install on your hosting with our complete support.                        
                    
                
                            
                    
                        
                            Is the application mobile responsive?                        
                    
                    
                        
                            Yes, all themes and interfaces are fully responsive and optimized for desktop, laptop, tablet, and mobile devices. We recommend testing on multiple devices to experience the complete responsiveness.                        
                    
                
                    
    



// Generate random captcha numbers
numb1 = Math.floor((Math.random() * 10) + 1);
numb2 = Math.floor((Math.random() * 10) + 1);
document.getElementById(&quot;numb1&quot;).innerHTML = numb1;
document.getElementById(&quot;numb2&quot;).innerHTML = numb2;

// Form submission handler
$(&quot;#demo&quot;).click(function() {
    var country_id = $('.country_id').val();
    let numbers = numb1 + numb2;
    let number = $('#number').val();

    // Validation
    if ($(&quot;.first_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your first name&quot;);
        return;
    }
    if ($(&quot;.last_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your last name&quot;);
        return;
    }
    if ($(&quot;.company_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your business name&quot;);
        return;
    }
    if ($(&quot;.whatsapp_number&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your WhatsApp number&quot;);
        return;
    }
    if (country_id === &quot;&quot;) {
        alert(&quot;Please select your country&quot;);
        return;
    }
    if ($(&quot;.email&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your email address&quot;);
        return;
    }
    if (number == &quot;&quot;) {
        alert('Please solve the math problem');
        return;
    }
    
    if (numbers == number) {
        let first_name = $(&quot;.first_name&quot;).val();
        let last_name = $(&quot;.last_name&quot;).val();
        let company_name = $(&quot;.company_name&quot;).val();
        let whatsapp_number = $(&quot;.whatsapp_number&quot;).val();
        let email = $(&quot;.email&quot;).val();

        $(&quot;.btn_submit&quot;).hide();
        $(&quot;.alert_msg&quot;).hide();
        $(&quot;.btn_loading&quot;).show();

        var form = new FormData();
        form.append(&quot;first_name&quot;, first_name);
        form.append(&quot;last_name&quot;, last_name);
        form.append(&quot;email&quot;, email);
        form.append(&quot;company_name&quot;, company_name);
        form.append(&quot;whatsapp_number&quot;, whatsapp_number);
        form.append(&quot;country_id&quot;, country_id);
        form.append(&quot;lead_type&quot;, &quot;demo&quot;);

        var settings = {
            &quot;url&quot;: &quot;https://app.phptravels.com/api_demo_signup.php&quot;,
            &quot;method&quot;: &quot;POST&quot;,
            &quot;timeout&quot;: 0,
            &quot;headers&quot;: {},
            &quot;processData&quot;: false,
            &quot;mimeType&quot;: &quot;multipart/form-data&quot;,
            &quot;contentType&quot;: false,
            &quot;data&quot;: form
        };

        $.ajax(settings).done(function(response) {
            $(&quot;.btn_submit&quot;).show();
            $(&quot;.btn_loading&quot;).hide();

            try {
                var result;
                if (typeof response === 'string') {
                    result = JSON.parse(response);
                } else {
                    result = response;
                }
                // Show thank you block if success
                if (result.status === true || result.status === &quot;true&quot; || result.status == true) {
                    $(&quot;.from_box&quot;).hide();
                    $(&quot;.completed&quot;).css('display', 'block');
                 } else {
                    $(&quot;.alert_msg&quot;).show();
                    $(&quot;.alert_msg p&quot;).text(result.message || &quot;An error occurred&quot;);
                }
            } catch (e) {
                $(&quot;.alert_msg&quot;).show();
                $(&quot;.alert_msg p&quot;).text(&quot;An error occurred. Please try again.&quot;);
            }
        }).fail(function(xhr, status, error) {
            $(&quot;.btn_submit&quot;).show();
            $(&quot;.btn_loading&quot;).hide();
            $(&quot;.alert_msg&quot;).show();
            $(&quot;.alert_msg p&quot;).text(&quot;Connection error. Please try again.&quot;);
        });
    } else {
        alert(&quot;The math result is incorrect. Please try again.&quot;);
    }
});





  
  
  




Aggregated Solution
With our multiple channel aggregation feature now we can get inventory from different API's with realtime pricing and booking.




  GDS &amp; OTA Integration
  Realtime API's and Dashboards
  100% Opensource Platform Structure
  Highly Scalable and Extensive
  Secure by Additinal Layers
  Latest Technology Implemented
  Self-Hosted Structure
  Developer Friendly Documentation
  Live Testing Demonstration








    We are Growing!

        
            
                
                
                    
                

                
                
                    
                        
                            
                                
                                    4K+
                                
                                Live Websites
                            
                        
                        
                        
                                
                                    180K+
                                
                                Users
                            
                        
                        
                        
                                
                                    6M+
                                
                                Bookings
                            
                        

                    
                

                
                const counters = document.querySelectorAll('.counter');
                // Main function
                for(let n of counters) {
                  const updateCount = () => {
                    const target = + n.getAttribute('data-target');
                    const count = + n.innerText;
                    const speed = 500; // change animation speed here
                    const inc = target / speed;
                    if(count &lt; target) {
                      n.innerText = Math.ceil(count + inc);
                      setTimeout(updateCount, 1);
                    } else {
                      n.innerText = target;
                    }
                  }
                  updateCount();
                }
                

                
                
                    
                        
                        
                    

                    

                    
                        
                            
                        
                        
                            
                        
                    
                
            
    

    



      

        
        
          
            
              
                
                
              
            
            
              
                
                  
                    24FLIGHTS
                  
                
                
                  Nancy - 24fights.com
                  
                    PHPTRAVELS Transformed our Business with Exceptional
                    Solution &amp; Service.
                  
                  
                    View Website
                    
                      
                        
                      
                    
                  
                
              
            
          
        

        
        
          
            
              
                
                
              
            
            
              
                
                  TAZKIRA
                
                
                  M. Saeed - Tazkira.ae
                  
                      PHPTRAVELS is the real Secret Behind Tazkira's 80% of Online Sales &amp; Bookings
                  

                  View Website
                      
                          
                      
                  
                
              
            
          
        

        
        
          
            
              
                
                
              
            
            
              
                
                    BOOKING JORDAN
                
                
                    Khalid Nawalah - Bookingjordan.com
                  
                      Our Achievements are Powered by PHPTRAVELS &amp; Their Super Supportive Team.
                  

                  
                    View Website
                    
                        
                    
                
                
              
            
          
        

        
        
          
          
            
              
            
          
          
            
              
            
          
        
      
    




// ALL SLIDERS
const sliderElements = document.querySelectorAll(&quot;.slider--container&quot;);
const totalSliderElements = sliderElements.length;

// THIS INDICATE THE THE CURRENTLY SHOW SLIDER VALUE
let currentSlider = 1;

// SLIDER CONTROLS
const sliderControls = document.querySelectorAll(&quot;.slider--control&quot;);

sliderControls.forEach((sliderControl) => {
  sliderControl.addEventListener(&quot;click&quot;, function () {
    // THIS WILL GTE THE VALUE(DATA_VALUE) ACCORDING TO WHICH SLIDER MOIVE ForWARD OR REVERSE
    const _sliderValue = Number(this.getAttribute(&quot;data-value&quot;));

    // SET THE THE VALUE(DATA_HIDDEN) TO TRUE OF THE CUrRENTLY ACTIVE SLIDER
    sliderElements[currentSlider - 1].setAttribute(&quot;data-hidden&quot;, &quot;true&quot;);

    // INCREASE OR DECREASE THE SLIDER VALUE ACCORDINGLY
    currentSlider = currentSlider + _sliderValue;

    // CHECK IF THE CURRENT VALUE IS GREATE THEN THE TOTAL NUMBER OF THE AVAILABLE SLIDER
    // OR EQUAL TO ZERO
    if (currentSlider - 1 === totalSliderElements) {
      currentSlider = 1;
    } else if (currentSlider === 0) {
      currentSlider = totalSliderElements;
    }

    // REQUESTED SLIDER
    sliderElements[currentSlider - 1].setAttribute(&quot;data-hidden&quot;, &quot;false&quot;);
    sliderElements[currentSlider - 1].classList.add(&quot;custom--animation&quot;);
  });
});

// WHEN CLICK ON FIRST CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client1() {
  document.getElementById(&quot;client1&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube1&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}

// WHEN CLICK ON SECOND CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client2() {
  document.getElementById(&quot;client2&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube2&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}

// WHEN CLICK ON THIRD CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client3() {
  document.getElementById(&quot;client3&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube3&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}






// const myCarouselElement = document.querySelector(&quot;#autoSlider&quot;);

// myCarouselElement.addEventListener(&quot;slide.bs.carousel&quot;, (event) => {
//   document.querySelector(&quot;.carousel-item.active&quot;).classList.remove(&quot;show&quot;);
// });

// myCarouselElement.addEventListener(&quot;slid.bs.carousel&quot;, (event) => {
//   document.querySelector(&quot;.carousel-item.active&quot;).classList.add(&quot;show&quot;);
// });

// FIRST SLDIER CARD DATA
const firstSlider = [

  {
    url: &quot;https://tazkira.ae/&quot;,
    imgSrc: &quot;&quot;
  },


];

// THIS TEMPLATE CONATINS THE HTML FOR EACH SLDIE CARD
// const sliderCard = document.querySelector(&quot;#sliderCard&quot;);
// GETTING THE SLIDER ONE WRAPPER
// const carouselItem1 = document.querySelector(&quot;.carousel--item-1 > div.d-flex&quot;);
// CALLING THE CREATE SLDIER CARD FUNCTION
// createSliderCard(firstSlider, carouselItem1);

// SECOND SLIDER CARD DATA
// const sectSlider = [
//   {

//   }

// ];

// GETTING THE SECOND SLIDER WRAPPER
// const carouselItem2 = document.querySelector(&quot;.carousel--item-2 > div.d-flex&quot;);
// CALLING THE CREATE SLDIER CARD FUNCTION
// createSliderCard(sectSlider, carouselItem2);

// THIS FUNCTION WILL CRETAE THE CARD FOR EACH SLIDER AND APPEND IN THE SPECIFIC SLIDER WRAPPER
// function createSliderCard(cardData, cardPID) {
//   let _slideItem;
//   cardData.forEach((_slide) => {
//     _slideItem = sliderCard.content.cloneNode(true);

//     _slideItem.querySelector(&quot;a&quot;).setAttribute(&quot;href&quot;, _slide.url);
//     _slideItem.querySelector(&quot;a > img&quot;).setAttribute(&quot;src&quot;, _slide.imgSrc);
//     _slideItem.querySelector(&quot;a > img&quot;).setAttribute(&quot;href&quot;, _slide.alt);

//     cardPID.appendChild(_slideItem);
//   });
// }




/* Modern Newsletter Section */
.modern-newsletter {
    background: linear-gradient(135deg, #1a1a1a 0%, #2d2d2d 100%);
    padding: 4rem 0;
    position: relative;
    overflow: hidden;
}

.modern-newsletter::before {
    content: '';
    position: absolute;
    top: 0;
    right: 0;
    width: 100%;
    height: 100%;
    background: url('data:image/svg+xml,&lt;svg xmlns=&quot;http://www.w3.org/2000/svg&quot; viewBox=&quot;0 0 100 100&quot;>&lt;defs>&lt;pattern id=&quot;grid&quot; width=&quot;10&quot; height=&quot;10&quot; patternUnits=&quot;userSpaceOnUse&quot;>&lt;path d=&quot;M 10 0 L 0 0 0 10&quot; fill=&quot;none&quot; stroke=&quot;rgba(255,255,255,0.05)&quot; stroke-width=&quot;1&quot;/>&lt;/pattern>&lt;/defs>&lt;rect width=&quot;100&quot; height=&quot;100&quot; fill=&quot;url(%23grid)&quot;/>&lt;/svg>') repeat;
    pointer-events: none;
}

.newsletter-content {
    position: relative;
    z-index: 2;
}

.newsletter-icon {
    width: 60px;
    height: 60px;
    margin-bottom: 1rem;
}

.newsletter-title {
    font-size: 2rem;
    font-weight: 700;
    color: white;
    margin-bottom: 0.5rem;
}

.newsletter-subtitle {
    color: rgba(255,255,255,0.8);
    font-size: 1.1rem;
    margin-bottom: 2rem;
}

.newsletter-form {
    display: flex;
    gap: 1rem;
    max-width: 500px;
    margin: 0 auto;
}

.newsletter-input {
    flex: 1;
    padding: 1rem 1.5rem;
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 8px;
    background: rgba(255,255,255,0.1);
    color: white;
    font-size: 1rem;
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
}

.newsletter-input::placeholder {
    color: rgba(255,255,255,0.6);
}

.newsletter-input:focus {
    outline: none;
    border-color: #007bff;
    background: rgba(255,255,255,0.15);
}

.newsletter-btn {
    padding: 1rem 2rem;
    background: #007bff;
    border: none;
    border-radius: 8px;
    color: white;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;
}

.newsletter-btn:hover {
    background: #0056b3;
    transform: translateY(-2px);
}

/* Modern Footer */
.modern-footer {
    background: #0a0a0a;
    color: white;
    padding: 4rem 0 0;
    position: relative;
}

.footer-main {
    padding-bottom: 3rem;
    border-bottom: 1px solid rgba(255,255,255,0.1);
}

.footer-brand {
    margin-bottom: 2rem;
}

.brand-logo-container {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
}

.brand-logo {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    object-fit: cover;
}

.brand-text h3 {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
    color: white;
}

.brand-text p {
    font-size: 0.9rem;
    color: rgba(255,255,255,0.7);
    margin: 0;
}

.brand-description {
    color: rgba(255,255,255,0.7);
    line-height: 1.6;
    margin-bottom: 2rem;
    max-width: 350px;
}

.social-links {
    display: flex;
    gap: 1rem;
}

.social-link {
    width: 44px;
    height: 44px;
    background: rgba(255,255,255,0.1);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    text-decoration: none;
}

.social-link:hover {
    background: #007bff;
    transform: translateY(-2px);
}

.social-link svg {
    width: 20px;
    height: 20px;
    fill: white;
}

.footer-section {
    margin-bottom: 2rem;
}

.footer-section h4 {
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: white;
    position: relative;
}

.footer-section h4::after {
    content: '';
    position: absolute;
    bottom: -8px;
    left: 0;
    width: 30px;
    height: 2px;
    background: #007bff;
}

.footer-links {
    list-style: none;
    padding: 0;
    margin: 0;
}

.footer-links li {
    margin-bottom: 0.75rem;
}

.footer-links a {
    color: rgba(255,255,255,0.7);
    text-decoration: none;
    transition: color 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
}

.footer-links a:hover {
    color: #007bff;
}

.hiring-badge {
    background: linear-gradient(45deg, #ff4757, #ff6b7a);
    color: white;
    font-size: 0.7rem;
    padding: 0.2rem 0.5rem;
    border-radius: 12px;
    font-weight: 600;
    text-transform: uppercase;
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
}

.payment-section {
    background: rgba(255,255,255,0.03);
    padding: 2rem;
    border-radius: 12px;
    margin-bottom: 2rem;
}

.payment-title {
    font-weight: 600;
    margin-bottom: 1rem;
    color: white;
}

.payment-description {
    color: rgba(255,255,255,0.7);
    line-height: 1.6;
    margin-bottom: 1.5rem;
    font-size: 0.9rem;
}

.payment-gateway-image {
    max-width: 100%;
    height: auto;
    opacity: 0.8;
    transition: opacity 0.3s ease;
}

.payment-gateway-image:hover {
    opacity: 1;
}

.certifications {
    display: flex;
    gap: 2rem;
    align-items: center;
}

.cert-item img {
    /* max-width: 100px; */
    height: auto;
    opacity: 0.7;
    transition: opacity 0.3s ease;
}

.cert-item img:hover {
    opacity: 1;
}

.footer-bottom {
    padding: 2rem 0;
    background: rgba(0,0,0,0.5);
}

.footer-bottom-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
}

.copyright {
    color: rgba(255,255,255,0.6);
    font-size: 0.9rem;
}

.legal-links {
    display: flex;
    gap: 2rem;
    flex-wrap: wrap;
}

.legal-links a {
    color: rgba(255,255,255,0.6);
    text-decoration: none;
    font-size: 0.9rem;
    transition: color 0.3s ease;
}

.legal-links a:hover {
    color: #007bff;
}

.whatsapp-float {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    z-index: 1000;
    animation: float 3s ease-in-out infinite;
}

.whatsapp-btn {
    width: 60px;
    height: 60px;
    background: linear-gradient(135deg, #25d366, #128c7e);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
    box-shadow: 0 4px 20px rgba(37, 211, 102, 0.4);
    transition: all 0.3s ease;
}

.whatsapp-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 6px 25px rgba(37, 211, 102, 0.6);
}

.whatsapp-btn svg {
    width: 30px;
    height: 30px;
    fill: white;
}

@keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
}

/* Responsive Design */
@media (max-width: 768px) {
    .modern-newsletter {
        padding: 3rem 0;
        text-align: center;
    }
    
    .newsletter-title {
        font-size: 1.5rem;
    }
    
    .newsletter-form {
        flex-direction: column;
    }
    
    .modern-footer {
        padding: 3rem 0 0;
    }
    
    .footer-bottom-content {
        flex-direction: column;
        text-align: center;
    }
    
    .legal-links {
        justify-content: center;
    }
    
    .certifications {
        flex-direction: column;
        gap: 1rem;
    }
    
    .social-links {
        justify-content: center;
    }
}

@media (max-width: 576px) {
    .brand-logo-container {
        flex-direction: column;
        text-align: center;
    }
    
    .legal-links {
        flex-direction: column;
        gap: 1rem;
    }
    
    .whatsapp-float {
        bottom: 1rem;
        right: 1rem;
    }
    
    .whatsapp-btn {
        width: 50px;
        height: 50px;
    }
    
    .whatsapp-btn svg {
        width: 25px;
        height: 25px;
    }
}




    
        
            
                
            
            Stay Updated
            Get the latest updates on travel technology and industry insights
            
                
                Subscribe
            
        
    




    
        
            
                
                
                    
                        
                            
                            
                                PHPTRAVELS
                                Travel Tech Partner
                            
                        
                        Leading travel technology solutions for modern businesses. Build, customize, and scale your travel platform with our comprehensive booking engine.
                        
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                    
                    
                

                
                                    
                        
                            Product
                            
                                                                    
                                        
                                            Demo                                                                                    
                                    
                                                                    
                                        
                                            Mobile Apps                                                                                    
                                    
                                                                    
                                        
                                            Pricing                                                                                    
                                    
                                                                    
                                        
                                            Features                                                                                    
                                    
                                                                    
                                        
                                            Technology                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Resources
                            
                                                                    
                                        
                                            Changelog                                                                                    
                                    
                                                                    
                                        
                                            Updates                                                                                    
                                    
                                                                    
                                        
                                            Requirements                                                                                    
                                    
                                                                    
                                        
                                            Affiliate                                                                                    
                                    
                                                                    
                                        
                                            Road Map                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Services
                            
                                                                    
                                        
                                            Product Services                                                                                    
                                    
                                                                    
                                        
                                            Customizations                                                                                    
                                    
                                                                    
                                        
                                            EAN Integration                                                                                    
                                    
                                                                    
                                        
                                            Cloud Hosting                                                                                    
                                    
                                                                    
                                        
                                            Support                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Company
                            
                                                                    
                                        
                                            About Us                                                                                    
                                    
                                                                    
                                        
                                            Blog                                                                                    
                                    
                                                                    
                                        
                                            The Team                                                                                    
                                    
                                                                    
                                        
                                            Contact Us                                                                                    
                                    
                                                                    
                                        
                                            Jobs                                                                                            We're Hiring!
                                                                                    
                                    
                                                            
                        
                    
                            

            
            
                
                    
                        Accepted Payment Methods
                        
                            We support all major payment gateways including PayPal, Credit Cards, Western Union, Skrill, and Transferwise for your convenience.
                        
                        
                    
                
                
                    
                        
                            
                        
                        
                            
                        
                    
                
            
        
    

    
    
        
            
                
                    © 2023 PHPTRAVELS. All Rights Reserved.
                
                
                                            
                            Privacy Policy                        
                                            
                            Terms of Service                        
                                            
                            Abuse Policy                        
                                            
                            Live Chat                        
                                            
                            Content Program                        
                                    
            
        
    

 









   
   
   

   

   
   setTimeout(function() {
      window.__lc = window.__lc || {};
      window.__lc.license = 4618001;
      ;(function(n,t,c){function i(n){return e._h?e._h.apply(null,n):e._q.push(n)}var e={_q:[],_h:null,_v:&quot;2.0&quot;,on:function(){i([&quot;on&quot;,c.call(arguments)])},once:function(){i([&quot;once&quot;,c.call(arguments)])},off:function(){i([&quot;off&quot;,c.call(arguments)])},get:function(){if(!e._h)throw new Error(&quot;[LiveChatWidget] You can't use getters before load.&quot;);return i([&quot;get&quot;,c.call(arguments)])},call:function(){i([&quot;call&quot;,c.call(arguments)])},init:function(){var n=t.createElement(&quot;script&quot;);n.async=!0,n.type=&quot;text/javascript&quot;,n.src=&quot;https://cdn.livechatinc.com/tracking.js&quot;,t.head.appendChild(n)}};!n.__lc.asyncInit&amp;&amp;e.init(),n.LiveChatWidget=n.LiveChatWidget||e}(window,document,[].slice))
   }, 5000);
   





// import Swup from 'https://unpkg.com/swup@4?module';
// const swup = new Swup();







   // document.addEventListener('DOMContentLoaded', function () {
   //    let exitIntentShown = false;

   //    // Check if the user has opted out of seeing the modal for the next 90 days
   //    const isOptedOut = localStorage.getItem('exitIntentOptOut');
   //    const optOutExpiration = localStorage.getItem('optOutExpiration');

   //    if (isOptedOut &amp;&amp; optOutExpiration) {
   //       const expirationDate = new Date(optOutExpiration);
   //       const currentDate = new Date();
   //       if (currentDate &lt; expirationDate) {
   //          // User has opted out and the 90-day period has not expired
   //          exitIntentShown = true;
   //       }
   //    }

   //    document.addEventListener('mouseleave', function (e) {
   //       if (e.clientY &lt; 0 &amp;&amp; !exitIntentShown) {
   //          const exitModal = new bootstrap.Modal(document.getElementById('exitModal'));
   //          exitModal.show();
   //          exitIntentShown = true;

   //          // Reset the exit intent flag when modal is closed
   //          document.getElementById('exitModal').addEventListener('hidden.bs.modal', function () {
   //             exitIntentShown = false;
   //          });
   //       }
   //    });

   //    document.getElementById('dontShowAgainBtn').addEventListener('click', function () {
   //       // Set a flag in localStorage to indicate that the user opted out
   //       const expirationDate = new Date();
   //       expirationDate.setDate(expirationDate.getDate() + 90); // Set expiration date for 90 days
   //       localStorage.setItem('exitIntentOptOut', 'true');
   //       localStorage.setItem('optOutExpiration', expirationDate.toISOString()); // Store the expiration date
   //       // Hide the modal
   //       const exitModal = new bootstrap.Modal(document.getElementById('exitModal'));
   //       exitModal.hide();
   //    });
   // });





// Get the current URL parameters
function getQueryParam(param) {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(param);
}

// Update all links with the referral ID
function updateLinksWithRef(refID) {
    if (!refID) return; // If no valid refID, don't update links

    const links = document.querySelectorAll('a');

    links.forEach(link => {
        const url = new URL(link.href);

        // Append the ref parameter to the URL only if it doesn't already exist
        if (!url.searchParams.has('ref')) {
            url.searchParams.append('ref', refID);
        }

        link.href = url.toString();
    });
}

// Check if the URL has a ref parameter
const refID = getQueryParam('ref');
if (refID &amp;&amp; refID !== 'null') {
    // Store the refID in sessionStorage so it persists across page loads
    sessionStorage.setItem('ref', refID);
}

// Ensure that links include the ref parameter only if it's present in the session or the URL
const storedRef = sessionStorage.getItem('ref');
if (storedRef &amp;&amp; storedRef !== 'null') {
    updateLinksWithRef(storedRef);
}






// SHOW - HIDE MENU ON MOBILE
function togglemenu() { var element = document.getElementById(&quot;responsive-menu&quot;); element.classList.toggle(&quot;show-menu&quot;);  }
function anchor() { var element = document.getElementById(&quot;responsive-menu&quot;); element.classList.remove(&quot;show-menu&quot;); }

/*! Waves v0.7.6  */
(function(window,factory){'use strict';if(typeof define==='function'&amp;&amp;define.amd){define([],function(){window.Waves=factory.call(window);return window.Waves})}else if(typeof exports==='object'){module.exports=factory.call(window)}else{window.Waves=factory.call(window)}})(typeof global==='object'?global:this,function(){'use strict';var Waves=Waves||{};var $$=document.querySelectorAll.bind(document);var toString=Object.prototype.toString;var isTouchAvailable='ontouchstart' in window;function isWindow(obj){return obj!==null&amp;&amp;obj===obj.window}
function getWindow(elem){return isWindow(elem)?elem:elem.nodeType===9&amp;&amp;elem.defaultView}
function isObject(value){var type=typeof value;return type==='function'||type==='object'&amp;&amp;!!value}
function isDOMNode(obj){return isObject(obj)&amp;&amp;obj.nodeType>0}
function getWavesElements(nodes){var stringRepr=toString.call(nodes);if(stringRepr==='[object String]'){return $$(nodes)}else if(isObject(nodes)&amp;&amp;/^\[object (Array|HTMLCollection|NodeList|Object)\]$/.test(stringRepr)&amp;&amp;nodes.hasOwnProperty('length')){return nodes}else if(isDOMNode(nodes)){return[nodes]}
return[]}
function offset(elem){var docElem,win,box={top:0,left:0},doc=elem&amp;&amp;elem.ownerDocument;docElem=doc.documentElement;if(typeof elem.getBoundingClientRect!==typeof undefined){box=elem.getBoundingClientRect()}
win=getWindow(doc);return{top:box.top+win.pageYOffset-docElem.clientTop,left:box.left+win.pageXOffset-docElem.clientLeft}}
function convertStyle(styleObj){var style='';for(var prop in styleObj){if(styleObj.hasOwnProperty(prop)){style+=(prop+':'+styleObj[prop]+';')}}
return style}
var Effect={duration:750,delay:200,show:function(e,element,velocity){if(e.button===2){return!1}
element=element||this;var ripple=document.createElement('div');ripple.className='waves-ripple waves-rippling';element.appendChild(ripple);var pos=offset(element);var relativeY=0;var relativeX=0;if('touches' in e&amp;&amp;e.touches.length){relativeY=(e.touches[0].pageY-pos.top);relativeX=(e.touches[0].pageX-pos.left)}else{relativeY=(e.pageY-pos.top);relativeX=(e.pageX-pos.left)}
relativeX=relativeX>=0?relativeX:0;relativeY=relativeY>=0?relativeY:0;var scale='scale('+((element.clientWidth/100)*3)+')';var translate='translate(0,0)';if(velocity){translate='translate('+(velocity.x)+'px, '+(velocity.y)+'px)'}
ripple.setAttribute('data-hold',Date.now());ripple.setAttribute('data-x',relativeX);ripple.setAttribute('data-y',relativeY);ripple.setAttribute('data-scale',scale);ripple.setAttribute('data-translate',translate);var rippleStyle={top:relativeY+'px',left:relativeX+'px'};ripple.classList.add('waves-notransition');ripple.setAttribute('style',convertStyle(rippleStyle));ripple.classList.remove('waves-notransition');rippleStyle['-webkit-transform']=scale+' '+translate;rippleStyle['-moz-transform']=scale+' '+translate;rippleStyle['-ms-transform']=scale+' '+translate;rippleStyle['-o-transform']=scale+' '+translate;rippleStyle.transform=scale+' '+translate;rippleStyle.opacity='1';var duration=e.type==='mousemove'?2500:Effect.duration;rippleStyle['-webkit-transition-duration']=duration+'ms';rippleStyle['-moz-transition-duration']=duration+'ms';rippleStyle['-o-transition-duration']=duration+'ms';rippleStyle['transition-duration']=duration+'ms';ripple.setAttribute('style',convertStyle(rippleStyle))},hide:function(e,element){element=element||this;var ripples=element.getElementsByClassName('waves-rippling');for(var i=0,len=ripples.length;i&lt;len;i++){removeRipple(e,element,ripples[i])}
if(isTouchAvailable){element.removeEventListener('touchend',Effect.hide);element.removeEventListener('touchcancel',Effect.hide)}
element.removeEventListener('mouseup',Effect.hide);element.removeEventListener('mouseleave',Effect.hide)}};var TagWrapper={input:function(element){var parent=element.parentNode;if(parent.tagName.toLowerCase()==='i'&amp;&amp;parent.classList.contains('waves-effect')){return}
var wrapper=document.createElement('i');wrapper.className=element.className+' waves-input-wrapper';element.className='waves-button-input';parent.replaceChild(wrapper,element);wrapper.appendChild(element);var elementStyle=window.getComputedStyle(element,null);var color=elementStyle.color;var backgroundColor=elementStyle.backgroundColor;wrapper.setAttribute('style','color:'+color+';background:'+backgroundColor);element.setAttribute('style','background-color:rgba(0,0,0,0);')},img:function(element){var parent=element.parentNode;if(parent.tagName.toLowerCase()==='i'&amp;&amp;parent.classList.contains('waves-effect')){return}
var wrapper=document.createElement('i');parent.replaceChild(wrapper,element);wrapper.appendChild(element)}};function removeRipple(e,el,ripple){if(!ripple){return}
ripple.classList.remove('waves-rippling');var relativeX=ripple.getAttribute('data-x');var relativeY=ripple.getAttribute('data-y');var scale=ripple.getAttribute('data-scale');var translate=ripple.getAttribute('data-translate');var diff=Date.now()-Number(ripple.getAttribute('data-hold'));var delay=350-diff;if(delay&lt;0){delay=0}
if(e.type==='mousemove'){delay=150}
var duration=e.type==='mousemove'?2500:Effect.duration;setTimeout(function(){var style={top:relativeY+'px',left:relativeX+'px',opacity:'0','-webkit-transition-duration':duration+'ms','-moz-transition-duration':duration+'ms','-o-transition-duration':duration+'ms','transition-duration':duration+'ms','-webkit-transform':scale+' '+translate,'-moz-transform':scale+' '+translate,'-ms-transform':scale+' '+translate,'-o-transform':scale+' '+translate,'transform':scale+' '+translate};ripple.setAttribute('style',convertStyle(style));setTimeout(function(){try{el.removeChild(ripple)}catch(e){return!1}},duration)},delay)}
var TouchHandler={touches:0,allowEvent:function(e){var allow=!0;if(/^(mousedown|mousemove)$/.test(e.type)&amp;&amp;TouchHandler.touches){allow=!1}
return allow},registerEvent:function(e){var eType=e.type;if(eType==='touchstart'){TouchHandler.touches+=1}else if(/^(touchend|touchcancel)$/.test(eType)){setTimeout(function(){if(TouchHandler.touches){TouchHandler.touches-=1}},500)}}};function getWavesEffectElement(e){if(TouchHandler.allowEvent(e)===!1){return null}
var element=null;var target=e.target||e.srcElement;while(target.parentElement){if((!(target instanceof SVGElement))&amp;&amp;target.classList.contains('waves-effect')){element=target;break}
target=target.parentElement}
return element}
function showEffect(e){var element=getWavesEffectElement(e);if(element!==null){if(element.disabled||element.getAttribute('disabled')||element.classList.contains('disabled')){return}
TouchHandler.registerEvent(e);if(e.type==='touchstart'&amp;&amp;Effect.delay){var hidden=!1;var timer=setTimeout(function(){timer=null;Effect.show(e,element)},Effect.delay);var hideEffect=function(hideEvent){if(timer){clearTimeout(timer);timer=null;Effect.show(e,element)}
if(!hidden){hidden=!0;Effect.hide(hideEvent,element)}
removeListeners()};var touchMove=function(moveEvent){if(timer){clearTimeout(timer);timer=null}
hideEffect(moveEvent);removeListeners()};element.addEventListener('touchmove',touchMove,!1);element.addEventListener('touchend',hideEffect,!1);element.addEventListener('touchcancel',hideEffect,!1);var removeListeners=function(){element.removeEventListener('touchmove',touchMove);element.removeEventListener('touchend',hideEffect);element.removeEventListener('touchcancel',hideEffect)}}else{Effect.show(e,element);if(isTouchAvailable){element.addEventListener('touchend',Effect.hide,!1);element.addEventListener('touchcancel',Effect.hide,!1)}
element.addEventListener('mouseup',Effect.hide,!1);element.addEventListener('mouseleave',Effect.hide,!1)}}}
Waves.init=function(options){var body=document.body;options=options||{};if('duration' in options){Effect.duration=options.duration}
if('delay' in options){Effect.delay=options.delay}
if(isTouchAvailable){body.addEventListener('touchstart',showEffect,!1);body.addEventListener('touchcancel',TouchHandler.registerEvent,!1);body.addEventListener('touchend',TouchHandler.registerEvent,!1)}
body.addEventListener('mousedown',showEffect,!1)};Waves.attach=function(elements,classes){elements=getWavesElements(elements);if(toString.call(classes)==='[object Array]'){classes=classes.join(' ')}
classes=classes?' '+classes:'';var element,tagName;for(var i=0,len=elements.length;i&lt;len;i++){element=elements[i];tagName=element.tagName.toLowerCase();if(['input','img'].indexOf(tagName)!==-1){TagWrapper[tagName](element);element=element.parentElement}
if(element.className.indexOf('waves-effect')===-1){element.className+=' waves-effect'+classes}}};Waves.ripple=function(elements,options){elements=getWavesElements(elements);var elementsLen=elements.length;options=options||{};options.wait=options.wait||0;options.position=options.position||null;if(elementsLen){var element,pos,off,centre={},i=0;var mousedown={type:'mousedown',button:1};var hideRipple=function(mouseup,element){return function(){Effect.hide(mouseup,element)}};for(;i&lt;elementsLen;i++){element=elements[i];pos=options.position||{x:element.clientWidth/2,y:element.clientHeight/2};off=offset(element);centre.x=off.left+pos.x;centre.y=off.top+pos.y;mousedown.pageX=centre.x;mousedown.pageY=centre.y;Effect.show(mousedown,element);if(options.wait>=0&amp;&amp;options.wait!==null){var mouseup={type:'mouseup',button:1};setTimeout(hideRipple(mouseup,element),options.wait)}}}};Waves.calm=function(elements){elements=getWavesElements(elements);var mouseup={type:'mouseup',button:1};for(var i=0,len=elements.length;i&lt;len;i++){Effect.hide(mouseup,elements[i])}};Waves.displayEffect=function(options){console.error('Waves.displayEffect() has been deprecated and will be removed in future version. Please use Waves.init() to initialize Waves effect');Waves.init(options)};return Waves})

Waves.init();
Waves.attach('a');
Waves.attach('.logo');

// CONSOLE
console.log(&quot;%cHi there! 👋 We are phptravels. we are disrupting the travel industry world-wide.&quot;, &quot;font-size:14px&quot;);
console.log(&quot;%cAny doubts, get in touch at info@phptravels.com&quot;, &quot;font-size:12px&quot;);



(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement('script');d.innerHTML=&quot;window.__CF$cv$params={r:'95d87a4f391d712c',t:'MTc1MjIzODc2My4wMDAwMDA='};var a=document.createElement('script');a.nonce='';a.src='/cdn-cgi/challenge-platform/scripts/jsd/main.js';document.getElementsByTagName('head')[0].appendChild(a);&quot;;b.getElementsByTagName('head')[0].appendChild(d)}}if(document.body){var a=document.createElement('iframe');a.height=1;a.width=1;a.style.position='absolute';a.style.top=0;a.style.left=0;a.style.border='none';a.style.visibility='hidden';document.body.appendChild(a);if('loading'!==document.readyState)c();else if(window.addEventListener)document.addEventListener('DOMContentLoaded',c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);'loading'!==document.readyState&amp;&amp;(document.onreadystatechange=e,c())}}}})();

  /html[1]</value>
      <webElementGuid>cdac4a32-12dc-444b-8cba-5d679a548830</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>e0b52161-663d-4ad8-bcce-d7ca0583832e</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
      <webElementGuid>251af5b7-406f-43ad-9e75-f8c5afb99483</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>98430ca5-81bc-4e78-850c-51a8efad76fd</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;
Book Your Free Demo Now - Phptravels






































  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());
  gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;G-6TWLEQHRC1&quot; , &quot;'&quot; , &quot;);




   {
   &quot;@context&quot; : &quot;http://schema.org&quot;,
   &quot;@type&quot; : &quot;Corporation&quot;,
   &quot;brand&quot;: &quot;PHPTRAVELS&quot;,
   &quot;description&quot;: &quot;Targeting an incredible small niche, PHPtravels is a script designed for travel agencies that want to move their operations online, helping them setup a fully-working hotel room booking system.\n\nPHPtravels lets users search hotels, check room availability, book a room for their desired time period, and then pay for it, all via a simple interface, that not only looks good, but is also very manageable and customizable via a specially crafted administration panel.\n\nVarious accounts are available, for customers, for travel agents, and above all, for the site webmaster (admin).\n\nAnything on the PHPtravels website is customizable, from the frontend skin, to the active languages, available currencies, and not last, the hotels and rooms.\n\nIf you can get past the fact you need a commercial license to run PHPtravels, the system can be of incredible help if you plan to sell vacations and trips online as well.&quot;,
   &quot;name&quot; : &quot;PHPTRAVELS&quot;,
   &quot;founders&quot;: [
      &quot;Qasim Hussain&quot;
   ],
   &quot;foundingDate&quot;: &quot;2014-05&quot;,
   &quot;foundingLocation&quot;: &quot;Lahore&quot;,
   &quot;knowsAbout&quot;: &quot;Create online travel agency using PHPTRAVELS products&quot;,
   &quot;legalName&quot;: &quot;PHPTRAVELS&quot;,
   &quot;logo&quot; : &quot;https://phptravels.com/assets/img/pages/media/icon-primary.png&quot;,
   &quot;numberOfEmployees&quot;: &quot;15&quot;,
   &quot;ownershipFundingInfo&quot;: &quot;https://phptravels.com/about-us/&quot;,
   &quot;url&quot; : &quot;https://phptravels.com/demo&quot;,
   &quot;sameAs&quot; : [
      &quot;https://www.facebook.com/phptravels&quot;,
      &quot;https://www.twitter.com/phptravels&quot;,
      &quot;https://snapchat.com/add/phptravels&quot;,
      &quot;https://instagram.com/phptravels_/&quot;,
      &quot;https://www.youtube.com/user/phptravels&quot;,
      &quot;https://www.linkedin.com/company/phptravels&quot;,
      &quot;https://www.pinterest.com/phptravels_pin/&quot;
   ],
   &quot;slogan&quot;: &quot;Travel technology partner&quot;,
   &quot;tickerSymbol&quot;: [
      &quot;NYSE:SHOP&quot;,
      &quot;TSX:SHOP&quot;
   ],
   &quot;awards&quot;: &quot;https://www.ivisa.com/visa-blog/php-travels&quot;
      }






  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());
  gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AW-419817699&quot; , &quot;'&quot; , &quot;);




// SAVE TRAFFIC TO DATABASE ON PAGE LOAD
setTimeout(function() {
var requestUrl = &quot;https://phptravels.com/visitor_details&quot;;
      fetch(requestUrl)
      .then(function(response) {
         return response.json();
      })
      .then(function(c) {
            if (typeof c.country_code === &quot;undefined&quot;) {
               // If country_code is undefined, log and return or handle appropriately
               console.log(&quot;Localhost traffic not counted&quot;);
            } else {
               console.log(c);
               var country = c.country_code.toUpperCase();

                const formdata = new FormData();
                formdata.append(&quot;country_code&quot;, country);
                formdata.append(&quot;user_ip&quot;, c.user_ip);
                formdata.append(&quot;page_link&quot;, window.location.href);
                formdata.append(&quot;user_agent&quot;, c.user_agent);
                formdata.append(&quot;http_referer&quot;, &quot;Direct Traffic&quot;);

                const requestOptions = {
                method: &quot;POST&quot;,
                body: formdata,
                redirect: &quot;follow&quot;
                };

                fetch(&quot;https://app.phptravels.com/api_website_traffic?&quot;, requestOptions)
                .then((response) => response.text())
                // .then((result) => console.log(result))
                .catch((error) => console.error(error));

            }
      });
  }, 2500);

  (function () {
      window.usermaven = window.usermaven || (function () { (window.usermavenQ = window.usermavenQ || []).push(arguments); })
      var t = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;),
          s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0];
      t.defer = true;
      t.id = &quot; , &quot;'&quot; , &quot;um-tracker&quot; , &quot;'&quot; , &quot;;
      t.setAttribute(&quot; , &quot;'&quot; , &quot;data-tracking-host&quot; , &quot;'&quot; , &quot;, &quot;https://events.usermaven.com&quot;)
      t.setAttribute(&quot; , &quot;'&quot; , &quot;data-key&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UMlduvNwrU&quot; , &quot;'&quot; , &quot;);
      t.setAttribute(&quot; , &quot;'&quot; , &quot;data-autocapture&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;);
      t.src = &quot; , &quot;'&quot; , &quot;https://t.usermaven.com/lib.js&quot; , &quot;'&quot; , &quot;;
      s.parentNode.insertBefore(t, s);
  })();



#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}





  
  
      
        
          We stand with Palestine
          
            
              .cls-1{fill:#fff;}
              .cls-2{fill:#007a3d;}
              .cls-3{fill:#ce1126;}
            
          Palestine
          
          
          
          
          Because Humanity Matters
        
      
  

    
      
        
          

          
               
                  
                  
               
            

          
          
            PHPTRAVELS  
            Travel Tech Partner
            
          
        

        
        
        

        
          
            
              
                Product
                
                  
                
              
              
              
                    Themes
                    
                  
                
                
                    Integrations
                    
                  
                
                
                  Customization

                    
                  
                
                
                    Technology

                  
                
                    Requirements

                    
                  
              
            
            
              
                Features
                
                  
                
              
              
                
                    Flights Modules

                  
                
                
                    Hotels Modules

                  
                
                    Tours Modules

                  
                
                    Cars Modules

                  
                
                    Blogs Modules

                  
                
                    CMS Modules

                  
                
                    Offers Modules

                  
                
                    Newsletter Modules

                  
              
            
            
              
                Company
                
                  
                
              
              
                
                    Clients

                  
                
                    Blogs

                  
                
                    Docs

                  
                
                    Contact Us

                  
                
                    About Us

                  
                
                    The Team

                  
                
                    Jobs

                  
                
                    Partners

                  
                
                    Media Kit
                    
                  
              
            
            
              Demo
            
            
              Pricing
            
          
          
            
              
              Talk to Sales
            Login
            Sign Up

          
        
      
    
  
  
/*! jQuery v3.6.0 | (c) OpenJS Foundation and other contributors | jquery.org/license */
!function(e,t){&quot;use strict&quot;;&quot;object&quot;==typeof module&amp;&amp;&quot;object&quot;==typeof module.exports?module.exports=e.document?t(e,!0):function(e){if(!e.document)throw new Error(&quot;jQuery requires a window with a document&quot;);return t(e)}:t(e)}(&quot;undefined&quot;!=typeof window?window:this,function(C,e){&quot;use strict&quot;;var t=[],r=Object.getPrototypeOf,s=t.slice,g=t.flat?function(e){return t.flat.call(e)}:function(e){return t.concat.apply([],e)},u=t.push,i=t.indexOf,n={},o=n.toString,v=n.hasOwnProperty,a=v.toString,l=a.call(Object),y={},m=function(e){return&quot;function&quot;==typeof e&amp;&amp;&quot;number&quot;!=typeof e.nodeType&amp;&amp;&quot;function&quot;!=typeof e.item},x=function(e){return null!=e&amp;&amp;e===e.window},E=C.document,c={type:!0,src:!0,nonce:!0,noModule:!0};function b(e,t,n){var r,i,o=(n=n||E).createElement(&quot;script&quot;);if(o.text=e,t)for(r in c)(i=t[r]||t.getAttribute&amp;&amp;t.getAttribute(r))&amp;&amp;o.setAttribute(r,i);n.head.appendChild(o).parentNode.removeChild(o)}function w(e){return null==e?e+&quot;&quot;:&quot;object&quot;==typeof e||&quot;function&quot;==typeof e?n[o.call(e)]||&quot;object&quot;:typeof e}var f=&quot;3.6.0&quot;,S=function(e,t){return new S.fn.init(e,t)};function p(e){var t=!!e&amp;&amp;&quot;length&quot;in e&amp;&amp;e.length,n=w(e);return!m(e)&amp;&amp;!x(e)&amp;&amp;(&quot;array&quot;===n||0===t||&quot;number&quot;==typeof t&amp;&amp;0&lt;t&amp;&amp;t-1 in e)}S.fn=S.prototype={jquery:f,constructor:S,length:0,toArray:function(){return s.call(this)},get:function(e){return null==e?s.call(this):e&lt;0?this[e+this.length]:this[e]},pushStack:function(e){var t=S.merge(this.constructor(),e);return t.prevObject=this,t},each:function(e){return S.each(this,e)},map:function(n){return this.pushStack(S.map(this,function(e,t){return n.call(e,t,e)}))},slice:function(){return this.pushStack(s.apply(this,arguments))},first:function(){return this.eq(0)},last:function(){return this.eq(-1)},even:function(){return this.pushStack(S.grep(this,function(e,t){return(t+1)%2}))},odd:function(){return this.pushStack(S.grep(this,function(e,t){return t%2}))},eq:function(e){var t=this.length,n=+e+(e&lt;0?t:0);return this.pushStack(0&lt;=n&amp;&amp;n&lt;t?[this[n]]:[])},end:function(){return this.prevObject||this.constructor()},push:u,sort:t.sort,splice:t.splice},S.extend=S.fn.extend=function(){var e,t,n,r,i,o,a=arguments[0]||{},s=1,u=arguments.length,l=!1;for(&quot;boolean&quot;==typeof a&amp;&amp;(l=a,a=arguments[s]||{},s++),&quot;object&quot;==typeof a||m(a)||(a={}),s===u&amp;&amp;(a=this,s--);s&lt;u;s++)if(null!=(e=arguments[s]))for(t in e)r=e[t],&quot;__proto__&quot;!==t&amp;&amp;a!==r&amp;&amp;(l&amp;&amp;r&amp;&amp;(S.isPlainObject(r)||(i=Array.isArray(r)))?(n=a[t],o=i&amp;&amp;!Array.isArray(n)?[]:i||S.isPlainObject(n)?n:{},i=!1,a[t]=S.extend(l,o,r)):void 0!==r&amp;&amp;(a[t]=r));return a},S.extend({expando:&quot;jQuery&quot;+(f+Math.random()).replace(/\D/g,&quot;&quot;),isReady:!0,error:function(e){throw new Error(e)},noop:function(){},isPlainObject:function(e){var t,n;return!(!e||&quot;[object Object]&quot;!==o.call(e))&amp;&amp;(!(t=r(e))||&quot;function&quot;==typeof(n=v.call(t,&quot;constructor&quot;)&amp;&amp;t.constructor)&amp;&amp;a.call(n)===l)},isEmptyObject:function(e){var t;for(t in e)return!1;return!0},globalEval:function(e,t,n){b(e,{nonce:t&amp;&amp;t.nonce},n)},each:function(e,t){var n,r=0;if(p(e)){for(n=e.length;r&lt;n;r++)if(!1===t.call(e[r],r,e[r]))break}else for(r in e)if(!1===t.call(e[r],r,e[r]))break;return e},makeArray:function(e,t){var n=t||[];return null!=e&amp;&amp;(p(Object(e))?S.merge(n,&quot;string&quot;==typeof e?[e]:e):u.call(n,e)),n},inArray:function(e,t,n){return null==t?-1:i.call(t,e,n)},merge:function(e,t){for(var n=+t.length,r=0,i=e.length;r&lt;n;r++)e[i++]=t[r];return e.length=i,e},grep:function(e,t,n){for(var r=[],i=0,o=e.length,a=!n;i&lt;o;i++)!t(e[i],i)!==a&amp;&amp;r.push(e[i]);return r},map:function(e,t,n){var r,i,o=0,a=[];if(p(e))for(r=e.length;o&lt;r;o++)null!=(i=t(e[o],o,n))&amp;&amp;a.push(i);else for(o in e)null!=(i=t(e[o],o,n))&amp;&amp;a.push(i);return g(a)},guid:1,support:y}),&quot;function&quot;==typeof Symbol&amp;&amp;(S.fn[Symbol.iterator]=t[Symbol.iterator]),S.each(&quot;Boolean Number String Function Array Date RegExp Object Error Symbol&quot;.split(&quot; &quot;),function(e,t){n[&quot;[object &quot;+t+&quot;]&quot;]=t.toLowerCase()});var d=function(n){var e,d,b,o,i,h,f,g,w,u,l,T,C,a,E,v,s,c,y,S=&quot;sizzle&quot;+1*new Date,p=n.document,k=0,r=0,m=ue(),x=ue(),A=ue(),N=ue(),j=function(e,t){return e===t&amp;&amp;(l=!0),0},D={}.hasOwnProperty,t=[],q=t.pop,L=t.push,H=t.push,O=t.slice,P=function(e,t){for(var n=0,r=e.length;n&lt;r;n++)if(e[n]===t)return n;return-1},R=&quot;checked|selected|async|autofocus|autoplay|controls|defer|disabled|hidden|ismap|loop|multiple|open|readonly|required|scoped&quot;,M=&quot;[\\x20\\t\\r\\n\\f]&quot;,I=&quot;(?:\\\\[\\da-fA-F]{1,6}&quot;+M+&quot;?|\\\\[^\\r\\n\\f]|[\\w-]|[^\0-\\x7f])+&quot;,W=&quot;\\[&quot;+M+&quot;*(&quot;+I+&quot;)(?:&quot;+M+&quot;*([*^$|!~]?=)&quot;+M+&quot;*(?:&quot; , &quot;'&quot; , &quot;((?:\\\\.|[^\\\\&quot; , &quot;'&quot; , &quot;])*)&quot; , &quot;'&quot; , &quot;|\&quot;((?:\\\\.|[^\\\\\&quot;])*)\&quot;|(&quot;+I+&quot;))|)&quot;+M+&quot;*\\]&quot;,F=&quot;:(&quot;+I+&quot;)(?:\\(((&quot; , &quot;'&quot; , &quot;((?:\\\\.|[^\\\\&quot; , &quot;'&quot; , &quot;])*)&quot; , &quot;'&quot; , &quot;|\&quot;((?:\\\\.|[^\\\\\&quot;])*)\&quot;)|((?:\\\\.|[^\\\\()[\\]]|&quot;+W+&quot;)*)|.*)\\)|)&quot;,B=new RegExp(M+&quot;+&quot;,&quot;g&quot;),$=new RegExp(&quot;^&quot;+M+&quot;+|((?:^|[^\\\\])(?:\\\\.)*)&quot;+M+&quot;+$&quot;,&quot;g&quot;),_=new RegExp(&quot;^&quot;+M+&quot;*,&quot;+M+&quot;*&quot;),z=new RegExp(&quot;^&quot;+M+&quot;*([>+~]|&quot;+M+&quot;)&quot;+M+&quot;*&quot;),U=new RegExp(M+&quot;|>&quot;),X=new RegExp(F),V=new RegExp(&quot;^&quot;+I+&quot;$&quot;),G={ID:new RegExp(&quot;^#(&quot;+I+&quot;)&quot;),CLASS:new RegExp(&quot;^\\.(&quot;+I+&quot;)&quot;),TAG:new RegExp(&quot;^(&quot;+I+&quot;|[*])&quot;),ATTR:new RegExp(&quot;^&quot;+W),PSEUDO:new RegExp(&quot;^&quot;+F),CHILD:new RegExp(&quot;^:(only|first|last|nth|nth-last)-(child|of-type)(?:\\(&quot;+M+&quot;*(even|odd|(([+-]|)(\\d*)n|)&quot;+M+&quot;*(?:([+-]|)&quot;+M+&quot;*(\\d+)|))&quot;+M+&quot;*\\)|)&quot;,&quot;i&quot;),bool:new RegExp(&quot;^(?:&quot;+R+&quot;)$&quot;,&quot;i&quot;),needsContext:new RegExp(&quot;^&quot;+M+&quot;*[>+~]|:(even|odd|eq|gt|lt|nth|first|last)(?:\\(&quot;+M+&quot;*((?:-\\d)?\\d*)&quot;+M+&quot;*\\)|)(?=[^-]|$)&quot;,&quot;i&quot;)},Y=/HTML$/i,Q=/^(?:input|select|textarea|button)$/i,J=/^h\d$/i,K=/^[^{]+\{\s*\[native \w/,Z=/^(?:#([\w-]+)|(\w+)|\.([\w-]+))$/,ee=/[+~]/,te=new RegExp(&quot;\\\\[\\da-fA-F]{1,6}&quot;+M+&quot;?|\\\\([^\\r\\n\\f])&quot;,&quot;g&quot;),ne=function(e,t){var n=&quot;0x&quot;+e.slice(1)-65536;return t||(n&lt;0?String.fromCharCode(n+65536):String.fromCharCode(n>>10|55296,1023&amp;n|56320))},re=/([\0-\x1f\x7f]|^-?\d)|^-$|[^\0-\x1f\x7f-\uFFFF\w-]/g,ie=function(e,t){return t?&quot;\0&quot;===e?&quot;\ufffd&quot;:e.slice(0,-1)+&quot;\\&quot;+e.charCodeAt(e.length-1).toString(16)+&quot; &quot;:&quot;\\&quot;+e},oe=function(){T()},ae=be(function(e){return!0===e.disabled&amp;&amp;&quot;fieldset&quot;===e.nodeName.toLowerCase()},{dir:&quot;parentNode&quot;,next:&quot;legend&quot;});try{H.apply(t=O.call(p.childNodes),p.childNodes),t[p.childNodes.length].nodeType}catch(e){H={apply:t.length?function(e,t){L.apply(e,O.call(t))}:function(e,t){var n=e.length,r=0;while(e[n++]=t[r++]);e.length=n-1}}}function se(t,e,n,r){var i,o,a,s,u,l,c,f=e&amp;&amp;e.ownerDocument,p=e?e.nodeType:9;if(n=n||[],&quot;string&quot;!=typeof t||!t||1!==p&amp;&amp;9!==p&amp;&amp;11!==p)return n;if(!r&amp;&amp;(T(e),e=e||C,E)){if(11!==p&amp;&amp;(u=Z.exec(t)))if(i=u[1]){if(9===p){if(!(a=e.getElementById(i)))return n;if(a.id===i)return n.push(a),n}else if(f&amp;&amp;(a=f.getElementById(i))&amp;&amp;y(e,a)&amp;&amp;a.id===i)return n.push(a),n}else{if(u[2])return H.apply(n,e.getElementsByTagName(t)),n;if((i=u[3])&amp;&amp;d.getElementsByClassName&amp;&amp;e.getElementsByClassName)return H.apply(n,e.getElementsByClassName(i)),n}if(d.qsa&amp;&amp;!N[t+&quot; &quot;]&amp;&amp;(!v||!v.test(t))&amp;&amp;(1!==p||&quot;object&quot;!==e.nodeName.toLowerCase())){if(c=t,f=e,1===p&amp;&amp;(U.test(t)||z.test(t))){(f=ee.test(t)&amp;&amp;ye(e.parentNode)||e)===e&amp;&amp;d.scope||((s=e.getAttribute(&quot;id&quot;))?s=s.replace(re,ie):e.setAttribute(&quot;id&quot;,s=S)),o=(l=h(t)).length;while(o--)l[o]=(s?&quot;#&quot;+s:&quot;:scope&quot;)+&quot; &quot;+xe(l[o]);c=l.join(&quot;,&quot;)}try{return H.apply(n,f.querySelectorAll(c)),n}catch(e){N(t,!0)}finally{s===S&amp;&amp;e.removeAttribute(&quot;id&quot;)}}}return g(t.replace($,&quot;$1&quot;),e,n,r)}function ue(){var r=[];return function e(t,n){return r.push(t+&quot; &quot;)>b.cacheLength&amp;&amp;delete e[r.shift()],e[t+&quot; &quot;]=n}}function le(e){return e[S]=!0,e}function ce(e){var t=C.createElement(&quot;fieldset&quot;);try{return!!e(t)}catch(e){return!1}finally{t.parentNode&amp;&amp;t.parentNode.removeChild(t),t=null}}function fe(e,t){var n=e.split(&quot;|&quot;),r=n.length;while(r--)b.attrHandle[n[r]]=t}function pe(e,t){var n=t&amp;&amp;e,r=n&amp;&amp;1===e.nodeType&amp;&amp;1===t.nodeType&amp;&amp;e.sourceIndex-t.sourceIndex;if(r)return r;if(n)while(n=n.nextSibling)if(n===t)return-1;return e?1:-1}function de(t){return function(e){return&quot;input&quot;===e.nodeName.toLowerCase()&amp;&amp;e.type===t}}function he(n){return function(e){var t=e.nodeName.toLowerCase();return(&quot;input&quot;===t||&quot;button&quot;===t)&amp;&amp;e.type===n}}function ge(t){return function(e){return&quot;form&quot;in e?e.parentNode&amp;&amp;!1===e.disabled?&quot;label&quot;in e?&quot;label&quot;in e.parentNode?e.parentNode.disabled===t:e.disabled===t:e.isDisabled===t||e.isDisabled!==!t&amp;&amp;ae(e)===t:e.disabled===t:&quot;label&quot;in e&amp;&amp;e.disabled===t}}function ve(a){return le(function(o){return o=+o,le(function(e,t){var n,r=a([],e.length,o),i=r.length;while(i--)e[n=r[i]]&amp;&amp;(e[n]=!(t[n]=e[n]))})})}function ye(e){return e&amp;&amp;&quot;undefined&quot;!=typeof e.getElementsByTagName&amp;&amp;e}for(e in d=se.support={},i=se.isXML=function(e){var t=e&amp;&amp;e.namespaceURI,n=e&amp;&amp;(e.ownerDocument||e).documentElement;return!Y.test(t||n&amp;&amp;n.nodeName||&quot;HTML&quot;)},T=se.setDocument=function(e){var t,n,r=e?e.ownerDocument||e:p;return r!=C&amp;&amp;9===r.nodeType&amp;&amp;r.documentElement&amp;&amp;(a=(C=r).documentElement,E=!i(C),p!=C&amp;&amp;(n=C.defaultView)&amp;&amp;n.top!==n&amp;&amp;(n.addEventListener?n.addEventListener(&quot;unload&quot;,oe,!1):n.attachEvent&amp;&amp;n.attachEvent(&quot;onunload&quot;,oe)),d.scope=ce(function(e){return a.appendChild(e).appendChild(C.createElement(&quot;div&quot;)),&quot;undefined&quot;!=typeof e.querySelectorAll&amp;&amp;!e.querySelectorAll(&quot;:scope fieldset div&quot;).length}),d.attributes=ce(function(e){return e.className=&quot;i&quot;,!e.getAttribute(&quot;className&quot;)}),d.getElementsByTagName=ce(function(e){return e.appendChild(C.createComment(&quot;&quot;)),!e.getElementsByTagName(&quot;*&quot;).length}),d.getElementsByClassName=K.test(C.getElementsByClassName),d.getById=ce(function(e){return a.appendChild(e).id=S,!C.getElementsByName||!C.getElementsByName(S).length}),d.getById?(b.filter.ID=function(e){var t=e.replace(te,ne);return function(e){return e.getAttribute(&quot;id&quot;)===t}},b.find.ID=function(e,t){if(&quot;undefined&quot;!=typeof t.getElementById&amp;&amp;E){var n=t.getElementById(e);return n?[n]:[]}}):(b.filter.ID=function(e){var n=e.replace(te,ne);return function(e){var t=&quot;undefined&quot;!=typeof e.getAttributeNode&amp;&amp;e.getAttributeNode(&quot;id&quot;);return t&amp;&amp;t.value===n}},b.find.ID=function(e,t){if(&quot;undefined&quot;!=typeof t.getElementById&amp;&amp;E){var n,r,i,o=t.getElementById(e);if(o){if((n=o.getAttributeNode(&quot;id&quot;))&amp;&amp;n.value===e)return[o];i=t.getElementsByName(e),r=0;while(o=i[r++])if((n=o.getAttributeNode(&quot;id&quot;))&amp;&amp;n.value===e)return[o]}return[]}}),b.find.TAG=d.getElementsByTagName?function(e,t){return&quot;undefined&quot;!=typeof t.getElementsByTagName?t.getElementsByTagName(e):d.qsa?t.querySelectorAll(e):void 0}:function(e,t){var n,r=[],i=0,o=t.getElementsByTagName(e);if(&quot;*&quot;===e){while(n=o[i++])1===n.nodeType&amp;&amp;r.push(n);return r}return o},b.find.CLASS=d.getElementsByClassName&amp;&amp;function(e,t){if(&quot;undefined&quot;!=typeof t.getElementsByClassName&amp;&amp;E)return t.getElementsByClassName(e)},s=[],v=[],(d.qsa=K.test(C.querySelectorAll))&amp;&amp;(ce(function(e){var t;a.appendChild(e).innerHTML=&quot;&lt;a id=&quot; , &quot;'&quot; , &quot;&quot;+S+&quot;&quot; , &quot;'&quot; , &quot;>&lt;/a>&lt;select id=&quot; , &quot;'&quot; , &quot;&quot;+S+&quot;-\r\\&quot; , &quot;'&quot; , &quot; msallowcapture=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;>&lt;option selected=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;>&lt;/option>&lt;/select>&quot;,e.querySelectorAll(&quot;[msallowcapture^=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]&quot;).length&amp;&amp;v.push(&quot;[*^$]=&quot;+M+&quot;*(?:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;|\&quot;\&quot;)&quot;),e.querySelectorAll(&quot;[selected]&quot;).length||v.push(&quot;\\[&quot;+M+&quot;*(?:value|&quot;+R+&quot;)&quot;),e.querySelectorAll(&quot;[id~=&quot;+S+&quot;-]&quot;).length||v.push(&quot;~=&quot;),(t=C.createElement(&quot;input&quot;)).setAttribute(&quot;name&quot;,&quot;&quot;),e.appendChild(t),e.querySelectorAll(&quot;[name=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]&quot;).length||v.push(&quot;\\[&quot;+M+&quot;*name&quot;+M+&quot;*=&quot;+M+&quot;*(?:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;|\&quot;\&quot;)&quot;),e.querySelectorAll(&quot;:checked&quot;).length||v.push(&quot;:checked&quot;),e.querySelectorAll(&quot;a#&quot;+S+&quot;+*&quot;).length||v.push(&quot;.#.+[+~]&quot;),e.querySelectorAll(&quot;\\\f&quot;),v.push(&quot;[\\r\\n\\f]&quot;)}),ce(function(e){e.innerHTML=&quot;&lt;a href=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; disabled=&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;>&lt;/a>&lt;select disabled=&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;>&lt;option/>&lt;/select>&quot;;var t=C.createElement(&quot;input&quot;);t.setAttribute(&quot;type&quot;,&quot;hidden&quot;),e.appendChild(t).setAttribute(&quot;name&quot;,&quot;D&quot;),e.querySelectorAll(&quot;[name=d]&quot;).length&amp;&amp;v.push(&quot;name&quot;+M+&quot;*[*^$|!~]?=&quot;),2!==e.querySelectorAll(&quot;:enabled&quot;).length&amp;&amp;v.push(&quot;:enabled&quot;,&quot;:disabled&quot;),a.appendChild(e).disabled=!0,2!==e.querySelectorAll(&quot;:disabled&quot;).length&amp;&amp;v.push(&quot;:enabled&quot;,&quot;:disabled&quot;),e.querySelectorAll(&quot;*,:x&quot;),v.push(&quot;,.*:&quot;)})),(d.matchesSelector=K.test(c=a.matches||a.webkitMatchesSelector||a.mozMatchesSelector||a.oMatchesSelector||a.msMatchesSelector))&amp;&amp;ce(function(e){d.disconnectedMatch=c.call(e,&quot;*&quot;),c.call(e,&quot;[s!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]:x&quot;),s.push(&quot;!=&quot;,F)}),v=v.length&amp;&amp;new RegExp(v.join(&quot;|&quot;)),s=s.length&amp;&amp;new RegExp(s.join(&quot;|&quot;)),t=K.test(a.compareDocumentPosition),y=t||K.test(a.contains)?function(e,t){var n=9===e.nodeType?e.documentElement:e,r=t&amp;&amp;t.parentNode;return e===r||!(!r||1!==r.nodeType||!(n.contains?n.contains(r):e.compareDocumentPosition&amp;&amp;16&amp;e.compareDocumentPosition(r)))}:function(e,t){if(t)while(t=t.parentNode)if(t===e)return!0;return!1},j=t?function(e,t){if(e===t)return l=!0,0;var n=!e.compareDocumentPosition-!t.compareDocumentPosition;return n||(1&amp;(n=(e.ownerDocument||e)==(t.ownerDocument||t)?e.compareDocumentPosition(t):1)||!d.sortDetached&amp;&amp;t.compareDocumentPosition(e)===n?e==C||e.ownerDocument==p&amp;&amp;y(p,e)?-1:t==C||t.ownerDocument==p&amp;&amp;y(p,t)?1:u?P(u,e)-P(u,t):0:4&amp;n?-1:1)}:function(e,t){if(e===t)return l=!0,0;var n,r=0,i=e.parentNode,o=t.parentNode,a=[e],s=[t];if(!i||!o)return e==C?-1:t==C?1:i?-1:o?1:u?P(u,e)-P(u,t):0;if(i===o)return pe(e,t);n=e;while(n=n.parentNode)a.unshift(n);n=t;while(n=n.parentNode)s.unshift(n);while(a[r]===s[r])r++;return r?pe(a[r],s[r]):a[r]==p?-1:s[r]==p?1:0}),C},se.matches=function(e,t){return se(e,null,null,t)},se.matchesSelector=function(e,t){if(T(e),d.matchesSelector&amp;&amp;E&amp;&amp;!N[t+&quot; &quot;]&amp;&amp;(!s||!s.test(t))&amp;&amp;(!v||!v.test(t)))try{var n=c.call(e,t);if(n||d.disconnectedMatch||e.document&amp;&amp;11!==e.document.nodeType)return n}catch(e){N(t,!0)}return 0&lt;se(t,C,null,[e]).length},se.contains=function(e,t){return(e.ownerDocument||e)!=C&amp;&amp;T(e),y(e,t)},se.attr=function(e,t){(e.ownerDocument||e)!=C&amp;&amp;T(e);var n=b.attrHandle[t.toLowerCase()],r=n&amp;&amp;D.call(b.attrHandle,t.toLowerCase())?n(e,t,!E):void 0;return void 0!==r?r:d.attributes||!E?e.getAttribute(t):(r=e.getAttributeNode(t))&amp;&amp;r.specified?r.value:null},se.escape=function(e){return(e+&quot;&quot;).replace(re,ie)},se.error=function(e){throw new Error(&quot;Syntax error, unrecognized expression: &quot;+e)},se.uniqueSort=function(e){var t,n=[],r=0,i=0;if(l=!d.detectDuplicates,u=!d.sortStable&amp;&amp;e.slice(0),e.sort(j),l){while(t=e[i++])t===e[i]&amp;&amp;(r=n.push(i));while(r--)e.splice(n[r],1)}return u=null,e},o=se.getText=function(e){var t,n=&quot;&quot;,r=0,i=e.nodeType;if(i){if(1===i||9===i||11===i){if(&quot;string&quot;==typeof e.textContent)return e.textContent;for(e=e.firstChild;e;e=e.nextSibling)n+=o(e)}else if(3===i||4===i)return e.nodeValue}else while(t=e[r++])n+=o(t);return n},(b=se.selectors={cacheLength:50,createPseudo:le,match:G,attrHandle:{},find:{},relative:{&quot;>&quot;:{dir:&quot;parentNode&quot;,first:!0},&quot; &quot;:{dir:&quot;parentNode&quot;},&quot;+&quot;:{dir:&quot;previousSibling&quot;,first:!0},&quot;~&quot;:{dir:&quot;previousSibling&quot;}},preFilter:{ATTR:function(e){return e[1]=e[1].replace(te,ne),e[3]=(e[3]||e[4]||e[5]||&quot;&quot;).replace(te,ne),&quot;~=&quot;===e[2]&amp;&amp;(e[3]=&quot; &quot;+e[3]+&quot; &quot;),e.slice(0,4)},CHILD:function(e){return e[1]=e[1].toLowerCase(),&quot;nth&quot;===e[1].slice(0,3)?(e[3]||se.error(e[0]),e[4]=+(e[4]?e[5]+(e[6]||1):2*(&quot;even&quot;===e[3]||&quot;odd&quot;===e[3])),e[5]=+(e[7]+e[8]||&quot;odd&quot;===e[3])):e[3]&amp;&amp;se.error(e[0]),e},PSEUDO:function(e){var t,n=!e[6]&amp;&amp;e[2];return G.CHILD.test(e[0])?null:(e[3]?e[2]=e[4]||e[5]||&quot;&quot;:n&amp;&amp;X.test(n)&amp;&amp;(t=h(n,!0))&amp;&amp;(t=n.indexOf(&quot;)&quot;,n.length-t)-n.length)&amp;&amp;(e[0]=e[0].slice(0,t),e[2]=n.slice(0,t)),e.slice(0,3))}},filter:{TAG:function(e){var t=e.replace(te,ne).toLowerCase();return&quot;*&quot;===e?function(){return!0}:function(e){return e.nodeName&amp;&amp;e.nodeName.toLowerCase()===t}},CLASS:function(e){var t=m[e+&quot; &quot;];return t||(t=new RegExp(&quot;(^|&quot;+M+&quot;)&quot;+e+&quot;(&quot;+M+&quot;|$)&quot;))&amp;&amp;m(e,function(e){return t.test(&quot;string&quot;==typeof e.className&amp;&amp;e.className||&quot;undefined&quot;!=typeof e.getAttribute&amp;&amp;e.getAttribute(&quot;class&quot;)||&quot;&quot;)})},ATTR:function(n,r,i){return function(e){var t=se.attr(e,n);return null==t?&quot;!=&quot;===r:!r||(t+=&quot;&quot;,&quot;=&quot;===r?t===i:&quot;!=&quot;===r?t!==i:&quot;^=&quot;===r?i&amp;&amp;0===t.indexOf(i):&quot;*=&quot;===r?i&amp;&amp;-1&lt;t.indexOf(i):&quot;$=&quot;===r?i&amp;&amp;t.slice(-i.length)===i:&quot;~=&quot;===r?-1&lt;(&quot; &quot;+t.replace(B,&quot; &quot;)+&quot; &quot;).indexOf(i):&quot;|=&quot;===r&amp;&amp;(t===i||t.slice(0,i.length+1)===i+&quot;-&quot;))}},CHILD:function(h,e,t,g,v){var y=&quot;nth&quot;!==h.slice(0,3),m=&quot;last&quot;!==h.slice(-4),x=&quot;of-type&quot;===e;return 1===g&amp;&amp;0===v?function(e){return!!e.parentNode}:function(e,t,n){var r,i,o,a,s,u,l=y!==m?&quot;nextSibling&quot;:&quot;previousSibling&quot;,c=e.parentNode,f=x&amp;&amp;e.nodeName.toLowerCase(),p=!n&amp;&amp;!x,d=!1;if(c){if(y){while(l){a=e;while(a=a[l])if(x?a.nodeName.toLowerCase()===f:1===a.nodeType)return!1;u=l=&quot;only&quot;===h&amp;&amp;!u&amp;&amp;&quot;nextSibling&quot;}return!0}if(u=[m?c.firstChild:c.lastChild],m&amp;&amp;p){d=(s=(r=(i=(o=(a=c)[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]||[])[0]===k&amp;&amp;r[1])&amp;&amp;r[2],a=s&amp;&amp;c.childNodes[s];while(a=++s&amp;&amp;a&amp;&amp;a[l]||(d=s=0)||u.pop())if(1===a.nodeType&amp;&amp;++d&amp;&amp;a===e){i[h]=[k,s,d];break}}else if(p&amp;&amp;(d=s=(r=(i=(o=(a=e)[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]||[])[0]===k&amp;&amp;r[1]),!1===d)while(a=++s&amp;&amp;a&amp;&amp;a[l]||(d=s=0)||u.pop())if((x?a.nodeName.toLowerCase()===f:1===a.nodeType)&amp;&amp;++d&amp;&amp;(p&amp;&amp;((i=(o=a[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]=[k,d]),a===e))break;return(d-=v)===g||d%g==0&amp;&amp;0&lt;=d/g}}},PSEUDO:function(e,o){var t,a=b.pseudos[e]||b.setFilters[e.toLowerCase()]||se.error(&quot;unsupported pseudo: &quot;+e);return a[S]?a(o):1&lt;a.length?(t=[e,e,&quot;&quot;,o],b.setFilters.hasOwnProperty(e.toLowerCase())?le(function(e,t){var n,r=a(e,o),i=r.length;while(i--)e[n=P(e,r[i])]=!(t[n]=r[i])}):function(e){return a(e,0,t)}):a}},pseudos:{not:le(function(e){var r=[],i=[],s=f(e.replace($,&quot;$1&quot;));return s[S]?le(function(e,t,n,r){var i,o=s(e,null,r,[]),a=e.length;while(a--)(i=o[a])&amp;&amp;(e[a]=!(t[a]=i))}):function(e,t,n){return r[0]=e,s(r,null,n,i),r[0]=null,!i.pop()}}),has:le(function(t){return function(e){return 0&lt;se(t,e).length}}),contains:le(function(t){return t=t.replace(te,ne),function(e){return-1&lt;(e.textContent||o(e)).indexOf(t)}}),lang:le(function(n){return V.test(n||&quot;&quot;)||se.error(&quot;unsupported lang: &quot;+n),n=n.replace(te,ne).toLowerCase(),function(e){var t;do{if(t=E?e.lang:e.getAttribute(&quot;xml:lang&quot;)||e.getAttribute(&quot;lang&quot;))return(t=t.toLowerCase())===n||0===t.indexOf(n+&quot;-&quot;)}while((e=e.parentNode)&amp;&amp;1===e.nodeType);return!1}}),target:function(e){var t=n.location&amp;&amp;n.location.hash;return t&amp;&amp;t.slice(1)===e.id},root:function(e){return e===a},focus:function(e){return e===C.activeElement&amp;&amp;(!C.hasFocus||C.hasFocus())&amp;&amp;!!(e.type||e.href||~e.tabIndex)},enabled:ge(!1),disabled:ge(!0),checked:function(e){var t=e.nodeName.toLowerCase();return&quot;input&quot;===t&amp;&amp;!!e.checked||&quot;option&quot;===t&amp;&amp;!!e.selected},selected:function(e){return e.parentNode&amp;&amp;e.parentNode.selectedIndex,!0===e.selected},empty:function(e){for(e=e.firstChild;e;e=e.nextSibling)if(e.nodeType&lt;6)return!1;return!0},parent:function(e){return!b.pseudos.empty(e)},header:function(e){return J.test(e.nodeName)},input:function(e){return Q.test(e.nodeName)},button:function(e){var t=e.nodeName.toLowerCase();return&quot;input&quot;===t&amp;&amp;&quot;button&quot;===e.type||&quot;button&quot;===t},text:function(e){var t;return&quot;input&quot;===e.nodeName.toLowerCase()&amp;&amp;&quot;text&quot;===e.type&amp;&amp;(null==(t=e.getAttribute(&quot;type&quot;))||&quot;text&quot;===t.toLowerCase())},first:ve(function(){return[0]}),last:ve(function(e,t){return[t-1]}),eq:ve(function(e,t,n){return[n&lt;0?n+t:n]}),even:ve(function(e,t){for(var n=0;n&lt;t;n+=2)e.push(n);return e}),odd:ve(function(e,t){for(var n=1;n&lt;t;n+=2)e.push(n);return e}),lt:ve(function(e,t,n){for(var r=n&lt;0?n+t:t&lt;n?t:n;0&lt;=--r;)e.push(r);return e}),gt:ve(function(e,t,n){for(var r=n&lt;0?n+t:n;++r&lt;t;)e.push(r);return e})}}).pseudos.nth=b.pseudos.eq,{radio:!0,checkbox:!0,file:!0,password:!0,image:!0})b.pseudos[e]=de(e);for(e in{submit:!0,reset:!0})b.pseudos[e]=he(e);function me(){}function xe(e){for(var t=0,n=e.length,r=&quot;&quot;;t&lt;n;t++)r+=e[t].value;return r}function be(s,e,t){var u=e.dir,l=e.next,c=l||u,f=t&amp;&amp;&quot;parentNode&quot;===c,p=r++;return e.first?function(e,t,n){while(e=e[u])if(1===e.nodeType||f)return s(e,t,n);return!1}:function(e,t,n){var r,i,o,a=[k,p];if(n){while(e=e[u])if((1===e.nodeType||f)&amp;&amp;s(e,t,n))return!0}else while(e=e[u])if(1===e.nodeType||f)if(i=(o=e[S]||(e[S]={}))[e.uniqueID]||(o[e.uniqueID]={}),l&amp;&amp;l===e.nodeName.toLowerCase())e=e[u]||e;else{if((r=i[c])&amp;&amp;r[0]===k&amp;&amp;r[1]===p)return a[2]=r[2];if((i[c]=a)[2]=s(e,t,n))return!0}return!1}}function we(i){return 1&lt;i.length?function(e,t,n){var r=i.length;while(r--)if(!i[r](e,t,n))return!1;return!0}:i[0]}function Te(e,t,n,r,i){for(var o,a=[],s=0,u=e.length,l=null!=t;s&lt;u;s++)(o=e[s])&amp;&amp;(n&amp;&amp;!n(o,r,i)||(a.push(o),l&amp;&amp;t.push(s)));return a}function Ce(d,h,g,v,y,e){return v&amp;&amp;!v[S]&amp;&amp;(v=Ce(v)),y&amp;&amp;!y[S]&amp;&amp;(y=Ce(y,e)),le(function(e,t,n,r){var i,o,a,s=[],u=[],l=t.length,c=e||function(e,t,n){for(var r=0,i=t.length;r&lt;i;r++)se(e,t[r],n);return n}(h||&quot;*&quot;,n.nodeType?[n]:n,[]),f=!d||!e&amp;&amp;h?c:Te(c,s,d,n,r),p=g?y||(e?d:l||v)?[]:t:f;if(g&amp;&amp;g(f,p,n,r),v){i=Te(p,u),v(i,[],n,r),o=i.length;while(o--)(a=i[o])&amp;&amp;(p[u[o]]=!(f[u[o]]=a))}if(e){if(y||d){if(y){i=[],o=p.length;while(o--)(a=p[o])&amp;&amp;i.push(f[o]=a);y(null,p=[],i,r)}o=p.length;while(o--)(a=p[o])&amp;&amp;-1&lt;(i=y?P(e,a):s[o])&amp;&amp;(e[i]=!(t[i]=a))}}else p=Te(p===t?p.splice(l,p.length):p),y?y(null,t,p,r):H.apply(t,p)})}function Ee(e){for(var i,t,n,r=e.length,o=b.relative[e[0].type],a=o||b.relative[&quot; &quot;],s=o?1:0,u=be(function(e){return e===i},a,!0),l=be(function(e){return-1&lt;P(i,e)},a,!0),c=[function(e,t,n){var r=!o&amp;&amp;(n||t!==w)||((i=t).nodeType?u(e,t,n):l(e,t,n));return i=null,r}];s&lt;r;s++)if(t=b.relative[e[s].type])c=[be(we(c),t)];else{if((t=b.filter[e[s].type].apply(null,e[s].matches))[S]){for(n=++s;n&lt;r;n++)if(b.relative[e[n].type])break;return Ce(1&lt;s&amp;&amp;we(c),1&lt;s&amp;&amp;xe(e.slice(0,s-1).concat({value:&quot; &quot;===e[s-2].type?&quot;*&quot;:&quot;&quot;})).replace($,&quot;$1&quot;),t,s&lt;n&amp;&amp;Ee(e.slice(s,n)),n&lt;r&amp;&amp;Ee(e=e.slice(n)),n&lt;r&amp;&amp;xe(e))}c.push(t)}return we(c)}return me.prototype=b.filters=b.pseudos,b.setFilters=new me,h=se.tokenize=function(e,t){var n,r,i,o,a,s,u,l=x[e+&quot; &quot;];if(l)return t?0:l.slice(0);a=e,s=[],u=b.preFilter;while(a){for(o in n&amp;&amp;!(r=_.exec(a))||(r&amp;&amp;(a=a.slice(r[0].length)||a),s.push(i=[])),n=!1,(r=z.exec(a))&amp;&amp;(n=r.shift(),i.push({value:n,type:r[0].replace($,&quot; &quot;)}),a=a.slice(n.length)),b.filter)!(r=G[o].exec(a))||u[o]&amp;&amp;!(r=u[o](r))||(n=r.shift(),i.push({value:n,type:o,matches:r}),a=a.slice(n.length));if(!n)break}return t?a.length:a?se.error(e):x(e,s).slice(0)},f=se.compile=function(e,t){var n,v,y,m,x,r,i=[],o=[],a=A[e+&quot; &quot;];if(!a){t||(t=h(e)),n=t.length;while(n--)(a=Ee(t[n]))[S]?i.push(a):o.push(a);(a=A(e,(v=o,m=0&lt;(y=i).length,x=0&lt;v.length,r=function(e,t,n,r,i){var o,a,s,u=0,l=&quot;0&quot;,c=e&amp;&amp;[],f=[],p=w,d=e||x&amp;&amp;b.find.TAG(&quot;*&quot;,i),h=k+=null==p?1:Math.random()||.1,g=d.length;for(i&amp;&amp;(w=t==C||t||i);l!==g&amp;&amp;null!=(o=d[l]);l++){if(x&amp;&amp;o){a=0,t||o.ownerDocument==C||(T(o),n=!E);while(s=v[a++])if(s(o,t||C,n)){r.push(o);break}i&amp;&amp;(k=h)}m&amp;&amp;((o=!s&amp;&amp;o)&amp;&amp;u--,e&amp;&amp;c.push(o))}if(u+=l,m&amp;&amp;l!==u){a=0;while(s=y[a++])s(c,f,t,n);if(e){if(0&lt;u)while(l--)c[l]||f[l]||(f[l]=q.call(r));f=Te(f)}H.apply(r,f),i&amp;&amp;!e&amp;&amp;0&lt;f.length&amp;&amp;1&lt;u+y.length&amp;&amp;se.uniqueSort(r)}return i&amp;&amp;(k=h,w=p),c},m?le(r):r))).selector=e}return a},g=se.select=function(e,t,n,r){var i,o,a,s,u,l=&quot;function&quot;==typeof e&amp;&amp;e,c=!r&amp;&amp;h(e=l.selector||e);if(n=n||[],1===c.length){if(2&lt;(o=c[0]=c[0].slice(0)).length&amp;&amp;&quot;ID&quot;===(a=o[0]).type&amp;&amp;9===t.nodeType&amp;&amp;E&amp;&amp;b.relative[o[1].type]){if(!(t=(b.find.ID(a.matches[0].replace(te,ne),t)||[])[0]))return n;l&amp;&amp;(t=t.parentNode),e=e.slice(o.shift().value.length)}i=G.needsContext.test(e)?0:o.length;while(i--){if(a=o[i],b.relative[s=a.type])break;if((u=b.find[s])&amp;&amp;(r=u(a.matches[0].replace(te,ne),ee.test(o[0].type)&amp;&amp;ye(t.parentNode)||t))){if(o.splice(i,1),!(e=r.length&amp;&amp;xe(o)))return H.apply(n,r),n;break}}}return(l||f(e,c))(r,t,!E,n,!t||ee.test(e)&amp;&amp;ye(t.parentNode)||t),n},d.sortStable=S.split(&quot;&quot;).sort(j).join(&quot;&quot;)===S,d.detectDuplicates=!!l,T(),d.sortDetached=ce(function(e){return 1&amp;e.compareDocumentPosition(C.createElement(&quot;fieldset&quot;))}),ce(function(e){return e.innerHTML=&quot;&lt;a href=&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;>&lt;/a>&quot;,&quot;#&quot;===e.firstChild.getAttribute(&quot;href&quot;)})||fe(&quot;type|href|height|width&quot;,function(e,t,n){if(!n)return e.getAttribute(t,&quot;type&quot;===t.toLowerCase()?1:2)}),d.attributes&amp;&amp;ce(function(e){return e.innerHTML=&quot;&lt;input/>&quot;,e.firstChild.setAttribute(&quot;value&quot;,&quot;&quot;),&quot;&quot;===e.firstChild.getAttribute(&quot;value&quot;)})||fe(&quot;value&quot;,function(e,t,n){if(!n&amp;&amp;&quot;input&quot;===e.nodeName.toLowerCase())return e.defaultValue}),ce(function(e){return null==e.getAttribute(&quot;disabled&quot;)})||fe(R,function(e,t,n){var r;if(!n)return!0===e[t]?t.toLowerCase():(r=e.getAttributeNode(t))&amp;&amp;r.specified?r.value:null}),se}(C);S.find=d,S.expr=d.selectors,S.expr[&quot;:&quot;]=S.expr.pseudos,S.uniqueSort=S.unique=d.uniqueSort,S.text=d.getText,S.isXMLDoc=d.isXML,S.contains=d.contains,S.escapeSelector=d.escape;var h=function(e,t,n){var r=[],i=void 0!==n;while((e=e[t])&amp;&amp;9!==e.nodeType)if(1===e.nodeType){if(i&amp;&amp;S(e).is(n))break;r.push(e)}return r},T=function(e,t){for(var n=[];e;e=e.nextSibling)1===e.nodeType&amp;&amp;e!==t&amp;&amp;n.push(e);return n},k=S.expr.match.needsContext;function A(e,t){return e.nodeName&amp;&amp;e.nodeName.toLowerCase()===t.toLowerCase()}var N=/^&lt;([a-z][^\/\0>:\x20\t\r\n\f]*)[\x20\t\r\n\f]*\/?>(?:&lt;\/\1>|)$/i;function j(e,n,r){return m(n)?S.grep(e,function(e,t){return!!n.call(e,t,e)!==r}):n.nodeType?S.grep(e,function(e){return e===n!==r}):&quot;string&quot;!=typeof n?S.grep(e,function(e){return-1&lt;i.call(n,e)!==r}):S.filter(n,e,r)}S.filter=function(e,t,n){var r=t[0];return n&amp;&amp;(e=&quot;:not(&quot;+e+&quot;)&quot;),1===t.length&amp;&amp;1===r.nodeType?S.find.matchesSelector(r,e)?[r]:[]:S.find.matches(e,S.grep(t,function(e){return 1===e.nodeType}))},S.fn.extend({find:function(e){var t,n,r=this.length,i=this;if(&quot;string&quot;!=typeof e)return this.pushStack(S(e).filter(function(){for(t=0;t&lt;r;t++)if(S.contains(i[t],this))return!0}));for(n=this.pushStack([]),t=0;t&lt;r;t++)S.find(e,i[t],n);return 1&lt;r?S.uniqueSort(n):n},filter:function(e){return this.pushStack(j(this,e||[],!1))},not:function(e){return this.pushStack(j(this,e||[],!0))},is:function(e){return!!j(this,&quot;string&quot;==typeof e&amp;&amp;k.test(e)?S(e):e||[],!1).length}});var D,q=/^(?:\s*(&lt;[\w\W]+>)[^>]*|#([\w-]+))$/;(S.fn.init=function(e,t,n){var r,i;if(!e)return this;if(n=n||D,&quot;string&quot;==typeof e){if(!(r=&quot;&lt;&quot;===e[0]&amp;&amp;&quot;>&quot;===e[e.length-1]&amp;&amp;3&lt;=e.length?[null,e,null]:q.exec(e))||!r[1]&amp;&amp;t)return!t||t.jquery?(t||n).find(e):this.constructor(t).find(e);if(r[1]){if(t=t instanceof S?t[0]:t,S.merge(this,S.parseHTML(r[1],t&amp;&amp;t.nodeType?t.ownerDocument||t:E,!0)),N.test(r[1])&amp;&amp;S.isPlainObject(t))for(r in t)m(this[r])?this[r](t[r]):this.attr(r,t[r]);return this}return(i=E.getElementById(r[2]))&amp;&amp;(this[0]=i,this.length=1),this}return e.nodeType?(this[0]=e,this.length=1,this):m(e)?void 0!==n.ready?n.ready(e):e(S):S.makeArray(e,this)}).prototype=S.fn,D=S(E);var L=/^(?:parents|prev(?:Until|All))/,H={children:!0,contents:!0,next:!0,prev:!0};function O(e,t){while((e=e[t])&amp;&amp;1!==e.nodeType);return e}S.fn.extend({has:function(e){var t=S(e,this),n=t.length;return this.filter(function(){for(var e=0;e&lt;n;e++)if(S.contains(this,t[e]))return!0})},closest:function(e,t){var n,r=0,i=this.length,o=[],a=&quot;string&quot;!=typeof e&amp;&amp;S(e);if(!k.test(e))for(;r&lt;i;r++)for(n=this[r];n&amp;&amp;n!==t;n=n.parentNode)if(n.nodeType&lt;11&amp;&amp;(a?-1&lt;a.index(n):1===n.nodeType&amp;&amp;S.find.matchesSelector(n,e))){o.push(n);break}return this.pushStack(1&lt;o.length?S.uniqueSort(o):o)},index:function(e){return e?&quot;string&quot;==typeof e?i.call(S(e),this[0]):i.call(this,e.jquery?e[0]:e):this[0]&amp;&amp;this[0].parentNode?this.first().prevAll().length:-1},add:function(e,t){return this.pushStack(S.uniqueSort(S.merge(this.get(),S(e,t))))},addBack:function(e){return this.add(null==e?this.prevObject:this.prevObject.filter(e))}}),S.each({parent:function(e){var t=e.parentNode;return t&amp;&amp;11!==t.nodeType?t:null},parents:function(e){return h(e,&quot;parentNode&quot;)},parentsUntil:function(e,t,n){return h(e,&quot;parentNode&quot;,n)},next:function(e){return O(e,&quot;nextSibling&quot;)},prev:function(e){return O(e,&quot;previousSibling&quot;)},nextAll:function(e){return h(e,&quot;nextSibling&quot;)},prevAll:function(e){return h(e,&quot;previousSibling&quot;)},nextUntil:function(e,t,n){return h(e,&quot;nextSibling&quot;,n)},prevUntil:function(e,t,n){return h(e,&quot;previousSibling&quot;,n)},siblings:function(e){return T((e.parentNode||{}).firstChild,e)},children:function(e){return T(e.firstChild)},contents:function(e){return null!=e.contentDocument&amp;&amp;r(e.contentDocument)?e.contentDocument:(A(e,&quot;template&quot;)&amp;&amp;(e=e.content||e),S.merge([],e.childNodes))}},function(r,i){S.fn[r]=function(e,t){var n=S.map(this,i,e);return&quot;Until&quot;!==r.slice(-5)&amp;&amp;(t=e),t&amp;&amp;&quot;string&quot;==typeof t&amp;&amp;(n=S.filter(t,n)),1&lt;this.length&amp;&amp;(H[r]||S.uniqueSort(n),L.test(r)&amp;&amp;n.reverse()),this.pushStack(n)}});var P=/[^\x20\t\r\n\f]+/g;function R(e){return e}function M(e){throw e}function I(e,t,n,r){var i;try{e&amp;&amp;m(i=e.promise)?i.call(e).done(t).fail(n):e&amp;&amp;m(i=e.then)?i.call(e,t,n):t.apply(void 0,[e].slice(r))}catch(e){n.apply(void 0,[e])}}S.Callbacks=function(r){var e,n;r=&quot;string&quot;==typeof r?(e=r,n={},S.each(e.match(P)||[],function(e,t){n[t]=!0}),n):S.extend({},r);var i,t,o,a,s=[],u=[],l=-1,c=function(){for(a=a||r.once,o=i=!0;u.length;l=-1){t=u.shift();while(++l&lt;s.length)!1===s[l].apply(t[0],t[1])&amp;&amp;r.stopOnFalse&amp;&amp;(l=s.length,t=!1)}r.memory||(t=!1),i=!1,a&amp;&amp;(s=t?[]:&quot;&quot;)},f={add:function(){return s&amp;&amp;(t&amp;&amp;!i&amp;&amp;(l=s.length-1,u.push(t)),function n(e){S.each(e,function(e,t){m(t)?r.unique&amp;&amp;f.has(t)||s.push(t):t&amp;&amp;t.length&amp;&amp;&quot;string&quot;!==w(t)&amp;&amp;n(t)})}(arguments),t&amp;&amp;!i&amp;&amp;c()),this},remove:function(){return S.each(arguments,function(e,t){var n;while(-1&lt;(n=S.inArray(t,s,n)))s.splice(n,1),n&lt;=l&amp;&amp;l--}),this},has:function(e){return e?-1&lt;S.inArray(e,s):0&lt;s.length},empty:function(){return s&amp;&amp;(s=[]),this},disable:function(){return a=u=[],s=t=&quot;&quot;,this},disabled:function(){return!s},lock:function(){return a=u=[],t||i||(s=t=&quot;&quot;),this},locked:function(){return!!a},fireWith:function(e,t){return a||(t=[e,(t=t||[]).slice?t.slice():t],u.push(t),i||c()),this},fire:function(){return f.fireWith(this,arguments),this},fired:function(){return!!o}};return f},S.extend({Deferred:function(e){var o=[[&quot;notify&quot;,&quot;progress&quot;,S.Callbacks(&quot;memory&quot;),S.Callbacks(&quot;memory&quot;),2],[&quot;resolve&quot;,&quot;done&quot;,S.Callbacks(&quot;once memory&quot;),S.Callbacks(&quot;once memory&quot;),0,&quot;resolved&quot;],[&quot;reject&quot;,&quot;fail&quot;,S.Callbacks(&quot;once memory&quot;),S.Callbacks(&quot;once memory&quot;),1,&quot;rejected&quot;]],i=&quot;pending&quot;,a={state:function(){return i},always:function(){return s.done(arguments).fail(arguments),this},&quot;catch&quot;:function(e){return a.then(null,e)},pipe:function(){var i=arguments;return S.Deferred(function(r){S.each(o,function(e,t){var n=m(i[t[4]])&amp;&amp;i[t[4]];s[t[1]](function(){var e=n&amp;&amp;n.apply(this,arguments);e&amp;&amp;m(e.promise)?e.promise().progress(r.notify).done(r.resolve).fail(r.reject):r[t[0]+&quot;With&quot;](this,n?[e]:arguments)})}),i=null}).promise()},then:function(t,n,r){var u=0;function l(i,o,a,s){return function(){var n=this,r=arguments,e=function(){var e,t;if(!(i&lt;u)){if((e=a.apply(n,r))===o.promise())throw new TypeError(&quot;Thenable self-resolution&quot;);t=e&amp;&amp;(&quot;object&quot;==typeof e||&quot;function&quot;==typeof e)&amp;&amp;e.then,m(t)?s?t.call(e,l(u,o,R,s),l(u,o,M,s)):(u++,t.call(e,l(u,o,R,s),l(u,o,M,s),l(u,o,R,o.notifyWith))):(a!==R&amp;&amp;(n=void 0,r=[e]),(s||o.resolveWith)(n,r))}},t=s?e:function(){try{e()}catch(e){S.Deferred.exceptionHook&amp;&amp;S.Deferred.exceptionHook(e,t.stackTrace),u&lt;=i+1&amp;&amp;(a!==M&amp;&amp;(n=void 0,r=[e]),o.rejectWith(n,r))}};i?t():(S.Deferred.getStackHook&amp;&amp;(t.stackTrace=S.Deferred.getStackHook()),C.setTimeout(t))}}return S.Deferred(function(e){o[0][3].add(l(0,e,m(r)?r:R,e.notifyWith)),o[1][3].add(l(0,e,m(t)?t:R)),o[2][3].add(l(0,e,m(n)?n:M))}).promise()},promise:function(e){return null!=e?S.extend(e,a):a}},s={};return S.each(o,function(e,t){var n=t[2],r=t[5];a[t[1]]=n.add,r&amp;&amp;n.add(function(){i=r},o[3-e][2].disable,o[3-e][3].disable,o[0][2].lock,o[0][3].lock),n.add(t[3].fire),s[t[0]]=function(){return s[t[0]+&quot;With&quot;](this===s?void 0:this,arguments),this},s[t[0]+&quot;With&quot;]=n.fireWith}),a.promise(s),e&amp;&amp;e.call(s,s),s},when:function(e){var n=arguments.length,t=n,r=Array(t),i=s.call(arguments),o=S.Deferred(),a=function(t){return function(e){r[t]=this,i[t]=1&lt;arguments.length?s.call(arguments):e,--n||o.resolveWith(r,i)}};if(n&lt;=1&amp;&amp;(I(e,o.done(a(t)).resolve,o.reject,!n),&quot;pending&quot;===o.state()||m(i[t]&amp;&amp;i[t].then)))return o.then();while(t--)I(i[t],a(t),o.reject);return o.promise()}});var W=/^(Eval|Internal|Range|Reference|Syntax|Type|URI)Error$/;S.Deferred.exceptionHook=function(e,t){C.console&amp;&amp;C.console.warn&amp;&amp;e&amp;&amp;W.test(e.name)&amp;&amp;C.console.warn(&quot;jQuery.Deferred exception: &quot;+e.message,e.stack,t)},S.readyException=function(e){C.setTimeout(function(){throw e})};var F=S.Deferred();function B(){E.removeEventListener(&quot;DOMContentLoaded&quot;,B),C.removeEventListener(&quot;load&quot;,B),S.ready()}S.fn.ready=function(e){return F.then(e)[&quot;catch&quot;](function(e){S.readyException(e)}),this},S.extend({isReady:!1,readyWait:1,ready:function(e){(!0===e?--S.readyWait:S.isReady)||(S.isReady=!0)!==e&amp;&amp;0&lt;--S.readyWait||F.resolveWith(E,[S])}}),S.ready.then=F.then,&quot;complete&quot;===E.readyState||&quot;loading&quot;!==E.readyState&amp;&amp;!E.documentElement.doScroll?C.setTimeout(S.ready):(E.addEventListener(&quot;DOMContentLoaded&quot;,B),C.addEventListener(&quot;load&quot;,B));var $=function(e,t,n,r,i,o,a){var s=0,u=e.length,l=null==n;if(&quot;object&quot;===w(n))for(s in i=!0,n)$(e,t,s,n[s],!0,o,a);else if(void 0!==r&amp;&amp;(i=!0,m(r)||(a=!0),l&amp;&amp;(a?(t.call(e,r),t=null):(l=t,t=function(e,t,n){return l.call(S(e),n)})),t))for(;s&lt;u;s++)t(e[s],n,a?r:r.call(e[s],s,t(e[s],n)));return i?e:l?t.call(e):u?t(e[0],n):o},_=/^-ms-/,z=/-([a-z])/g;function U(e,t){return t.toUpperCase()}function X(e){return e.replace(_,&quot;ms-&quot;).replace(z,U)}var V=function(e){return 1===e.nodeType||9===e.nodeType||!+e.nodeType};function G(){this.expando=S.expando+G.uid++}G.uid=1,G.prototype={cache:function(e){var t=e[this.expando];return t||(t={},V(e)&amp;&amp;(e.nodeType?e[this.expando]=t:Object.defineProperty(e,this.expando,{value:t,configurable:!0}))),t},set:function(e,t,n){var r,i=this.cache(e);if(&quot;string&quot;==typeof t)i[X(t)]=n;else for(r in t)i[X(r)]=t[r];return i},get:function(e,t){return void 0===t?this.cache(e):e[this.expando]&amp;&amp;e[this.expando][X(t)]},access:function(e,t,n){return void 0===t||t&amp;&amp;&quot;string&quot;==typeof t&amp;&amp;void 0===n?this.get(e,t):(this.set(e,t,n),void 0!==n?n:t)},remove:function(e,t){var n,r=e[this.expando];if(void 0!==r){if(void 0!==t){n=(t=Array.isArray(t)?t.map(X):(t=X(t))in r?[t]:t.match(P)||[]).length;while(n--)delete r[t[n]]}(void 0===t||S.isEmptyObject(r))&amp;&amp;(e.nodeType?e[this.expando]=void 0:delete e[this.expando])}},hasData:function(e){var t=e[this.expando];return void 0!==t&amp;&amp;!S.isEmptyObject(t)}};var Y=new G,Q=new G,J=/^(?:\{[\w\W]*\}|\[[\w\W]*\])$/,K=/[A-Z]/g;function Z(e,t,n){var r,i;if(void 0===n&amp;&amp;1===e.nodeType)if(r=&quot;data-&quot;+t.replace(K,&quot;-$&amp;&quot;).toLowerCase(),&quot;string&quot;==typeof(n=e.getAttribute(r))){try{n=&quot;true&quot;===(i=n)||&quot;false&quot;!==i&amp;&amp;(&quot;null&quot;===i?null:i===+i+&quot;&quot;?+i:J.test(i)?JSON.parse(i):i)}catch(e){}Q.set(e,t,n)}else n=void 0;return n}S.extend({hasData:function(e){return Q.hasData(e)||Y.hasData(e)},data:function(e,t,n){return Q.access(e,t,n)},removeData:function(e,t){Q.remove(e,t)},_data:function(e,t,n){return Y.access(e,t,n)},_removeData:function(e,t){Y.remove(e,t)}}),S.fn.extend({data:function(n,e){var t,r,i,o=this[0],a=o&amp;&amp;o.attributes;if(void 0===n){if(this.length&amp;&amp;(i=Q.get(o),1===o.nodeType&amp;&amp;!Y.get(o,&quot;hasDataAttrs&quot;))){t=a.length;while(t--)a[t]&amp;&amp;0===(r=a[t].name).indexOf(&quot;data-&quot;)&amp;&amp;(r=X(r.slice(5)),Z(o,r,i[r]));Y.set(o,&quot;hasDataAttrs&quot;,!0)}return i}return&quot;object&quot;==typeof n?this.each(function(){Q.set(this,n)}):$(this,function(e){var t;if(o&amp;&amp;void 0===e)return void 0!==(t=Q.get(o,n))?t:void 0!==(t=Z(o,n))?t:void 0;this.each(function(){Q.set(this,n,e)})},null,e,1&lt;arguments.length,null,!0)},removeData:function(e){return this.each(function(){Q.remove(this,e)})}}),S.extend({queue:function(e,t,n){var r;if(e)return t=(t||&quot;fx&quot;)+&quot;queue&quot;,r=Y.get(e,t),n&amp;&amp;(!r||Array.isArray(n)?r=Y.access(e,t,S.makeArray(n)):r.push(n)),r||[]},dequeue:function(e,t){t=t||&quot;fx&quot;;var n=S.queue(e,t),r=n.length,i=n.shift(),o=S._queueHooks(e,t);&quot;inprogress&quot;===i&amp;&amp;(i=n.shift(),r--),i&amp;&amp;(&quot;fx&quot;===t&amp;&amp;n.unshift(&quot;inprogress&quot;),delete o.stop,i.call(e,function(){S.dequeue(e,t)},o)),!r&amp;&amp;o&amp;&amp;o.empty.fire()},_queueHooks:function(e,t){var n=t+&quot;queueHooks&quot;;return Y.get(e,n)||Y.access(e,n,{empty:S.Callbacks(&quot;once memory&quot;).add(function(){Y.remove(e,[t+&quot;queue&quot;,n])})})}}),S.fn.extend({queue:function(t,n){var e=2;return&quot;string&quot;!=typeof t&amp;&amp;(n=t,t=&quot;fx&quot;,e--),arguments.length&lt;e?S.queue(this[0],t):void 0===n?this:this.each(function(){var e=S.queue(this,t,n);S._queueHooks(this,t),&quot;fx&quot;===t&amp;&amp;&quot;inprogress&quot;!==e[0]&amp;&amp;S.dequeue(this,t)})},dequeue:function(e){return this.each(function(){S.dequeue(this,e)})},clearQueue:function(e){return this.queue(e||&quot;fx&quot;,[])},promise:function(e,t){var n,r=1,i=S.Deferred(),o=this,a=this.length,s=function(){--r||i.resolveWith(o,[o])};&quot;string&quot;!=typeof e&amp;&amp;(t=e,e=void 0),e=e||&quot;fx&quot;;while(a--)(n=Y.get(o[a],e+&quot;queueHooks&quot;))&amp;&amp;n.empty&amp;&amp;(r++,n.empty.add(s));return s(),i.promise(t)}});var ee=/[+-]?(?:\d*\.|)\d+(?:[eE][+-]?\d+|)/.source,te=new RegExp(&quot;^(?:([+-])=|)(&quot;+ee+&quot;)([a-z%]*)$&quot;,&quot;i&quot;),ne=[&quot;Top&quot;,&quot;Right&quot;,&quot;Bottom&quot;,&quot;Left&quot;],re=E.documentElement,ie=function(e){return S.contains(e.ownerDocument,e)},oe={composed:!0};re.getRootNode&amp;&amp;(ie=function(e){return S.contains(e.ownerDocument,e)||e.getRootNode(oe)===e.ownerDocument});var ae=function(e,t){return&quot;none&quot;===(e=t||e).style.display||&quot;&quot;===e.style.display&amp;&amp;ie(e)&amp;&amp;&quot;none&quot;===S.css(e,&quot;display&quot;)};function se(e,t,n,r){var i,o,a=20,s=r?function(){return r.cur()}:function(){return S.css(e,t,&quot;&quot;)},u=s(),l=n&amp;&amp;n[3]||(S.cssNumber[t]?&quot;&quot;:&quot;px&quot;),c=e.nodeType&amp;&amp;(S.cssNumber[t]||&quot;px&quot;!==l&amp;&amp;+u)&amp;&amp;te.exec(S.css(e,t));if(c&amp;&amp;c[3]!==l){u/=2,l=l||c[3],c=+u||1;while(a--)S.style(e,t,c+l),(1-o)*(1-(o=s()/u||.5))&lt;=0&amp;&amp;(a=0),c/=o;c*=2,S.style(e,t,c+l),n=n||[]}return n&amp;&amp;(c=+c||+u||0,i=n[1]?c+(n[1]+1)*n[2]:+n[2],r&amp;&amp;(r.unit=l,r.start=c,r.end=i)),i}var ue={};function le(e,t){for(var n,r,i,o,a,s,u,l=[],c=0,f=e.length;c&lt;f;c++)(r=e[c]).style&amp;&amp;(n=r.style.display,t?(&quot;none&quot;===n&amp;&amp;(l[c]=Y.get(r,&quot;display&quot;)||null,l[c]||(r.style.display=&quot;&quot;)),&quot;&quot;===r.style.display&amp;&amp;ae(r)&amp;&amp;(l[c]=(u=a=o=void 0,a=(i=r).ownerDocument,s=i.nodeName,(u=ue[s])||(o=a.body.appendChild(a.createElement(s)),u=S.css(o,&quot;display&quot;),o.parentNode.removeChild(o),&quot;none&quot;===u&amp;&amp;(u=&quot;block&quot;),ue[s]=u)))):&quot;none&quot;!==n&amp;&amp;(l[c]=&quot;none&quot;,Y.set(r,&quot;display&quot;,n)));for(c=0;c&lt;f;c++)null!=l[c]&amp;&amp;(e[c].style.display=l[c]);return e}S.fn.extend({show:function(){return le(this,!0)},hide:function(){return le(this)},toggle:function(e){return&quot;boolean&quot;==typeof e?e?this.show():this.hide():this.each(function(){ae(this)?S(this).show():S(this).hide()})}});var ce,fe,pe=/^(?:checkbox|radio)$/i,de=/&lt;([a-z][^\/\0>\x20\t\r\n\f]*)/i,he=/^$|^module$|\/(?:java|ecma)script/i;ce=E.createDocumentFragment().appendChild(E.createElement(&quot;div&quot;)),(fe=E.createElement(&quot;input&quot;)).setAttribute(&quot;type&quot;,&quot;radio&quot;),fe.setAttribute(&quot;checked&quot;,&quot;checked&quot;),fe.setAttribute(&quot;name&quot;,&quot;t&quot;),ce.appendChild(fe),y.checkClone=ce.cloneNode(!0).cloneNode(!0).lastChild.checked,ce.innerHTML=&quot;&lt;textarea>x&lt;/textarea>&quot;,y.noCloneChecked=!!ce.cloneNode(!0).lastChild.defaultValue,ce.innerHTML=&quot;&lt;option>&lt;/option>&quot;,y.option=!!ce.lastChild;var ge={thead:[1,&quot;&lt;table>&quot;,&quot;&lt;/table>&quot;],col:[2,&quot;&lt;table>&lt;colgroup>&quot;,&quot;&lt;/colgroup>&lt;/table>&quot;],tr:[2,&quot;&lt;table>&lt;tbody>&quot;,&quot;&lt;/tbody>&lt;/table>&quot;],td:[3,&quot;&lt;table>&lt;tbody>&lt;tr>&quot;,&quot;&lt;/tr>&lt;/tbody>&lt;/table>&quot;],_default:[0,&quot;&quot;,&quot;&quot;]};function ve(e,t){var n;return n=&quot;undefined&quot;!=typeof e.getElementsByTagName?e.getElementsByTagName(t||&quot;*&quot;):&quot;undefined&quot;!=typeof e.querySelectorAll?e.querySelectorAll(t||&quot;*&quot;):[],void 0===t||t&amp;&amp;A(e,t)?S.merge([e],n):n}function ye(e,t){for(var n=0,r=e.length;n&lt;r;n++)Y.set(e[n],&quot;globalEval&quot;,!t||Y.get(t[n],&quot;globalEval&quot;))}ge.tbody=ge.tfoot=ge.colgroup=ge.caption=ge.thead,ge.th=ge.td,y.option||(ge.optgroup=ge.option=[1,&quot;&lt;select multiple=&quot; , &quot;'&quot; , &quot;multiple&quot; , &quot;'&quot; , &quot;>&quot;,&quot;&lt;/select>&quot;]);var me=/&lt;|&amp;#?\w+;/;function xe(e,t,n,r,i){for(var o,a,s,u,l,c,f=t.createDocumentFragment(),p=[],d=0,h=e.length;d&lt;h;d++)if((o=e[d])||0===o)if(&quot;object&quot;===w(o))S.merge(p,o.nodeType?[o]:o);else if(me.test(o)){a=a||f.appendChild(t.createElement(&quot;div&quot;)),s=(de.exec(o)||[&quot;&quot;,&quot;&quot;])[1].toLowerCase(),u=ge[s]||ge._default,a.innerHTML=u[1]+S.htmlPrefilter(o)+u[2],c=u[0];while(c--)a=a.lastChild;S.merge(p,a.childNodes),(a=f.firstChild).textContent=&quot;&quot;}else p.push(t.createTextNode(o));f.textContent=&quot;&quot;,d=0;while(o=p[d++])if(r&amp;&amp;-1&lt;S.inArray(o,r))i&amp;&amp;i.push(o);else if(l=ie(o),a=ve(f.appendChild(o),&quot;script&quot;),l&amp;&amp;ye(a),n){c=0;while(o=a[c++])he.test(o.type||&quot;&quot;)&amp;&amp;n.push(o)}return f}var be=/^([^.]*)(?:\.(.+)|)/;function we(){return!0}function Te(){return!1}function Ce(e,t){return e===function(){try{return E.activeElement}catch(e){}}()==(&quot;focus&quot;===t)}function Ee(e,t,n,r,i,o){var a,s;if(&quot;object&quot;==typeof t){for(s in&quot;string&quot;!=typeof n&amp;&amp;(r=r||n,n=void 0),t)Ee(e,s,n,r,t[s],o);return e}if(null==r&amp;&amp;null==i?(i=n,r=n=void 0):null==i&amp;&amp;(&quot;string&quot;==typeof n?(i=r,r=void 0):(i=r,r=n,n=void 0)),!1===i)i=Te;else if(!i)return e;return 1===o&amp;&amp;(a=i,(i=function(e){return S().off(e),a.apply(this,arguments)}).guid=a.guid||(a.guid=S.guid++)),e.each(function(){S.event.add(this,t,i,r,n)})}function Se(e,i,o){o?(Y.set(e,i,!1),S.event.add(e,i,{namespace:!1,handler:function(e){var t,n,r=Y.get(this,i);if(1&amp;e.isTrigger&amp;&amp;this[i]){if(r.length)(S.event.special[i]||{}).delegateType&amp;&amp;e.stopPropagation();else if(r=s.call(arguments),Y.set(this,i,r),t=o(this,i),this[i](),r!==(n=Y.get(this,i))||t?Y.set(this,i,!1):n={},r!==n)return e.stopImmediatePropagation(),e.preventDefault(),n&amp;&amp;n.value}else r.length&amp;&amp;(Y.set(this,i,{value:S.event.trigger(S.extend(r[0],S.Event.prototype),r.slice(1),this)}),e.stopImmediatePropagation())}})):void 0===Y.get(e,i)&amp;&amp;S.event.add(e,i,we)}S.event={global:{},add:function(t,e,n,r,i){var o,a,s,u,l,c,f,p,d,h,g,v=Y.get(t);if(V(t)){n.handler&amp;&amp;(n=(o=n).handler,i=o.selector),i&amp;&amp;S.find.matchesSelector(re,i),n.guid||(n.guid=S.guid++),(u=v.events)||(u=v.events=Object.create(null)),(a=v.handle)||(a=v.handle=function(e){return&quot;undefined&quot;!=typeof S&amp;&amp;S.event.triggered!==e.type?S.event.dispatch.apply(t,arguments):void 0}),l=(e=(e||&quot;&quot;).match(P)||[&quot;&quot;]).length;while(l--)d=g=(s=be.exec(e[l])||[])[1],h=(s[2]||&quot;&quot;).split(&quot;.&quot;).sort(),d&amp;&amp;(f=S.event.special[d]||{},d=(i?f.delegateType:f.bindType)||d,f=S.event.special[d]||{},c=S.extend({type:d,origType:g,data:r,handler:n,guid:n.guid,selector:i,needsContext:i&amp;&amp;S.expr.match.needsContext.test(i),namespace:h.join(&quot;.&quot;)},o),(p=u[d])||((p=u[d]=[]).delegateCount=0,f.setup&amp;&amp;!1!==f.setup.call(t,r,h,a)||t.addEventListener&amp;&amp;t.addEventListener(d,a)),f.add&amp;&amp;(f.add.call(t,c),c.handler.guid||(c.handler.guid=n.guid)),i?p.splice(p.delegateCount++,0,c):p.push(c),S.event.global[d]=!0)}},remove:function(e,t,n,r,i){var o,a,s,u,l,c,f,p,d,h,g,v=Y.hasData(e)&amp;&amp;Y.get(e);if(v&amp;&amp;(u=v.events)){l=(t=(t||&quot;&quot;).match(P)||[&quot;&quot;]).length;while(l--)if(d=g=(s=be.exec(t[l])||[])[1],h=(s[2]||&quot;&quot;).split(&quot;.&quot;).sort(),d){f=S.event.special[d]||{},p=u[d=(r?f.delegateType:f.bindType)||d]||[],s=s[2]&amp;&amp;new RegExp(&quot;(^|\\.)&quot;+h.join(&quot;\\.(?:.*\\.|)&quot;)+&quot;(\\.|$)&quot;),a=o=p.length;while(o--)c=p[o],!i&amp;&amp;g!==c.origType||n&amp;&amp;n.guid!==c.guid||s&amp;&amp;!s.test(c.namespace)||r&amp;&amp;r!==c.selector&amp;&amp;(&quot;**&quot;!==r||!c.selector)||(p.splice(o,1),c.selector&amp;&amp;p.delegateCount--,f.remove&amp;&amp;f.remove.call(e,c));a&amp;&amp;!p.length&amp;&amp;(f.teardown&amp;&amp;!1!==f.teardown.call(e,h,v.handle)||S.removeEvent(e,d,v.handle),delete u[d])}else for(d in u)S.event.remove(e,d+t[l],n,r,!0);S.isEmptyObject(u)&amp;&amp;Y.remove(e,&quot;handle events&quot;)}},dispatch:function(e){var t,n,r,i,o,a,s=new Array(arguments.length),u=S.event.fix(e),l=(Y.get(this,&quot;events&quot;)||Object.create(null))[u.type]||[],c=S.event.special[u.type]||{};for(s[0]=u,t=1;t&lt;arguments.length;t++)s[t]=arguments[t];if(u.delegateTarget=this,!c.preDispatch||!1!==c.preDispatch.call(this,u)){a=S.event.handlers.call(this,u,l),t=0;while((i=a[t++])&amp;&amp;!u.isPropagationStopped()){u.currentTarget=i.elem,n=0;while((o=i.handlers[n++])&amp;&amp;!u.isImmediatePropagationStopped())u.rnamespace&amp;&amp;!1!==o.namespace&amp;&amp;!u.rnamespace.test(o.namespace)||(u.handleObj=o,u.data=o.data,void 0!==(r=((S.event.special[o.origType]||{}).handle||o.handler).apply(i.elem,s))&amp;&amp;!1===(u.result=r)&amp;&amp;(u.preventDefault(),u.stopPropagation()))}return c.postDispatch&amp;&amp;c.postDispatch.call(this,u),u.result}},handlers:function(e,t){var n,r,i,o,a,s=[],u=t.delegateCount,l=e.target;if(u&amp;&amp;l.nodeType&amp;&amp;!(&quot;click&quot;===e.type&amp;&amp;1&lt;=e.button))for(;l!==this;l=l.parentNode||this)if(1===l.nodeType&amp;&amp;(&quot;click&quot;!==e.type||!0!==l.disabled)){for(o=[],a={},n=0;n&lt;u;n++)void 0===a[i=(r=t[n]).selector+&quot; &quot;]&amp;&amp;(a[i]=r.needsContext?-1&lt;S(i,this).index(l):S.find(i,this,null,[l]).length),a[i]&amp;&amp;o.push(r);o.length&amp;&amp;s.push({elem:l,handlers:o})}return l=this,u&lt;t.length&amp;&amp;s.push({elem:l,handlers:t.slice(u)}),s},addProp:function(t,e){Object.defineProperty(S.Event.prototype,t,{enumerable:!0,configurable:!0,get:m(e)?function(){if(this.originalEvent)return e(this.originalEvent)}:function(){if(this.originalEvent)return this.originalEvent[t]},set:function(e){Object.defineProperty(this,t,{enumerable:!0,configurable:!0,writable:!0,value:e})}})},fix:function(e){return e[S.expando]?e:new S.Event(e)},special:{load:{noBubble:!0},click:{setup:function(e){var t=this||e;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Se(t,&quot;click&quot;,we),!1},trigger:function(e){var t=this||e;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Se(t,&quot;click&quot;),!0},_default:function(e){var t=e.target;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Y.get(t,&quot;click&quot;)||A(t,&quot;a&quot;)}},beforeunload:{postDispatch:function(e){void 0!==e.result&amp;&amp;e.originalEvent&amp;&amp;(e.originalEvent.returnValue=e.result)}}}},S.removeEvent=function(e,t,n){e.removeEventListener&amp;&amp;e.removeEventListener(t,n)},S.Event=function(e,t){if(!(this instanceof S.Event))return new S.Event(e,t);e&amp;&amp;e.type?(this.originalEvent=e,this.type=e.type,this.isDefaultPrevented=e.defaultPrevented||void 0===e.defaultPrevented&amp;&amp;!1===e.returnValue?we:Te,this.target=e.target&amp;&amp;3===e.target.nodeType?e.target.parentNode:e.target,this.currentTarget=e.currentTarget,this.relatedTarget=e.relatedTarget):this.type=e,t&amp;&amp;S.extend(this,t),this.timeStamp=e&amp;&amp;e.timeStamp||Date.now(),this[S.expando]=!0},S.Event.prototype={constructor:S.Event,isDefaultPrevented:Te,isPropagationStopped:Te,isImmediatePropagationStopped:Te,isSimulated:!1,preventDefault:function(){var e=this.originalEvent;this.isDefaultPrevented=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.preventDefault()},stopPropagation:function(){var e=this.originalEvent;this.isPropagationStopped=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.stopPropagation()},stopImmediatePropagation:function(){var e=this.originalEvent;this.isImmediatePropagationStopped=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.stopImmediatePropagation(),this.stopPropagation()}},S.each({altKey:!0,bubbles:!0,cancelable:!0,changedTouches:!0,ctrlKey:!0,detail:!0,eventPhase:!0,metaKey:!0,pageX:!0,pageY:!0,shiftKey:!0,view:!0,&quot;char&quot;:!0,code:!0,charCode:!0,key:!0,keyCode:!0,button:!0,buttons:!0,clientX:!0,clientY:!0,offsetX:!0,offsetY:!0,pointerId:!0,pointerType:!0,screenX:!0,screenY:!0,targetTouches:!0,toElement:!0,touches:!0,which:!0},S.event.addProp),S.each({focus:&quot;focusin&quot;,blur:&quot;focusout&quot;},function(e,t){S.event.special[e]={setup:function(){return Se(this,e,Ce),!1},trigger:function(){return Se(this,e),!0},_default:function(){return!0},delegateType:t}}),S.each({mouseenter:&quot;mouseover&quot;,mouseleave:&quot;mouseout&quot;,pointerenter:&quot;pointerover&quot;,pointerleave:&quot;pointerout&quot;},function(e,i){S.event.special[e]={delegateType:i,bindType:i,handle:function(e){var t,n=e.relatedTarget,r=e.handleObj;return n&amp;&amp;(n===this||S.contains(this,n))||(e.type=r.origType,t=r.handler.apply(this,arguments),e.type=i),t}}}),S.fn.extend({on:function(e,t,n,r){return Ee(this,e,t,n,r)},one:function(e,t,n,r){return Ee(this,e,t,n,r,1)},off:function(e,t,n){var r,i;if(e&amp;&amp;e.preventDefault&amp;&amp;e.handleObj)return r=e.handleObj,S(e.delegateTarget).off(r.namespace?r.origType+&quot;.&quot;+r.namespace:r.origType,r.selector,r.handler),this;if(&quot;object&quot;==typeof e){for(i in e)this.off(i,t,e[i]);return this}return!1!==t&amp;&amp;&quot;function&quot;!=typeof t||(n=t,t=void 0),!1===n&amp;&amp;(n=Te),this.each(function(){S.event.remove(this,e,n,t)})}});var ke=/&lt;script|&lt;style|&lt;link/i,Ae=/checked\s*(?:[^=]|=\s*.checked.)/i,Ne=/^\s*&lt;!(?:\[CDATA\[|--)|(?:\]\]|--)>\s*$/g;function je(e,t){return A(e,&quot;table&quot;)&amp;&amp;A(11!==t.nodeType?t:t.firstChild,&quot;tr&quot;)&amp;&amp;S(e).children(&quot;tbody&quot;)[0]||e}function De(e){return e.type=(null!==e.getAttribute(&quot;type&quot;))+&quot;/&quot;+e.type,e}function qe(e){return&quot;true/&quot;===(e.type||&quot;&quot;).slice(0,5)?e.type=e.type.slice(5):e.removeAttribute(&quot;type&quot;),e}function Le(e,t){var n,r,i,o,a,s;if(1===t.nodeType){if(Y.hasData(e)&amp;&amp;(s=Y.get(e).events))for(i in Y.remove(t,&quot;handle events&quot;),s)for(n=0,r=s[i].length;n&lt;r;n++)S.event.add(t,i,s[i][n]);Q.hasData(e)&amp;&amp;(o=Q.access(e),a=S.extend({},o),Q.set(t,a))}}function He(n,r,i,o){r=g(r);var e,t,a,s,u,l,c=0,f=n.length,p=f-1,d=r[0],h=m(d);if(h||1&lt;f&amp;&amp;&quot;string&quot;==typeof d&amp;&amp;!y.checkClone&amp;&amp;Ae.test(d))return n.each(function(e){var t=n.eq(e);h&amp;&amp;(r[0]=d.call(this,e,t.html())),He(t,r,i,o)});if(f&amp;&amp;(t=(e=xe(r,n[0].ownerDocument,!1,n,o)).firstChild,1===e.childNodes.length&amp;&amp;(e=t),t||o)){for(s=(a=S.map(ve(e,&quot;script&quot;),De)).length;c&lt;f;c++)u=e,c!==p&amp;&amp;(u=S.clone(u,!0,!0),s&amp;&amp;S.merge(a,ve(u,&quot;script&quot;))),i.call(n[c],u,c);if(s)for(l=a[a.length-1].ownerDocument,S.map(a,qe),c=0;c&lt;s;c++)u=a[c],he.test(u.type||&quot;&quot;)&amp;&amp;!Y.access(u,&quot;globalEval&quot;)&amp;&amp;S.contains(l,u)&amp;&amp;(u.src&amp;&amp;&quot;module&quot;!==(u.type||&quot;&quot;).toLowerCase()?S._evalUrl&amp;&amp;!u.noModule&amp;&amp;S._evalUrl(u.src,{nonce:u.nonce||u.getAttribute(&quot;nonce&quot;)},l):b(u.textContent.replace(Ne,&quot;&quot;),u,l))}return n}function Oe(e,t,n){for(var r,i=t?S.filter(t,e):e,o=0;null!=(r=i[o]);o++)n||1!==r.nodeType||S.cleanData(ve(r)),r.parentNode&amp;&amp;(n&amp;&amp;ie(r)&amp;&amp;ye(ve(r,&quot;script&quot;)),r.parentNode.removeChild(r));return e}S.extend({htmlPrefilter:function(e){return e},clone:function(e,t,n){var r,i,o,a,s,u,l,c=e.cloneNode(!0),f=ie(e);if(!(y.noCloneChecked||1!==e.nodeType&amp;&amp;11!==e.nodeType||S.isXMLDoc(e)))for(a=ve(c),r=0,i=(o=ve(e)).length;r&lt;i;r++)s=o[r],u=a[r],void 0,&quot;input&quot;===(l=u.nodeName.toLowerCase())&amp;&amp;pe.test(s.type)?u.checked=s.checked:&quot;input&quot;!==l&amp;&amp;&quot;textarea&quot;!==l||(u.defaultValue=s.defaultValue);if(t)if(n)for(o=o||ve(e),a=a||ve(c),r=0,i=o.length;r&lt;i;r++)Le(o[r],a[r]);else Le(e,c);return 0&lt;(a=ve(c,&quot;script&quot;)).length&amp;&amp;ye(a,!f&amp;&amp;ve(e,&quot;script&quot;)),c},cleanData:function(e){for(var t,n,r,i=S.event.special,o=0;void 0!==(n=e[o]);o++)if(V(n)){if(t=n[Y.expando]){if(t.events)for(r in t.events)i[r]?S.event.remove(n,r):S.removeEvent(n,r,t.handle);n[Y.expando]=void 0}n[Q.expando]&amp;&amp;(n[Q.expando]=void 0)}}}),S.fn.extend({detach:function(e){return Oe(this,e,!0)},remove:function(e){return Oe(this,e)},text:function(e){return $(this,function(e){return void 0===e?S.text(this):this.empty().each(function(){1!==this.nodeType&amp;&amp;11!==this.nodeType&amp;&amp;9!==this.nodeType||(this.textContent=e)})},null,e,arguments.length)},append:function(){return He(this,arguments,function(e){1!==this.nodeType&amp;&amp;11!==this.nodeType&amp;&amp;9!==this.nodeType||je(this,e).appendChild(e)})},prepend:function(){return He(this,arguments,function(e){if(1===this.nodeType||11===this.nodeType||9===this.nodeType){var t=je(this,e);t.insertBefore(e,t.firstChild)}})},before:function(){return He(this,arguments,function(e){this.parentNode&amp;&amp;this.parentNode.insertBefore(e,this)})},after:function(){return He(this,arguments,function(e){this.parentNode&amp;&amp;this.parentNode.insertBefore(e,this.nextSibling)})},empty:function(){for(var e,t=0;null!=(e=this[t]);t++)1===e.nodeType&amp;&amp;(S.cleanData(ve(e,!1)),e.textContent=&quot;&quot;);return this},clone:function(e,t){return e=null!=e&amp;&amp;e,t=null==t?e:t,this.map(function(){return S.clone(this,e,t)})},html:function(e){return $(this,function(e){var t=this[0]||{},n=0,r=this.length;if(void 0===e&amp;&amp;1===t.nodeType)return t.innerHTML;if(&quot;string&quot;==typeof e&amp;&amp;!ke.test(e)&amp;&amp;!ge[(de.exec(e)||[&quot;&quot;,&quot;&quot;])[1].toLowerCase()]){e=S.htmlPrefilter(e);try{for(;n&lt;r;n++)1===(t=this[n]||{}).nodeType&amp;&amp;(S.cleanData(ve(t,!1)),t.innerHTML=e);t=0}catch(e){}}t&amp;&amp;this.empty().append(e)},null,e,arguments.length)},replaceWith:function(){var n=[];return He(this,arguments,function(e){var t=this.parentNode;S.inArray(this,n)&lt;0&amp;&amp;(S.cleanData(ve(this)),t&amp;&amp;t.replaceChild(e,this))},n)}}),S.each({appendTo:&quot;append&quot;,prependTo:&quot;prepend&quot;,insertBefore:&quot;before&quot;,insertAfter:&quot;after&quot;,replaceAll:&quot;replaceWith&quot;},function(e,a){S.fn[e]=function(e){for(var t,n=[],r=S(e),i=r.length-1,o=0;o&lt;=i;o++)t=o===i?this:this.clone(!0),S(r[o])[a](t),u.apply(n,t.get());return this.pushStack(n)}});var Pe=new RegExp(&quot;^(&quot;+ee+&quot;)(?!px)[a-z%]+$&quot;,&quot;i&quot;),Re=function(e){var t=e.ownerDocument.defaultView;return t&amp;&amp;t.opener||(t=C),t.getComputedStyle(e)},Me=function(e,t,n){var r,i,o={};for(i in t)o[i]=e.style[i],e.style[i]=t[i];for(i in r=n.call(e),t)e.style[i]=o[i];return r},Ie=new RegExp(ne.join(&quot;|&quot;),&quot;i&quot;);function We(e,t,n){var r,i,o,a,s=e.style;return(n=n||Re(e))&amp;&amp;(&quot;&quot;!==(a=n.getPropertyValue(t)||n[t])||ie(e)||(a=S.style(e,t)),!y.pixelBoxStyles()&amp;&amp;Pe.test(a)&amp;&amp;Ie.test(t)&amp;&amp;(r=s.width,i=s.minWidth,o=s.maxWidth,s.minWidth=s.maxWidth=s.width=a,a=n.width,s.width=r,s.minWidth=i,s.maxWidth=o)),void 0!==a?a+&quot;&quot;:a}function Fe(e,t){return{get:function(){if(!e())return(this.get=t).apply(this,arguments);delete this.get}}}!function(){function e(){if(l){u.style.cssText=&quot;position:absolute;left:-11111px;width:60px;margin-top:1px;padding:0;border:0&quot;,l.style.cssText=&quot;position:relative;display:block;box-sizing:border-box;overflow:scroll;margin:auto;border:1px;padding:1px;width:60%;top:1%&quot;,re.appendChild(u).appendChild(l);var e=C.getComputedStyle(l);n=&quot;1%&quot;!==e.top,s=12===t(e.marginLeft),l.style.right=&quot;60%&quot;,o=36===t(e.right),r=36===t(e.width),l.style.position=&quot;absolute&quot;,i=12===t(l.offsetWidth/3),re.removeChild(u),l=null}}function t(e){return Math.round(parseFloat(e))}var n,r,i,o,a,s,u=E.createElement(&quot;div&quot;),l=E.createElement(&quot;div&quot;);l.style&amp;&amp;(l.style.backgroundClip=&quot;content-box&quot;,l.cloneNode(!0).style.backgroundClip=&quot;&quot;,y.clearCloneStyle=&quot;content-box&quot;===l.style.backgroundClip,S.extend(y,{boxSizingReliable:function(){return e(),r},pixelBoxStyles:function(){return e(),o},pixelPosition:function(){return e(),n},reliableMarginLeft:function(){return e(),s},scrollboxSize:function(){return e(),i},reliableTrDimensions:function(){var e,t,n,r;return null==a&amp;&amp;(e=E.createElement(&quot;table&quot;),t=E.createElement(&quot;tr&quot;),n=E.createElement(&quot;div&quot;),e.style.cssText=&quot;position:absolute;left:-11111px;border-collapse:separate&quot;,t.style.cssText=&quot;border:1px solid&quot;,t.style.height=&quot;1px&quot;,n.style.height=&quot;9px&quot;,n.style.display=&quot;block&quot;,re.appendChild(e).appendChild(t).appendChild(n),r=C.getComputedStyle(t),a=parseInt(r.height,10)+parseInt(r.borderTopWidth,10)+parseInt(r.borderBottomWidth,10)===t.offsetHeight,re.removeChild(e)),a}}))}();var Be=[&quot;Webkit&quot;,&quot;Moz&quot;,&quot;ms&quot;],$e=E.createElement(&quot;div&quot;).style,_e={};function ze(e){var t=S.cssProps[e]||_e[e];return t||(e in $e?e:_e[e]=function(e){var t=e[0].toUpperCase()+e.slice(1),n=Be.length;while(n--)if((e=Be[n]+t)in $e)return e}(e)||e)}var Ue=/^(none|table(?!-c[ea]).+)/,Xe=/^--/,Ve={position:&quot;absolute&quot;,visibility:&quot;hidden&quot;,display:&quot;block&quot;},Ge={letterSpacing:&quot;0&quot;,fontWeight:&quot;400&quot;};function Ye(e,t,n){var r=te.exec(t);return r?Math.max(0,r[2]-(n||0))+(r[3]||&quot;px&quot;):t}function Qe(e,t,n,r,i,o){var a=&quot;width&quot;===t?1:0,s=0,u=0;if(n===(r?&quot;border&quot;:&quot;content&quot;))return 0;for(;a&lt;4;a+=2)&quot;margin&quot;===n&amp;&amp;(u+=S.css(e,n+ne[a],!0,i)),r?(&quot;content&quot;===n&amp;&amp;(u-=S.css(e,&quot;padding&quot;+ne[a],!0,i)),&quot;margin&quot;!==n&amp;&amp;(u-=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i))):(u+=S.css(e,&quot;padding&quot;+ne[a],!0,i),&quot;padding&quot;!==n?u+=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i):s+=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i));return!r&amp;&amp;0&lt;=o&amp;&amp;(u+=Math.max(0,Math.ceil(e[&quot;offset&quot;+t[0].toUpperCase()+t.slice(1)]-o-u-s-.5))||0),u}function Je(e,t,n){var r=Re(e),i=(!y.boxSizingReliable()||n)&amp;&amp;&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,r),o=i,a=We(e,t,r),s=&quot;offset&quot;+t[0].toUpperCase()+t.slice(1);if(Pe.test(a)){if(!n)return a;a=&quot;auto&quot;}return(!y.boxSizingReliable()&amp;&amp;i||!y.reliableTrDimensions()&amp;&amp;A(e,&quot;tr&quot;)||&quot;auto&quot;===a||!parseFloat(a)&amp;&amp;&quot;inline&quot;===S.css(e,&quot;display&quot;,!1,r))&amp;&amp;e.getClientRects().length&amp;&amp;(i=&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,r),(o=s in e)&amp;&amp;(a=e[s])),(a=parseFloat(a)||0)+Qe(e,t,n||(i?&quot;border&quot;:&quot;content&quot;),o,r,a)+&quot;px&quot;}function Ke(e,t,n,r,i){return new Ke.prototype.init(e,t,n,r,i)}S.extend({cssHooks:{opacity:{get:function(e,t){if(t){var n=We(e,&quot;opacity&quot;);return&quot;&quot;===n?&quot;1&quot;:n}}}},cssNumber:{animationIterationCount:!0,columnCount:!0,fillOpacity:!0,flexGrow:!0,flexShrink:!0,fontWeight:!0,gridArea:!0,gridColumn:!0,gridColumnEnd:!0,gridColumnStart:!0,gridRow:!0,gridRowEnd:!0,gridRowStart:!0,lineHeight:!0,opacity:!0,order:!0,orphans:!0,widows:!0,zIndex:!0,zoom:!0},cssProps:{},style:function(e,t,n,r){if(e&amp;&amp;3!==e.nodeType&amp;&amp;8!==e.nodeType&amp;&amp;e.style){var i,o,a,s=X(t),u=Xe.test(t),l=e.style;if(u||(t=ze(s)),a=S.cssHooks[t]||S.cssHooks[s],void 0===n)return a&amp;&amp;&quot;get&quot;in a&amp;&amp;void 0!==(i=a.get(e,!1,r))?i:l[t];&quot;string&quot;===(o=typeof n)&amp;&amp;(i=te.exec(n))&amp;&amp;i[1]&amp;&amp;(n=se(e,t,i),o=&quot;number&quot;),null!=n&amp;&amp;n==n&amp;&amp;(&quot;number&quot;!==o||u||(n+=i&amp;&amp;i[3]||(S.cssNumber[s]?&quot;&quot;:&quot;px&quot;)),y.clearCloneStyle||&quot;&quot;!==n||0!==t.indexOf(&quot;background&quot;)||(l[t]=&quot;inherit&quot;),a&amp;&amp;&quot;set&quot;in a&amp;&amp;void 0===(n=a.set(e,n,r))||(u?l.setProperty(t,n):l[t]=n))}},css:function(e,t,n,r){var i,o,a,s=X(t);return Xe.test(t)||(t=ze(s)),(a=S.cssHooks[t]||S.cssHooks[s])&amp;&amp;&quot;get&quot;in a&amp;&amp;(i=a.get(e,!0,n)),void 0===i&amp;&amp;(i=We(e,t,r)),&quot;normal&quot;===i&amp;&amp;t in Ge&amp;&amp;(i=Ge[t]),&quot;&quot;===n||n?(o=parseFloat(i),!0===n||isFinite(o)?o||0:i):i}}),S.each([&quot;height&quot;,&quot;width&quot;],function(e,u){S.cssHooks[u]={get:function(e,t,n){if(t)return!Ue.test(S.css(e,&quot;display&quot;))||e.getClientRects().length&amp;&amp;e.getBoundingClientRect().width?Je(e,u,n):Me(e,Ve,function(){return Je(e,u,n)})},set:function(e,t,n){var r,i=Re(e),o=!y.scrollboxSize()&amp;&amp;&quot;absolute&quot;===i.position,a=(o||n)&amp;&amp;&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,i),s=n?Qe(e,u,n,a,i):0;return a&amp;&amp;o&amp;&amp;(s-=Math.ceil(e[&quot;offset&quot;+u[0].toUpperCase()+u.slice(1)]-parseFloat(i[u])-Qe(e,u,&quot;border&quot;,!1,i)-.5)),s&amp;&amp;(r=te.exec(t))&amp;&amp;&quot;px&quot;!==(r[3]||&quot;px&quot;)&amp;&amp;(e.style[u]=t,t=S.css(e,u)),Ye(0,t,s)}}}),S.cssHooks.marginLeft=Fe(y.reliableMarginLeft,function(e,t){if(t)return(parseFloat(We(e,&quot;marginLeft&quot;))||e.getBoundingClientRect().left-Me(e,{marginLeft:0},function(){return e.getBoundingClientRect().left}))+&quot;px&quot;}),S.each({margin:&quot;&quot;,padding:&quot;&quot;,border:&quot;Width&quot;},function(i,o){S.cssHooks[i+o]={expand:function(e){for(var t=0,n={},r=&quot;string&quot;==typeof e?e.split(&quot; &quot;):[e];t&lt;4;t++)n[i+ne[t]+o]=r[t]||r[t-2]||r[0];return n}},&quot;margin&quot;!==i&amp;&amp;(S.cssHooks[i+o].set=Ye)}),S.fn.extend({css:function(e,t){return $(this,function(e,t,n){var r,i,o={},a=0;if(Array.isArray(t)){for(r=Re(e),i=t.length;a&lt;i;a++)o[t[a]]=S.css(e,t[a],!1,r);return o}return void 0!==n?S.style(e,t,n):S.css(e,t)},e,t,1&lt;arguments.length)}}),((S.Tween=Ke).prototype={constructor:Ke,init:function(e,t,n,r,i,o){this.elem=e,this.prop=n,this.easing=i||S.easing._default,this.options=t,this.start=this.now=this.cur(),this.end=r,this.unit=o||(S.cssNumber[n]?&quot;&quot;:&quot;px&quot;)},cur:function(){var e=Ke.propHooks[this.prop];return e&amp;&amp;e.get?e.get(this):Ke.propHooks._default.get(this)},run:function(e){var t,n=Ke.propHooks[this.prop];return this.options.duration?this.pos=t=S.easing[this.easing](e,this.options.duration*e,0,1,this.options.duration):this.pos=t=e,this.now=(this.end-this.start)*t+this.start,this.options.step&amp;&amp;this.options.step.call(this.elem,this.now,this),n&amp;&amp;n.set?n.set(this):Ke.propHooks._default.set(this),this}}).init.prototype=Ke.prototype,(Ke.propHooks={_default:{get:function(e){var t;return 1!==e.elem.nodeType||null!=e.elem[e.prop]&amp;&amp;null==e.elem.style[e.prop]?e.elem[e.prop]:(t=S.css(e.elem,e.prop,&quot;&quot;))&amp;&amp;&quot;auto&quot;!==t?t:0},set:function(e){S.fx.step[e.prop]?S.fx.step[e.prop](e):1!==e.elem.nodeType||!S.cssHooks[e.prop]&amp;&amp;null==e.elem.style[ze(e.prop)]?e.elem[e.prop]=e.now:S.style(e.elem,e.prop,e.now+e.unit)}}}).scrollTop=Ke.propHooks.scrollLeft={set:function(e){e.elem.nodeType&amp;&amp;e.elem.parentNode&amp;&amp;(e.elem[e.prop]=e.now)}},S.easing={linear:function(e){return e},swing:function(e){return.5-Math.cos(e*Math.PI)/2},_default:&quot;swing&quot;},S.fx=Ke.prototype.init,S.fx.step={};var Ze,et,tt,nt,rt=/^(?:toggle|show|hide)$/,it=/queueHooks$/;function ot(){et&amp;&amp;(!1===E.hidden&amp;&amp;C.requestAnimationFrame?C.requestAnimationFrame(ot):C.setTimeout(ot,S.fx.interval),S.fx.tick())}function at(){return C.setTimeout(function(){Ze=void 0}),Ze=Date.now()}function st(e,t){var n,r=0,i={height:e};for(t=t?1:0;r&lt;4;r+=2-t)i[&quot;margin&quot;+(n=ne[r])]=i[&quot;padding&quot;+n]=e;return t&amp;&amp;(i.opacity=i.width=e),i}function ut(e,t,n){for(var r,i=(lt.tweeners[t]||[]).concat(lt.tweeners[&quot;*&quot;]),o=0,a=i.length;o&lt;a;o++)if(r=i[o].call(n,t,e))return r}function lt(o,e,t){var n,a,r=0,i=lt.prefilters.length,s=S.Deferred().always(function(){delete u.elem}),u=function(){if(a)return!1;for(var e=Ze||at(),t=Math.max(0,l.startTime+l.duration-e),n=1-(t/l.duration||0),r=0,i=l.tweens.length;r&lt;i;r++)l.tweens[r].run(n);return s.notifyWith(o,[l,n,t]),n&lt;1&amp;&amp;i?t:(i||s.notifyWith(o,[l,1,0]),s.resolveWith(o,[l]),!1)},l=s.promise({elem:o,props:S.extend({},e),opts:S.extend(!0,{specialEasing:{},easing:S.easing._default},t),originalProperties:e,originalOptions:t,startTime:Ze||at(),duration:t.duration,tweens:[],createTween:function(e,t){var n=S.Tween(o,l.opts,e,t,l.opts.specialEasing[e]||l.opts.easing);return l.tweens.push(n),n},stop:function(e){var t=0,n=e?l.tweens.length:0;if(a)return this;for(a=!0;t&lt;n;t++)l.tweens[t].run(1);return e?(s.notifyWith(o,[l,1,0]),s.resolveWith(o,[l,e])):s.rejectWith(o,[l,e]),this}}),c=l.props;for(!function(e,t){var n,r,i,o,a;for(n in e)if(i=t[r=X(n)],o=e[n],Array.isArray(o)&amp;&amp;(i=o[1],o=e[n]=o[0]),n!==r&amp;&amp;(e[r]=o,delete e[n]),(a=S.cssHooks[r])&amp;&amp;&quot;expand&quot;in a)for(n in o=a.expand(o),delete e[r],o)n in e||(e[n]=o[n],t[n]=i);else t[r]=i}(c,l.opts.specialEasing);r&lt;i;r++)if(n=lt.prefilters[r].call(l,o,c,l.opts))return m(n.stop)&amp;&amp;(S._queueHooks(l.elem,l.opts.queue).stop=n.stop.bind(n)),n;return S.map(c,ut,l),m(l.opts.start)&amp;&amp;l.opts.start.call(o,l),l.progress(l.opts.progress).done(l.opts.done,l.opts.complete).fail(l.opts.fail).always(l.opts.always),S.fx.timer(S.extend(u,{elem:o,anim:l,queue:l.opts.queue})),l}S.Animation=S.extend(lt,{tweeners:{&quot;*&quot;:[function(e,t){var n=this.createTween(e,t);return se(n.elem,e,te.exec(t),n),n}]},tweener:function(e,t){m(e)?(t=e,e=[&quot;*&quot;]):e=e.match(P);for(var n,r=0,i=e.length;r&lt;i;r++)n=e[r],lt.tweeners[n]=lt.tweeners[n]||[],lt.tweeners[n].unshift(t)},prefilters:[function(e,t,n){var r,i,o,a,s,u,l,c,f=&quot;width&quot;in t||&quot;height&quot;in t,p=this,d={},h=e.style,g=e.nodeType&amp;&amp;ae(e),v=Y.get(e,&quot;fxshow&quot;);for(r in n.queue||(null==(a=S._queueHooks(e,&quot;fx&quot;)).unqueued&amp;&amp;(a.unqueued=0,s=a.empty.fire,a.empty.fire=function(){a.unqueued||s()}),a.unqueued++,p.always(function(){p.always(function(){a.unqueued--,S.queue(e,&quot;fx&quot;).length||a.empty.fire()})})),t)if(i=t[r],rt.test(i)){if(delete t[r],o=o||&quot;toggle&quot;===i,i===(g?&quot;hide&quot;:&quot;show&quot;)){if(&quot;show&quot;!==i||!v||void 0===v[r])continue;g=!0}d[r]=v&amp;&amp;v[r]||S.style(e,r)}if((u=!S.isEmptyObject(t))||!S.isEmptyObject(d))for(r in f&amp;&amp;1===e.nodeType&amp;&amp;(n.overflow=[h.overflow,h.overflowX,h.overflowY],null==(l=v&amp;&amp;v.display)&amp;&amp;(l=Y.get(e,&quot;display&quot;)),&quot;none&quot;===(c=S.css(e,&quot;display&quot;))&amp;&amp;(l?c=l:(le([e],!0),l=e.style.display||l,c=S.css(e,&quot;display&quot;),le([e]))),(&quot;inline&quot;===c||&quot;inline-block&quot;===c&amp;&amp;null!=l)&amp;&amp;&quot;none&quot;===S.css(e,&quot;float&quot;)&amp;&amp;(u||(p.done(function(){h.display=l}),null==l&amp;&amp;(c=h.display,l=&quot;none&quot;===c?&quot;&quot;:c)),h.display=&quot;inline-block&quot;)),n.overflow&amp;&amp;(h.overflow=&quot;hidden&quot;,p.always(function(){h.overflow=n.overflow[0],h.overflowX=n.overflow[1],h.overflowY=n.overflow[2]})),u=!1,d)u||(v?&quot;hidden&quot;in v&amp;&amp;(g=v.hidden):v=Y.access(e,&quot;fxshow&quot;,{display:l}),o&amp;&amp;(v.hidden=!g),g&amp;&amp;le([e],!0),p.done(function(){for(r in g||le([e]),Y.remove(e,&quot;fxshow&quot;),d)S.style(e,r,d[r])})),u=ut(g?v[r]:0,r,p),r in v||(v[r]=u.start,g&amp;&amp;(u.end=u.start,u.start=0))}],prefilter:function(e,t){t?lt.prefilters.unshift(e):lt.prefilters.push(e)}}),S.speed=function(e,t,n){var r=e&amp;&amp;&quot;object&quot;==typeof e?S.extend({},e):{complete:n||!n&amp;&amp;t||m(e)&amp;&amp;e,duration:e,easing:n&amp;&amp;t||t&amp;&amp;!m(t)&amp;&amp;t};return S.fx.off?r.duration=0:&quot;number&quot;!=typeof r.duration&amp;&amp;(r.duration in S.fx.speeds?r.duration=S.fx.speeds[r.duration]:r.duration=S.fx.speeds._default),null!=r.queue&amp;&amp;!0!==r.queue||(r.queue=&quot;fx&quot;),r.old=r.complete,r.complete=function(){m(r.old)&amp;&amp;r.old.call(this),r.queue&amp;&amp;S.dequeue(this,r.queue)},r},S.fn.extend({fadeTo:function(e,t,n,r){return this.filter(ae).css(&quot;opacity&quot;,0).show().end().animate({opacity:t},e,n,r)},animate:function(t,e,n,r){var i=S.isEmptyObject(t),o=S.speed(e,n,r),a=function(){var e=lt(this,S.extend({},t),o);(i||Y.get(this,&quot;finish&quot;))&amp;&amp;e.stop(!0)};return a.finish=a,i||!1===o.queue?this.each(a):this.queue(o.queue,a)},stop:function(i,e,o){var a=function(e){var t=e.stop;delete e.stop,t(o)};return&quot;string&quot;!=typeof i&amp;&amp;(o=e,e=i,i=void 0),e&amp;&amp;this.queue(i||&quot;fx&quot;,[]),this.each(function(){var e=!0,t=null!=i&amp;&amp;i+&quot;queueHooks&quot;,n=S.timers,r=Y.get(this);if(t)r[t]&amp;&amp;r[t].stop&amp;&amp;a(r[t]);else for(t in r)r[t]&amp;&amp;r[t].stop&amp;&amp;it.test(t)&amp;&amp;a(r[t]);for(t=n.length;t--;)n[t].elem!==this||null!=i&amp;&amp;n[t].queue!==i||(n[t].anim.stop(o),e=!1,n.splice(t,1));!e&amp;&amp;o||S.dequeue(this,i)})},finish:function(a){return!1!==a&amp;&amp;(a=a||&quot;fx&quot;),this.each(function(){var e,t=Y.get(this),n=t[a+&quot;queue&quot;],r=t[a+&quot;queueHooks&quot;],i=S.timers,o=n?n.length:0;for(t.finish=!0,S.queue(this,a,[]),r&amp;&amp;r.stop&amp;&amp;r.stop.call(this,!0),e=i.length;e--;)i[e].elem===this&amp;&amp;i[e].queue===a&amp;&amp;(i[e].anim.stop(!0),i.splice(e,1));for(e=0;e&lt;o;e++)n[e]&amp;&amp;n[e].finish&amp;&amp;n[e].finish.call(this);delete t.finish})}}),S.each([&quot;toggle&quot;,&quot;show&quot;,&quot;hide&quot;],function(e,r){var i=S.fn[r];S.fn[r]=function(e,t,n){return null==e||&quot;boolean&quot;==typeof e?i.apply(this,arguments):this.animate(st(r,!0),e,t,n)}}),S.each({slideDown:st(&quot;show&quot;),slideUp:st(&quot;hide&quot;),slideToggle:st(&quot;toggle&quot;),fadeIn:{opacity:&quot;show&quot;},fadeOut:{opacity:&quot;hide&quot;},fadeToggle:{opacity:&quot;toggle&quot;}},function(e,r){S.fn[e]=function(e,t,n){return this.animate(r,e,t,n)}}),S.timers=[],S.fx.tick=function(){var e,t=0,n=S.timers;for(Ze=Date.now();t&lt;n.length;t++)(e=n[t])()||n[t]!==e||n.splice(t--,1);n.length||S.fx.stop(),Ze=void 0},S.fx.timer=function(e){S.timers.push(e),S.fx.start()},S.fx.interval=13,S.fx.start=function(){et||(et=!0,ot())},S.fx.stop=function(){et=null},S.fx.speeds={slow:600,fast:200,_default:400},S.fn.delay=function(r,e){return r=S.fx&amp;&amp;S.fx.speeds[r]||r,e=e||&quot;fx&quot;,this.queue(e,function(e,t){var n=C.setTimeout(e,r);t.stop=function(){C.clearTimeout(n)}})},tt=E.createElement(&quot;input&quot;),nt=E.createElement(&quot;select&quot;).appendChild(E.createElement(&quot;option&quot;)),tt.type=&quot;checkbox&quot;,y.checkOn=&quot;&quot;!==tt.value,y.optSelected=nt.selected,(tt=E.createElement(&quot;input&quot;)).value=&quot;t&quot;,tt.type=&quot;radio&quot;,y.radioValue=&quot;t&quot;===tt.value;var ct,ft=S.expr.attrHandle;S.fn.extend({attr:function(e,t){return $(this,S.attr,e,t,1&lt;arguments.length)},removeAttr:function(e){return this.each(function(){S.removeAttr(this,e)})}}),S.extend({attr:function(e,t,n){var r,i,o=e.nodeType;if(3!==o&amp;&amp;8!==o&amp;&amp;2!==o)return&quot;undefined&quot;==typeof e.getAttribute?S.prop(e,t,n):(1===o&amp;&amp;S.isXMLDoc(e)||(i=S.attrHooks[t.toLowerCase()]||(S.expr.match.bool.test(t)?ct:void 0)),void 0!==n?null===n?void S.removeAttr(e,t):i&amp;&amp;&quot;set&quot;in i&amp;&amp;void 0!==(r=i.set(e,n,t))?r:(e.setAttribute(t,n+&quot;&quot;),n):i&amp;&amp;&quot;get&quot;in i&amp;&amp;null!==(r=i.get(e,t))?r:null==(r=S.find.attr(e,t))?void 0:r)},attrHooks:{type:{set:function(e,t){if(!y.radioValue&amp;&amp;&quot;radio&quot;===t&amp;&amp;A(e,&quot;input&quot;)){var n=e.value;return e.setAttribute(&quot;type&quot;,t),n&amp;&amp;(e.value=n),t}}}},removeAttr:function(e,t){var n,r=0,i=t&amp;&amp;t.match(P);if(i&amp;&amp;1===e.nodeType)while(n=i[r++])e.removeAttribute(n)}}),ct={set:function(e,t,n){return!1===t?S.removeAttr(e,n):e.setAttribute(n,n),n}},S.each(S.expr.match.bool.source.match(/\w+/g),function(e,t){var a=ft[t]||S.find.attr;ft[t]=function(e,t,n){var r,i,o=t.toLowerCase();return n||(i=ft[o],ft[o]=r,r=null!=a(e,t,n)?o:null,ft[o]=i),r}});var pt=/^(?:input|select|textarea|button)$/i,dt=/^(?:a|area)$/i;function ht(e){return(e.match(P)||[]).join(&quot; &quot;)}function gt(e){return e.getAttribute&amp;&amp;e.getAttribute(&quot;class&quot;)||&quot;&quot;}function vt(e){return Array.isArray(e)?e:&quot;string&quot;==typeof e&amp;&amp;e.match(P)||[]}S.fn.extend({prop:function(e,t){return $(this,S.prop,e,t,1&lt;arguments.length)},removeProp:function(e){return this.each(function(){delete this[S.propFix[e]||e]})}}),S.extend({prop:function(e,t,n){var r,i,o=e.nodeType;if(3!==o&amp;&amp;8!==o&amp;&amp;2!==o)return 1===o&amp;&amp;S.isXMLDoc(e)||(t=S.propFix[t]||t,i=S.propHooks[t]),void 0!==n?i&amp;&amp;&quot;set&quot;in i&amp;&amp;void 0!==(r=i.set(e,n,t))?r:e[t]=n:i&amp;&amp;&quot;get&quot;in i&amp;&amp;null!==(r=i.get(e,t))?r:e[t]},propHooks:{tabIndex:{get:function(e){var t=S.find.attr(e,&quot;tabindex&quot;);return t?parseInt(t,10):pt.test(e.nodeName)||dt.test(e.nodeName)&amp;&amp;e.href?0:-1}}},propFix:{&quot;for&quot;:&quot;htmlFor&quot;,&quot;class&quot;:&quot;className&quot;}}),y.optSelected||(S.propHooks.selected={get:function(e){var t=e.parentNode;return t&amp;&amp;t.parentNode&amp;&amp;t.parentNode.selectedIndex,null},set:function(e){var t=e.parentNode;t&amp;&amp;(t.selectedIndex,t.parentNode&amp;&amp;t.parentNode.selectedIndex)}}),S.each([&quot;tabIndex&quot;,&quot;readOnly&quot;,&quot;maxLength&quot;,&quot;cellSpacing&quot;,&quot;cellPadding&quot;,&quot;rowSpan&quot;,&quot;colSpan&quot;,&quot;useMap&quot;,&quot;frameBorder&quot;,&quot;contentEditable&quot;],function(){S.propFix[this.toLowerCase()]=this}),S.fn.extend({addClass:function(t){var e,n,r,i,o,a,s,u=0;if(m(t))return this.each(function(e){S(this).addClass(t.call(this,e,gt(this)))});if((e=vt(t)).length)while(n=this[u++])if(i=gt(n),r=1===n.nodeType&amp;&amp;&quot; &quot;+ht(i)+&quot; &quot;){a=0;while(o=e[a++])r.indexOf(&quot; &quot;+o+&quot; &quot;)&lt;0&amp;&amp;(r+=o+&quot; &quot;);i!==(s=ht(r))&amp;&amp;n.setAttribute(&quot;class&quot;,s)}return this},removeClass:function(t){var e,n,r,i,o,a,s,u=0;if(m(t))return this.each(function(e){S(this).removeClass(t.call(this,e,gt(this)))});if(!arguments.length)return this.attr(&quot;class&quot;,&quot;&quot;);if((e=vt(t)).length)while(n=this[u++])if(i=gt(n),r=1===n.nodeType&amp;&amp;&quot; &quot;+ht(i)+&quot; &quot;){a=0;while(o=e[a++])while(-1&lt;r.indexOf(&quot; &quot;+o+&quot; &quot;))r=r.replace(&quot; &quot;+o+&quot; &quot;,&quot; &quot;);i!==(s=ht(r))&amp;&amp;n.setAttribute(&quot;class&quot;,s)}return this},toggleClass:function(i,t){var o=typeof i,a=&quot;string&quot;===o||Array.isArray(i);return&quot;boolean&quot;==typeof t&amp;&amp;a?t?this.addClass(i):this.removeClass(i):m(i)?this.each(function(e){S(this).toggleClass(i.call(this,e,gt(this),t),t)}):this.each(function(){var e,t,n,r;if(a){t=0,n=S(this),r=vt(i);while(e=r[t++])n.hasClass(e)?n.removeClass(e):n.addClass(e)}else void 0!==i&amp;&amp;&quot;boolean&quot;!==o||((e=gt(this))&amp;&amp;Y.set(this,&quot;__className__&quot;,e),this.setAttribute&amp;&amp;this.setAttribute(&quot;class&quot;,e||!1===i?&quot;&quot;:Y.get(this,&quot;__className__&quot;)||&quot;&quot;))})},hasClass:function(e){var t,n,r=0;t=&quot; &quot;+e+&quot; &quot;;while(n=this[r++])if(1===n.nodeType&amp;&amp;-1&lt;(&quot; &quot;+ht(gt(n))+&quot; &quot;).indexOf(t))return!0;return!1}});var yt=/\r/g;S.fn.extend({val:function(n){var r,e,i,t=this[0];return arguments.length?(i=m(n),this.each(function(e){var t;1===this.nodeType&amp;&amp;(null==(t=i?n.call(this,e,S(this).val()):n)?t=&quot;&quot;:&quot;number&quot;==typeof t?t+=&quot;&quot;:Array.isArray(t)&amp;&amp;(t=S.map(t,function(e){return null==e?&quot;&quot;:e+&quot;&quot;})),(r=S.valHooks[this.type]||S.valHooks[this.nodeName.toLowerCase()])&amp;&amp;&quot;set&quot;in r&amp;&amp;void 0!==r.set(this,t,&quot;value&quot;)||(this.value=t))})):t?(r=S.valHooks[t.type]||S.valHooks[t.nodeName.toLowerCase()])&amp;&amp;&quot;get&quot;in r&amp;&amp;void 0!==(e=r.get(t,&quot;value&quot;))?e:&quot;string&quot;==typeof(e=t.value)?e.replace(yt,&quot;&quot;):null==e?&quot;&quot;:e:void 0}}),S.extend({valHooks:{option:{get:function(e){var t=S.find.attr(e,&quot;value&quot;);return null!=t?t:ht(S.text(e))}},select:{get:function(e){var t,n,r,i=e.options,o=e.selectedIndex,a=&quot;select-one&quot;===e.type,s=a?null:[],u=a?o+1:i.length;for(r=o&lt;0?u:a?o:0;r&lt;u;r++)if(((n=i[r]).selected||r===o)&amp;&amp;!n.disabled&amp;&amp;(!n.parentNode.disabled||!A(n.parentNode,&quot;optgroup&quot;))){if(t=S(n).val(),a)return t;s.push(t)}return s},set:function(e,t){var n,r,i=e.options,o=S.makeArray(t),a=i.length;while(a--)((r=i[a]).selected=-1&lt;S.inArray(S.valHooks.option.get(r),o))&amp;&amp;(n=!0);return n||(e.selectedIndex=-1),o}}}}),S.each([&quot;radio&quot;,&quot;checkbox&quot;],function(){S.valHooks[this]={set:function(e,t){if(Array.isArray(t))return e.checked=-1&lt;S.inArray(S(e).val(),t)}},y.checkOn||(S.valHooks[this].get=function(e){return null===e.getAttribute(&quot;value&quot;)?&quot;on&quot;:e.value})}),y.focusin=&quot;onfocusin&quot;in C;var mt=/^(?:focusinfocus|focusoutblur)$/,xt=function(e){e.stopPropagation()};S.extend(S.event,{trigger:function(e,t,n,r){var i,o,a,s,u,l,c,f,p=[n||E],d=v.call(e,&quot;type&quot;)?e.type:e,h=v.call(e,&quot;namespace&quot;)?e.namespace.split(&quot;.&quot;):[];if(o=f=a=n=n||E,3!==n.nodeType&amp;&amp;8!==n.nodeType&amp;&amp;!mt.test(d+S.event.triggered)&amp;&amp;(-1&lt;d.indexOf(&quot;.&quot;)&amp;&amp;(d=(h=d.split(&quot;.&quot;)).shift(),h.sort()),u=d.indexOf(&quot;:&quot;)&lt;0&amp;&amp;&quot;on&quot;+d,(e=e[S.expando]?e:new S.Event(d,&quot;object&quot;==typeof e&amp;&amp;e)).isTrigger=r?2:3,e.namespace=h.join(&quot;.&quot;),e.rnamespace=e.namespace?new RegExp(&quot;(^|\\.)&quot;+h.join(&quot;\\.(?:.*\\.|)&quot;)+&quot;(\\.|$)&quot;):null,e.result=void 0,e.target||(e.target=n),t=null==t?[e]:S.makeArray(t,[e]),c=S.event.special[d]||{},r||!c.trigger||!1!==c.trigger.apply(n,t))){if(!r&amp;&amp;!c.noBubble&amp;&amp;!x(n)){for(s=c.delegateType||d,mt.test(s+d)||(o=o.parentNode);o;o=o.parentNode)p.push(o),a=o;a===(n.ownerDocument||E)&amp;&amp;p.push(a.defaultView||a.parentWindow||C)}i=0;while((o=p[i++])&amp;&amp;!e.isPropagationStopped())f=o,e.type=1&lt;i?s:c.bindType||d,(l=(Y.get(o,&quot;events&quot;)||Object.create(null))[e.type]&amp;&amp;Y.get(o,&quot;handle&quot;))&amp;&amp;l.apply(o,t),(l=u&amp;&amp;o[u])&amp;&amp;l.apply&amp;&amp;V(o)&amp;&amp;(e.result=l.apply(o,t),!1===e.result&amp;&amp;e.preventDefault());return e.type=d,r||e.isDefaultPrevented()||c._default&amp;&amp;!1!==c._default.apply(p.pop(),t)||!V(n)||u&amp;&amp;m(n[d])&amp;&amp;!x(n)&amp;&amp;((a=n[u])&amp;&amp;(n[u]=null),S.event.triggered=d,e.isPropagationStopped()&amp;&amp;f.addEventListener(d,xt),n[d](),e.isPropagationStopped()&amp;&amp;f.removeEventListener(d,xt),S.event.triggered=void 0,a&amp;&amp;(n[u]=a)),e.result}},simulate:function(e,t,n){var r=S.extend(new S.Event,n,{type:e,isSimulated:!0});S.event.trigger(r,null,t)}}),S.fn.extend({trigger:function(e,t){return this.each(function(){S.event.trigger(e,t,this)})},triggerHandler:function(e,t){var n=this[0];if(n)return S.event.trigger(e,t,n,!0)}}),y.focusin||S.each({focus:&quot;focusin&quot;,blur:&quot;focusout&quot;},function(n,r){var i=function(e){S.event.simulate(r,e.target,S.event.fix(e))};S.event.special[r]={setup:function(){var e=this.ownerDocument||this.document||this,t=Y.access(e,r);t||e.addEventListener(n,i,!0),Y.access(e,r,(t||0)+1)},teardown:function(){var e=this.ownerDocument||this.document||this,t=Y.access(e,r)-1;t?Y.access(e,r,t):(e.removeEventListener(n,i,!0),Y.remove(e,r))}}});var bt=C.location,wt={guid:Date.now()},Tt=/\?/;S.parseXML=function(e){var t,n;if(!e||&quot;string&quot;!=typeof e)return null;try{t=(new C.DOMParser).parseFromString(e,&quot;text/xml&quot;)}catch(e){}return n=t&amp;&amp;t.getElementsByTagName(&quot;parsererror&quot;)[0],t&amp;&amp;!n||S.error(&quot;Invalid XML: &quot;+(n?S.map(n.childNodes,function(e){return e.textContent}).join(&quot;\n&quot;):e)),t};var Ct=/\[\]$/,Et=/\r?\n/g,St=/^(?:submit|button|image|reset|file)$/i,kt=/^(?:input|select|textarea|keygen)/i;function At(n,e,r,i){var t;if(Array.isArray(e))S.each(e,function(e,t){r||Ct.test(n)?i(n,t):At(n+&quot;[&quot;+(&quot;object&quot;==typeof t&amp;&amp;null!=t?e:&quot;&quot;)+&quot;]&quot;,t,r,i)});else if(r||&quot;object&quot;!==w(e))i(n,e);else for(t in e)At(n+&quot;[&quot;+t+&quot;]&quot;,e[t],r,i)}S.param=function(e,t){var n,r=[],i=function(e,t){var n=m(t)?t():t;r[r.length]=encodeURIComponent(e)+&quot;=&quot;+encodeURIComponent(null==n?&quot;&quot;:n)};if(null==e)return&quot;&quot;;if(Array.isArray(e)||e.jquery&amp;&amp;!S.isPlainObject(e))S.each(e,function(){i(this.name,this.value)});else for(n in e)At(n,e[n],t,i);return r.join(&quot;&amp;&quot;)},S.fn.extend({serialize:function(){return S.param(this.serializeArray())},serializeArray:function(){return this.map(function(){var e=S.prop(this,&quot;elements&quot;);return e?S.makeArray(e):this}).filter(function(){var e=this.type;return this.name&amp;&amp;!S(this).is(&quot;:disabled&quot;)&amp;&amp;kt.test(this.nodeName)&amp;&amp;!St.test(e)&amp;&amp;(this.checked||!pe.test(e))}).map(function(e,t){var n=S(this).val();return null==n?null:Array.isArray(n)?S.map(n,function(e){return{name:t.name,value:e.replace(Et,&quot;\r\n&quot;)}}):{name:t.name,value:n.replace(Et,&quot;\r\n&quot;)}}).get()}});var Nt=/%20/g,jt=/#.*$/,Dt=/([?&amp;])_=[^&amp;]*/,qt=/^(.*?):[ \t]*([^\r\n]*)$/gm,Lt=/^(?:GET|HEAD)$/,Ht=/^\/\//,Ot={},Pt={},Rt=&quot;*/&quot;.concat(&quot;*&quot;),Mt=E.createElement(&quot;a&quot;);function It(o){return function(e,t){&quot;string&quot;!=typeof e&amp;&amp;(t=e,e=&quot;*&quot;);var n,r=0,i=e.toLowerCase().match(P)||[];if(m(t))while(n=i[r++])&quot;+&quot;===n[0]?(n=n.slice(1)||&quot;*&quot;,(o[n]=o[n]||[]).unshift(t)):(o[n]=o[n]||[]).push(t)}}function Wt(t,i,o,a){var s={},u=t===Pt;function l(e){var r;return s[e]=!0,S.each(t[e]||[],function(e,t){var n=t(i,o,a);return&quot;string&quot;!=typeof n||u||s[n]?u?!(r=n):void 0:(i.dataTypes.unshift(n),l(n),!1)}),r}return l(i.dataTypes[0])||!s[&quot;*&quot;]&amp;&amp;l(&quot;*&quot;)}function Ft(e,t){var n,r,i=S.ajaxSettings.flatOptions||{};for(n in t)void 0!==t[n]&amp;&amp;((i[n]?e:r||(r={}))[n]=t[n]);return r&amp;&amp;S.extend(!0,e,r),e}Mt.href=bt.href,S.extend({active:0,lastModified:{},etag:{},ajaxSettings:{url:bt.href,type:&quot;GET&quot;,isLocal:/^(?:about|app|app-storage|.+-extension|file|res|widget):$/.test(bt.protocol),global:!0,processData:!0,async:!0,contentType:&quot;application/x-www-form-urlencoded; charset=UTF-8&quot;,accepts:{&quot;*&quot;:Rt,text:&quot;text/plain&quot;,html:&quot;text/html&quot;,xml:&quot;application/xml, text/xml&quot;,json:&quot;application/json, text/javascript&quot;},contents:{xml:/\bxml\b/,html:/\bhtml/,json:/\bjson\b/},responseFields:{xml:&quot;responseXML&quot;,text:&quot;responseText&quot;,json:&quot;responseJSON&quot;},converters:{&quot;* text&quot;:String,&quot;text html&quot;:!0,&quot;text json&quot;:JSON.parse,&quot;text xml&quot;:S.parseXML},flatOptions:{url:!0,context:!0}},ajaxSetup:function(e,t){return t?Ft(Ft(e,S.ajaxSettings),t):Ft(S.ajaxSettings,e)},ajaxPrefilter:It(Ot),ajaxTransport:It(Pt),ajax:function(e,t){&quot;object&quot;==typeof e&amp;&amp;(t=e,e=void 0),t=t||{};var c,f,p,n,d,r,h,g,i,o,v=S.ajaxSetup({},t),y=v.context||v,m=v.context&amp;&amp;(y.nodeType||y.jquery)?S(y):S.event,x=S.Deferred(),b=S.Callbacks(&quot;once memory&quot;),w=v.statusCode||{},a={},s={},u=&quot;canceled&quot;,T={readyState:0,getResponseHeader:function(e){var t;if(h){if(!n){n={};while(t=qt.exec(p))n[t[1].toLowerCase()+&quot; &quot;]=(n[t[1].toLowerCase()+&quot; &quot;]||[]).concat(t[2])}t=n[e.toLowerCase()+&quot; &quot;]}return null==t?null:t.join(&quot;, &quot;)},getAllResponseHeaders:function(){return h?p:null},setRequestHeader:function(e,t){return null==h&amp;&amp;(e=s[e.toLowerCase()]=s[e.toLowerCase()]||e,a[e]=t),this},overrideMimeType:function(e){return null==h&amp;&amp;(v.mimeType=e),this},statusCode:function(e){var t;if(e)if(h)T.always(e[T.status]);else for(t in e)w[t]=[w[t],e[t]];return this},abort:function(e){var t=e||u;return c&amp;&amp;c.abort(t),l(0,t),this}};if(x.promise(T),v.url=((e||v.url||bt.href)+&quot;&quot;).replace(Ht,bt.protocol+&quot;//&quot;),v.type=t.method||t.type||v.method||v.type,v.dataTypes=(v.dataType||&quot;*&quot;).toLowerCase().match(P)||[&quot;&quot;],null==v.crossDomain){r=E.createElement(&quot;a&quot;);try{r.href=v.url,r.href=r.href,v.crossDomain=Mt.protocol+&quot;//&quot;+Mt.host!=r.protocol+&quot;//&quot;+r.host}catch(e){v.crossDomain=!0}}if(v.data&amp;&amp;v.processData&amp;&amp;&quot;string&quot;!=typeof v.data&amp;&amp;(v.data=S.param(v.data,v.traditional)),Wt(Ot,v,t,T),h)return T;for(i in(g=S.event&amp;&amp;v.global)&amp;&amp;0==S.active++&amp;&amp;S.event.trigger(&quot;ajaxStart&quot;),v.type=v.type.toUpperCase(),v.hasContent=!Lt.test(v.type),f=v.url.replace(jt,&quot;&quot;),v.hasContent?v.data&amp;&amp;v.processData&amp;&amp;0===(v.contentType||&quot;&quot;).indexOf(&quot;application/x-www-form-urlencoded&quot;)&amp;&amp;(v.data=v.data.replace(Nt,&quot;+&quot;)):(o=v.url.slice(f.length),v.data&amp;&amp;(v.processData||&quot;string&quot;==typeof v.data)&amp;&amp;(f+=(Tt.test(f)?&quot;&amp;&quot;:&quot;?&quot;)+v.data,delete v.data),!1===v.cache&amp;&amp;(f=f.replace(Dt,&quot;$1&quot;),o=(Tt.test(f)?&quot;&amp;&quot;:&quot;?&quot;)+&quot;_=&quot;+wt.guid+++o),v.url=f+o),v.ifModified&amp;&amp;(S.lastModified[f]&amp;&amp;T.setRequestHeader(&quot;If-Modified-Since&quot;,S.lastModified[f]),S.etag[f]&amp;&amp;T.setRequestHeader(&quot;If-None-Match&quot;,S.etag[f])),(v.data&amp;&amp;v.hasContent&amp;&amp;!1!==v.contentType||t.contentType)&amp;&amp;T.setRequestHeader(&quot;Content-Type&quot;,v.contentType),T.setRequestHeader(&quot;Accept&quot;,v.dataTypes[0]&amp;&amp;v.accepts[v.dataTypes[0]]?v.accepts[v.dataTypes[0]]+(&quot;*&quot;!==v.dataTypes[0]?&quot;, &quot;+Rt+&quot;; q=0.01&quot;:&quot;&quot;):v.accepts[&quot;*&quot;]),v.headers)T.setRequestHeader(i,v.headers[i]);if(v.beforeSend&amp;&amp;(!1===v.beforeSend.call(y,T,v)||h))return T.abort();if(u=&quot;abort&quot;,b.add(v.complete),T.done(v.success),T.fail(v.error),c=Wt(Pt,v,t,T)){if(T.readyState=1,g&amp;&amp;m.trigger(&quot;ajaxSend&quot;,[T,v]),h)return T;v.async&amp;&amp;0&lt;v.timeout&amp;&amp;(d=C.setTimeout(function(){T.abort(&quot;timeout&quot;)},v.timeout));try{h=!1,c.send(a,l)}catch(e){if(h)throw e;l(-1,e)}}else l(-1,&quot;No Transport&quot;);function l(e,t,n,r){var i,o,a,s,u,l=t;h||(h=!0,d&amp;&amp;C.clearTimeout(d),c=void 0,p=r||&quot;&quot;,T.readyState=0&lt;e?4:0,i=200&lt;=e&amp;&amp;e&lt;300||304===e,n&amp;&amp;(s=function(e,t,n){var r,i,o,a,s=e.contents,u=e.dataTypes;while(&quot;*&quot;===u[0])u.shift(),void 0===r&amp;&amp;(r=e.mimeType||t.getResponseHeader(&quot;Content-Type&quot;));if(r)for(i in s)if(s[i]&amp;&amp;s[i].test(r)){u.unshift(i);break}if(u[0]in n)o=u[0];else{for(i in n){if(!u[0]||e.converters[i+&quot; &quot;+u[0]]){o=i;break}a||(a=i)}o=o||a}if(o)return o!==u[0]&amp;&amp;u.unshift(o),n[o]}(v,T,n)),!i&amp;&amp;-1&lt;S.inArray(&quot;script&quot;,v.dataTypes)&amp;&amp;S.inArray(&quot;json&quot;,v.dataTypes)&lt;0&amp;&amp;(v.converters[&quot;text script&quot;]=function(){}),s=function(e,t,n,r){var i,o,a,s,u,l={},c=e.dataTypes.slice();if(c[1])for(a in e.converters)l[a.toLowerCase()]=e.converters[a];o=c.shift();while(o)if(e.responseFields[o]&amp;&amp;(n[e.responseFields[o]]=t),!u&amp;&amp;r&amp;&amp;e.dataFilter&amp;&amp;(t=e.dataFilter(t,e.dataType)),u=o,o=c.shift())if(&quot;*&quot;===o)o=u;else if(&quot;*&quot;!==u&amp;&amp;u!==o){if(!(a=l[u+&quot; &quot;+o]||l[&quot;* &quot;+o]))for(i in l)if((s=i.split(&quot; &quot;))[1]===o&amp;&amp;(a=l[u+&quot; &quot;+s[0]]||l[&quot;* &quot;+s[0]])){!0===a?a=l[i]:!0!==l[i]&amp;&amp;(o=s[0],c.unshift(s[1]));break}if(!0!==a)if(a&amp;&amp;e[&quot;throws&quot;])t=a(t);else try{t=a(t)}catch(e){return{state:&quot;parsererror&quot;,error:a?e:&quot;No conversion from &quot;+u+&quot; to &quot;+o}}}return{state:&quot;success&quot;,data:t}}(v,s,T,i),i?(v.ifModified&amp;&amp;((u=T.getResponseHeader(&quot;Last-Modified&quot;))&amp;&amp;(S.lastModified[f]=u),(u=T.getResponseHeader(&quot;etag&quot;))&amp;&amp;(S.etag[f]=u)),204===e||&quot;HEAD&quot;===v.type?l=&quot;nocontent&quot;:304===e?l=&quot;notmodified&quot;:(l=s.state,o=s.data,i=!(a=s.error))):(a=l,!e&amp;&amp;l||(l=&quot;error&quot;,e&lt;0&amp;&amp;(e=0))),T.status=e,T.statusText=(t||l)+&quot;&quot;,i?x.resolveWith(y,[o,l,T]):x.rejectWith(y,[T,l,a]),T.statusCode(w),w=void 0,g&amp;&amp;m.trigger(i?&quot;ajaxSuccess&quot;:&quot;ajaxError&quot;,[T,v,i?o:a]),b.fireWith(y,[T,l]),g&amp;&amp;(m.trigger(&quot;ajaxComplete&quot;,[T,v]),--S.active||S.event.trigger(&quot;ajaxStop&quot;)))}return T},getJSON:function(e,t,n){return S.get(e,t,n,&quot;json&quot;)},getScript:function(e,t){return S.get(e,void 0,t,&quot;script&quot;)}}),S.each([&quot;get&quot;,&quot;post&quot;],function(e,i){S[i]=function(e,t,n,r){return m(t)&amp;&amp;(r=r||n,n=t,t=void 0),S.ajax(S.extend({url:e,type:i,dataType:r,data:t,success:n},S.isPlainObject(e)&amp;&amp;e))}}),S.ajaxPrefilter(function(e){var t;for(t in e.headers)&quot;content-type&quot;===t.toLowerCase()&amp;&amp;(e.contentType=e.headers[t]||&quot;&quot;)}),S._evalUrl=function(e,t,n){return S.ajax({url:e,type:&quot;GET&quot;,dataType:&quot;script&quot;,cache:!0,async:!1,global:!1,converters:{&quot;text script&quot;:function(){}},dataFilter:function(e){S.globalEval(e,t,n)}})},S.fn.extend({wrapAll:function(e){var t;return this[0]&amp;&amp;(m(e)&amp;&amp;(e=e.call(this[0])),t=S(e,this[0].ownerDocument).eq(0).clone(!0),this[0].parentNode&amp;&amp;t.insertBefore(this[0]),t.map(function(){var e=this;while(e.firstElementChild)e=e.firstElementChild;return e}).append(this)),this},wrapInner:function(n){return m(n)?this.each(function(e){S(this).wrapInner(n.call(this,e))}):this.each(function(){var e=S(this),t=e.contents();t.length?t.wrapAll(n):e.append(n)})},wrap:function(t){var n=m(t);return this.each(function(e){S(this).wrapAll(n?t.call(this,e):t)})},unwrap:function(e){return this.parent(e).not(&quot;body&quot;).each(function(){S(this).replaceWith(this.childNodes)}),this}}),S.expr.pseudos.hidden=function(e){return!S.expr.pseudos.visible(e)},S.expr.pseudos.visible=function(e){return!!(e.offsetWidth||e.offsetHeight||e.getClientRects().length)},S.ajaxSettings.xhr=function(){try{return new C.XMLHttpRequest}catch(e){}};var Bt={0:200,1223:204},$t=S.ajaxSettings.xhr();y.cors=!!$t&amp;&amp;&quot;withCredentials&quot;in $t,y.ajax=$t=!!$t,S.ajaxTransport(function(i){var o,a;if(y.cors||$t&amp;&amp;!i.crossDomain)return{send:function(e,t){var n,r=i.xhr();if(r.open(i.type,i.url,i.async,i.username,i.password),i.xhrFields)for(n in i.xhrFields)r[n]=i.xhrFields[n];for(n in i.mimeType&amp;&amp;r.overrideMimeType&amp;&amp;r.overrideMimeType(i.mimeType),i.crossDomain||e[&quot;X-Requested-With&quot;]||(e[&quot;X-Requested-With&quot;]=&quot;XMLHttpRequest&quot;),e)r.setRequestHeader(n,e[n]);o=function(e){return function(){o&amp;&amp;(o=a=r.onload=r.onerror=r.onabort=r.ontimeout=r.onreadystatechange=null,&quot;abort&quot;===e?r.abort():&quot;error&quot;===e?&quot;number&quot;!=typeof r.status?t(0,&quot;error&quot;):t(r.status,r.statusText):t(Bt[r.status]||r.status,r.statusText,&quot;text&quot;!==(r.responseType||&quot;text&quot;)||&quot;string&quot;!=typeof r.responseText?{binary:r.response}:{text:r.responseText},r.getAllResponseHeaders()))}},r.onload=o(),a=r.onerror=r.ontimeout=o(&quot;error&quot;),void 0!==r.onabort?r.onabort=a:r.onreadystatechange=function(){4===r.readyState&amp;&amp;C.setTimeout(function(){o&amp;&amp;a()})},o=o(&quot;abort&quot;);try{r.send(i.hasContent&amp;&amp;i.data||null)}catch(e){if(o)throw e}},abort:function(){o&amp;&amp;o()}}}),S.ajaxPrefilter(function(e){e.crossDomain&amp;&amp;(e.contents.script=!1)}),S.ajaxSetup({accepts:{script:&quot;text/javascript, application/javascript, application/ecmascript, application/x-ecmascript&quot;},contents:{script:/\b(?:java|ecma)script\b/},converters:{&quot;text script&quot;:function(e){return S.globalEval(e),e}}}),S.ajaxPrefilter(&quot;script&quot;,function(e){void 0===e.cache&amp;&amp;(e.cache=!1),e.crossDomain&amp;&amp;(e.type=&quot;GET&quot;)}),S.ajaxTransport(&quot;script&quot;,function(n){var r,i;if(n.crossDomain||n.scriptAttrs)return{send:function(e,t){r=S(&quot;&lt;script>&quot;).attr(n.scriptAttrs||{}).prop({charset:n.scriptCharset,src:n.url}).on(&quot;load error&quot;,i=function(e){r.remove(),i=null,e&amp;&amp;t(&quot;error&quot;===e.type?404:200,e.type)}),E.head.appendChild(r[0])},abort:function(){i&amp;&amp;i()}}});var _t,zt=[],Ut=/(=)\?(?=&amp;|$)|\?\?/;S.ajaxSetup({jsonp:&quot;callback&quot;,jsonpCallback:function(){var e=zt.pop()||S.expando+&quot;_&quot;+wt.guid++;return this[e]=!0,e}}),S.ajaxPrefilter(&quot;json jsonp&quot;,function(e,t,n){var r,i,o,a=!1!==e.jsonp&amp;&amp;(Ut.test(e.url)?&quot;url&quot;:&quot;string&quot;==typeof e.data&amp;&amp;0===(e.contentType||&quot;&quot;).indexOf(&quot;application/x-www-form-urlencoded&quot;)&amp;&amp;Ut.test(e.data)&amp;&amp;&quot;data&quot;);if(a||&quot;jsonp&quot;===e.dataTypes[0])return r=e.jsonpCallback=m(e.jsonpCallback)?e.jsonpCallback():e.jsonpCallback,a?e[a]=e[a].replace(Ut,&quot;$1&quot;+r):!1!==e.jsonp&amp;&amp;(e.url+=(Tt.test(e.url)?&quot;&amp;&quot;:&quot;?&quot;)+e.jsonp+&quot;=&quot;+r),e.converters[&quot;script json&quot;]=function(){return o||S.error(r+&quot; was not called&quot;),o[0]},e.dataTypes[0]=&quot;json&quot;,i=C[r],C[r]=function(){o=arguments},n.always(function(){void 0===i?S(C).removeProp(r):C[r]=i,e[r]&amp;&amp;(e.jsonpCallback=t.jsonpCallback,zt.push(r)),o&amp;&amp;m(i)&amp;&amp;i(o[0]),o=i=void 0}),&quot;script&quot;}),y.createHTMLDocument=((_t=E.implementation.createHTMLDocument(&quot;&quot;).body).innerHTML=&quot;&lt;form>&lt;/form>&lt;form>&lt;/form>&quot;,2===_t.childNodes.length),S.parseHTML=function(e,t,n){return&quot;string&quot;!=typeof e?[]:(&quot;boolean&quot;==typeof t&amp;&amp;(n=t,t=!1),t||(y.createHTMLDocument?((r=(t=E.implementation.createHTMLDocument(&quot;&quot;)).createElement(&quot;base&quot;)).href=E.location.href,t.head.appendChild(r)):t=E),o=!n&amp;&amp;[],(i=N.exec(e))?[t.createElement(i[1])]:(i=xe([e],t,o),o&amp;&amp;o.length&amp;&amp;S(o).remove(),S.merge([],i.childNodes)));var r,i,o},S.fn.load=function(e,t,n){var r,i,o,a=this,s=e.indexOf(&quot; &quot;);return-1&lt;s&amp;&amp;(r=ht(e.slice(s)),e=e.slice(0,s)),m(t)?(n=t,t=void 0):t&amp;&amp;&quot;object&quot;==typeof t&amp;&amp;(i=&quot;POST&quot;),0&lt;a.length&amp;&amp;S.ajax({url:e,type:i||&quot;GET&quot;,dataType:&quot;html&quot;,data:t}).done(function(e){o=arguments,a.html(r?S(&quot;&lt;div>&quot;).append(S.parseHTML(e)).find(r):e)}).always(n&amp;&amp;function(e,t){a.each(function(){n.apply(this,o||[e.responseText,t,e])})}),this},S.expr.pseudos.animated=function(t){return S.grep(S.timers,function(e){return t===e.elem}).length},S.offset={setOffset:function(e,t,n){var r,i,o,a,s,u,l=S.css(e,&quot;position&quot;),c=S(e),f={};&quot;static&quot;===l&amp;&amp;(e.style.position=&quot;relative&quot;),s=c.offset(),o=S.css(e,&quot;top&quot;),u=S.css(e,&quot;left&quot;),(&quot;absolute&quot;===l||&quot;fixed&quot;===l)&amp;&amp;-1&lt;(o+u).indexOf(&quot;auto&quot;)?(a=(r=c.position()).top,i=r.left):(a=parseFloat(o)||0,i=parseFloat(u)||0),m(t)&amp;&amp;(t=t.call(e,n,S.extend({},s))),null!=t.top&amp;&amp;(f.top=t.top-s.top+a),null!=t.left&amp;&amp;(f.left=t.left-s.left+i),&quot;using&quot;in t?t.using.call(e,f):c.css(f)}},S.fn.extend({offset:function(t){if(arguments.length)return void 0===t?this:this.each(function(e){S.offset.setOffset(this,t,e)});var e,n,r=this[0];return r?r.getClientRects().length?(e=r.getBoundingClientRect(),n=r.ownerDocument.defaultView,{top:e.top+n.pageYOffset,left:e.left+n.pageXOffset}):{top:0,left:0}:void 0},position:function(){if(this[0]){var e,t,n,r=this[0],i={top:0,left:0};if(&quot;fixed&quot;===S.css(r,&quot;position&quot;))t=r.getBoundingClientRect();else{t=this.offset(),n=r.ownerDocument,e=r.offsetParent||n.documentElement;while(e&amp;&amp;(e===n.body||e===n.documentElement)&amp;&amp;&quot;static&quot;===S.css(e,&quot;position&quot;))e=e.parentNode;e&amp;&amp;e!==r&amp;&amp;1===e.nodeType&amp;&amp;((i=S(e).offset()).top+=S.css(e,&quot;borderTopWidth&quot;,!0),i.left+=S.css(e,&quot;borderLeftWidth&quot;,!0))}return{top:t.top-i.top-S.css(r,&quot;marginTop&quot;,!0),left:t.left-i.left-S.css(r,&quot;marginLeft&quot;,!0)}}},offsetParent:function(){return this.map(function(){var e=this.offsetParent;while(e&amp;&amp;&quot;static&quot;===S.css(e,&quot;position&quot;))e=e.offsetParent;return e||re})}}),S.each({scrollLeft:&quot;pageXOffset&quot;,scrollTop:&quot;pageYOffset&quot;},function(t,i){var o=&quot;pageYOffset&quot;===i;S.fn[t]=function(e){return $(this,function(e,t,n){var r;if(x(e)?r=e:9===e.nodeType&amp;&amp;(r=e.defaultView),void 0===n)return r?r[i]:e[t];r?r.scrollTo(o?r.pageXOffset:n,o?n:r.pageYOffset):e[t]=n},t,e,arguments.length)}}),S.each([&quot;top&quot;,&quot;left&quot;],function(e,n){S.cssHooks[n]=Fe(y.pixelPosition,function(e,t){if(t)return t=We(e,n),Pe.test(t)?S(e).position()[n]+&quot;px&quot;:t})}),S.each({Height:&quot;height&quot;,Width:&quot;width&quot;},function(a,s){S.each({padding:&quot;inner&quot;+a,content:s,&quot;&quot;:&quot;outer&quot;+a},function(r,o){S.fn[o]=function(e,t){var n=arguments.length&amp;&amp;(r||&quot;boolean&quot;!=typeof e),i=r||(!0===e||!0===t?&quot;margin&quot;:&quot;border&quot;);return $(this,function(e,t,n){var r;return x(e)?0===o.indexOf(&quot;outer&quot;)?e[&quot;inner&quot;+a]:e.document.documentElement[&quot;client&quot;+a]:9===e.nodeType?(r=e.documentElement,Math.max(e.body[&quot;scroll&quot;+a],r[&quot;scroll&quot;+a],e.body[&quot;offset&quot;+a],r[&quot;offset&quot;+a],r[&quot;client&quot;+a])):void 0===n?S.css(e,t,i):S.style(e,t,n,i)},s,n?e:void 0,n)}})}),S.each([&quot;ajaxStart&quot;,&quot;ajaxStop&quot;,&quot;ajaxComplete&quot;,&quot;ajaxError&quot;,&quot;ajaxSuccess&quot;,&quot;ajaxSend&quot;],function(e,t){S.fn[t]=function(e){return this.on(t,e)}}),S.fn.extend({bind:function(e,t,n){return this.on(e,null,t,n)},unbind:function(e,t){return this.off(e,null,t)},delegate:function(e,t,n,r){return this.on(t,e,n,r)},undelegate:function(e,t,n){return 1===arguments.length?this.off(e,&quot;**&quot;):this.off(t,e||&quot;**&quot;,n)},hover:function(e,t){return this.mouseenter(e).mouseleave(t||e)}}),S.each(&quot;blur focus focusin focusout resize scroll click dblclick mousedown mouseup mousemove mouseover mouseout mouseenter mouseleave change select submit keydown keypress keyup contextmenu&quot;.split(&quot; &quot;),function(e,n){S.fn[n]=function(e,t){return 0&lt;arguments.length?this.on(n,null,e,t):this.trigger(n)}});var Xt=/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g;S.proxy=function(e,t){var n,r,i;if(&quot;string&quot;==typeof t&amp;&amp;(n=e[t],t=e,e=n),m(e))return r=s.call(arguments,2),(i=function(){return e.apply(t||this,r.concat(s.call(arguments)))}).guid=e.guid=e.guid||S.guid++,i},S.holdReady=function(e){e?S.readyWait++:S.ready(!0)},S.isArray=Array.isArray,S.parseJSON=JSON.parse,S.nodeName=A,S.isFunction=m,S.isWindow=x,S.camelCase=X,S.type=w,S.now=Date.now,S.isNumeric=function(e){var t=S.type(e);return(&quot;number&quot;===t||&quot;string&quot;===t)&amp;&amp;!isNaN(e-parseFloat(e))},S.trim=function(e){return null==e?&quot;&quot;:(e+&quot;&quot;).replace(Xt,&quot;&quot;)},&quot;function&quot;==typeof define&amp;&amp;define.amd&amp;&amp;define(&quot;jquery&quot;,[],function(){return S});var Vt=C.jQuery,Gt=C.$;return S.noConflict=function(e){return C.$===S&amp;&amp;(C.$=Gt),e&amp;&amp;C.jQuery===S&amp;&amp;(C.jQuery=Vt),S},&quot;undefined&quot;==typeof e&amp;&amp;(C.jQuery=C.$=S),S});


/* Demo page specific styles */
.demo-headline {
    background-color: #110D95;
    background-image: url(&quot; , &quot;'&quot; , &quot;https://phptravels.com/assets/img/head.webp&quot; , &quot;'&quot; , &quot;);
    background-position: bottom right;
    background-repeat: no-repeat;
    background-size: contain;
    color: white;
    padding: 5rem 0;
    margin-bottom: 3rem;
    position: relative;
    overflow: hidden;
}
.demo-headline-content {
    position: relative;
    z-index: 2;
    max-width: 60%;
}
.demo-headline-small {
    font-size: 0.9rem;
    font-weight: 500;
    letter-spacing: 2px;
    text-transform: uppercase;
    margin-bottom: 1rem;
    opacity: 0.9;
}
.demo-headline h1 {
    font-size: 3.5rem;
    font-weight: 700;
    letter-spacing: -2px;
    margin-bottom: 1.5rem;
    line-height: 1.1;
}
.demo-headline p {
    font-size: 1.3rem;
    opacity: 0.9;
    font-weight: 300;
    margin-top: 0;
    line-height: 1.5;
    margin-bottom: 2rem;
}
.demo-section {
    padding: 4rem 0;
    background: #f8f9fa;
}
.demo-form-container {
    background: white;
    border-radius: 16px;
    border: 1px solid #e8e8e8;
    padding: 2rem;
    height: 100%;
    /* box-shadow removed */
}
.demo-form-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 1.5rem;
}
.demo-form-divider {
    border-color: #e8e8e8;
    margin: 1.5rem 0;
}
.demo-form-group {
    margin-bottom: 1rem;
}
.demo-form-control {
    border: 1px solid #e8e8e8;
    border-radius: 8px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
}
.demo-form-control:focus {
    border-color: #110D95;
    box-shadow: none;
}
.demo-submit-btn {
    background: #007bff;
    border: none;
    border-radius: 8px;
    padding: 1rem 2rem;
    font-weight: 600;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
}
.demo-submit-btn:hover {
    background: #0056b3;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,123,255,0.3);
}
.demo-submit-btn:disabled {
    background: #6c757d;
    transform: none;
    box-shadow: none;
}
.demo-captcha-container {
    background: #f8f9fa;
    border-radius: 8px;
    padding: 1rem;
    text-align: center;
    margin-bottom: 1rem;
}
.demo-captcha-result {
    border: 1px solid #e8e8e8;
    border-radius: 8px;
    height: 58px;
    font-size: 1.1rem;
    text-align: center;
}
.demo-image-container {
    background: rgba(124, 145, 161, 0.11);
    border-radius: 16px;
    padding: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
}
.demo-image-container img {
    max-width: 100%;
    height: auto;
    object-fit: cover;
    border-radius: 8px;
}
.demo-success-container {
    text-align: center;
    padding: 2rem 0;
}
.demo-success-icon {
    margin-bottom: 1.5rem;
}
.demo-success-title {
    font-size: 2rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 1rem;
}
.demo-success-text {
    color: #666;
    line-height: 1.6;
    font-size: 1rem;
}
.demo-faq-section {
    padding: 4rem 0;
    background: white;
}
.demo-faq-title {
    font-size: 2rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 2rem;
}
.demo-faq-subtitle {
    font-size: 1rem;
    color: #666;
    margin-bottom: 3rem;
}
.demo-accordion-item {
    border: 1px solid #e8e8e8;
    border-radius: 12px !important;
    margin-bottom: 1rem;
    overflow: hidden;
}
.demo-accordion-button {
    background: white;
    border: none;
    padding: 1.25rem 1.5rem;
    font-weight: 600;
    color: #1a1a1a;
    font-size: 1rem;
}
.demo-accordion-button:not(.collapsed) {
    background: #f8f9fa;
    color: #110D95;
    box-shadow: none;
}
.demo-accordion-button:focus {
    box-shadow: 0 0 0 0.2rem rgba(0,123,255,0.25);
}
.demo-accordion-body {
    padding: 1.5rem;
    background: white;
    color: #666;
    line-height: 1.6;
}
.demo-loading-container {
    text-align: center;
    padding: 3rem 0;
}
.demo-alert {
    border-radius: 8px;
    padding: 0.75rem 1rem;
    margin-top: 1rem;
    border: none;
    background: #f8d7da;
    color: #721c24;
}
/* Responsive adjustments */
@media (max-width: 768px) {
    .demo-headline-content {
        max-width: 100%;
        text-align: center;
    }
    .demo-headline {
        padding: 2.5rem 0;
    }
    .demo-headline h1 {
        font-size: 2.2rem;
    }
    .demo-headline p {
        font-size: 1rem;
    }
    .demo-section {
        padding: 2rem 0;
    }
    .demo-form-container {
        padding: 1.5rem;
        margin-bottom: 2rem;
    }
}
@media (max-width: 576px) {
    .demo-headline h1 {
        font-size: 1.8rem;
    }
    .demo-form-container {
        padding: 1rem;
    }
    .demo-faq-section {
        padding: 2rem 0;
    }
}
/* Animation styles for success checkmark */
#check-group {
    animation: 0.32s ease-in-out 1.03s check-group;
    transform-origin: center;
}
#check-group #check {
    animation: 0.34s cubic-bezier(0.65, 0, 1, 1) 0.8s forwards check;
    stroke-dasharray: 0, 75px;
    stroke-linecap: round;
    stroke-linejoin: round;
}
#check-group #outline {
    animation: 0.38s ease-in outline;
    transform: rotate(0deg);
    transform-origin: center;
}
#check-group #white-circle {
    animation: 0.35s ease-in 0.35s forwards circle;
    transform: none;
    transform-origin: center;
}
@keyframes outline {
    from { stroke-dasharray: 0, 345.576px; }
    to { stroke-dasharray: 345.576px, 345.576px; }
}
@keyframes circle {
    from { transform: scale(1); }
    to { transform: scale(0); }
}
@keyframes check {
    from { stroke-dasharray: 0, 75px; }
    to { stroke-dasharray: 75px, 75px; }
}
@keyframes check-group {
    from { transform: scale(1); }
    50% { transform: scale(1.09); }
    to { transform: scale(1); }
}
/* Utility classes */
.no-spin-buttons {
    -webkit-appearance: none;
    -moz-appearance: textfield;
}
input[type=&quot;number&quot;]::-webkit-inner-spin-button,
input[type=&quot;number&quot;]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
}



    
        
            Demo
            Live Demonstration
            Test drive our complete travel booking platform with full features and functionality
        
    



    
        
            
                
                    Request Demo Access
                    

                    
                        
                            
                                
                                    
                                    First Name
                                
                            
                            
                                
                                    
                                    Last Name
                                
                            
                        

                        
                            
                                
                                    
                                            Country
    Afghanistan +93
    Albania +355
    Algeria +213
    American Samoa +1684
    Andorra +376
    Angola +244
    Anguilla +1264
    Antarctica +0
    Antigua and Barbuda +1268
    Argentina +54
    Armenia +374
    Aruba +297
    Australia +61
    Austria +43
    Azerbaijan +994
    Bahamas +1242
    Bahrain +973
    Bangladesh +880
    Barbados +1246
    Belarus +375
    Belgium +32
    Belize +501
    Benin +229
    Bermuda +1441
    Bhutan +975
    Bolivia +591
    Bosnia and Herzegovina +387
    Botswana +267
    Brazil +55
    British Indian Ocean Territory +246
    Brunei Darussalam +673
    Bulgaria +359
    Burkina Faso +226
    Burundi +257
    Cambodia +855
    Cameroon +237
    Canada +1
    Cape Verde +238
    Cayman Islands +1345
    Central African Republic +236
    Chad +235
    Chile +56
    China +86
    Christmas Island +61
    Cocos (Keeling) Islands +672
    Colombia +57
    Comoros +269
    Congo +243
    Congo, the Democratic Republic of the +242
    Cook Islands +682
    Costa Rica +506
    Cote D&quot; , &quot;'&quot; , &quot;Ivoire +225
    Croatia +385
    Cuba +53
    Cyprus +357
    Czech Republic +420
    Denmark +45
    Djibouti +253
    Dominica +1767
    Dominican Republic +1809
    Ecuador +593
    Egypt +20
    El Salvador +503
    Equatorial Guinea +240
    Eritrea +291
    Estonia +372
    Ethiopia +251
    Falkland Islands (Malvinas) +500
    Faroe Islands +298
    Fiji +679
    Finland +358
    France +33
    French Guiana +594
    French Polynesia +689
    French Southern Territories +0
    Gabon +241
    Gambia +220
    Georgia +995
    Germany +49
    Ghana +233
    Gibraltar +350
    Greece +30
    Greenland +299
    Grenada +1473
    Guadeloupe +590
    Guam +1671
    Guatemala +502
    Guinea +224
    Guinea-Bissau +245
    Guyana +592
    Haiti +509
    Holy See (Vatican City State) +39
    Honduras +504
    Hong Kong +852
    Hungary +36
    Iceland +354
    India +91
    Indonesia +62
    Iran, Islamic Republic of +98
    Iraq +964
    Ireland +353
    Israel +972
    Italy +39
    Jamaica +1876
    Japan +81
    Jordan +962
    Kazakhstan +7
    Kenya +254
    Kiribati +686
    Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of +850
    Korea, Republic of +82
    Kuwait +965
    Kyrgyzstan +996
    Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic +856
    Latvia +371
    Lebanon +961
    Lesotho +266
    Liberia +231
    Libyan Arab Jamahiriya +218
    Liechtenstein +423
    Lithuania +370
    Luxembourg +352
    Macao +853
    Macedonia, the Former Yugoslav Republic of +389
    Madagascar +261
    Malawi +265
    Malaysia +60
    Maldives +960
    Mali +223
    Malta +356
    Marshall Islands +692
    Martinique +596
    Mauritania +222
    Mauritius +230
    Mayotte +269
    Mexico +52
    Micronesia, Federated States of +691
    Moldova, Republic of +373
    Monaco +377
    Mongolia +976
    Montserrat +1664
    Morocco +212
    Mozambique +258
    Myanmar +95
    Namibia +264
    Nauru +674
    Nepal +977
    Netherlands +31
    Netherlands Antilles +599
    New Caledonia +687
    New Zealand +64
    Nicaragua +505
    Niger +227
    Nigeria +234
    Niue +683
    Norfolk Island +672
    Northern Mariana Islands +1670
    Norway +47
    Oman +968
    Pakistan +92
    Palau +680
    Palestinian Territory, Occupied +970
    Panama +507
    Papua New Guinea +675
    Paraguay +595
    Peru +51
    Philippines +63
    Pitcairn +0
    Poland +48
    Portugal +351
    Puerto Rico +1787
    Qatar +974
    Reunion +262
    Romania +40
    Russia +70
    Rwanda +250
    Saint Kitts and Nevis +1869
    Saint Lucia +1758
    Saint Pierre and Miquelon +508
    Saint Vincent and the Grenadines +1784
    Samoa +684
    San Marino +378
    Sao Tome and Principe +239
    Saudi Arabia +966
    Senegal +221
    Serbia and Montenegro +381
    Seychelles +248
    Sierra Leone +232
    Singapore +65
    Slovakia +421
    Slovenia +386
    Solomon Islands +677
    Somalia +252
    South Africa +27
    South Georgia and the South Sandwich Islands +0
    Spain +34
    Sri Lanka +94
    Sudan +249
    Swaziland +268
    Sweden +46
    Switzerland +41
    Syrian Arab Republic +963
    Taiwan, Province of China +886
    Tajikistan +992
    Tanzania, United Republic of +255
    Thailand +66
    Timor-Leste +670
    Togo +228
    Tokelau +690
    Tonga +676
    Trinidad and Tobago +1868
    Tunisia +216
    Turkey +90
    Turkmenistan +7370
    Turks and Caicos Islands +1649
    Tuvalu +688
    Uganda +256
    Ukraine +380
    United Arab Emirates +971
    United Kingdom +44
    United States +1
    Uruguay +598
    Uzbekistan +998
    Vanuatu +678
    Venezuela +58
    Viet Nam +84
    Virgin Islands, British +1284
    Virgin Islands, U.s. +1340
    Wallis and Futuna +681
    Western Sahara +212
    Yemen +967
    Zambia +260
    Zimbabwe +263
    Serbia +381
    Asia / Pacific Region +0
    Montenegro +382
    Aland Islands +358
    Curacao +599
    Guernsey +44
    Isle of Man +44
    Jersey +44
    Kosovo +381
    Saint Barthelemy +590
    Saint Martin +590
    Sint Maarten +1
    South Sudan +211



var requestUrl = &quot;https://ipwhois.app/json/&quot;;
fetch(requestUrl)
.then(function(response) { return response.json(); })
.then(function(c) {
var user_country = c[&quot; , &quot;'&quot; , &quot;country_phone&quot; , &quot;'&quot; , &quot;];
user_country = user_country.replace(&quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
$(&quot;[data-country-phonecode=&quot; , &quot;'&quot; , &quot;&quot; + user_country + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).prop(&quot;selected&quot;, true);
// $(&quot;[name=&quot; , &quot;'&quot; , &quot;country_id&quot; , &quot;'&quot; , &quot;]&quot;).val(user_country)
console.log(user_country);
});
                                    
                                    Select Country
                                
                            
                            
                                
                                    
                                    WhatsApp Number
                                
                            
                        

                        
                            
                            Business Name
                        

                        
                            
                            Email Address
                        

                        
                            
                                
                                    Submit
                                    
                                        
                                    
                                

                                
                                    
                                
                            
                            
                                
                                    
                                        
                                            
                                                4 + 10 = ?
                                            
                                        
                                    
                                    
                                
                            
                        

                        
                            The whatsapp number is not valid. avoid adding country number, Zero or + signs before the number
                        

                        
                            
                                
                                    
                                
                                
                                    
                                
                                
                                    
                                
                            
                        
                    
                    
                    
                        
                        
                            
                                
                                    
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                         Thank you!
                        We have sent your demo credentials to your email please check your mailbox. if demo credentials not found inbox please check spam folder
                    
                
            
            
                
                    
                
            
        
    



    
        Frequently Asked Questions
        Find answers to common questions about our demo platform and testing process.
        
                            
                    
                        
                            How can I test the PHPTRAVELS demo?                        
                    
                    
                        
                            Submit the demo request form to receive complete access credentials. You&quot; , &quot;'&quot; , &quot;ll be able to test everything from the admin panel to the client interface on our live demo platform.                        
                    
                
                            
                    
                        
                            Can I save content during testing?                        
                    
                    
                        
                            Yes, but please note that the database content resets every few minutes to maintain demo integrity. However, all content can be fully customized in your purchased version.                        
                    
                
                            
                    
                        
                            What features are included in the demo?                        
                    
                    
                        
                            Our demo includes all features needed to run your travel business. Test the complete application from backend administration to frontend booking experience at no cost.                        
                    
                
                            
                    
                        
                            Do you charge for demo testing?                        
                    
                    
                        
                            No, we offer completely free application testing. Access our demo anytime by submitting the request form - no hidden charges or commitments.                        
                    
                
                            
                    
                        
                            Can I test the application on my own hosting?                        
                    
                    
                        
                            Demo testing is available exclusively on our servers for security reasons. Once satisfied with the demo, you can purchase and install on your hosting with our complete support.                        
                    
                
                            
                    
                        
                            Is the application mobile responsive?                        
                    
                    
                        
                            Yes, all themes and interfaces are fully responsive and optimized for desktop, laptop, tablet, and mobile devices. We recommend testing on multiple devices to experience the complete responsiveness.                        
                    
                
                    
    



// Generate random captcha numbers
numb1 = Math.floor((Math.random() * 10) + 1);
numb2 = Math.floor((Math.random() * 10) + 1);
document.getElementById(&quot;numb1&quot;).innerHTML = numb1;
document.getElementById(&quot;numb2&quot;).innerHTML = numb2;

// Form submission handler
$(&quot;#demo&quot;).click(function() {
    var country_id = $(&quot; , &quot;'&quot; , &quot;.country_id&quot; , &quot;'&quot; , &quot;).val();
    let numbers = numb1 + numb2;
    let number = $(&quot; , &quot;'&quot; , &quot;#number&quot; , &quot;'&quot; , &quot;).val();

    // Validation
    if ($(&quot;.first_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your first name&quot;);
        return;
    }
    if ($(&quot;.last_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your last name&quot;);
        return;
    }
    if ($(&quot;.company_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your business name&quot;);
        return;
    }
    if ($(&quot;.whatsapp_number&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your WhatsApp number&quot;);
        return;
    }
    if (country_id === &quot;&quot;) {
        alert(&quot;Please select your country&quot;);
        return;
    }
    if ($(&quot;.email&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your email address&quot;);
        return;
    }
    if (number == &quot;&quot;) {
        alert(&quot; , &quot;'&quot; , &quot;Please solve the math problem&quot; , &quot;'&quot; , &quot;);
        return;
    }
    
    if (numbers == number) {
        let first_name = $(&quot;.first_name&quot;).val();
        let last_name = $(&quot;.last_name&quot;).val();
        let company_name = $(&quot;.company_name&quot;).val();
        let whatsapp_number = $(&quot;.whatsapp_number&quot;).val();
        let email = $(&quot;.email&quot;).val();

        $(&quot;.btn_submit&quot;).hide();
        $(&quot;.alert_msg&quot;).hide();
        $(&quot;.btn_loading&quot;).show();

        var form = new FormData();
        form.append(&quot;first_name&quot;, first_name);
        form.append(&quot;last_name&quot;, last_name);
        form.append(&quot;email&quot;, email);
        form.append(&quot;company_name&quot;, company_name);
        form.append(&quot;whatsapp_number&quot;, whatsapp_number);
        form.append(&quot;country_id&quot;, country_id);
        form.append(&quot;lead_type&quot;, &quot;demo&quot;);

        var settings = {
            &quot;url&quot;: &quot;https://app.phptravels.com/api_demo_signup.php&quot;,
            &quot;method&quot;: &quot;POST&quot;,
            &quot;timeout&quot;: 0,
            &quot;headers&quot;: {},
            &quot;processData&quot;: false,
            &quot;mimeType&quot;: &quot;multipart/form-data&quot;,
            &quot;contentType&quot;: false,
            &quot;data&quot;: form
        };

        $.ajax(settings).done(function(response) {
            $(&quot;.btn_submit&quot;).show();
            $(&quot;.btn_loading&quot;).hide();

            try {
                var result;
                if (typeof response === &quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;) {
                    result = JSON.parse(response);
                } else {
                    result = response;
                }
                // Show thank you block if success
                if (result.status === true || result.status === &quot;true&quot; || result.status == true) {
                    $(&quot;.from_box&quot;).hide();
                    $(&quot;.completed&quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
                 } else {
                    $(&quot;.alert_msg&quot;).show();
                    $(&quot;.alert_msg p&quot;).text(result.message || &quot;An error occurred&quot;);
                }
            } catch (e) {
                $(&quot;.alert_msg&quot;).show();
                $(&quot;.alert_msg p&quot;).text(&quot;An error occurred. Please try again.&quot;);
            }
        }).fail(function(xhr, status, error) {
            $(&quot;.btn_submit&quot;).show();
            $(&quot;.btn_loading&quot;).hide();
            $(&quot;.alert_msg&quot;).show();
            $(&quot;.alert_msg p&quot;).text(&quot;Connection error. Please try again.&quot;);
        });
    } else {
        alert(&quot;The math result is incorrect. Please try again.&quot;);
    }
});





  
  
  




Aggregated Solution
With our multiple channel aggregation feature now we can get inventory from different API&quot; , &quot;'&quot; , &quot;s with realtime pricing and booking.




  GDS &amp; OTA Integration
  Realtime API&quot; , &quot;'&quot; , &quot;s and Dashboards
  100% Opensource Platform Structure
  Highly Scalable and Extensive
  Secure by Additinal Layers
  Latest Technology Implemented
  Self-Hosted Structure
  Developer Friendly Documentation
  Live Testing Demonstration








    We are Growing!

        
            
                
                
                    
                

                
                
                    
                        
                            
                                
                                    4K+
                                
                                Live Websites
                            
                        
                        
                        
                                
                                    180K+
                                
                                Users
                            
                        
                        
                        
                                
                                    6M+
                                
                                Bookings
                            
                        

                    
                

                
                const counters = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;.counter&quot; , &quot;'&quot; , &quot;);
                // Main function
                for(let n of counters) {
                  const updateCount = () => {
                    const target = + n.getAttribute(&quot; , &quot;'&quot; , &quot;data-target&quot; , &quot;'&quot; , &quot;);
                    const count = + n.innerText;
                    const speed = 500; // change animation speed here
                    const inc = target / speed;
                    if(count &lt; target) {
                      n.innerText = Math.ceil(count + inc);
                      setTimeout(updateCount, 1);
                    } else {
                      n.innerText = target;
                    }
                  }
                  updateCount();
                }
                

                
                
                    
                        
                        
                    

                    

                    
                        
                            
                        
                        
                            
                        
                    
                
            
    

    



      

        
        
          
            
              
                
                
              
            
            
              
                
                  
                    24FLIGHTS
                  
                
                
                  Nancy - 24fights.com
                  
                    PHPTRAVELS Transformed our Business with Exceptional
                    Solution &amp; Service.
                  
                  
                    View Website
                    
                      
                        
                      
                    
                  
                
              
            
          
        

        
        
          
            
              
                
                
              
            
            
              
                
                  TAZKIRA
                
                
                  M. Saeed - Tazkira.ae
                  
                      PHPTRAVELS is the real Secret Behind Tazkira&quot; , &quot;'&quot; , &quot;s 80% of Online Sales &amp; Bookings
                  

                  View Website
                      
                          
                      
                  
                
              
            
          
        

        
        
          
            
              
                
                
              
            
            
              
                
                    BOOKING JORDAN
                
                
                    Khalid Nawalah - Bookingjordan.com
                  
                      Our Achievements are Powered by PHPTRAVELS &amp; Their Super Supportive Team.
                  

                  
                    View Website
                    
                        
                    
                
                
              
            
          
        

        
        
          
          
            
              
            
          
          
            
              
            
          
        
      
    




// ALL SLIDERS
const sliderElements = document.querySelectorAll(&quot;.slider--container&quot;);
const totalSliderElements = sliderElements.length;

// THIS INDICATE THE THE CURRENTLY SHOW SLIDER VALUE
let currentSlider = 1;

// SLIDER CONTROLS
const sliderControls = document.querySelectorAll(&quot;.slider--control&quot;);

sliderControls.forEach((sliderControl) => {
  sliderControl.addEventListener(&quot;click&quot;, function () {
    // THIS WILL GTE THE VALUE(DATA_VALUE) ACCORDING TO WHICH SLIDER MOIVE ForWARD OR REVERSE
    const _sliderValue = Number(this.getAttribute(&quot;data-value&quot;));

    // SET THE THE VALUE(DATA_HIDDEN) TO TRUE OF THE CUrRENTLY ACTIVE SLIDER
    sliderElements[currentSlider - 1].setAttribute(&quot;data-hidden&quot;, &quot;true&quot;);

    // INCREASE OR DECREASE THE SLIDER VALUE ACCORDINGLY
    currentSlider = currentSlider + _sliderValue;

    // CHECK IF THE CURRENT VALUE IS GREATE THEN THE TOTAL NUMBER OF THE AVAILABLE SLIDER
    // OR EQUAL TO ZERO
    if (currentSlider - 1 === totalSliderElements) {
      currentSlider = 1;
    } else if (currentSlider === 0) {
      currentSlider = totalSliderElements;
    }

    // REQUESTED SLIDER
    sliderElements[currentSlider - 1].setAttribute(&quot;data-hidden&quot;, &quot;false&quot;);
    sliderElements[currentSlider - 1].classList.add(&quot;custom--animation&quot;);
  });
});

// WHEN CLICK ON FIRST CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client1() {
  document.getElementById(&quot;client1&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube1&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}

// WHEN CLICK ON SECOND CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client2() {
  document.getElementById(&quot;client2&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube2&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}

// WHEN CLICK ON THIRD CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client3() {
  document.getElementById(&quot;client3&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube3&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}






// const myCarouselElement = document.querySelector(&quot;#autoSlider&quot;);

// myCarouselElement.addEventListener(&quot;slide.bs.carousel&quot;, (event) => {
//   document.querySelector(&quot;.carousel-item.active&quot;).classList.remove(&quot;show&quot;);
// });

// myCarouselElement.addEventListener(&quot;slid.bs.carousel&quot;, (event) => {
//   document.querySelector(&quot;.carousel-item.active&quot;).classList.add(&quot;show&quot;);
// });

// FIRST SLDIER CARD DATA
const firstSlider = [

  {
    url: &quot;https://tazkira.ae/&quot;,
    imgSrc: &quot;&quot;
  },


];

// THIS TEMPLATE CONATINS THE HTML FOR EACH SLDIE CARD
// const sliderCard = document.querySelector(&quot;#sliderCard&quot;);
// GETTING THE SLIDER ONE WRAPPER
// const carouselItem1 = document.querySelector(&quot;.carousel--item-1 > div.d-flex&quot;);
// CALLING THE CREATE SLDIER CARD FUNCTION
// createSliderCard(firstSlider, carouselItem1);

// SECOND SLIDER CARD DATA
// const sectSlider = [
//   {

//   }

// ];

// GETTING THE SECOND SLIDER WRAPPER
// const carouselItem2 = document.querySelector(&quot;.carousel--item-2 > div.d-flex&quot;);
// CALLING THE CREATE SLDIER CARD FUNCTION
// createSliderCard(sectSlider, carouselItem2);

// THIS FUNCTION WILL CRETAE THE CARD FOR EACH SLIDER AND APPEND IN THE SPECIFIC SLIDER WRAPPER
// function createSliderCard(cardData, cardPID) {
//   let _slideItem;
//   cardData.forEach((_slide) => {
//     _slideItem = sliderCard.content.cloneNode(true);

//     _slideItem.querySelector(&quot;a&quot;).setAttribute(&quot;href&quot;, _slide.url);
//     _slideItem.querySelector(&quot;a > img&quot;).setAttribute(&quot;src&quot;, _slide.imgSrc);
//     _slideItem.querySelector(&quot;a > img&quot;).setAttribute(&quot;href&quot;, _slide.alt);

//     cardPID.appendChild(_slideItem);
//   });
// }




/* Modern Newsletter Section */
.modern-newsletter {
    background: linear-gradient(135deg, #1a1a1a 0%, #2d2d2d 100%);
    padding: 4rem 0;
    position: relative;
    overflow: hidden;
}

.modern-newsletter::before {
    content: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    position: absolute;
    top: 0;
    right: 0;
    width: 100%;
    height: 100%;
    background: url(&quot; , &quot;'&quot; , &quot;data:image/svg+xml,&lt;svg xmlns=&quot;http://www.w3.org/2000/svg&quot; viewBox=&quot;0 0 100 100&quot;>&lt;defs>&lt;pattern id=&quot;grid&quot; width=&quot;10&quot; height=&quot;10&quot; patternUnits=&quot;userSpaceOnUse&quot;>&lt;path d=&quot;M 10 0 L 0 0 0 10&quot; fill=&quot;none&quot; stroke=&quot;rgba(255,255,255,0.05)&quot; stroke-width=&quot;1&quot;/>&lt;/pattern>&lt;/defs>&lt;rect width=&quot;100&quot; height=&quot;100&quot; fill=&quot;url(%23grid)&quot;/>&lt;/svg>&quot; , &quot;'&quot; , &quot;) repeat;
    pointer-events: none;
}

.newsletter-content {
    position: relative;
    z-index: 2;
}

.newsletter-icon {
    width: 60px;
    height: 60px;
    margin-bottom: 1rem;
}

.newsletter-title {
    font-size: 2rem;
    font-weight: 700;
    color: white;
    margin-bottom: 0.5rem;
}

.newsletter-subtitle {
    color: rgba(255,255,255,0.8);
    font-size: 1.1rem;
    margin-bottom: 2rem;
}

.newsletter-form {
    display: flex;
    gap: 1rem;
    max-width: 500px;
    margin: 0 auto;
}

.newsletter-input {
    flex: 1;
    padding: 1rem 1.5rem;
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 8px;
    background: rgba(255,255,255,0.1);
    color: white;
    font-size: 1rem;
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
}

.newsletter-input::placeholder {
    color: rgba(255,255,255,0.6);
}

.newsletter-input:focus {
    outline: none;
    border-color: #007bff;
    background: rgba(255,255,255,0.15);
}

.newsletter-btn {
    padding: 1rem 2rem;
    background: #007bff;
    border: none;
    border-radius: 8px;
    color: white;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;
}

.newsletter-btn:hover {
    background: #0056b3;
    transform: translateY(-2px);
}

/* Modern Footer */
.modern-footer {
    background: #0a0a0a;
    color: white;
    padding: 4rem 0 0;
    position: relative;
}

.footer-main {
    padding-bottom: 3rem;
    border-bottom: 1px solid rgba(255,255,255,0.1);
}

.footer-brand {
    margin-bottom: 2rem;
}

.brand-logo-container {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
}

.brand-logo {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    object-fit: cover;
}

.brand-text h3 {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
    color: white;
}

.brand-text p {
    font-size: 0.9rem;
    color: rgba(255,255,255,0.7);
    margin: 0;
}

.brand-description {
    color: rgba(255,255,255,0.7);
    line-height: 1.6;
    margin-bottom: 2rem;
    max-width: 350px;
}

.social-links {
    display: flex;
    gap: 1rem;
}

.social-link {
    width: 44px;
    height: 44px;
    background: rgba(255,255,255,0.1);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    text-decoration: none;
}

.social-link:hover {
    background: #007bff;
    transform: translateY(-2px);
}

.social-link svg {
    width: 20px;
    height: 20px;
    fill: white;
}

.footer-section {
    margin-bottom: 2rem;
}

.footer-section h4 {
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: white;
    position: relative;
}

.footer-section h4::after {
    content: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    position: absolute;
    bottom: -8px;
    left: 0;
    width: 30px;
    height: 2px;
    background: #007bff;
}

.footer-links {
    list-style: none;
    padding: 0;
    margin: 0;
}

.footer-links li {
    margin-bottom: 0.75rem;
}

.footer-links a {
    color: rgba(255,255,255,0.7);
    text-decoration: none;
    transition: color 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
}

.footer-links a:hover {
    color: #007bff;
}

.hiring-badge {
    background: linear-gradient(45deg, #ff4757, #ff6b7a);
    color: white;
    font-size: 0.7rem;
    padding: 0.2rem 0.5rem;
    border-radius: 12px;
    font-weight: 600;
    text-transform: uppercase;
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
}

.payment-section {
    background: rgba(255,255,255,0.03);
    padding: 2rem;
    border-radius: 12px;
    margin-bottom: 2rem;
}

.payment-title {
    font-weight: 600;
    margin-bottom: 1rem;
    color: white;
}

.payment-description {
    color: rgba(255,255,255,0.7);
    line-height: 1.6;
    margin-bottom: 1.5rem;
    font-size: 0.9rem;
}

.payment-gateway-image {
    max-width: 100%;
    height: auto;
    opacity: 0.8;
    transition: opacity 0.3s ease;
}

.payment-gateway-image:hover {
    opacity: 1;
}

.certifications {
    display: flex;
    gap: 2rem;
    align-items: center;
}

.cert-item img {
    /* max-width: 100px; */
    height: auto;
    opacity: 0.7;
    transition: opacity 0.3s ease;
}

.cert-item img:hover {
    opacity: 1;
}

.footer-bottom {
    padding: 2rem 0;
    background: rgba(0,0,0,0.5);
}

.footer-bottom-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
}

.copyright {
    color: rgba(255,255,255,0.6);
    font-size: 0.9rem;
}

.legal-links {
    display: flex;
    gap: 2rem;
    flex-wrap: wrap;
}

.legal-links a {
    color: rgba(255,255,255,0.6);
    text-decoration: none;
    font-size: 0.9rem;
    transition: color 0.3s ease;
}

.legal-links a:hover {
    color: #007bff;
}

.whatsapp-float {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    z-index: 1000;
    animation: float 3s ease-in-out infinite;
}

.whatsapp-btn {
    width: 60px;
    height: 60px;
    background: linear-gradient(135deg, #25d366, #128c7e);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
    box-shadow: 0 4px 20px rgba(37, 211, 102, 0.4);
    transition: all 0.3s ease;
}

.whatsapp-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 6px 25px rgba(37, 211, 102, 0.6);
}

.whatsapp-btn svg {
    width: 30px;
    height: 30px;
    fill: white;
}

@keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
}

/* Responsive Design */
@media (max-width: 768px) {
    .modern-newsletter {
        padding: 3rem 0;
        text-align: center;
    }
    
    .newsletter-title {
        font-size: 1.5rem;
    }
    
    .newsletter-form {
        flex-direction: column;
    }
    
    .modern-footer {
        padding: 3rem 0 0;
    }
    
    .footer-bottom-content {
        flex-direction: column;
        text-align: center;
    }
    
    .legal-links {
        justify-content: center;
    }
    
    .certifications {
        flex-direction: column;
        gap: 1rem;
    }
    
    .social-links {
        justify-content: center;
    }
}

@media (max-width: 576px) {
    .brand-logo-container {
        flex-direction: column;
        text-align: center;
    }
    
    .legal-links {
        flex-direction: column;
        gap: 1rem;
    }
    
    .whatsapp-float {
        bottom: 1rem;
        right: 1rem;
    }
    
    .whatsapp-btn {
        width: 50px;
        height: 50px;
    }
    
    .whatsapp-btn svg {
        width: 25px;
        height: 25px;
    }
}




    
        
            
                
            
            Stay Updated
            Get the latest updates on travel technology and industry insights
            
                
                Subscribe
            
        
    




    
        
            
                
                
                    
                        
                            
                            
                                PHPTRAVELS
                                Travel Tech Partner
                            
                        
                        Leading travel technology solutions for modern businesses. Build, customize, and scale your travel platform with our comprehensive booking engine.
                        
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                    
                    
                

                
                                    
                        
                            Product
                            
                                                                    
                                        
                                            Demo                                                                                    
                                    
                                                                    
                                        
                                            Mobile Apps                                                                                    
                                    
                                                                    
                                        
                                            Pricing                                                                                    
                                    
                                                                    
                                        
                                            Features                                                                                    
                                    
                                                                    
                                        
                                            Technology                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Resources
                            
                                                                    
                                        
                                            Changelog                                                                                    
                                    
                                                                    
                                        
                                            Updates                                                                                    
                                    
                                                                    
                                        
                                            Requirements                                                                                    
                                    
                                                                    
                                        
                                            Affiliate                                                                                    
                                    
                                                                    
                                        
                                            Road Map                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Services
                            
                                                                    
                                        
                                            Product Services                                                                                    
                                    
                                                                    
                                        
                                            Customizations                                                                                    
                                    
                                                                    
                                        
                                            EAN Integration                                                                                    
                                    
                                                                    
                                        
                                            Cloud Hosting                                                                                    
                                    
                                                                    
                                        
                                            Support                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Company
                            
                                                                    
                                        
                                            About Us                                                                                    
                                    
                                                                    
                                        
                                            Blog                                                                                    
                                    
                                                                    
                                        
                                            The Team                                                                                    
                                    
                                                                    
                                        
                                            Contact Us                                                                                    
                                    
                                                                    
                                        
                                            Jobs                                                                                            We&quot; , &quot;'&quot; , &quot;re Hiring!
                                                                                    
                                    
                                                            
                        
                    
                            

            
            
                
                    
                        Accepted Payment Methods
                        
                            We support all major payment gateways including PayPal, Credit Cards, Western Union, Skrill, and Transferwise for your convenience.
                        
                        
                    
                
                
                    
                        
                            
                        
                        
                            
                        
                    
                
            
        
    

    
    
        
            
                
                    © 2023 PHPTRAVELS. All Rights Reserved.
                
                
                                            
                            Privacy Policy                        
                                            
                            Terms of Service                        
                                            
                            Abuse Policy                        
                                            
                            Live Chat                        
                                            
                            Content Program                        
                                    
            
        
    

 









   
   
   

   

   
   setTimeout(function() {
      window.__lc = window.__lc || {};
      window.__lc.license = 4618001;
      ;(function(n,t,c){function i(n){return e._h?e._h.apply(null,n):e._q.push(n)}var e={_q:[],_h:null,_v:&quot;2.0&quot;,on:function(){i([&quot;on&quot;,c.call(arguments)])},once:function(){i([&quot;once&quot;,c.call(arguments)])},off:function(){i([&quot;off&quot;,c.call(arguments)])},get:function(){if(!e._h)throw new Error(&quot;[LiveChatWidget] You can&quot; , &quot;'&quot; , &quot;t use getters before load.&quot;);return i([&quot;get&quot;,c.call(arguments)])},call:function(){i([&quot;call&quot;,c.call(arguments)])},init:function(){var n=t.createElement(&quot;script&quot;);n.async=!0,n.type=&quot;text/javascript&quot;,n.src=&quot;https://cdn.livechatinc.com/tracking.js&quot;,t.head.appendChild(n)}};!n.__lc.asyncInit&amp;&amp;e.init(),n.LiveChatWidget=n.LiveChatWidget||e}(window,document,[].slice))
   }, 5000);
   





// import Swup from &quot; , &quot;'&quot; , &quot;https://unpkg.com/swup@4?module&quot; , &quot;'&quot; , &quot;;
// const swup = new Swup();







   // document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, function () {
   //    let exitIntentShown = false;

   //    // Check if the user has opted out of seeing the modal for the next 90 days
   //    const isOptedOut = localStorage.getItem(&quot; , &quot;'&quot; , &quot;exitIntentOptOut&quot; , &quot;'&quot; , &quot;);
   //    const optOutExpiration = localStorage.getItem(&quot; , &quot;'&quot; , &quot;optOutExpiration&quot; , &quot;'&quot; , &quot;);

   //    if (isOptedOut &amp;&amp; optOutExpiration) {
   //       const expirationDate = new Date(optOutExpiration);
   //       const currentDate = new Date();
   //       if (currentDate &lt; expirationDate) {
   //          // User has opted out and the 90-day period has not expired
   //          exitIntentShown = true;
   //       }
   //    }

   //    document.addEventListener(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function (e) {
   //       if (e.clientY &lt; 0 &amp;&amp; !exitIntentShown) {
   //          const exitModal = new bootstrap.Modal(document.getElementById(&quot; , &quot;'&quot; , &quot;exitModal&quot; , &quot;'&quot; , &quot;));
   //          exitModal.show();
   //          exitIntentShown = true;

   //          // Reset the exit intent flag when modal is closed
   //          document.getElementById(&quot; , &quot;'&quot; , &quot;exitModal&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;hidden.bs.modal&quot; , &quot;'&quot; , &quot;, function () {
   //             exitIntentShown = false;
   //          });
   //       }
   //    });

   //    document.getElementById(&quot; , &quot;'&quot; , &quot;dontShowAgainBtn&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
   //       // Set a flag in localStorage to indicate that the user opted out
   //       const expirationDate = new Date();
   //       expirationDate.setDate(expirationDate.getDate() + 90); // Set expiration date for 90 days
   //       localStorage.setItem(&quot; , &quot;'&quot; , &quot;exitIntentOptOut&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;);
   //       localStorage.setItem(&quot; , &quot;'&quot; , &quot;optOutExpiration&quot; , &quot;'&quot; , &quot;, expirationDate.toISOString()); // Store the expiration date
   //       // Hide the modal
   //       const exitModal = new bootstrap.Modal(document.getElementById(&quot; , &quot;'&quot; , &quot;exitModal&quot; , &quot;'&quot; , &quot;));
   //       exitModal.hide();
   //    });
   // });





// Get the current URL parameters
function getQueryParam(param) {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(param);
}

// Update all links with the referral ID
function updateLinksWithRef(refID) {
    if (!refID) return; // If no valid refID, don&quot; , &quot;'&quot; , &quot;t update links

    const links = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;);

    links.forEach(link => {
        const url = new URL(link.href);

        // Append the ref parameter to the URL only if it doesn&quot; , &quot;'&quot; , &quot;t already exist
        if (!url.searchParams.has(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;)) {
            url.searchParams.append(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;, refID);
        }

        link.href = url.toString();
    });
}

// Check if the URL has a ref parameter
const refID = getQueryParam(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;);
if (refID &amp;&amp; refID !== &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;) {
    // Store the refID in sessionStorage so it persists across page loads
    sessionStorage.setItem(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;, refID);
}

// Ensure that links include the ref parameter only if it&quot; , &quot;'&quot; , &quot;s present in the session or the URL
const storedRef = sessionStorage.getItem(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;);
if (storedRef &amp;&amp; storedRef !== &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;) {
    updateLinksWithRef(storedRef);
}






// SHOW - HIDE MENU ON MOBILE
function togglemenu() { var element = document.getElementById(&quot;responsive-menu&quot;); element.classList.toggle(&quot;show-menu&quot;);  }
function anchor() { var element = document.getElementById(&quot;responsive-menu&quot;); element.classList.remove(&quot;show-menu&quot;); }

/*! Waves v0.7.6  */
(function(window,factory){&quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;if(typeof define===&quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;&amp;&amp;define.amd){define([],function(){window.Waves=factory.call(window);return window.Waves})}else if(typeof exports===&quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;){module.exports=factory.call(window)}else{window.Waves=factory.call(window)}})(typeof global===&quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;?global:this,function(){&quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;var Waves=Waves||{};var $$=document.querySelectorAll.bind(document);var toString=Object.prototype.toString;var isTouchAvailable=&quot; , &quot;'&quot; , &quot;ontouchstart&quot; , &quot;'&quot; , &quot; in window;function isWindow(obj){return obj!==null&amp;&amp;obj===obj.window}
function getWindow(elem){return isWindow(elem)?elem:elem.nodeType===9&amp;&amp;elem.defaultView}
function isObject(value){var type=typeof value;return type===&quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;||type===&quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;&amp;&amp;!!value}
function isDOMNode(obj){return isObject(obj)&amp;&amp;obj.nodeType>0}
function getWavesElements(nodes){var stringRepr=toString.call(nodes);if(stringRepr===&quot; , &quot;'&quot; , &quot;[object String]&quot; , &quot;'&quot; , &quot;){return $$(nodes)}else if(isObject(nodes)&amp;&amp;/^\[object (Array|HTMLCollection|NodeList|Object)\]$/.test(stringRepr)&amp;&amp;nodes.hasOwnProperty(&quot; , &quot;'&quot; , &quot;length&quot; , &quot;'&quot; , &quot;)){return nodes}else if(isDOMNode(nodes)){return[nodes]}
return[]}
function offset(elem){var docElem,win,box={top:0,left:0},doc=elem&amp;&amp;elem.ownerDocument;docElem=doc.documentElement;if(typeof elem.getBoundingClientRect!==typeof undefined){box=elem.getBoundingClientRect()}
win=getWindow(doc);return{top:box.top+win.pageYOffset-docElem.clientTop,left:box.left+win.pageXOffset-docElem.clientLeft}}
function convertStyle(styleObj){var style=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;for(var prop in styleObj){if(styleObj.hasOwnProperty(prop)){style+=(prop+&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;+styleObj[prop]+&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;)}}
return style}
var Effect={duration:750,delay:200,show:function(e,element,velocity){if(e.button===2){return!1}
element=element||this;var ripple=document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);ripple.className=&quot; , &quot;'&quot; , &quot;waves-ripple waves-rippling&quot; , &quot;'&quot; , &quot;;element.appendChild(ripple);var pos=offset(element);var relativeY=0;var relativeX=0;if(&quot; , &quot;'&quot; , &quot;touches&quot; , &quot;'&quot; , &quot; in e&amp;&amp;e.touches.length){relativeY=(e.touches[0].pageY-pos.top);relativeX=(e.touches[0].pageX-pos.left)}else{relativeY=(e.pageY-pos.top);relativeX=(e.pageX-pos.left)}
relativeX=relativeX>=0?relativeX:0;relativeY=relativeY>=0?relativeY:0;var scale=&quot; , &quot;'&quot; , &quot;scale(&quot; , &quot;'&quot; , &quot;+((element.clientWidth/100)*3)+&quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot;;var translate=&quot; , &quot;'&quot; , &quot;translate(0,0)&quot; , &quot;'&quot; , &quot;;if(velocity){translate=&quot; , &quot;'&quot; , &quot;translate(&quot; , &quot;'&quot; , &quot;+(velocity.x)+&quot; , &quot;'&quot; , &quot;px, &quot; , &quot;'&quot; , &quot;+(velocity.y)+&quot; , &quot;'&quot; , &quot;px)&quot; , &quot;'&quot; , &quot;}
ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-hold&quot; , &quot;'&quot; , &quot;,Date.now());ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-x&quot; , &quot;'&quot; , &quot;,relativeX);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-y&quot; , &quot;'&quot; , &quot;,relativeY);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-scale&quot; , &quot;'&quot; , &quot;,scale);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-translate&quot; , &quot;'&quot; , &quot;,translate);var rippleStyle={top:relativeY+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;,left:relativeX+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;};ripple.classList.add(&quot; , &quot;'&quot; , &quot;waves-notransition&quot; , &quot;'&quot; , &quot;);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,convertStyle(rippleStyle));ripple.classList.remove(&quot; , &quot;'&quot; , &quot;waves-notransition&quot; , &quot;'&quot; , &quot;);rippleStyle[&quot; , &quot;'&quot; , &quot;-webkit-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle[&quot; , &quot;'&quot; , &quot;-moz-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle[&quot; , &quot;'&quot; , &quot;-ms-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle[&quot; , &quot;'&quot; , &quot;-o-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle.transform=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle.opacity=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;var duration=e.type===&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;?2500:Effect.duration;rippleStyle[&quot; , &quot;'&quot; , &quot;-webkit-transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;rippleStyle[&quot; , &quot;'&quot; , &quot;-moz-transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;rippleStyle[&quot; , &quot;'&quot; , &quot;-o-transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;rippleStyle[&quot; , &quot;'&quot; , &quot;transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;ripple.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,convertStyle(rippleStyle))},hide:function(e,element){element=element||this;var ripples=element.getElementsByClassName(&quot; , &quot;'&quot; , &quot;waves-rippling&quot; , &quot;'&quot; , &quot;);for(var i=0,len=ripples.length;i&lt;len;i++){removeRipple(e,element,ripples[i])}
if(isTouchAvailable){element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,Effect.hide);element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,Effect.hide)}
element.removeEventListener(&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,Effect.hide);element.removeEventListener(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;,Effect.hide)}};var TagWrapper={input:function(element){var parent=element.parentNode;if(parent.tagName.toLowerCase()===&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;&amp;&amp;parent.classList.contains(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)){return}
var wrapper=document.createElement(&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;);wrapper.className=element.className+&quot; , &quot;'&quot; , &quot; waves-input-wrapper&quot; , &quot;'&quot; , &quot;;element.className=&quot; , &quot;'&quot; , &quot;waves-button-input&quot; , &quot;'&quot; , &quot;;parent.replaceChild(wrapper,element);wrapper.appendChild(element);var elementStyle=window.getComputedStyle(element,null);var color=elementStyle.color;var backgroundColor=elementStyle.backgroundColor;wrapper.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;color:&quot; , &quot;'&quot; , &quot;+color+&quot; , &quot;'&quot; , &quot;;background:&quot; , &quot;'&quot; , &quot;+backgroundColor);element.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;background-color:rgba(0,0,0,0);&quot; , &quot;'&quot; , &quot;)},img:function(element){var parent=element.parentNode;if(parent.tagName.toLowerCase()===&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;&amp;&amp;parent.classList.contains(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)){return}
var wrapper=document.createElement(&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;);parent.replaceChild(wrapper,element);wrapper.appendChild(element)}};function removeRipple(e,el,ripple){if(!ripple){return}
ripple.classList.remove(&quot; , &quot;'&quot; , &quot;waves-rippling&quot; , &quot;'&quot; , &quot;);var relativeX=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-x&quot; , &quot;'&quot; , &quot;);var relativeY=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-y&quot; , &quot;'&quot; , &quot;);var scale=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-scale&quot; , &quot;'&quot; , &quot;);var translate=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-translate&quot; , &quot;'&quot; , &quot;);var diff=Date.now()-Number(ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-hold&quot; , &quot;'&quot; , &quot;));var delay=350-diff;if(delay&lt;0){delay=0}
if(e.type===&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;){delay=150}
var duration=e.type===&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;?2500:Effect.duration;setTimeout(function(){var style={top:relativeY+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;,left:relativeX+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;,opacity:&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-webkit-transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-moz-transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-o-transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-webkit-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;-moz-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;-ms-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;-o-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate};ripple.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,convertStyle(style));setTimeout(function(){try{el.removeChild(ripple)}catch(e){return!1}},duration)},delay)}
var TouchHandler={touches:0,allowEvent:function(e){var allow=!0;if(/^(mousedown|mousemove)$/.test(e.type)&amp;&amp;TouchHandler.touches){allow=!1}
return allow},registerEvent:function(e){var eType=e.type;if(eType===&quot; , &quot;'&quot; , &quot;touchstart&quot; , &quot;'&quot; , &quot;){TouchHandler.touches+=1}else if(/^(touchend|touchcancel)$/.test(eType)){setTimeout(function(){if(TouchHandler.touches){TouchHandler.touches-=1}},500)}}};function getWavesEffectElement(e){if(TouchHandler.allowEvent(e)===!1){return null}
var element=null;var target=e.target||e.srcElement;while(target.parentElement){if((!(target instanceof SVGElement))&amp;&amp;target.classList.contains(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)){element=target;break}
target=target.parentElement}
return element}
function showEffect(e){var element=getWavesEffectElement(e);if(element!==null){if(element.disabled||element.getAttribute(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;)||element.classList.contains(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;)){return}
TouchHandler.registerEvent(e);if(e.type===&quot; , &quot;'&quot; , &quot;touchstart&quot; , &quot;'&quot; , &quot;&amp;&amp;Effect.delay){var hidden=!1;var timer=setTimeout(function(){timer=null;Effect.show(e,element)},Effect.delay);var hideEffect=function(hideEvent){if(timer){clearTimeout(timer);timer=null;Effect.show(e,element)}
if(!hidden){hidden=!0;Effect.hide(hideEvent,element)}
removeListeners()};var touchMove=function(moveEvent){if(timer){clearTimeout(timer);timer=null}
hideEffect(moveEvent);removeListeners()};element.addEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;,touchMove,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,hideEffect,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,hideEffect,!1);var removeListeners=function(){element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;,touchMove);element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,hideEffect);element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,hideEffect)}}else{Effect.show(e,element);if(isTouchAvailable){element.addEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,Effect.hide,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,Effect.hide,!1)}
element.addEventListener(&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,Effect.hide,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;,Effect.hide,!1)}}}
Waves.init=function(options){var body=document.body;options=options||{};if(&quot; , &quot;'&quot; , &quot;duration&quot; , &quot;'&quot; , &quot; in options){Effect.duration=options.duration}
if(&quot; , &quot;'&quot; , &quot;delay&quot; , &quot;'&quot; , &quot; in options){Effect.delay=options.delay}
if(isTouchAvailable){body.addEventListener(&quot; , &quot;'&quot; , &quot;touchstart&quot; , &quot;'&quot; , &quot;,showEffect,!1);body.addEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,TouchHandler.registerEvent,!1);body.addEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,TouchHandler.registerEvent,!1)}
body.addEventListener(&quot; , &quot;'&quot; , &quot;mousedown&quot; , &quot;'&quot; , &quot;,showEffect,!1)};Waves.attach=function(elements,classes){elements=getWavesElements(elements);if(toString.call(classes)===&quot; , &quot;'&quot; , &quot;[object Array]&quot; , &quot;'&quot; , &quot;){classes=classes.join(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;)}
classes=classes?&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+classes:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;var element,tagName;for(var i=0,len=elements.length;i&lt;len;i++){element=elements[i];tagName=element.tagName.toLowerCase();if([&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;].indexOf(tagName)!==-1){TagWrapper[tagName](element);element=element.parentElement}
if(element.className.indexOf(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)===-1){element.className+=&quot; , &quot;'&quot; , &quot; waves-effect&quot; , &quot;'&quot; , &quot;+classes}}};Waves.ripple=function(elements,options){elements=getWavesElements(elements);var elementsLen=elements.length;options=options||{};options.wait=options.wait||0;options.position=options.position||null;if(elementsLen){var element,pos,off,centre={},i=0;var mousedown={type:&quot; , &quot;'&quot; , &quot;mousedown&quot; , &quot;'&quot; , &quot;,button:1};var hideRipple=function(mouseup,element){return function(){Effect.hide(mouseup,element)}};for(;i&lt;elementsLen;i++){element=elements[i];pos=options.position||{x:element.clientWidth/2,y:element.clientHeight/2};off=offset(element);centre.x=off.left+pos.x;centre.y=off.top+pos.y;mousedown.pageX=centre.x;mousedown.pageY=centre.y;Effect.show(mousedown,element);if(options.wait>=0&amp;&amp;options.wait!==null){var mouseup={type:&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,button:1};setTimeout(hideRipple(mouseup,element),options.wait)}}}};Waves.calm=function(elements){elements=getWavesElements(elements);var mouseup={type:&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,button:1};for(var i=0,len=elements.length;i&lt;len;i++){Effect.hide(mouseup,elements[i])}};Waves.displayEffect=function(options){console.error(&quot; , &quot;'&quot; , &quot;Waves.displayEffect() has been deprecated and will be removed in future version. Please use Waves.init() to initialize Waves effect&quot; , &quot;'&quot; , &quot;);Waves.init(options)};return Waves})

Waves.init();
Waves.attach(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;);
Waves.attach(&quot; , &quot;'&quot; , &quot;.logo&quot; , &quot;'&quot; , &quot;);

// CONSOLE
console.log(&quot;%cHi there! 👋 We are phptravels. we are disrupting the travel industry world-wide.&quot;, &quot;font-size:14px&quot;);
console.log(&quot;%cAny doubts, get in touch at info@phptravels.com&quot;, &quot;font-size:12px&quot;);



(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);d.innerHTML=&quot;window.__CF$cv$params={r:&quot; , &quot;'&quot; , &quot;95d87a4f391d712c&quot; , &quot;'&quot; , &quot;,t:&quot; , &quot;'&quot; , &quot;MTc1MjIzODc2My4wMDAwMDA=&quot; , &quot;'&quot; , &quot;};var a=document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);a.nonce=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;a.src=&quot; , &quot;'&quot; , &quot;/cdn-cgi/challenge-platform/scripts/jsd/main.js&quot; , &quot;'&quot; , &quot;;document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(a);&quot;;b.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(d)}}if(document.body){var a=document.createElement(&quot; , &quot;'&quot; , &quot;iframe&quot; , &quot;'&quot; , &quot;);a.height=1;a.width=1;a.style.position=&quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;a.style.top=0;a.style.left=0;a.style.border=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;a.style.visibility=&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;document.body.appendChild(a);if(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState)c();else if(window.addEventListener)document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;,c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState&amp;&amp;(document.onreadystatechange=e,c())}}}})();

  /html[1]&quot;) or . = concat(&quot;
Book Your Free Demo Now - Phptravels






































  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());
  gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;G-6TWLEQHRC1&quot; , &quot;'&quot; , &quot;);




   {
   &quot;@context&quot; : &quot;http://schema.org&quot;,
   &quot;@type&quot; : &quot;Corporation&quot;,
   &quot;brand&quot;: &quot;PHPTRAVELS&quot;,
   &quot;description&quot;: &quot;Targeting an incredible small niche, PHPtravels is a script designed for travel agencies that want to move their operations online, helping them setup a fully-working hotel room booking system.\n\nPHPtravels lets users search hotels, check room availability, book a room for their desired time period, and then pay for it, all via a simple interface, that not only looks good, but is also very manageable and customizable via a specially crafted administration panel.\n\nVarious accounts are available, for customers, for travel agents, and above all, for the site webmaster (admin).\n\nAnything on the PHPtravels website is customizable, from the frontend skin, to the active languages, available currencies, and not last, the hotels and rooms.\n\nIf you can get past the fact you need a commercial license to run PHPtravels, the system can be of incredible help if you plan to sell vacations and trips online as well.&quot;,
   &quot;name&quot; : &quot;PHPTRAVELS&quot;,
   &quot;founders&quot;: [
      &quot;Qasim Hussain&quot;
   ],
   &quot;foundingDate&quot;: &quot;2014-05&quot;,
   &quot;foundingLocation&quot;: &quot;Lahore&quot;,
   &quot;knowsAbout&quot;: &quot;Create online travel agency using PHPTRAVELS products&quot;,
   &quot;legalName&quot;: &quot;PHPTRAVELS&quot;,
   &quot;logo&quot; : &quot;https://phptravels.com/assets/img/pages/media/icon-primary.png&quot;,
   &quot;numberOfEmployees&quot;: &quot;15&quot;,
   &quot;ownershipFundingInfo&quot;: &quot;https://phptravels.com/about-us/&quot;,
   &quot;url&quot; : &quot;https://phptravels.com/demo&quot;,
   &quot;sameAs&quot; : [
      &quot;https://www.facebook.com/phptravels&quot;,
      &quot;https://www.twitter.com/phptravels&quot;,
      &quot;https://snapchat.com/add/phptravels&quot;,
      &quot;https://instagram.com/phptravels_/&quot;,
      &quot;https://www.youtube.com/user/phptravels&quot;,
      &quot;https://www.linkedin.com/company/phptravels&quot;,
      &quot;https://www.pinterest.com/phptravels_pin/&quot;
   ],
   &quot;slogan&quot;: &quot;Travel technology partner&quot;,
   &quot;tickerSymbol&quot;: [
      &quot;NYSE:SHOP&quot;,
      &quot;TSX:SHOP&quot;
   ],
   &quot;awards&quot;: &quot;https://www.ivisa.com/visa-blog/php-travels&quot;
      }






  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag(&quot; , &quot;'&quot; , &quot;js&quot; , &quot;'&quot; , &quot;, new Date());
  gtag(&quot; , &quot;'&quot; , &quot;config&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AW-419817699&quot; , &quot;'&quot; , &quot;);




// SAVE TRAFFIC TO DATABASE ON PAGE LOAD
setTimeout(function() {
var requestUrl = &quot;https://phptravels.com/visitor_details&quot;;
      fetch(requestUrl)
      .then(function(response) {
         return response.json();
      })
      .then(function(c) {
            if (typeof c.country_code === &quot;undefined&quot;) {
               // If country_code is undefined, log and return or handle appropriately
               console.log(&quot;Localhost traffic not counted&quot;);
            } else {
               console.log(c);
               var country = c.country_code.toUpperCase();

                const formdata = new FormData();
                formdata.append(&quot;country_code&quot;, country);
                formdata.append(&quot;user_ip&quot;, c.user_ip);
                formdata.append(&quot;page_link&quot;, window.location.href);
                formdata.append(&quot;user_agent&quot;, c.user_agent);
                formdata.append(&quot;http_referer&quot;, &quot;Direct Traffic&quot;);

                const requestOptions = {
                method: &quot;POST&quot;,
                body: formdata,
                redirect: &quot;follow&quot;
                };

                fetch(&quot;https://app.phptravels.com/api_website_traffic?&quot;, requestOptions)
                .then((response) => response.text())
                // .then((result) => console.log(result))
                .catch((error) => console.error(error));

            }
      });
  }, 2500);

  (function () {
      window.usermaven = window.usermaven || (function () { (window.usermavenQ = window.usermavenQ || []).push(arguments); })
      var t = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;),
          s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0];
      t.defer = true;
      t.id = &quot; , &quot;'&quot; , &quot;um-tracker&quot; , &quot;'&quot; , &quot;;
      t.setAttribute(&quot; , &quot;'&quot; , &quot;data-tracking-host&quot; , &quot;'&quot; , &quot;, &quot;https://events.usermaven.com&quot;)
      t.setAttribute(&quot; , &quot;'&quot; , &quot;data-key&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UMlduvNwrU&quot; , &quot;'&quot; , &quot;);
      t.setAttribute(&quot; , &quot;'&quot; , &quot;data-autocapture&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;);
      t.src = &quot; , &quot;'&quot; , &quot;https://t.usermaven.com/lib.js&quot; , &quot;'&quot; , &quot;;
      s.parentNode.insertBefore(t, s);
  })();



#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon katalon-div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} katalon-div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} katalon-div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}





  
  
      
        
          We stand with Palestine
          
            
              .cls-1{fill:#fff;}
              .cls-2{fill:#007a3d;}
              .cls-3{fill:#ce1126;}
            
          Palestine
          
          
          
          
          Because Humanity Matters
        
      
  

    
      
        
          

          
               
                  
                  
               
            

          
          
            PHPTRAVELS  
            Travel Tech Partner
            
          
        

        
        
        

        
          
            
              
                Product
                
                  
                
              
              
              
                    Themes
                    
                  
                
                
                    Integrations
                    
                  
                
                
                  Customization

                    
                  
                
                
                    Technology

                  
                
                    Requirements

                    
                  
              
            
            
              
                Features
                
                  
                
              
              
                
                    Flights Modules

                  
                
                
                    Hotels Modules

                  
                
                    Tours Modules

                  
                
                    Cars Modules

                  
                
                    Blogs Modules

                  
                
                    CMS Modules

                  
                
                    Offers Modules

                  
                
                    Newsletter Modules

                  
              
            
            
              
                Company
                
                  
                
              
              
                
                    Clients

                  
                
                    Blogs

                  
                
                    Docs

                  
                
                    Contact Us

                  
                
                    About Us

                  
                
                    The Team

                  
                
                    Jobs

                  
                
                    Partners

                  
                
                    Media Kit
                    
                  
              
            
            
              Demo
            
            
              Pricing
            
          
          
            
              
              Talk to Sales
            Login
            Sign Up

          
        
      
    
  
  
/*! jQuery v3.6.0 | (c) OpenJS Foundation and other contributors | jquery.org/license */
!function(e,t){&quot;use strict&quot;;&quot;object&quot;==typeof module&amp;&amp;&quot;object&quot;==typeof module.exports?module.exports=e.document?t(e,!0):function(e){if(!e.document)throw new Error(&quot;jQuery requires a window with a document&quot;);return t(e)}:t(e)}(&quot;undefined&quot;!=typeof window?window:this,function(C,e){&quot;use strict&quot;;var t=[],r=Object.getPrototypeOf,s=t.slice,g=t.flat?function(e){return t.flat.call(e)}:function(e){return t.concat.apply([],e)},u=t.push,i=t.indexOf,n={},o=n.toString,v=n.hasOwnProperty,a=v.toString,l=a.call(Object),y={},m=function(e){return&quot;function&quot;==typeof e&amp;&amp;&quot;number&quot;!=typeof e.nodeType&amp;&amp;&quot;function&quot;!=typeof e.item},x=function(e){return null!=e&amp;&amp;e===e.window},E=C.document,c={type:!0,src:!0,nonce:!0,noModule:!0};function b(e,t,n){var r,i,o=(n=n||E).createElement(&quot;script&quot;);if(o.text=e,t)for(r in c)(i=t[r]||t.getAttribute&amp;&amp;t.getAttribute(r))&amp;&amp;o.setAttribute(r,i);n.head.appendChild(o).parentNode.removeChild(o)}function w(e){return null==e?e+&quot;&quot;:&quot;object&quot;==typeof e||&quot;function&quot;==typeof e?n[o.call(e)]||&quot;object&quot;:typeof e}var f=&quot;3.6.0&quot;,S=function(e,t){return new S.fn.init(e,t)};function p(e){var t=!!e&amp;&amp;&quot;length&quot;in e&amp;&amp;e.length,n=w(e);return!m(e)&amp;&amp;!x(e)&amp;&amp;(&quot;array&quot;===n||0===t||&quot;number&quot;==typeof t&amp;&amp;0&lt;t&amp;&amp;t-1 in e)}S.fn=S.prototype={jquery:f,constructor:S,length:0,toArray:function(){return s.call(this)},get:function(e){return null==e?s.call(this):e&lt;0?this[e+this.length]:this[e]},pushStack:function(e){var t=S.merge(this.constructor(),e);return t.prevObject=this,t},each:function(e){return S.each(this,e)},map:function(n){return this.pushStack(S.map(this,function(e,t){return n.call(e,t,e)}))},slice:function(){return this.pushStack(s.apply(this,arguments))},first:function(){return this.eq(0)},last:function(){return this.eq(-1)},even:function(){return this.pushStack(S.grep(this,function(e,t){return(t+1)%2}))},odd:function(){return this.pushStack(S.grep(this,function(e,t){return t%2}))},eq:function(e){var t=this.length,n=+e+(e&lt;0?t:0);return this.pushStack(0&lt;=n&amp;&amp;n&lt;t?[this[n]]:[])},end:function(){return this.prevObject||this.constructor()},push:u,sort:t.sort,splice:t.splice},S.extend=S.fn.extend=function(){var e,t,n,r,i,o,a=arguments[0]||{},s=1,u=arguments.length,l=!1;for(&quot;boolean&quot;==typeof a&amp;&amp;(l=a,a=arguments[s]||{},s++),&quot;object&quot;==typeof a||m(a)||(a={}),s===u&amp;&amp;(a=this,s--);s&lt;u;s++)if(null!=(e=arguments[s]))for(t in e)r=e[t],&quot;__proto__&quot;!==t&amp;&amp;a!==r&amp;&amp;(l&amp;&amp;r&amp;&amp;(S.isPlainObject(r)||(i=Array.isArray(r)))?(n=a[t],o=i&amp;&amp;!Array.isArray(n)?[]:i||S.isPlainObject(n)?n:{},i=!1,a[t]=S.extend(l,o,r)):void 0!==r&amp;&amp;(a[t]=r));return a},S.extend({expando:&quot;jQuery&quot;+(f+Math.random()).replace(/\D/g,&quot;&quot;),isReady:!0,error:function(e){throw new Error(e)},noop:function(){},isPlainObject:function(e){var t,n;return!(!e||&quot;[object Object]&quot;!==o.call(e))&amp;&amp;(!(t=r(e))||&quot;function&quot;==typeof(n=v.call(t,&quot;constructor&quot;)&amp;&amp;t.constructor)&amp;&amp;a.call(n)===l)},isEmptyObject:function(e){var t;for(t in e)return!1;return!0},globalEval:function(e,t,n){b(e,{nonce:t&amp;&amp;t.nonce},n)},each:function(e,t){var n,r=0;if(p(e)){for(n=e.length;r&lt;n;r++)if(!1===t.call(e[r],r,e[r]))break}else for(r in e)if(!1===t.call(e[r],r,e[r]))break;return e},makeArray:function(e,t){var n=t||[];return null!=e&amp;&amp;(p(Object(e))?S.merge(n,&quot;string&quot;==typeof e?[e]:e):u.call(n,e)),n},inArray:function(e,t,n){return null==t?-1:i.call(t,e,n)},merge:function(e,t){for(var n=+t.length,r=0,i=e.length;r&lt;n;r++)e[i++]=t[r];return e.length=i,e},grep:function(e,t,n){for(var r=[],i=0,o=e.length,a=!n;i&lt;o;i++)!t(e[i],i)!==a&amp;&amp;r.push(e[i]);return r},map:function(e,t,n){var r,i,o=0,a=[];if(p(e))for(r=e.length;o&lt;r;o++)null!=(i=t(e[o],o,n))&amp;&amp;a.push(i);else for(o in e)null!=(i=t(e[o],o,n))&amp;&amp;a.push(i);return g(a)},guid:1,support:y}),&quot;function&quot;==typeof Symbol&amp;&amp;(S.fn[Symbol.iterator]=t[Symbol.iterator]),S.each(&quot;Boolean Number String Function Array Date RegExp Object Error Symbol&quot;.split(&quot; &quot;),function(e,t){n[&quot;[object &quot;+t+&quot;]&quot;]=t.toLowerCase()});var d=function(n){var e,d,b,o,i,h,f,g,w,u,l,T,C,a,E,v,s,c,y,S=&quot;sizzle&quot;+1*new Date,p=n.document,k=0,r=0,m=ue(),x=ue(),A=ue(),N=ue(),j=function(e,t){return e===t&amp;&amp;(l=!0),0},D={}.hasOwnProperty,t=[],q=t.pop,L=t.push,H=t.push,O=t.slice,P=function(e,t){for(var n=0,r=e.length;n&lt;r;n++)if(e[n]===t)return n;return-1},R=&quot;checked|selected|async|autofocus|autoplay|controls|defer|disabled|hidden|ismap|loop|multiple|open|readonly|required|scoped&quot;,M=&quot;[\\x20\\t\\r\\n\\f]&quot;,I=&quot;(?:\\\\[\\da-fA-F]{1,6}&quot;+M+&quot;?|\\\\[^\\r\\n\\f]|[\\w-]|[^\0-\\x7f])+&quot;,W=&quot;\\[&quot;+M+&quot;*(&quot;+I+&quot;)(?:&quot;+M+&quot;*([*^$|!~]?=)&quot;+M+&quot;*(?:&quot; , &quot;'&quot; , &quot;((?:\\\\.|[^\\\\&quot; , &quot;'&quot; , &quot;])*)&quot; , &quot;'&quot; , &quot;|\&quot;((?:\\\\.|[^\\\\\&quot;])*)\&quot;|(&quot;+I+&quot;))|)&quot;+M+&quot;*\\]&quot;,F=&quot;:(&quot;+I+&quot;)(?:\\(((&quot; , &quot;'&quot; , &quot;((?:\\\\.|[^\\\\&quot; , &quot;'&quot; , &quot;])*)&quot; , &quot;'&quot; , &quot;|\&quot;((?:\\\\.|[^\\\\\&quot;])*)\&quot;)|((?:\\\\.|[^\\\\()[\\]]|&quot;+W+&quot;)*)|.*)\\)|)&quot;,B=new RegExp(M+&quot;+&quot;,&quot;g&quot;),$=new RegExp(&quot;^&quot;+M+&quot;+|((?:^|[^\\\\])(?:\\\\.)*)&quot;+M+&quot;+$&quot;,&quot;g&quot;),_=new RegExp(&quot;^&quot;+M+&quot;*,&quot;+M+&quot;*&quot;),z=new RegExp(&quot;^&quot;+M+&quot;*([>+~]|&quot;+M+&quot;)&quot;+M+&quot;*&quot;),U=new RegExp(M+&quot;|>&quot;),X=new RegExp(F),V=new RegExp(&quot;^&quot;+I+&quot;$&quot;),G={ID:new RegExp(&quot;^#(&quot;+I+&quot;)&quot;),CLASS:new RegExp(&quot;^\\.(&quot;+I+&quot;)&quot;),TAG:new RegExp(&quot;^(&quot;+I+&quot;|[*])&quot;),ATTR:new RegExp(&quot;^&quot;+W),PSEUDO:new RegExp(&quot;^&quot;+F),CHILD:new RegExp(&quot;^:(only|first|last|nth|nth-last)-(child|of-type)(?:\\(&quot;+M+&quot;*(even|odd|(([+-]|)(\\d*)n|)&quot;+M+&quot;*(?:([+-]|)&quot;+M+&quot;*(\\d+)|))&quot;+M+&quot;*\\)|)&quot;,&quot;i&quot;),bool:new RegExp(&quot;^(?:&quot;+R+&quot;)$&quot;,&quot;i&quot;),needsContext:new RegExp(&quot;^&quot;+M+&quot;*[>+~]|:(even|odd|eq|gt|lt|nth|first|last)(?:\\(&quot;+M+&quot;*((?:-\\d)?\\d*)&quot;+M+&quot;*\\)|)(?=[^-]|$)&quot;,&quot;i&quot;)},Y=/HTML$/i,Q=/^(?:input|select|textarea|button)$/i,J=/^h\d$/i,K=/^[^{]+\{\s*\[native \w/,Z=/^(?:#([\w-]+)|(\w+)|\.([\w-]+))$/,ee=/[+~]/,te=new RegExp(&quot;\\\\[\\da-fA-F]{1,6}&quot;+M+&quot;?|\\\\([^\\r\\n\\f])&quot;,&quot;g&quot;),ne=function(e,t){var n=&quot;0x&quot;+e.slice(1)-65536;return t||(n&lt;0?String.fromCharCode(n+65536):String.fromCharCode(n>>10|55296,1023&amp;n|56320))},re=/([\0-\x1f\x7f]|^-?\d)|^-$|[^\0-\x1f\x7f-\uFFFF\w-]/g,ie=function(e,t){return t?&quot;\0&quot;===e?&quot;\ufffd&quot;:e.slice(0,-1)+&quot;\\&quot;+e.charCodeAt(e.length-1).toString(16)+&quot; &quot;:&quot;\\&quot;+e},oe=function(){T()},ae=be(function(e){return!0===e.disabled&amp;&amp;&quot;fieldset&quot;===e.nodeName.toLowerCase()},{dir:&quot;parentNode&quot;,next:&quot;legend&quot;});try{H.apply(t=O.call(p.childNodes),p.childNodes),t[p.childNodes.length].nodeType}catch(e){H={apply:t.length?function(e,t){L.apply(e,O.call(t))}:function(e,t){var n=e.length,r=0;while(e[n++]=t[r++]);e.length=n-1}}}function se(t,e,n,r){var i,o,a,s,u,l,c,f=e&amp;&amp;e.ownerDocument,p=e?e.nodeType:9;if(n=n||[],&quot;string&quot;!=typeof t||!t||1!==p&amp;&amp;9!==p&amp;&amp;11!==p)return n;if(!r&amp;&amp;(T(e),e=e||C,E)){if(11!==p&amp;&amp;(u=Z.exec(t)))if(i=u[1]){if(9===p){if(!(a=e.getElementById(i)))return n;if(a.id===i)return n.push(a),n}else if(f&amp;&amp;(a=f.getElementById(i))&amp;&amp;y(e,a)&amp;&amp;a.id===i)return n.push(a),n}else{if(u[2])return H.apply(n,e.getElementsByTagName(t)),n;if((i=u[3])&amp;&amp;d.getElementsByClassName&amp;&amp;e.getElementsByClassName)return H.apply(n,e.getElementsByClassName(i)),n}if(d.qsa&amp;&amp;!N[t+&quot; &quot;]&amp;&amp;(!v||!v.test(t))&amp;&amp;(1!==p||&quot;object&quot;!==e.nodeName.toLowerCase())){if(c=t,f=e,1===p&amp;&amp;(U.test(t)||z.test(t))){(f=ee.test(t)&amp;&amp;ye(e.parentNode)||e)===e&amp;&amp;d.scope||((s=e.getAttribute(&quot;id&quot;))?s=s.replace(re,ie):e.setAttribute(&quot;id&quot;,s=S)),o=(l=h(t)).length;while(o--)l[o]=(s?&quot;#&quot;+s:&quot;:scope&quot;)+&quot; &quot;+xe(l[o]);c=l.join(&quot;,&quot;)}try{return H.apply(n,f.querySelectorAll(c)),n}catch(e){N(t,!0)}finally{s===S&amp;&amp;e.removeAttribute(&quot;id&quot;)}}}return g(t.replace($,&quot;$1&quot;),e,n,r)}function ue(){var r=[];return function e(t,n){return r.push(t+&quot; &quot;)>b.cacheLength&amp;&amp;delete e[r.shift()],e[t+&quot; &quot;]=n}}function le(e){return e[S]=!0,e}function ce(e){var t=C.createElement(&quot;fieldset&quot;);try{return!!e(t)}catch(e){return!1}finally{t.parentNode&amp;&amp;t.parentNode.removeChild(t),t=null}}function fe(e,t){var n=e.split(&quot;|&quot;),r=n.length;while(r--)b.attrHandle[n[r]]=t}function pe(e,t){var n=t&amp;&amp;e,r=n&amp;&amp;1===e.nodeType&amp;&amp;1===t.nodeType&amp;&amp;e.sourceIndex-t.sourceIndex;if(r)return r;if(n)while(n=n.nextSibling)if(n===t)return-1;return e?1:-1}function de(t){return function(e){return&quot;input&quot;===e.nodeName.toLowerCase()&amp;&amp;e.type===t}}function he(n){return function(e){var t=e.nodeName.toLowerCase();return(&quot;input&quot;===t||&quot;button&quot;===t)&amp;&amp;e.type===n}}function ge(t){return function(e){return&quot;form&quot;in e?e.parentNode&amp;&amp;!1===e.disabled?&quot;label&quot;in e?&quot;label&quot;in e.parentNode?e.parentNode.disabled===t:e.disabled===t:e.isDisabled===t||e.isDisabled!==!t&amp;&amp;ae(e)===t:e.disabled===t:&quot;label&quot;in e&amp;&amp;e.disabled===t}}function ve(a){return le(function(o){return o=+o,le(function(e,t){var n,r=a([],e.length,o),i=r.length;while(i--)e[n=r[i]]&amp;&amp;(e[n]=!(t[n]=e[n]))})})}function ye(e){return e&amp;&amp;&quot;undefined&quot;!=typeof e.getElementsByTagName&amp;&amp;e}for(e in d=se.support={},i=se.isXML=function(e){var t=e&amp;&amp;e.namespaceURI,n=e&amp;&amp;(e.ownerDocument||e).documentElement;return!Y.test(t||n&amp;&amp;n.nodeName||&quot;HTML&quot;)},T=se.setDocument=function(e){var t,n,r=e?e.ownerDocument||e:p;return r!=C&amp;&amp;9===r.nodeType&amp;&amp;r.documentElement&amp;&amp;(a=(C=r).documentElement,E=!i(C),p!=C&amp;&amp;(n=C.defaultView)&amp;&amp;n.top!==n&amp;&amp;(n.addEventListener?n.addEventListener(&quot;unload&quot;,oe,!1):n.attachEvent&amp;&amp;n.attachEvent(&quot;onunload&quot;,oe)),d.scope=ce(function(e){return a.appendChild(e).appendChild(C.createElement(&quot;div&quot;)),&quot;undefined&quot;!=typeof e.querySelectorAll&amp;&amp;!e.querySelectorAll(&quot;:scope fieldset div&quot;).length}),d.attributes=ce(function(e){return e.className=&quot;i&quot;,!e.getAttribute(&quot;className&quot;)}),d.getElementsByTagName=ce(function(e){return e.appendChild(C.createComment(&quot;&quot;)),!e.getElementsByTagName(&quot;*&quot;).length}),d.getElementsByClassName=K.test(C.getElementsByClassName),d.getById=ce(function(e){return a.appendChild(e).id=S,!C.getElementsByName||!C.getElementsByName(S).length}),d.getById?(b.filter.ID=function(e){var t=e.replace(te,ne);return function(e){return e.getAttribute(&quot;id&quot;)===t}},b.find.ID=function(e,t){if(&quot;undefined&quot;!=typeof t.getElementById&amp;&amp;E){var n=t.getElementById(e);return n?[n]:[]}}):(b.filter.ID=function(e){var n=e.replace(te,ne);return function(e){var t=&quot;undefined&quot;!=typeof e.getAttributeNode&amp;&amp;e.getAttributeNode(&quot;id&quot;);return t&amp;&amp;t.value===n}},b.find.ID=function(e,t){if(&quot;undefined&quot;!=typeof t.getElementById&amp;&amp;E){var n,r,i,o=t.getElementById(e);if(o){if((n=o.getAttributeNode(&quot;id&quot;))&amp;&amp;n.value===e)return[o];i=t.getElementsByName(e),r=0;while(o=i[r++])if((n=o.getAttributeNode(&quot;id&quot;))&amp;&amp;n.value===e)return[o]}return[]}}),b.find.TAG=d.getElementsByTagName?function(e,t){return&quot;undefined&quot;!=typeof t.getElementsByTagName?t.getElementsByTagName(e):d.qsa?t.querySelectorAll(e):void 0}:function(e,t){var n,r=[],i=0,o=t.getElementsByTagName(e);if(&quot;*&quot;===e){while(n=o[i++])1===n.nodeType&amp;&amp;r.push(n);return r}return o},b.find.CLASS=d.getElementsByClassName&amp;&amp;function(e,t){if(&quot;undefined&quot;!=typeof t.getElementsByClassName&amp;&amp;E)return t.getElementsByClassName(e)},s=[],v=[],(d.qsa=K.test(C.querySelectorAll))&amp;&amp;(ce(function(e){var t;a.appendChild(e).innerHTML=&quot;&lt;a id=&quot; , &quot;'&quot; , &quot;&quot;+S+&quot;&quot; , &quot;'&quot; , &quot;>&lt;/a>&lt;select id=&quot; , &quot;'&quot; , &quot;&quot;+S+&quot;-\r\\&quot; , &quot;'&quot; , &quot; msallowcapture=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;>&lt;option selected=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;>&lt;/option>&lt;/select>&quot;,e.querySelectorAll(&quot;[msallowcapture^=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]&quot;).length&amp;&amp;v.push(&quot;[*^$]=&quot;+M+&quot;*(?:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;|\&quot;\&quot;)&quot;),e.querySelectorAll(&quot;[selected]&quot;).length||v.push(&quot;\\[&quot;+M+&quot;*(?:value|&quot;+R+&quot;)&quot;),e.querySelectorAll(&quot;[id~=&quot;+S+&quot;-]&quot;).length||v.push(&quot;~=&quot;),(t=C.createElement(&quot;input&quot;)).setAttribute(&quot;name&quot;,&quot;&quot;),e.appendChild(t),e.querySelectorAll(&quot;[name=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]&quot;).length||v.push(&quot;\\[&quot;+M+&quot;*name&quot;+M+&quot;*=&quot;+M+&quot;*(?:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;|\&quot;\&quot;)&quot;),e.querySelectorAll(&quot;:checked&quot;).length||v.push(&quot;:checked&quot;),e.querySelectorAll(&quot;a#&quot;+S+&quot;+*&quot;).length||v.push(&quot;.#.+[+~]&quot;),e.querySelectorAll(&quot;\\\f&quot;),v.push(&quot;[\\r\\n\\f]&quot;)}),ce(function(e){e.innerHTML=&quot;&lt;a href=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; disabled=&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;>&lt;/a>&lt;select disabled=&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;>&lt;option/>&lt;/select>&quot;;var t=C.createElement(&quot;input&quot;);t.setAttribute(&quot;type&quot;,&quot;hidden&quot;),e.appendChild(t).setAttribute(&quot;name&quot;,&quot;D&quot;),e.querySelectorAll(&quot;[name=d]&quot;).length&amp;&amp;v.push(&quot;name&quot;+M+&quot;*[*^$|!~]?=&quot;),2!==e.querySelectorAll(&quot;:enabled&quot;).length&amp;&amp;v.push(&quot;:enabled&quot;,&quot;:disabled&quot;),a.appendChild(e).disabled=!0,2!==e.querySelectorAll(&quot;:disabled&quot;).length&amp;&amp;v.push(&quot;:enabled&quot;,&quot;:disabled&quot;),e.querySelectorAll(&quot;*,:x&quot;),v.push(&quot;,.*:&quot;)})),(d.matchesSelector=K.test(c=a.matches||a.webkitMatchesSelector||a.mozMatchesSelector||a.oMatchesSelector||a.msMatchesSelector))&amp;&amp;ce(function(e){d.disconnectedMatch=c.call(e,&quot;*&quot;),c.call(e,&quot;[s!=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;]:x&quot;),s.push(&quot;!=&quot;,F)}),v=v.length&amp;&amp;new RegExp(v.join(&quot;|&quot;)),s=s.length&amp;&amp;new RegExp(s.join(&quot;|&quot;)),t=K.test(a.compareDocumentPosition),y=t||K.test(a.contains)?function(e,t){var n=9===e.nodeType?e.documentElement:e,r=t&amp;&amp;t.parentNode;return e===r||!(!r||1!==r.nodeType||!(n.contains?n.contains(r):e.compareDocumentPosition&amp;&amp;16&amp;e.compareDocumentPosition(r)))}:function(e,t){if(t)while(t=t.parentNode)if(t===e)return!0;return!1},j=t?function(e,t){if(e===t)return l=!0,0;var n=!e.compareDocumentPosition-!t.compareDocumentPosition;return n||(1&amp;(n=(e.ownerDocument||e)==(t.ownerDocument||t)?e.compareDocumentPosition(t):1)||!d.sortDetached&amp;&amp;t.compareDocumentPosition(e)===n?e==C||e.ownerDocument==p&amp;&amp;y(p,e)?-1:t==C||t.ownerDocument==p&amp;&amp;y(p,t)?1:u?P(u,e)-P(u,t):0:4&amp;n?-1:1)}:function(e,t){if(e===t)return l=!0,0;var n,r=0,i=e.parentNode,o=t.parentNode,a=[e],s=[t];if(!i||!o)return e==C?-1:t==C?1:i?-1:o?1:u?P(u,e)-P(u,t):0;if(i===o)return pe(e,t);n=e;while(n=n.parentNode)a.unshift(n);n=t;while(n=n.parentNode)s.unshift(n);while(a[r]===s[r])r++;return r?pe(a[r],s[r]):a[r]==p?-1:s[r]==p?1:0}),C},se.matches=function(e,t){return se(e,null,null,t)},se.matchesSelector=function(e,t){if(T(e),d.matchesSelector&amp;&amp;E&amp;&amp;!N[t+&quot; &quot;]&amp;&amp;(!s||!s.test(t))&amp;&amp;(!v||!v.test(t)))try{var n=c.call(e,t);if(n||d.disconnectedMatch||e.document&amp;&amp;11!==e.document.nodeType)return n}catch(e){N(t,!0)}return 0&lt;se(t,C,null,[e]).length},se.contains=function(e,t){return(e.ownerDocument||e)!=C&amp;&amp;T(e),y(e,t)},se.attr=function(e,t){(e.ownerDocument||e)!=C&amp;&amp;T(e);var n=b.attrHandle[t.toLowerCase()],r=n&amp;&amp;D.call(b.attrHandle,t.toLowerCase())?n(e,t,!E):void 0;return void 0!==r?r:d.attributes||!E?e.getAttribute(t):(r=e.getAttributeNode(t))&amp;&amp;r.specified?r.value:null},se.escape=function(e){return(e+&quot;&quot;).replace(re,ie)},se.error=function(e){throw new Error(&quot;Syntax error, unrecognized expression: &quot;+e)},se.uniqueSort=function(e){var t,n=[],r=0,i=0;if(l=!d.detectDuplicates,u=!d.sortStable&amp;&amp;e.slice(0),e.sort(j),l){while(t=e[i++])t===e[i]&amp;&amp;(r=n.push(i));while(r--)e.splice(n[r],1)}return u=null,e},o=se.getText=function(e){var t,n=&quot;&quot;,r=0,i=e.nodeType;if(i){if(1===i||9===i||11===i){if(&quot;string&quot;==typeof e.textContent)return e.textContent;for(e=e.firstChild;e;e=e.nextSibling)n+=o(e)}else if(3===i||4===i)return e.nodeValue}else while(t=e[r++])n+=o(t);return n},(b=se.selectors={cacheLength:50,createPseudo:le,match:G,attrHandle:{},find:{},relative:{&quot;>&quot;:{dir:&quot;parentNode&quot;,first:!0},&quot; &quot;:{dir:&quot;parentNode&quot;},&quot;+&quot;:{dir:&quot;previousSibling&quot;,first:!0},&quot;~&quot;:{dir:&quot;previousSibling&quot;}},preFilter:{ATTR:function(e){return e[1]=e[1].replace(te,ne),e[3]=(e[3]||e[4]||e[5]||&quot;&quot;).replace(te,ne),&quot;~=&quot;===e[2]&amp;&amp;(e[3]=&quot; &quot;+e[3]+&quot; &quot;),e.slice(0,4)},CHILD:function(e){return e[1]=e[1].toLowerCase(),&quot;nth&quot;===e[1].slice(0,3)?(e[3]||se.error(e[0]),e[4]=+(e[4]?e[5]+(e[6]||1):2*(&quot;even&quot;===e[3]||&quot;odd&quot;===e[3])),e[5]=+(e[7]+e[8]||&quot;odd&quot;===e[3])):e[3]&amp;&amp;se.error(e[0]),e},PSEUDO:function(e){var t,n=!e[6]&amp;&amp;e[2];return G.CHILD.test(e[0])?null:(e[3]?e[2]=e[4]||e[5]||&quot;&quot;:n&amp;&amp;X.test(n)&amp;&amp;(t=h(n,!0))&amp;&amp;(t=n.indexOf(&quot;)&quot;,n.length-t)-n.length)&amp;&amp;(e[0]=e[0].slice(0,t),e[2]=n.slice(0,t)),e.slice(0,3))}},filter:{TAG:function(e){var t=e.replace(te,ne).toLowerCase();return&quot;*&quot;===e?function(){return!0}:function(e){return e.nodeName&amp;&amp;e.nodeName.toLowerCase()===t}},CLASS:function(e){var t=m[e+&quot; &quot;];return t||(t=new RegExp(&quot;(^|&quot;+M+&quot;)&quot;+e+&quot;(&quot;+M+&quot;|$)&quot;))&amp;&amp;m(e,function(e){return t.test(&quot;string&quot;==typeof e.className&amp;&amp;e.className||&quot;undefined&quot;!=typeof e.getAttribute&amp;&amp;e.getAttribute(&quot;class&quot;)||&quot;&quot;)})},ATTR:function(n,r,i){return function(e){var t=se.attr(e,n);return null==t?&quot;!=&quot;===r:!r||(t+=&quot;&quot;,&quot;=&quot;===r?t===i:&quot;!=&quot;===r?t!==i:&quot;^=&quot;===r?i&amp;&amp;0===t.indexOf(i):&quot;*=&quot;===r?i&amp;&amp;-1&lt;t.indexOf(i):&quot;$=&quot;===r?i&amp;&amp;t.slice(-i.length)===i:&quot;~=&quot;===r?-1&lt;(&quot; &quot;+t.replace(B,&quot; &quot;)+&quot; &quot;).indexOf(i):&quot;|=&quot;===r&amp;&amp;(t===i||t.slice(0,i.length+1)===i+&quot;-&quot;))}},CHILD:function(h,e,t,g,v){var y=&quot;nth&quot;!==h.slice(0,3),m=&quot;last&quot;!==h.slice(-4),x=&quot;of-type&quot;===e;return 1===g&amp;&amp;0===v?function(e){return!!e.parentNode}:function(e,t,n){var r,i,o,a,s,u,l=y!==m?&quot;nextSibling&quot;:&quot;previousSibling&quot;,c=e.parentNode,f=x&amp;&amp;e.nodeName.toLowerCase(),p=!n&amp;&amp;!x,d=!1;if(c){if(y){while(l){a=e;while(a=a[l])if(x?a.nodeName.toLowerCase()===f:1===a.nodeType)return!1;u=l=&quot;only&quot;===h&amp;&amp;!u&amp;&amp;&quot;nextSibling&quot;}return!0}if(u=[m?c.firstChild:c.lastChild],m&amp;&amp;p){d=(s=(r=(i=(o=(a=c)[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]||[])[0]===k&amp;&amp;r[1])&amp;&amp;r[2],a=s&amp;&amp;c.childNodes[s];while(a=++s&amp;&amp;a&amp;&amp;a[l]||(d=s=0)||u.pop())if(1===a.nodeType&amp;&amp;++d&amp;&amp;a===e){i[h]=[k,s,d];break}}else if(p&amp;&amp;(d=s=(r=(i=(o=(a=e)[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]||[])[0]===k&amp;&amp;r[1]),!1===d)while(a=++s&amp;&amp;a&amp;&amp;a[l]||(d=s=0)||u.pop())if((x?a.nodeName.toLowerCase()===f:1===a.nodeType)&amp;&amp;++d&amp;&amp;(p&amp;&amp;((i=(o=a[S]||(a[S]={}))[a.uniqueID]||(o[a.uniqueID]={}))[h]=[k,d]),a===e))break;return(d-=v)===g||d%g==0&amp;&amp;0&lt;=d/g}}},PSEUDO:function(e,o){var t,a=b.pseudos[e]||b.setFilters[e.toLowerCase()]||se.error(&quot;unsupported pseudo: &quot;+e);return a[S]?a(o):1&lt;a.length?(t=[e,e,&quot;&quot;,o],b.setFilters.hasOwnProperty(e.toLowerCase())?le(function(e,t){var n,r=a(e,o),i=r.length;while(i--)e[n=P(e,r[i])]=!(t[n]=r[i])}):function(e){return a(e,0,t)}):a}},pseudos:{not:le(function(e){var r=[],i=[],s=f(e.replace($,&quot;$1&quot;));return s[S]?le(function(e,t,n,r){var i,o=s(e,null,r,[]),a=e.length;while(a--)(i=o[a])&amp;&amp;(e[a]=!(t[a]=i))}):function(e,t,n){return r[0]=e,s(r,null,n,i),r[0]=null,!i.pop()}}),has:le(function(t){return function(e){return 0&lt;se(t,e).length}}),contains:le(function(t){return t=t.replace(te,ne),function(e){return-1&lt;(e.textContent||o(e)).indexOf(t)}}),lang:le(function(n){return V.test(n||&quot;&quot;)||se.error(&quot;unsupported lang: &quot;+n),n=n.replace(te,ne).toLowerCase(),function(e){var t;do{if(t=E?e.lang:e.getAttribute(&quot;xml:lang&quot;)||e.getAttribute(&quot;lang&quot;))return(t=t.toLowerCase())===n||0===t.indexOf(n+&quot;-&quot;)}while((e=e.parentNode)&amp;&amp;1===e.nodeType);return!1}}),target:function(e){var t=n.location&amp;&amp;n.location.hash;return t&amp;&amp;t.slice(1)===e.id},root:function(e){return e===a},focus:function(e){return e===C.activeElement&amp;&amp;(!C.hasFocus||C.hasFocus())&amp;&amp;!!(e.type||e.href||~e.tabIndex)},enabled:ge(!1),disabled:ge(!0),checked:function(e){var t=e.nodeName.toLowerCase();return&quot;input&quot;===t&amp;&amp;!!e.checked||&quot;option&quot;===t&amp;&amp;!!e.selected},selected:function(e){return e.parentNode&amp;&amp;e.parentNode.selectedIndex,!0===e.selected},empty:function(e){for(e=e.firstChild;e;e=e.nextSibling)if(e.nodeType&lt;6)return!1;return!0},parent:function(e){return!b.pseudos.empty(e)},header:function(e){return J.test(e.nodeName)},input:function(e){return Q.test(e.nodeName)},button:function(e){var t=e.nodeName.toLowerCase();return&quot;input&quot;===t&amp;&amp;&quot;button&quot;===e.type||&quot;button&quot;===t},text:function(e){var t;return&quot;input&quot;===e.nodeName.toLowerCase()&amp;&amp;&quot;text&quot;===e.type&amp;&amp;(null==(t=e.getAttribute(&quot;type&quot;))||&quot;text&quot;===t.toLowerCase())},first:ve(function(){return[0]}),last:ve(function(e,t){return[t-1]}),eq:ve(function(e,t,n){return[n&lt;0?n+t:n]}),even:ve(function(e,t){for(var n=0;n&lt;t;n+=2)e.push(n);return e}),odd:ve(function(e,t){for(var n=1;n&lt;t;n+=2)e.push(n);return e}),lt:ve(function(e,t,n){for(var r=n&lt;0?n+t:t&lt;n?t:n;0&lt;=--r;)e.push(r);return e}),gt:ve(function(e,t,n){for(var r=n&lt;0?n+t:n;++r&lt;t;)e.push(r);return e})}}).pseudos.nth=b.pseudos.eq,{radio:!0,checkbox:!0,file:!0,password:!0,image:!0})b.pseudos[e]=de(e);for(e in{submit:!0,reset:!0})b.pseudos[e]=he(e);function me(){}function xe(e){for(var t=0,n=e.length,r=&quot;&quot;;t&lt;n;t++)r+=e[t].value;return r}function be(s,e,t){var u=e.dir,l=e.next,c=l||u,f=t&amp;&amp;&quot;parentNode&quot;===c,p=r++;return e.first?function(e,t,n){while(e=e[u])if(1===e.nodeType||f)return s(e,t,n);return!1}:function(e,t,n){var r,i,o,a=[k,p];if(n){while(e=e[u])if((1===e.nodeType||f)&amp;&amp;s(e,t,n))return!0}else while(e=e[u])if(1===e.nodeType||f)if(i=(o=e[S]||(e[S]={}))[e.uniqueID]||(o[e.uniqueID]={}),l&amp;&amp;l===e.nodeName.toLowerCase())e=e[u]||e;else{if((r=i[c])&amp;&amp;r[0]===k&amp;&amp;r[1]===p)return a[2]=r[2];if((i[c]=a)[2]=s(e,t,n))return!0}return!1}}function we(i){return 1&lt;i.length?function(e,t,n){var r=i.length;while(r--)if(!i[r](e,t,n))return!1;return!0}:i[0]}function Te(e,t,n,r,i){for(var o,a=[],s=0,u=e.length,l=null!=t;s&lt;u;s++)(o=e[s])&amp;&amp;(n&amp;&amp;!n(o,r,i)||(a.push(o),l&amp;&amp;t.push(s)));return a}function Ce(d,h,g,v,y,e){return v&amp;&amp;!v[S]&amp;&amp;(v=Ce(v)),y&amp;&amp;!y[S]&amp;&amp;(y=Ce(y,e)),le(function(e,t,n,r){var i,o,a,s=[],u=[],l=t.length,c=e||function(e,t,n){for(var r=0,i=t.length;r&lt;i;r++)se(e,t[r],n);return n}(h||&quot;*&quot;,n.nodeType?[n]:n,[]),f=!d||!e&amp;&amp;h?c:Te(c,s,d,n,r),p=g?y||(e?d:l||v)?[]:t:f;if(g&amp;&amp;g(f,p,n,r),v){i=Te(p,u),v(i,[],n,r),o=i.length;while(o--)(a=i[o])&amp;&amp;(p[u[o]]=!(f[u[o]]=a))}if(e){if(y||d){if(y){i=[],o=p.length;while(o--)(a=p[o])&amp;&amp;i.push(f[o]=a);y(null,p=[],i,r)}o=p.length;while(o--)(a=p[o])&amp;&amp;-1&lt;(i=y?P(e,a):s[o])&amp;&amp;(e[i]=!(t[i]=a))}}else p=Te(p===t?p.splice(l,p.length):p),y?y(null,t,p,r):H.apply(t,p)})}function Ee(e){for(var i,t,n,r=e.length,o=b.relative[e[0].type],a=o||b.relative[&quot; &quot;],s=o?1:0,u=be(function(e){return e===i},a,!0),l=be(function(e){return-1&lt;P(i,e)},a,!0),c=[function(e,t,n){var r=!o&amp;&amp;(n||t!==w)||((i=t).nodeType?u(e,t,n):l(e,t,n));return i=null,r}];s&lt;r;s++)if(t=b.relative[e[s].type])c=[be(we(c),t)];else{if((t=b.filter[e[s].type].apply(null,e[s].matches))[S]){for(n=++s;n&lt;r;n++)if(b.relative[e[n].type])break;return Ce(1&lt;s&amp;&amp;we(c),1&lt;s&amp;&amp;xe(e.slice(0,s-1).concat({value:&quot; &quot;===e[s-2].type?&quot;*&quot;:&quot;&quot;})).replace($,&quot;$1&quot;),t,s&lt;n&amp;&amp;Ee(e.slice(s,n)),n&lt;r&amp;&amp;Ee(e=e.slice(n)),n&lt;r&amp;&amp;xe(e))}c.push(t)}return we(c)}return me.prototype=b.filters=b.pseudos,b.setFilters=new me,h=se.tokenize=function(e,t){var n,r,i,o,a,s,u,l=x[e+&quot; &quot;];if(l)return t?0:l.slice(0);a=e,s=[],u=b.preFilter;while(a){for(o in n&amp;&amp;!(r=_.exec(a))||(r&amp;&amp;(a=a.slice(r[0].length)||a),s.push(i=[])),n=!1,(r=z.exec(a))&amp;&amp;(n=r.shift(),i.push({value:n,type:r[0].replace($,&quot; &quot;)}),a=a.slice(n.length)),b.filter)!(r=G[o].exec(a))||u[o]&amp;&amp;!(r=u[o](r))||(n=r.shift(),i.push({value:n,type:o,matches:r}),a=a.slice(n.length));if(!n)break}return t?a.length:a?se.error(e):x(e,s).slice(0)},f=se.compile=function(e,t){var n,v,y,m,x,r,i=[],o=[],a=A[e+&quot; &quot;];if(!a){t||(t=h(e)),n=t.length;while(n--)(a=Ee(t[n]))[S]?i.push(a):o.push(a);(a=A(e,(v=o,m=0&lt;(y=i).length,x=0&lt;v.length,r=function(e,t,n,r,i){var o,a,s,u=0,l=&quot;0&quot;,c=e&amp;&amp;[],f=[],p=w,d=e||x&amp;&amp;b.find.TAG(&quot;*&quot;,i),h=k+=null==p?1:Math.random()||.1,g=d.length;for(i&amp;&amp;(w=t==C||t||i);l!==g&amp;&amp;null!=(o=d[l]);l++){if(x&amp;&amp;o){a=0,t||o.ownerDocument==C||(T(o),n=!E);while(s=v[a++])if(s(o,t||C,n)){r.push(o);break}i&amp;&amp;(k=h)}m&amp;&amp;((o=!s&amp;&amp;o)&amp;&amp;u--,e&amp;&amp;c.push(o))}if(u+=l,m&amp;&amp;l!==u){a=0;while(s=y[a++])s(c,f,t,n);if(e){if(0&lt;u)while(l--)c[l]||f[l]||(f[l]=q.call(r));f=Te(f)}H.apply(r,f),i&amp;&amp;!e&amp;&amp;0&lt;f.length&amp;&amp;1&lt;u+y.length&amp;&amp;se.uniqueSort(r)}return i&amp;&amp;(k=h,w=p),c},m?le(r):r))).selector=e}return a},g=se.select=function(e,t,n,r){var i,o,a,s,u,l=&quot;function&quot;==typeof e&amp;&amp;e,c=!r&amp;&amp;h(e=l.selector||e);if(n=n||[],1===c.length){if(2&lt;(o=c[0]=c[0].slice(0)).length&amp;&amp;&quot;ID&quot;===(a=o[0]).type&amp;&amp;9===t.nodeType&amp;&amp;E&amp;&amp;b.relative[o[1].type]){if(!(t=(b.find.ID(a.matches[0].replace(te,ne),t)||[])[0]))return n;l&amp;&amp;(t=t.parentNode),e=e.slice(o.shift().value.length)}i=G.needsContext.test(e)?0:o.length;while(i--){if(a=o[i],b.relative[s=a.type])break;if((u=b.find[s])&amp;&amp;(r=u(a.matches[0].replace(te,ne),ee.test(o[0].type)&amp;&amp;ye(t.parentNode)||t))){if(o.splice(i,1),!(e=r.length&amp;&amp;xe(o)))return H.apply(n,r),n;break}}}return(l||f(e,c))(r,t,!E,n,!t||ee.test(e)&amp;&amp;ye(t.parentNode)||t),n},d.sortStable=S.split(&quot;&quot;).sort(j).join(&quot;&quot;)===S,d.detectDuplicates=!!l,T(),d.sortDetached=ce(function(e){return 1&amp;e.compareDocumentPosition(C.createElement(&quot;fieldset&quot;))}),ce(function(e){return e.innerHTML=&quot;&lt;a href=&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;>&lt;/a>&quot;,&quot;#&quot;===e.firstChild.getAttribute(&quot;href&quot;)})||fe(&quot;type|href|height|width&quot;,function(e,t,n){if(!n)return e.getAttribute(t,&quot;type&quot;===t.toLowerCase()?1:2)}),d.attributes&amp;&amp;ce(function(e){return e.innerHTML=&quot;&lt;input/>&quot;,e.firstChild.setAttribute(&quot;value&quot;,&quot;&quot;),&quot;&quot;===e.firstChild.getAttribute(&quot;value&quot;)})||fe(&quot;value&quot;,function(e,t,n){if(!n&amp;&amp;&quot;input&quot;===e.nodeName.toLowerCase())return e.defaultValue}),ce(function(e){return null==e.getAttribute(&quot;disabled&quot;)})||fe(R,function(e,t,n){var r;if(!n)return!0===e[t]?t.toLowerCase():(r=e.getAttributeNode(t))&amp;&amp;r.specified?r.value:null}),se}(C);S.find=d,S.expr=d.selectors,S.expr[&quot;:&quot;]=S.expr.pseudos,S.uniqueSort=S.unique=d.uniqueSort,S.text=d.getText,S.isXMLDoc=d.isXML,S.contains=d.contains,S.escapeSelector=d.escape;var h=function(e,t,n){var r=[],i=void 0!==n;while((e=e[t])&amp;&amp;9!==e.nodeType)if(1===e.nodeType){if(i&amp;&amp;S(e).is(n))break;r.push(e)}return r},T=function(e,t){for(var n=[];e;e=e.nextSibling)1===e.nodeType&amp;&amp;e!==t&amp;&amp;n.push(e);return n},k=S.expr.match.needsContext;function A(e,t){return e.nodeName&amp;&amp;e.nodeName.toLowerCase()===t.toLowerCase()}var N=/^&lt;([a-z][^\/\0>:\x20\t\r\n\f]*)[\x20\t\r\n\f]*\/?>(?:&lt;\/\1>|)$/i;function j(e,n,r){return m(n)?S.grep(e,function(e,t){return!!n.call(e,t,e)!==r}):n.nodeType?S.grep(e,function(e){return e===n!==r}):&quot;string&quot;!=typeof n?S.grep(e,function(e){return-1&lt;i.call(n,e)!==r}):S.filter(n,e,r)}S.filter=function(e,t,n){var r=t[0];return n&amp;&amp;(e=&quot;:not(&quot;+e+&quot;)&quot;),1===t.length&amp;&amp;1===r.nodeType?S.find.matchesSelector(r,e)?[r]:[]:S.find.matches(e,S.grep(t,function(e){return 1===e.nodeType}))},S.fn.extend({find:function(e){var t,n,r=this.length,i=this;if(&quot;string&quot;!=typeof e)return this.pushStack(S(e).filter(function(){for(t=0;t&lt;r;t++)if(S.contains(i[t],this))return!0}));for(n=this.pushStack([]),t=0;t&lt;r;t++)S.find(e,i[t],n);return 1&lt;r?S.uniqueSort(n):n},filter:function(e){return this.pushStack(j(this,e||[],!1))},not:function(e){return this.pushStack(j(this,e||[],!0))},is:function(e){return!!j(this,&quot;string&quot;==typeof e&amp;&amp;k.test(e)?S(e):e||[],!1).length}});var D,q=/^(?:\s*(&lt;[\w\W]+>)[^>]*|#([\w-]+))$/;(S.fn.init=function(e,t,n){var r,i;if(!e)return this;if(n=n||D,&quot;string&quot;==typeof e){if(!(r=&quot;&lt;&quot;===e[0]&amp;&amp;&quot;>&quot;===e[e.length-1]&amp;&amp;3&lt;=e.length?[null,e,null]:q.exec(e))||!r[1]&amp;&amp;t)return!t||t.jquery?(t||n).find(e):this.constructor(t).find(e);if(r[1]){if(t=t instanceof S?t[0]:t,S.merge(this,S.parseHTML(r[1],t&amp;&amp;t.nodeType?t.ownerDocument||t:E,!0)),N.test(r[1])&amp;&amp;S.isPlainObject(t))for(r in t)m(this[r])?this[r](t[r]):this.attr(r,t[r]);return this}return(i=E.getElementById(r[2]))&amp;&amp;(this[0]=i,this.length=1),this}return e.nodeType?(this[0]=e,this.length=1,this):m(e)?void 0!==n.ready?n.ready(e):e(S):S.makeArray(e,this)}).prototype=S.fn,D=S(E);var L=/^(?:parents|prev(?:Until|All))/,H={children:!0,contents:!0,next:!0,prev:!0};function O(e,t){while((e=e[t])&amp;&amp;1!==e.nodeType);return e}S.fn.extend({has:function(e){var t=S(e,this),n=t.length;return this.filter(function(){for(var e=0;e&lt;n;e++)if(S.contains(this,t[e]))return!0})},closest:function(e,t){var n,r=0,i=this.length,o=[],a=&quot;string&quot;!=typeof e&amp;&amp;S(e);if(!k.test(e))for(;r&lt;i;r++)for(n=this[r];n&amp;&amp;n!==t;n=n.parentNode)if(n.nodeType&lt;11&amp;&amp;(a?-1&lt;a.index(n):1===n.nodeType&amp;&amp;S.find.matchesSelector(n,e))){o.push(n);break}return this.pushStack(1&lt;o.length?S.uniqueSort(o):o)},index:function(e){return e?&quot;string&quot;==typeof e?i.call(S(e),this[0]):i.call(this,e.jquery?e[0]:e):this[0]&amp;&amp;this[0].parentNode?this.first().prevAll().length:-1},add:function(e,t){return this.pushStack(S.uniqueSort(S.merge(this.get(),S(e,t))))},addBack:function(e){return this.add(null==e?this.prevObject:this.prevObject.filter(e))}}),S.each({parent:function(e){var t=e.parentNode;return t&amp;&amp;11!==t.nodeType?t:null},parents:function(e){return h(e,&quot;parentNode&quot;)},parentsUntil:function(e,t,n){return h(e,&quot;parentNode&quot;,n)},next:function(e){return O(e,&quot;nextSibling&quot;)},prev:function(e){return O(e,&quot;previousSibling&quot;)},nextAll:function(e){return h(e,&quot;nextSibling&quot;)},prevAll:function(e){return h(e,&quot;previousSibling&quot;)},nextUntil:function(e,t,n){return h(e,&quot;nextSibling&quot;,n)},prevUntil:function(e,t,n){return h(e,&quot;previousSibling&quot;,n)},siblings:function(e){return T((e.parentNode||{}).firstChild,e)},children:function(e){return T(e.firstChild)},contents:function(e){return null!=e.contentDocument&amp;&amp;r(e.contentDocument)?e.contentDocument:(A(e,&quot;template&quot;)&amp;&amp;(e=e.content||e),S.merge([],e.childNodes))}},function(r,i){S.fn[r]=function(e,t){var n=S.map(this,i,e);return&quot;Until&quot;!==r.slice(-5)&amp;&amp;(t=e),t&amp;&amp;&quot;string&quot;==typeof t&amp;&amp;(n=S.filter(t,n)),1&lt;this.length&amp;&amp;(H[r]||S.uniqueSort(n),L.test(r)&amp;&amp;n.reverse()),this.pushStack(n)}});var P=/[^\x20\t\r\n\f]+/g;function R(e){return e}function M(e){throw e}function I(e,t,n,r){var i;try{e&amp;&amp;m(i=e.promise)?i.call(e).done(t).fail(n):e&amp;&amp;m(i=e.then)?i.call(e,t,n):t.apply(void 0,[e].slice(r))}catch(e){n.apply(void 0,[e])}}S.Callbacks=function(r){var e,n;r=&quot;string&quot;==typeof r?(e=r,n={},S.each(e.match(P)||[],function(e,t){n[t]=!0}),n):S.extend({},r);var i,t,o,a,s=[],u=[],l=-1,c=function(){for(a=a||r.once,o=i=!0;u.length;l=-1){t=u.shift();while(++l&lt;s.length)!1===s[l].apply(t[0],t[1])&amp;&amp;r.stopOnFalse&amp;&amp;(l=s.length,t=!1)}r.memory||(t=!1),i=!1,a&amp;&amp;(s=t?[]:&quot;&quot;)},f={add:function(){return s&amp;&amp;(t&amp;&amp;!i&amp;&amp;(l=s.length-1,u.push(t)),function n(e){S.each(e,function(e,t){m(t)?r.unique&amp;&amp;f.has(t)||s.push(t):t&amp;&amp;t.length&amp;&amp;&quot;string&quot;!==w(t)&amp;&amp;n(t)})}(arguments),t&amp;&amp;!i&amp;&amp;c()),this},remove:function(){return S.each(arguments,function(e,t){var n;while(-1&lt;(n=S.inArray(t,s,n)))s.splice(n,1),n&lt;=l&amp;&amp;l--}),this},has:function(e){return e?-1&lt;S.inArray(e,s):0&lt;s.length},empty:function(){return s&amp;&amp;(s=[]),this},disable:function(){return a=u=[],s=t=&quot;&quot;,this},disabled:function(){return!s},lock:function(){return a=u=[],t||i||(s=t=&quot;&quot;),this},locked:function(){return!!a},fireWith:function(e,t){return a||(t=[e,(t=t||[]).slice?t.slice():t],u.push(t),i||c()),this},fire:function(){return f.fireWith(this,arguments),this},fired:function(){return!!o}};return f},S.extend({Deferred:function(e){var o=[[&quot;notify&quot;,&quot;progress&quot;,S.Callbacks(&quot;memory&quot;),S.Callbacks(&quot;memory&quot;),2],[&quot;resolve&quot;,&quot;done&quot;,S.Callbacks(&quot;once memory&quot;),S.Callbacks(&quot;once memory&quot;),0,&quot;resolved&quot;],[&quot;reject&quot;,&quot;fail&quot;,S.Callbacks(&quot;once memory&quot;),S.Callbacks(&quot;once memory&quot;),1,&quot;rejected&quot;]],i=&quot;pending&quot;,a={state:function(){return i},always:function(){return s.done(arguments).fail(arguments),this},&quot;catch&quot;:function(e){return a.then(null,e)},pipe:function(){var i=arguments;return S.Deferred(function(r){S.each(o,function(e,t){var n=m(i[t[4]])&amp;&amp;i[t[4]];s[t[1]](function(){var e=n&amp;&amp;n.apply(this,arguments);e&amp;&amp;m(e.promise)?e.promise().progress(r.notify).done(r.resolve).fail(r.reject):r[t[0]+&quot;With&quot;](this,n?[e]:arguments)})}),i=null}).promise()},then:function(t,n,r){var u=0;function l(i,o,a,s){return function(){var n=this,r=arguments,e=function(){var e,t;if(!(i&lt;u)){if((e=a.apply(n,r))===o.promise())throw new TypeError(&quot;Thenable self-resolution&quot;);t=e&amp;&amp;(&quot;object&quot;==typeof e||&quot;function&quot;==typeof e)&amp;&amp;e.then,m(t)?s?t.call(e,l(u,o,R,s),l(u,o,M,s)):(u++,t.call(e,l(u,o,R,s),l(u,o,M,s),l(u,o,R,o.notifyWith))):(a!==R&amp;&amp;(n=void 0,r=[e]),(s||o.resolveWith)(n,r))}},t=s?e:function(){try{e()}catch(e){S.Deferred.exceptionHook&amp;&amp;S.Deferred.exceptionHook(e,t.stackTrace),u&lt;=i+1&amp;&amp;(a!==M&amp;&amp;(n=void 0,r=[e]),o.rejectWith(n,r))}};i?t():(S.Deferred.getStackHook&amp;&amp;(t.stackTrace=S.Deferred.getStackHook()),C.setTimeout(t))}}return S.Deferred(function(e){o[0][3].add(l(0,e,m(r)?r:R,e.notifyWith)),o[1][3].add(l(0,e,m(t)?t:R)),o[2][3].add(l(0,e,m(n)?n:M))}).promise()},promise:function(e){return null!=e?S.extend(e,a):a}},s={};return S.each(o,function(e,t){var n=t[2],r=t[5];a[t[1]]=n.add,r&amp;&amp;n.add(function(){i=r},o[3-e][2].disable,o[3-e][3].disable,o[0][2].lock,o[0][3].lock),n.add(t[3].fire),s[t[0]]=function(){return s[t[0]+&quot;With&quot;](this===s?void 0:this,arguments),this},s[t[0]+&quot;With&quot;]=n.fireWith}),a.promise(s),e&amp;&amp;e.call(s,s),s},when:function(e){var n=arguments.length,t=n,r=Array(t),i=s.call(arguments),o=S.Deferred(),a=function(t){return function(e){r[t]=this,i[t]=1&lt;arguments.length?s.call(arguments):e,--n||o.resolveWith(r,i)}};if(n&lt;=1&amp;&amp;(I(e,o.done(a(t)).resolve,o.reject,!n),&quot;pending&quot;===o.state()||m(i[t]&amp;&amp;i[t].then)))return o.then();while(t--)I(i[t],a(t),o.reject);return o.promise()}});var W=/^(Eval|Internal|Range|Reference|Syntax|Type|URI)Error$/;S.Deferred.exceptionHook=function(e,t){C.console&amp;&amp;C.console.warn&amp;&amp;e&amp;&amp;W.test(e.name)&amp;&amp;C.console.warn(&quot;jQuery.Deferred exception: &quot;+e.message,e.stack,t)},S.readyException=function(e){C.setTimeout(function(){throw e})};var F=S.Deferred();function B(){E.removeEventListener(&quot;DOMContentLoaded&quot;,B),C.removeEventListener(&quot;load&quot;,B),S.ready()}S.fn.ready=function(e){return F.then(e)[&quot;catch&quot;](function(e){S.readyException(e)}),this},S.extend({isReady:!1,readyWait:1,ready:function(e){(!0===e?--S.readyWait:S.isReady)||(S.isReady=!0)!==e&amp;&amp;0&lt;--S.readyWait||F.resolveWith(E,[S])}}),S.ready.then=F.then,&quot;complete&quot;===E.readyState||&quot;loading&quot;!==E.readyState&amp;&amp;!E.documentElement.doScroll?C.setTimeout(S.ready):(E.addEventListener(&quot;DOMContentLoaded&quot;,B),C.addEventListener(&quot;load&quot;,B));var $=function(e,t,n,r,i,o,a){var s=0,u=e.length,l=null==n;if(&quot;object&quot;===w(n))for(s in i=!0,n)$(e,t,s,n[s],!0,o,a);else if(void 0!==r&amp;&amp;(i=!0,m(r)||(a=!0),l&amp;&amp;(a?(t.call(e,r),t=null):(l=t,t=function(e,t,n){return l.call(S(e),n)})),t))for(;s&lt;u;s++)t(e[s],n,a?r:r.call(e[s],s,t(e[s],n)));return i?e:l?t.call(e):u?t(e[0],n):o},_=/^-ms-/,z=/-([a-z])/g;function U(e,t){return t.toUpperCase()}function X(e){return e.replace(_,&quot;ms-&quot;).replace(z,U)}var V=function(e){return 1===e.nodeType||9===e.nodeType||!+e.nodeType};function G(){this.expando=S.expando+G.uid++}G.uid=1,G.prototype={cache:function(e){var t=e[this.expando];return t||(t={},V(e)&amp;&amp;(e.nodeType?e[this.expando]=t:Object.defineProperty(e,this.expando,{value:t,configurable:!0}))),t},set:function(e,t,n){var r,i=this.cache(e);if(&quot;string&quot;==typeof t)i[X(t)]=n;else for(r in t)i[X(r)]=t[r];return i},get:function(e,t){return void 0===t?this.cache(e):e[this.expando]&amp;&amp;e[this.expando][X(t)]},access:function(e,t,n){return void 0===t||t&amp;&amp;&quot;string&quot;==typeof t&amp;&amp;void 0===n?this.get(e,t):(this.set(e,t,n),void 0!==n?n:t)},remove:function(e,t){var n,r=e[this.expando];if(void 0!==r){if(void 0!==t){n=(t=Array.isArray(t)?t.map(X):(t=X(t))in r?[t]:t.match(P)||[]).length;while(n--)delete r[t[n]]}(void 0===t||S.isEmptyObject(r))&amp;&amp;(e.nodeType?e[this.expando]=void 0:delete e[this.expando])}},hasData:function(e){var t=e[this.expando];return void 0!==t&amp;&amp;!S.isEmptyObject(t)}};var Y=new G,Q=new G,J=/^(?:\{[\w\W]*\}|\[[\w\W]*\])$/,K=/[A-Z]/g;function Z(e,t,n){var r,i;if(void 0===n&amp;&amp;1===e.nodeType)if(r=&quot;data-&quot;+t.replace(K,&quot;-$&amp;&quot;).toLowerCase(),&quot;string&quot;==typeof(n=e.getAttribute(r))){try{n=&quot;true&quot;===(i=n)||&quot;false&quot;!==i&amp;&amp;(&quot;null&quot;===i?null:i===+i+&quot;&quot;?+i:J.test(i)?JSON.parse(i):i)}catch(e){}Q.set(e,t,n)}else n=void 0;return n}S.extend({hasData:function(e){return Q.hasData(e)||Y.hasData(e)},data:function(e,t,n){return Q.access(e,t,n)},removeData:function(e,t){Q.remove(e,t)},_data:function(e,t,n){return Y.access(e,t,n)},_removeData:function(e,t){Y.remove(e,t)}}),S.fn.extend({data:function(n,e){var t,r,i,o=this[0],a=o&amp;&amp;o.attributes;if(void 0===n){if(this.length&amp;&amp;(i=Q.get(o),1===o.nodeType&amp;&amp;!Y.get(o,&quot;hasDataAttrs&quot;))){t=a.length;while(t--)a[t]&amp;&amp;0===(r=a[t].name).indexOf(&quot;data-&quot;)&amp;&amp;(r=X(r.slice(5)),Z(o,r,i[r]));Y.set(o,&quot;hasDataAttrs&quot;,!0)}return i}return&quot;object&quot;==typeof n?this.each(function(){Q.set(this,n)}):$(this,function(e){var t;if(o&amp;&amp;void 0===e)return void 0!==(t=Q.get(o,n))?t:void 0!==(t=Z(o,n))?t:void 0;this.each(function(){Q.set(this,n,e)})},null,e,1&lt;arguments.length,null,!0)},removeData:function(e){return this.each(function(){Q.remove(this,e)})}}),S.extend({queue:function(e,t,n){var r;if(e)return t=(t||&quot;fx&quot;)+&quot;queue&quot;,r=Y.get(e,t),n&amp;&amp;(!r||Array.isArray(n)?r=Y.access(e,t,S.makeArray(n)):r.push(n)),r||[]},dequeue:function(e,t){t=t||&quot;fx&quot;;var n=S.queue(e,t),r=n.length,i=n.shift(),o=S._queueHooks(e,t);&quot;inprogress&quot;===i&amp;&amp;(i=n.shift(),r--),i&amp;&amp;(&quot;fx&quot;===t&amp;&amp;n.unshift(&quot;inprogress&quot;),delete o.stop,i.call(e,function(){S.dequeue(e,t)},o)),!r&amp;&amp;o&amp;&amp;o.empty.fire()},_queueHooks:function(e,t){var n=t+&quot;queueHooks&quot;;return Y.get(e,n)||Y.access(e,n,{empty:S.Callbacks(&quot;once memory&quot;).add(function(){Y.remove(e,[t+&quot;queue&quot;,n])})})}}),S.fn.extend({queue:function(t,n){var e=2;return&quot;string&quot;!=typeof t&amp;&amp;(n=t,t=&quot;fx&quot;,e--),arguments.length&lt;e?S.queue(this[0],t):void 0===n?this:this.each(function(){var e=S.queue(this,t,n);S._queueHooks(this,t),&quot;fx&quot;===t&amp;&amp;&quot;inprogress&quot;!==e[0]&amp;&amp;S.dequeue(this,t)})},dequeue:function(e){return this.each(function(){S.dequeue(this,e)})},clearQueue:function(e){return this.queue(e||&quot;fx&quot;,[])},promise:function(e,t){var n,r=1,i=S.Deferred(),o=this,a=this.length,s=function(){--r||i.resolveWith(o,[o])};&quot;string&quot;!=typeof e&amp;&amp;(t=e,e=void 0),e=e||&quot;fx&quot;;while(a--)(n=Y.get(o[a],e+&quot;queueHooks&quot;))&amp;&amp;n.empty&amp;&amp;(r++,n.empty.add(s));return s(),i.promise(t)}});var ee=/[+-]?(?:\d*\.|)\d+(?:[eE][+-]?\d+|)/.source,te=new RegExp(&quot;^(?:([+-])=|)(&quot;+ee+&quot;)([a-z%]*)$&quot;,&quot;i&quot;),ne=[&quot;Top&quot;,&quot;Right&quot;,&quot;Bottom&quot;,&quot;Left&quot;],re=E.documentElement,ie=function(e){return S.contains(e.ownerDocument,e)},oe={composed:!0};re.getRootNode&amp;&amp;(ie=function(e){return S.contains(e.ownerDocument,e)||e.getRootNode(oe)===e.ownerDocument});var ae=function(e,t){return&quot;none&quot;===(e=t||e).style.display||&quot;&quot;===e.style.display&amp;&amp;ie(e)&amp;&amp;&quot;none&quot;===S.css(e,&quot;display&quot;)};function se(e,t,n,r){var i,o,a=20,s=r?function(){return r.cur()}:function(){return S.css(e,t,&quot;&quot;)},u=s(),l=n&amp;&amp;n[3]||(S.cssNumber[t]?&quot;&quot;:&quot;px&quot;),c=e.nodeType&amp;&amp;(S.cssNumber[t]||&quot;px&quot;!==l&amp;&amp;+u)&amp;&amp;te.exec(S.css(e,t));if(c&amp;&amp;c[3]!==l){u/=2,l=l||c[3],c=+u||1;while(a--)S.style(e,t,c+l),(1-o)*(1-(o=s()/u||.5))&lt;=0&amp;&amp;(a=0),c/=o;c*=2,S.style(e,t,c+l),n=n||[]}return n&amp;&amp;(c=+c||+u||0,i=n[1]?c+(n[1]+1)*n[2]:+n[2],r&amp;&amp;(r.unit=l,r.start=c,r.end=i)),i}var ue={};function le(e,t){for(var n,r,i,o,a,s,u,l=[],c=0,f=e.length;c&lt;f;c++)(r=e[c]).style&amp;&amp;(n=r.style.display,t?(&quot;none&quot;===n&amp;&amp;(l[c]=Y.get(r,&quot;display&quot;)||null,l[c]||(r.style.display=&quot;&quot;)),&quot;&quot;===r.style.display&amp;&amp;ae(r)&amp;&amp;(l[c]=(u=a=o=void 0,a=(i=r).ownerDocument,s=i.nodeName,(u=ue[s])||(o=a.body.appendChild(a.createElement(s)),u=S.css(o,&quot;display&quot;),o.parentNode.removeChild(o),&quot;none&quot;===u&amp;&amp;(u=&quot;block&quot;),ue[s]=u)))):&quot;none&quot;!==n&amp;&amp;(l[c]=&quot;none&quot;,Y.set(r,&quot;display&quot;,n)));for(c=0;c&lt;f;c++)null!=l[c]&amp;&amp;(e[c].style.display=l[c]);return e}S.fn.extend({show:function(){return le(this,!0)},hide:function(){return le(this)},toggle:function(e){return&quot;boolean&quot;==typeof e?e?this.show():this.hide():this.each(function(){ae(this)?S(this).show():S(this).hide()})}});var ce,fe,pe=/^(?:checkbox|radio)$/i,de=/&lt;([a-z][^\/\0>\x20\t\r\n\f]*)/i,he=/^$|^module$|\/(?:java|ecma)script/i;ce=E.createDocumentFragment().appendChild(E.createElement(&quot;div&quot;)),(fe=E.createElement(&quot;input&quot;)).setAttribute(&quot;type&quot;,&quot;radio&quot;),fe.setAttribute(&quot;checked&quot;,&quot;checked&quot;),fe.setAttribute(&quot;name&quot;,&quot;t&quot;),ce.appendChild(fe),y.checkClone=ce.cloneNode(!0).cloneNode(!0).lastChild.checked,ce.innerHTML=&quot;&lt;textarea>x&lt;/textarea>&quot;,y.noCloneChecked=!!ce.cloneNode(!0).lastChild.defaultValue,ce.innerHTML=&quot;&lt;option>&lt;/option>&quot;,y.option=!!ce.lastChild;var ge={thead:[1,&quot;&lt;table>&quot;,&quot;&lt;/table>&quot;],col:[2,&quot;&lt;table>&lt;colgroup>&quot;,&quot;&lt;/colgroup>&lt;/table>&quot;],tr:[2,&quot;&lt;table>&lt;tbody>&quot;,&quot;&lt;/tbody>&lt;/table>&quot;],td:[3,&quot;&lt;table>&lt;tbody>&lt;tr>&quot;,&quot;&lt;/tr>&lt;/tbody>&lt;/table>&quot;],_default:[0,&quot;&quot;,&quot;&quot;]};function ve(e,t){var n;return n=&quot;undefined&quot;!=typeof e.getElementsByTagName?e.getElementsByTagName(t||&quot;*&quot;):&quot;undefined&quot;!=typeof e.querySelectorAll?e.querySelectorAll(t||&quot;*&quot;):[],void 0===t||t&amp;&amp;A(e,t)?S.merge([e],n):n}function ye(e,t){for(var n=0,r=e.length;n&lt;r;n++)Y.set(e[n],&quot;globalEval&quot;,!t||Y.get(t[n],&quot;globalEval&quot;))}ge.tbody=ge.tfoot=ge.colgroup=ge.caption=ge.thead,ge.th=ge.td,y.option||(ge.optgroup=ge.option=[1,&quot;&lt;select multiple=&quot; , &quot;'&quot; , &quot;multiple&quot; , &quot;'&quot; , &quot;>&quot;,&quot;&lt;/select>&quot;]);var me=/&lt;|&amp;#?\w+;/;function xe(e,t,n,r,i){for(var o,a,s,u,l,c,f=t.createDocumentFragment(),p=[],d=0,h=e.length;d&lt;h;d++)if((o=e[d])||0===o)if(&quot;object&quot;===w(o))S.merge(p,o.nodeType?[o]:o);else if(me.test(o)){a=a||f.appendChild(t.createElement(&quot;div&quot;)),s=(de.exec(o)||[&quot;&quot;,&quot;&quot;])[1].toLowerCase(),u=ge[s]||ge._default,a.innerHTML=u[1]+S.htmlPrefilter(o)+u[2],c=u[0];while(c--)a=a.lastChild;S.merge(p,a.childNodes),(a=f.firstChild).textContent=&quot;&quot;}else p.push(t.createTextNode(o));f.textContent=&quot;&quot;,d=0;while(o=p[d++])if(r&amp;&amp;-1&lt;S.inArray(o,r))i&amp;&amp;i.push(o);else if(l=ie(o),a=ve(f.appendChild(o),&quot;script&quot;),l&amp;&amp;ye(a),n){c=0;while(o=a[c++])he.test(o.type||&quot;&quot;)&amp;&amp;n.push(o)}return f}var be=/^([^.]*)(?:\.(.+)|)/;function we(){return!0}function Te(){return!1}function Ce(e,t){return e===function(){try{return E.activeElement}catch(e){}}()==(&quot;focus&quot;===t)}function Ee(e,t,n,r,i,o){var a,s;if(&quot;object&quot;==typeof t){for(s in&quot;string&quot;!=typeof n&amp;&amp;(r=r||n,n=void 0),t)Ee(e,s,n,r,t[s],o);return e}if(null==r&amp;&amp;null==i?(i=n,r=n=void 0):null==i&amp;&amp;(&quot;string&quot;==typeof n?(i=r,r=void 0):(i=r,r=n,n=void 0)),!1===i)i=Te;else if(!i)return e;return 1===o&amp;&amp;(a=i,(i=function(e){return S().off(e),a.apply(this,arguments)}).guid=a.guid||(a.guid=S.guid++)),e.each(function(){S.event.add(this,t,i,r,n)})}function Se(e,i,o){o?(Y.set(e,i,!1),S.event.add(e,i,{namespace:!1,handler:function(e){var t,n,r=Y.get(this,i);if(1&amp;e.isTrigger&amp;&amp;this[i]){if(r.length)(S.event.special[i]||{}).delegateType&amp;&amp;e.stopPropagation();else if(r=s.call(arguments),Y.set(this,i,r),t=o(this,i),this[i](),r!==(n=Y.get(this,i))||t?Y.set(this,i,!1):n={},r!==n)return e.stopImmediatePropagation(),e.preventDefault(),n&amp;&amp;n.value}else r.length&amp;&amp;(Y.set(this,i,{value:S.event.trigger(S.extend(r[0],S.Event.prototype),r.slice(1),this)}),e.stopImmediatePropagation())}})):void 0===Y.get(e,i)&amp;&amp;S.event.add(e,i,we)}S.event={global:{},add:function(t,e,n,r,i){var o,a,s,u,l,c,f,p,d,h,g,v=Y.get(t);if(V(t)){n.handler&amp;&amp;(n=(o=n).handler,i=o.selector),i&amp;&amp;S.find.matchesSelector(re,i),n.guid||(n.guid=S.guid++),(u=v.events)||(u=v.events=Object.create(null)),(a=v.handle)||(a=v.handle=function(e){return&quot;undefined&quot;!=typeof S&amp;&amp;S.event.triggered!==e.type?S.event.dispatch.apply(t,arguments):void 0}),l=(e=(e||&quot;&quot;).match(P)||[&quot;&quot;]).length;while(l--)d=g=(s=be.exec(e[l])||[])[1],h=(s[2]||&quot;&quot;).split(&quot;.&quot;).sort(),d&amp;&amp;(f=S.event.special[d]||{},d=(i?f.delegateType:f.bindType)||d,f=S.event.special[d]||{},c=S.extend({type:d,origType:g,data:r,handler:n,guid:n.guid,selector:i,needsContext:i&amp;&amp;S.expr.match.needsContext.test(i),namespace:h.join(&quot;.&quot;)},o),(p=u[d])||((p=u[d]=[]).delegateCount=0,f.setup&amp;&amp;!1!==f.setup.call(t,r,h,a)||t.addEventListener&amp;&amp;t.addEventListener(d,a)),f.add&amp;&amp;(f.add.call(t,c),c.handler.guid||(c.handler.guid=n.guid)),i?p.splice(p.delegateCount++,0,c):p.push(c),S.event.global[d]=!0)}},remove:function(e,t,n,r,i){var o,a,s,u,l,c,f,p,d,h,g,v=Y.hasData(e)&amp;&amp;Y.get(e);if(v&amp;&amp;(u=v.events)){l=(t=(t||&quot;&quot;).match(P)||[&quot;&quot;]).length;while(l--)if(d=g=(s=be.exec(t[l])||[])[1],h=(s[2]||&quot;&quot;).split(&quot;.&quot;).sort(),d){f=S.event.special[d]||{},p=u[d=(r?f.delegateType:f.bindType)||d]||[],s=s[2]&amp;&amp;new RegExp(&quot;(^|\\.)&quot;+h.join(&quot;\\.(?:.*\\.|)&quot;)+&quot;(\\.|$)&quot;),a=o=p.length;while(o--)c=p[o],!i&amp;&amp;g!==c.origType||n&amp;&amp;n.guid!==c.guid||s&amp;&amp;!s.test(c.namespace)||r&amp;&amp;r!==c.selector&amp;&amp;(&quot;**&quot;!==r||!c.selector)||(p.splice(o,1),c.selector&amp;&amp;p.delegateCount--,f.remove&amp;&amp;f.remove.call(e,c));a&amp;&amp;!p.length&amp;&amp;(f.teardown&amp;&amp;!1!==f.teardown.call(e,h,v.handle)||S.removeEvent(e,d,v.handle),delete u[d])}else for(d in u)S.event.remove(e,d+t[l],n,r,!0);S.isEmptyObject(u)&amp;&amp;Y.remove(e,&quot;handle events&quot;)}},dispatch:function(e){var t,n,r,i,o,a,s=new Array(arguments.length),u=S.event.fix(e),l=(Y.get(this,&quot;events&quot;)||Object.create(null))[u.type]||[],c=S.event.special[u.type]||{};for(s[0]=u,t=1;t&lt;arguments.length;t++)s[t]=arguments[t];if(u.delegateTarget=this,!c.preDispatch||!1!==c.preDispatch.call(this,u)){a=S.event.handlers.call(this,u,l),t=0;while((i=a[t++])&amp;&amp;!u.isPropagationStopped()){u.currentTarget=i.elem,n=0;while((o=i.handlers[n++])&amp;&amp;!u.isImmediatePropagationStopped())u.rnamespace&amp;&amp;!1!==o.namespace&amp;&amp;!u.rnamespace.test(o.namespace)||(u.handleObj=o,u.data=o.data,void 0!==(r=((S.event.special[o.origType]||{}).handle||o.handler).apply(i.elem,s))&amp;&amp;!1===(u.result=r)&amp;&amp;(u.preventDefault(),u.stopPropagation()))}return c.postDispatch&amp;&amp;c.postDispatch.call(this,u),u.result}},handlers:function(e,t){var n,r,i,o,a,s=[],u=t.delegateCount,l=e.target;if(u&amp;&amp;l.nodeType&amp;&amp;!(&quot;click&quot;===e.type&amp;&amp;1&lt;=e.button))for(;l!==this;l=l.parentNode||this)if(1===l.nodeType&amp;&amp;(&quot;click&quot;!==e.type||!0!==l.disabled)){for(o=[],a={},n=0;n&lt;u;n++)void 0===a[i=(r=t[n]).selector+&quot; &quot;]&amp;&amp;(a[i]=r.needsContext?-1&lt;S(i,this).index(l):S.find(i,this,null,[l]).length),a[i]&amp;&amp;o.push(r);o.length&amp;&amp;s.push({elem:l,handlers:o})}return l=this,u&lt;t.length&amp;&amp;s.push({elem:l,handlers:t.slice(u)}),s},addProp:function(t,e){Object.defineProperty(S.Event.prototype,t,{enumerable:!0,configurable:!0,get:m(e)?function(){if(this.originalEvent)return e(this.originalEvent)}:function(){if(this.originalEvent)return this.originalEvent[t]},set:function(e){Object.defineProperty(this,t,{enumerable:!0,configurable:!0,writable:!0,value:e})}})},fix:function(e){return e[S.expando]?e:new S.Event(e)},special:{load:{noBubble:!0},click:{setup:function(e){var t=this||e;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Se(t,&quot;click&quot;,we),!1},trigger:function(e){var t=this||e;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Se(t,&quot;click&quot;),!0},_default:function(e){var t=e.target;return pe.test(t.type)&amp;&amp;t.click&amp;&amp;A(t,&quot;input&quot;)&amp;&amp;Y.get(t,&quot;click&quot;)||A(t,&quot;a&quot;)}},beforeunload:{postDispatch:function(e){void 0!==e.result&amp;&amp;e.originalEvent&amp;&amp;(e.originalEvent.returnValue=e.result)}}}},S.removeEvent=function(e,t,n){e.removeEventListener&amp;&amp;e.removeEventListener(t,n)},S.Event=function(e,t){if(!(this instanceof S.Event))return new S.Event(e,t);e&amp;&amp;e.type?(this.originalEvent=e,this.type=e.type,this.isDefaultPrevented=e.defaultPrevented||void 0===e.defaultPrevented&amp;&amp;!1===e.returnValue?we:Te,this.target=e.target&amp;&amp;3===e.target.nodeType?e.target.parentNode:e.target,this.currentTarget=e.currentTarget,this.relatedTarget=e.relatedTarget):this.type=e,t&amp;&amp;S.extend(this,t),this.timeStamp=e&amp;&amp;e.timeStamp||Date.now(),this[S.expando]=!0},S.Event.prototype={constructor:S.Event,isDefaultPrevented:Te,isPropagationStopped:Te,isImmediatePropagationStopped:Te,isSimulated:!1,preventDefault:function(){var e=this.originalEvent;this.isDefaultPrevented=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.preventDefault()},stopPropagation:function(){var e=this.originalEvent;this.isPropagationStopped=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.stopPropagation()},stopImmediatePropagation:function(){var e=this.originalEvent;this.isImmediatePropagationStopped=we,e&amp;&amp;!this.isSimulated&amp;&amp;e.stopImmediatePropagation(),this.stopPropagation()}},S.each({altKey:!0,bubbles:!0,cancelable:!0,changedTouches:!0,ctrlKey:!0,detail:!0,eventPhase:!0,metaKey:!0,pageX:!0,pageY:!0,shiftKey:!0,view:!0,&quot;char&quot;:!0,code:!0,charCode:!0,key:!0,keyCode:!0,button:!0,buttons:!0,clientX:!0,clientY:!0,offsetX:!0,offsetY:!0,pointerId:!0,pointerType:!0,screenX:!0,screenY:!0,targetTouches:!0,toElement:!0,touches:!0,which:!0},S.event.addProp),S.each({focus:&quot;focusin&quot;,blur:&quot;focusout&quot;},function(e,t){S.event.special[e]={setup:function(){return Se(this,e,Ce),!1},trigger:function(){return Se(this,e),!0},_default:function(){return!0},delegateType:t}}),S.each({mouseenter:&quot;mouseover&quot;,mouseleave:&quot;mouseout&quot;,pointerenter:&quot;pointerover&quot;,pointerleave:&quot;pointerout&quot;},function(e,i){S.event.special[e]={delegateType:i,bindType:i,handle:function(e){var t,n=e.relatedTarget,r=e.handleObj;return n&amp;&amp;(n===this||S.contains(this,n))||(e.type=r.origType,t=r.handler.apply(this,arguments),e.type=i),t}}}),S.fn.extend({on:function(e,t,n,r){return Ee(this,e,t,n,r)},one:function(e,t,n,r){return Ee(this,e,t,n,r,1)},off:function(e,t,n){var r,i;if(e&amp;&amp;e.preventDefault&amp;&amp;e.handleObj)return r=e.handleObj,S(e.delegateTarget).off(r.namespace?r.origType+&quot;.&quot;+r.namespace:r.origType,r.selector,r.handler),this;if(&quot;object&quot;==typeof e){for(i in e)this.off(i,t,e[i]);return this}return!1!==t&amp;&amp;&quot;function&quot;!=typeof t||(n=t,t=void 0),!1===n&amp;&amp;(n=Te),this.each(function(){S.event.remove(this,e,n,t)})}});var ke=/&lt;script|&lt;style|&lt;link/i,Ae=/checked\s*(?:[^=]|=\s*.checked.)/i,Ne=/^\s*&lt;!(?:\[CDATA\[|--)|(?:\]\]|--)>\s*$/g;function je(e,t){return A(e,&quot;table&quot;)&amp;&amp;A(11!==t.nodeType?t:t.firstChild,&quot;tr&quot;)&amp;&amp;S(e).children(&quot;tbody&quot;)[0]||e}function De(e){return e.type=(null!==e.getAttribute(&quot;type&quot;))+&quot;/&quot;+e.type,e}function qe(e){return&quot;true/&quot;===(e.type||&quot;&quot;).slice(0,5)?e.type=e.type.slice(5):e.removeAttribute(&quot;type&quot;),e}function Le(e,t){var n,r,i,o,a,s;if(1===t.nodeType){if(Y.hasData(e)&amp;&amp;(s=Y.get(e).events))for(i in Y.remove(t,&quot;handle events&quot;),s)for(n=0,r=s[i].length;n&lt;r;n++)S.event.add(t,i,s[i][n]);Q.hasData(e)&amp;&amp;(o=Q.access(e),a=S.extend({},o),Q.set(t,a))}}function He(n,r,i,o){r=g(r);var e,t,a,s,u,l,c=0,f=n.length,p=f-1,d=r[0],h=m(d);if(h||1&lt;f&amp;&amp;&quot;string&quot;==typeof d&amp;&amp;!y.checkClone&amp;&amp;Ae.test(d))return n.each(function(e){var t=n.eq(e);h&amp;&amp;(r[0]=d.call(this,e,t.html())),He(t,r,i,o)});if(f&amp;&amp;(t=(e=xe(r,n[0].ownerDocument,!1,n,o)).firstChild,1===e.childNodes.length&amp;&amp;(e=t),t||o)){for(s=(a=S.map(ve(e,&quot;script&quot;),De)).length;c&lt;f;c++)u=e,c!==p&amp;&amp;(u=S.clone(u,!0,!0),s&amp;&amp;S.merge(a,ve(u,&quot;script&quot;))),i.call(n[c],u,c);if(s)for(l=a[a.length-1].ownerDocument,S.map(a,qe),c=0;c&lt;s;c++)u=a[c],he.test(u.type||&quot;&quot;)&amp;&amp;!Y.access(u,&quot;globalEval&quot;)&amp;&amp;S.contains(l,u)&amp;&amp;(u.src&amp;&amp;&quot;module&quot;!==(u.type||&quot;&quot;).toLowerCase()?S._evalUrl&amp;&amp;!u.noModule&amp;&amp;S._evalUrl(u.src,{nonce:u.nonce||u.getAttribute(&quot;nonce&quot;)},l):b(u.textContent.replace(Ne,&quot;&quot;),u,l))}return n}function Oe(e,t,n){for(var r,i=t?S.filter(t,e):e,o=0;null!=(r=i[o]);o++)n||1!==r.nodeType||S.cleanData(ve(r)),r.parentNode&amp;&amp;(n&amp;&amp;ie(r)&amp;&amp;ye(ve(r,&quot;script&quot;)),r.parentNode.removeChild(r));return e}S.extend({htmlPrefilter:function(e){return e},clone:function(e,t,n){var r,i,o,a,s,u,l,c=e.cloneNode(!0),f=ie(e);if(!(y.noCloneChecked||1!==e.nodeType&amp;&amp;11!==e.nodeType||S.isXMLDoc(e)))for(a=ve(c),r=0,i=(o=ve(e)).length;r&lt;i;r++)s=o[r],u=a[r],void 0,&quot;input&quot;===(l=u.nodeName.toLowerCase())&amp;&amp;pe.test(s.type)?u.checked=s.checked:&quot;input&quot;!==l&amp;&amp;&quot;textarea&quot;!==l||(u.defaultValue=s.defaultValue);if(t)if(n)for(o=o||ve(e),a=a||ve(c),r=0,i=o.length;r&lt;i;r++)Le(o[r],a[r]);else Le(e,c);return 0&lt;(a=ve(c,&quot;script&quot;)).length&amp;&amp;ye(a,!f&amp;&amp;ve(e,&quot;script&quot;)),c},cleanData:function(e){for(var t,n,r,i=S.event.special,o=0;void 0!==(n=e[o]);o++)if(V(n)){if(t=n[Y.expando]){if(t.events)for(r in t.events)i[r]?S.event.remove(n,r):S.removeEvent(n,r,t.handle);n[Y.expando]=void 0}n[Q.expando]&amp;&amp;(n[Q.expando]=void 0)}}}),S.fn.extend({detach:function(e){return Oe(this,e,!0)},remove:function(e){return Oe(this,e)},text:function(e){return $(this,function(e){return void 0===e?S.text(this):this.empty().each(function(){1!==this.nodeType&amp;&amp;11!==this.nodeType&amp;&amp;9!==this.nodeType||(this.textContent=e)})},null,e,arguments.length)},append:function(){return He(this,arguments,function(e){1!==this.nodeType&amp;&amp;11!==this.nodeType&amp;&amp;9!==this.nodeType||je(this,e).appendChild(e)})},prepend:function(){return He(this,arguments,function(e){if(1===this.nodeType||11===this.nodeType||9===this.nodeType){var t=je(this,e);t.insertBefore(e,t.firstChild)}})},before:function(){return He(this,arguments,function(e){this.parentNode&amp;&amp;this.parentNode.insertBefore(e,this)})},after:function(){return He(this,arguments,function(e){this.parentNode&amp;&amp;this.parentNode.insertBefore(e,this.nextSibling)})},empty:function(){for(var e,t=0;null!=(e=this[t]);t++)1===e.nodeType&amp;&amp;(S.cleanData(ve(e,!1)),e.textContent=&quot;&quot;);return this},clone:function(e,t){return e=null!=e&amp;&amp;e,t=null==t?e:t,this.map(function(){return S.clone(this,e,t)})},html:function(e){return $(this,function(e){var t=this[0]||{},n=0,r=this.length;if(void 0===e&amp;&amp;1===t.nodeType)return t.innerHTML;if(&quot;string&quot;==typeof e&amp;&amp;!ke.test(e)&amp;&amp;!ge[(de.exec(e)||[&quot;&quot;,&quot;&quot;])[1].toLowerCase()]){e=S.htmlPrefilter(e);try{for(;n&lt;r;n++)1===(t=this[n]||{}).nodeType&amp;&amp;(S.cleanData(ve(t,!1)),t.innerHTML=e);t=0}catch(e){}}t&amp;&amp;this.empty().append(e)},null,e,arguments.length)},replaceWith:function(){var n=[];return He(this,arguments,function(e){var t=this.parentNode;S.inArray(this,n)&lt;0&amp;&amp;(S.cleanData(ve(this)),t&amp;&amp;t.replaceChild(e,this))},n)}}),S.each({appendTo:&quot;append&quot;,prependTo:&quot;prepend&quot;,insertBefore:&quot;before&quot;,insertAfter:&quot;after&quot;,replaceAll:&quot;replaceWith&quot;},function(e,a){S.fn[e]=function(e){for(var t,n=[],r=S(e),i=r.length-1,o=0;o&lt;=i;o++)t=o===i?this:this.clone(!0),S(r[o])[a](t),u.apply(n,t.get());return this.pushStack(n)}});var Pe=new RegExp(&quot;^(&quot;+ee+&quot;)(?!px)[a-z%]+$&quot;,&quot;i&quot;),Re=function(e){var t=e.ownerDocument.defaultView;return t&amp;&amp;t.opener||(t=C),t.getComputedStyle(e)},Me=function(e,t,n){var r,i,o={};for(i in t)o[i]=e.style[i],e.style[i]=t[i];for(i in r=n.call(e),t)e.style[i]=o[i];return r},Ie=new RegExp(ne.join(&quot;|&quot;),&quot;i&quot;);function We(e,t,n){var r,i,o,a,s=e.style;return(n=n||Re(e))&amp;&amp;(&quot;&quot;!==(a=n.getPropertyValue(t)||n[t])||ie(e)||(a=S.style(e,t)),!y.pixelBoxStyles()&amp;&amp;Pe.test(a)&amp;&amp;Ie.test(t)&amp;&amp;(r=s.width,i=s.minWidth,o=s.maxWidth,s.minWidth=s.maxWidth=s.width=a,a=n.width,s.width=r,s.minWidth=i,s.maxWidth=o)),void 0!==a?a+&quot;&quot;:a}function Fe(e,t){return{get:function(){if(!e())return(this.get=t).apply(this,arguments);delete this.get}}}!function(){function e(){if(l){u.style.cssText=&quot;position:absolute;left:-11111px;width:60px;margin-top:1px;padding:0;border:0&quot;,l.style.cssText=&quot;position:relative;display:block;box-sizing:border-box;overflow:scroll;margin:auto;border:1px;padding:1px;width:60%;top:1%&quot;,re.appendChild(u).appendChild(l);var e=C.getComputedStyle(l);n=&quot;1%&quot;!==e.top,s=12===t(e.marginLeft),l.style.right=&quot;60%&quot;,o=36===t(e.right),r=36===t(e.width),l.style.position=&quot;absolute&quot;,i=12===t(l.offsetWidth/3),re.removeChild(u),l=null}}function t(e){return Math.round(parseFloat(e))}var n,r,i,o,a,s,u=E.createElement(&quot;div&quot;),l=E.createElement(&quot;div&quot;);l.style&amp;&amp;(l.style.backgroundClip=&quot;content-box&quot;,l.cloneNode(!0).style.backgroundClip=&quot;&quot;,y.clearCloneStyle=&quot;content-box&quot;===l.style.backgroundClip,S.extend(y,{boxSizingReliable:function(){return e(),r},pixelBoxStyles:function(){return e(),o},pixelPosition:function(){return e(),n},reliableMarginLeft:function(){return e(),s},scrollboxSize:function(){return e(),i},reliableTrDimensions:function(){var e,t,n,r;return null==a&amp;&amp;(e=E.createElement(&quot;table&quot;),t=E.createElement(&quot;tr&quot;),n=E.createElement(&quot;div&quot;),e.style.cssText=&quot;position:absolute;left:-11111px;border-collapse:separate&quot;,t.style.cssText=&quot;border:1px solid&quot;,t.style.height=&quot;1px&quot;,n.style.height=&quot;9px&quot;,n.style.display=&quot;block&quot;,re.appendChild(e).appendChild(t).appendChild(n),r=C.getComputedStyle(t),a=parseInt(r.height,10)+parseInt(r.borderTopWidth,10)+parseInt(r.borderBottomWidth,10)===t.offsetHeight,re.removeChild(e)),a}}))}();var Be=[&quot;Webkit&quot;,&quot;Moz&quot;,&quot;ms&quot;],$e=E.createElement(&quot;div&quot;).style,_e={};function ze(e){var t=S.cssProps[e]||_e[e];return t||(e in $e?e:_e[e]=function(e){var t=e[0].toUpperCase()+e.slice(1),n=Be.length;while(n--)if((e=Be[n]+t)in $e)return e}(e)||e)}var Ue=/^(none|table(?!-c[ea]).+)/,Xe=/^--/,Ve={position:&quot;absolute&quot;,visibility:&quot;hidden&quot;,display:&quot;block&quot;},Ge={letterSpacing:&quot;0&quot;,fontWeight:&quot;400&quot;};function Ye(e,t,n){var r=te.exec(t);return r?Math.max(0,r[2]-(n||0))+(r[3]||&quot;px&quot;):t}function Qe(e,t,n,r,i,o){var a=&quot;width&quot;===t?1:0,s=0,u=0;if(n===(r?&quot;border&quot;:&quot;content&quot;))return 0;for(;a&lt;4;a+=2)&quot;margin&quot;===n&amp;&amp;(u+=S.css(e,n+ne[a],!0,i)),r?(&quot;content&quot;===n&amp;&amp;(u-=S.css(e,&quot;padding&quot;+ne[a],!0,i)),&quot;margin&quot;!==n&amp;&amp;(u-=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i))):(u+=S.css(e,&quot;padding&quot;+ne[a],!0,i),&quot;padding&quot;!==n?u+=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i):s+=S.css(e,&quot;border&quot;+ne[a]+&quot;Width&quot;,!0,i));return!r&amp;&amp;0&lt;=o&amp;&amp;(u+=Math.max(0,Math.ceil(e[&quot;offset&quot;+t[0].toUpperCase()+t.slice(1)]-o-u-s-.5))||0),u}function Je(e,t,n){var r=Re(e),i=(!y.boxSizingReliable()||n)&amp;&amp;&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,r),o=i,a=We(e,t,r),s=&quot;offset&quot;+t[0].toUpperCase()+t.slice(1);if(Pe.test(a)){if(!n)return a;a=&quot;auto&quot;}return(!y.boxSizingReliable()&amp;&amp;i||!y.reliableTrDimensions()&amp;&amp;A(e,&quot;tr&quot;)||&quot;auto&quot;===a||!parseFloat(a)&amp;&amp;&quot;inline&quot;===S.css(e,&quot;display&quot;,!1,r))&amp;&amp;e.getClientRects().length&amp;&amp;(i=&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,r),(o=s in e)&amp;&amp;(a=e[s])),(a=parseFloat(a)||0)+Qe(e,t,n||(i?&quot;border&quot;:&quot;content&quot;),o,r,a)+&quot;px&quot;}function Ke(e,t,n,r,i){return new Ke.prototype.init(e,t,n,r,i)}S.extend({cssHooks:{opacity:{get:function(e,t){if(t){var n=We(e,&quot;opacity&quot;);return&quot;&quot;===n?&quot;1&quot;:n}}}},cssNumber:{animationIterationCount:!0,columnCount:!0,fillOpacity:!0,flexGrow:!0,flexShrink:!0,fontWeight:!0,gridArea:!0,gridColumn:!0,gridColumnEnd:!0,gridColumnStart:!0,gridRow:!0,gridRowEnd:!0,gridRowStart:!0,lineHeight:!0,opacity:!0,order:!0,orphans:!0,widows:!0,zIndex:!0,zoom:!0},cssProps:{},style:function(e,t,n,r){if(e&amp;&amp;3!==e.nodeType&amp;&amp;8!==e.nodeType&amp;&amp;e.style){var i,o,a,s=X(t),u=Xe.test(t),l=e.style;if(u||(t=ze(s)),a=S.cssHooks[t]||S.cssHooks[s],void 0===n)return a&amp;&amp;&quot;get&quot;in a&amp;&amp;void 0!==(i=a.get(e,!1,r))?i:l[t];&quot;string&quot;===(o=typeof n)&amp;&amp;(i=te.exec(n))&amp;&amp;i[1]&amp;&amp;(n=se(e,t,i),o=&quot;number&quot;),null!=n&amp;&amp;n==n&amp;&amp;(&quot;number&quot;!==o||u||(n+=i&amp;&amp;i[3]||(S.cssNumber[s]?&quot;&quot;:&quot;px&quot;)),y.clearCloneStyle||&quot;&quot;!==n||0!==t.indexOf(&quot;background&quot;)||(l[t]=&quot;inherit&quot;),a&amp;&amp;&quot;set&quot;in a&amp;&amp;void 0===(n=a.set(e,n,r))||(u?l.setProperty(t,n):l[t]=n))}},css:function(e,t,n,r){var i,o,a,s=X(t);return Xe.test(t)||(t=ze(s)),(a=S.cssHooks[t]||S.cssHooks[s])&amp;&amp;&quot;get&quot;in a&amp;&amp;(i=a.get(e,!0,n)),void 0===i&amp;&amp;(i=We(e,t,r)),&quot;normal&quot;===i&amp;&amp;t in Ge&amp;&amp;(i=Ge[t]),&quot;&quot;===n||n?(o=parseFloat(i),!0===n||isFinite(o)?o||0:i):i}}),S.each([&quot;height&quot;,&quot;width&quot;],function(e,u){S.cssHooks[u]={get:function(e,t,n){if(t)return!Ue.test(S.css(e,&quot;display&quot;))||e.getClientRects().length&amp;&amp;e.getBoundingClientRect().width?Je(e,u,n):Me(e,Ve,function(){return Je(e,u,n)})},set:function(e,t,n){var r,i=Re(e),o=!y.scrollboxSize()&amp;&amp;&quot;absolute&quot;===i.position,a=(o||n)&amp;&amp;&quot;border-box&quot;===S.css(e,&quot;boxSizing&quot;,!1,i),s=n?Qe(e,u,n,a,i):0;return a&amp;&amp;o&amp;&amp;(s-=Math.ceil(e[&quot;offset&quot;+u[0].toUpperCase()+u.slice(1)]-parseFloat(i[u])-Qe(e,u,&quot;border&quot;,!1,i)-.5)),s&amp;&amp;(r=te.exec(t))&amp;&amp;&quot;px&quot;!==(r[3]||&quot;px&quot;)&amp;&amp;(e.style[u]=t,t=S.css(e,u)),Ye(0,t,s)}}}),S.cssHooks.marginLeft=Fe(y.reliableMarginLeft,function(e,t){if(t)return(parseFloat(We(e,&quot;marginLeft&quot;))||e.getBoundingClientRect().left-Me(e,{marginLeft:0},function(){return e.getBoundingClientRect().left}))+&quot;px&quot;}),S.each({margin:&quot;&quot;,padding:&quot;&quot;,border:&quot;Width&quot;},function(i,o){S.cssHooks[i+o]={expand:function(e){for(var t=0,n={},r=&quot;string&quot;==typeof e?e.split(&quot; &quot;):[e];t&lt;4;t++)n[i+ne[t]+o]=r[t]||r[t-2]||r[0];return n}},&quot;margin&quot;!==i&amp;&amp;(S.cssHooks[i+o].set=Ye)}),S.fn.extend({css:function(e,t){return $(this,function(e,t,n){var r,i,o={},a=0;if(Array.isArray(t)){for(r=Re(e),i=t.length;a&lt;i;a++)o[t[a]]=S.css(e,t[a],!1,r);return o}return void 0!==n?S.style(e,t,n):S.css(e,t)},e,t,1&lt;arguments.length)}}),((S.Tween=Ke).prototype={constructor:Ke,init:function(e,t,n,r,i,o){this.elem=e,this.prop=n,this.easing=i||S.easing._default,this.options=t,this.start=this.now=this.cur(),this.end=r,this.unit=o||(S.cssNumber[n]?&quot;&quot;:&quot;px&quot;)},cur:function(){var e=Ke.propHooks[this.prop];return e&amp;&amp;e.get?e.get(this):Ke.propHooks._default.get(this)},run:function(e){var t,n=Ke.propHooks[this.prop];return this.options.duration?this.pos=t=S.easing[this.easing](e,this.options.duration*e,0,1,this.options.duration):this.pos=t=e,this.now=(this.end-this.start)*t+this.start,this.options.step&amp;&amp;this.options.step.call(this.elem,this.now,this),n&amp;&amp;n.set?n.set(this):Ke.propHooks._default.set(this),this}}).init.prototype=Ke.prototype,(Ke.propHooks={_default:{get:function(e){var t;return 1!==e.elem.nodeType||null!=e.elem[e.prop]&amp;&amp;null==e.elem.style[e.prop]?e.elem[e.prop]:(t=S.css(e.elem,e.prop,&quot;&quot;))&amp;&amp;&quot;auto&quot;!==t?t:0},set:function(e){S.fx.step[e.prop]?S.fx.step[e.prop](e):1!==e.elem.nodeType||!S.cssHooks[e.prop]&amp;&amp;null==e.elem.style[ze(e.prop)]?e.elem[e.prop]=e.now:S.style(e.elem,e.prop,e.now+e.unit)}}}).scrollTop=Ke.propHooks.scrollLeft={set:function(e){e.elem.nodeType&amp;&amp;e.elem.parentNode&amp;&amp;(e.elem[e.prop]=e.now)}},S.easing={linear:function(e){return e},swing:function(e){return.5-Math.cos(e*Math.PI)/2},_default:&quot;swing&quot;},S.fx=Ke.prototype.init,S.fx.step={};var Ze,et,tt,nt,rt=/^(?:toggle|show|hide)$/,it=/queueHooks$/;function ot(){et&amp;&amp;(!1===E.hidden&amp;&amp;C.requestAnimationFrame?C.requestAnimationFrame(ot):C.setTimeout(ot,S.fx.interval),S.fx.tick())}function at(){return C.setTimeout(function(){Ze=void 0}),Ze=Date.now()}function st(e,t){var n,r=0,i={height:e};for(t=t?1:0;r&lt;4;r+=2-t)i[&quot;margin&quot;+(n=ne[r])]=i[&quot;padding&quot;+n]=e;return t&amp;&amp;(i.opacity=i.width=e),i}function ut(e,t,n){for(var r,i=(lt.tweeners[t]||[]).concat(lt.tweeners[&quot;*&quot;]),o=0,a=i.length;o&lt;a;o++)if(r=i[o].call(n,t,e))return r}function lt(o,e,t){var n,a,r=0,i=lt.prefilters.length,s=S.Deferred().always(function(){delete u.elem}),u=function(){if(a)return!1;for(var e=Ze||at(),t=Math.max(0,l.startTime+l.duration-e),n=1-(t/l.duration||0),r=0,i=l.tweens.length;r&lt;i;r++)l.tweens[r].run(n);return s.notifyWith(o,[l,n,t]),n&lt;1&amp;&amp;i?t:(i||s.notifyWith(o,[l,1,0]),s.resolveWith(o,[l]),!1)},l=s.promise({elem:o,props:S.extend({},e),opts:S.extend(!0,{specialEasing:{},easing:S.easing._default},t),originalProperties:e,originalOptions:t,startTime:Ze||at(),duration:t.duration,tweens:[],createTween:function(e,t){var n=S.Tween(o,l.opts,e,t,l.opts.specialEasing[e]||l.opts.easing);return l.tweens.push(n),n},stop:function(e){var t=0,n=e?l.tweens.length:0;if(a)return this;for(a=!0;t&lt;n;t++)l.tweens[t].run(1);return e?(s.notifyWith(o,[l,1,0]),s.resolveWith(o,[l,e])):s.rejectWith(o,[l,e]),this}}),c=l.props;for(!function(e,t){var n,r,i,o,a;for(n in e)if(i=t[r=X(n)],o=e[n],Array.isArray(o)&amp;&amp;(i=o[1],o=e[n]=o[0]),n!==r&amp;&amp;(e[r]=o,delete e[n]),(a=S.cssHooks[r])&amp;&amp;&quot;expand&quot;in a)for(n in o=a.expand(o),delete e[r],o)n in e||(e[n]=o[n],t[n]=i);else t[r]=i}(c,l.opts.specialEasing);r&lt;i;r++)if(n=lt.prefilters[r].call(l,o,c,l.opts))return m(n.stop)&amp;&amp;(S._queueHooks(l.elem,l.opts.queue).stop=n.stop.bind(n)),n;return S.map(c,ut,l),m(l.opts.start)&amp;&amp;l.opts.start.call(o,l),l.progress(l.opts.progress).done(l.opts.done,l.opts.complete).fail(l.opts.fail).always(l.opts.always),S.fx.timer(S.extend(u,{elem:o,anim:l,queue:l.opts.queue})),l}S.Animation=S.extend(lt,{tweeners:{&quot;*&quot;:[function(e,t){var n=this.createTween(e,t);return se(n.elem,e,te.exec(t),n),n}]},tweener:function(e,t){m(e)?(t=e,e=[&quot;*&quot;]):e=e.match(P);for(var n,r=0,i=e.length;r&lt;i;r++)n=e[r],lt.tweeners[n]=lt.tweeners[n]||[],lt.tweeners[n].unshift(t)},prefilters:[function(e,t,n){var r,i,o,a,s,u,l,c,f=&quot;width&quot;in t||&quot;height&quot;in t,p=this,d={},h=e.style,g=e.nodeType&amp;&amp;ae(e),v=Y.get(e,&quot;fxshow&quot;);for(r in n.queue||(null==(a=S._queueHooks(e,&quot;fx&quot;)).unqueued&amp;&amp;(a.unqueued=0,s=a.empty.fire,a.empty.fire=function(){a.unqueued||s()}),a.unqueued++,p.always(function(){p.always(function(){a.unqueued--,S.queue(e,&quot;fx&quot;).length||a.empty.fire()})})),t)if(i=t[r],rt.test(i)){if(delete t[r],o=o||&quot;toggle&quot;===i,i===(g?&quot;hide&quot;:&quot;show&quot;)){if(&quot;show&quot;!==i||!v||void 0===v[r])continue;g=!0}d[r]=v&amp;&amp;v[r]||S.style(e,r)}if((u=!S.isEmptyObject(t))||!S.isEmptyObject(d))for(r in f&amp;&amp;1===e.nodeType&amp;&amp;(n.overflow=[h.overflow,h.overflowX,h.overflowY],null==(l=v&amp;&amp;v.display)&amp;&amp;(l=Y.get(e,&quot;display&quot;)),&quot;none&quot;===(c=S.css(e,&quot;display&quot;))&amp;&amp;(l?c=l:(le([e],!0),l=e.style.display||l,c=S.css(e,&quot;display&quot;),le([e]))),(&quot;inline&quot;===c||&quot;inline-block&quot;===c&amp;&amp;null!=l)&amp;&amp;&quot;none&quot;===S.css(e,&quot;float&quot;)&amp;&amp;(u||(p.done(function(){h.display=l}),null==l&amp;&amp;(c=h.display,l=&quot;none&quot;===c?&quot;&quot;:c)),h.display=&quot;inline-block&quot;)),n.overflow&amp;&amp;(h.overflow=&quot;hidden&quot;,p.always(function(){h.overflow=n.overflow[0],h.overflowX=n.overflow[1],h.overflowY=n.overflow[2]})),u=!1,d)u||(v?&quot;hidden&quot;in v&amp;&amp;(g=v.hidden):v=Y.access(e,&quot;fxshow&quot;,{display:l}),o&amp;&amp;(v.hidden=!g),g&amp;&amp;le([e],!0),p.done(function(){for(r in g||le([e]),Y.remove(e,&quot;fxshow&quot;),d)S.style(e,r,d[r])})),u=ut(g?v[r]:0,r,p),r in v||(v[r]=u.start,g&amp;&amp;(u.end=u.start,u.start=0))}],prefilter:function(e,t){t?lt.prefilters.unshift(e):lt.prefilters.push(e)}}),S.speed=function(e,t,n){var r=e&amp;&amp;&quot;object&quot;==typeof e?S.extend({},e):{complete:n||!n&amp;&amp;t||m(e)&amp;&amp;e,duration:e,easing:n&amp;&amp;t||t&amp;&amp;!m(t)&amp;&amp;t};return S.fx.off?r.duration=0:&quot;number&quot;!=typeof r.duration&amp;&amp;(r.duration in S.fx.speeds?r.duration=S.fx.speeds[r.duration]:r.duration=S.fx.speeds._default),null!=r.queue&amp;&amp;!0!==r.queue||(r.queue=&quot;fx&quot;),r.old=r.complete,r.complete=function(){m(r.old)&amp;&amp;r.old.call(this),r.queue&amp;&amp;S.dequeue(this,r.queue)},r},S.fn.extend({fadeTo:function(e,t,n,r){return this.filter(ae).css(&quot;opacity&quot;,0).show().end().animate({opacity:t},e,n,r)},animate:function(t,e,n,r){var i=S.isEmptyObject(t),o=S.speed(e,n,r),a=function(){var e=lt(this,S.extend({},t),o);(i||Y.get(this,&quot;finish&quot;))&amp;&amp;e.stop(!0)};return a.finish=a,i||!1===o.queue?this.each(a):this.queue(o.queue,a)},stop:function(i,e,o){var a=function(e){var t=e.stop;delete e.stop,t(o)};return&quot;string&quot;!=typeof i&amp;&amp;(o=e,e=i,i=void 0),e&amp;&amp;this.queue(i||&quot;fx&quot;,[]),this.each(function(){var e=!0,t=null!=i&amp;&amp;i+&quot;queueHooks&quot;,n=S.timers,r=Y.get(this);if(t)r[t]&amp;&amp;r[t].stop&amp;&amp;a(r[t]);else for(t in r)r[t]&amp;&amp;r[t].stop&amp;&amp;it.test(t)&amp;&amp;a(r[t]);for(t=n.length;t--;)n[t].elem!==this||null!=i&amp;&amp;n[t].queue!==i||(n[t].anim.stop(o),e=!1,n.splice(t,1));!e&amp;&amp;o||S.dequeue(this,i)})},finish:function(a){return!1!==a&amp;&amp;(a=a||&quot;fx&quot;),this.each(function(){var e,t=Y.get(this),n=t[a+&quot;queue&quot;],r=t[a+&quot;queueHooks&quot;],i=S.timers,o=n?n.length:0;for(t.finish=!0,S.queue(this,a,[]),r&amp;&amp;r.stop&amp;&amp;r.stop.call(this,!0),e=i.length;e--;)i[e].elem===this&amp;&amp;i[e].queue===a&amp;&amp;(i[e].anim.stop(!0),i.splice(e,1));for(e=0;e&lt;o;e++)n[e]&amp;&amp;n[e].finish&amp;&amp;n[e].finish.call(this);delete t.finish})}}),S.each([&quot;toggle&quot;,&quot;show&quot;,&quot;hide&quot;],function(e,r){var i=S.fn[r];S.fn[r]=function(e,t,n){return null==e||&quot;boolean&quot;==typeof e?i.apply(this,arguments):this.animate(st(r,!0),e,t,n)}}),S.each({slideDown:st(&quot;show&quot;),slideUp:st(&quot;hide&quot;),slideToggle:st(&quot;toggle&quot;),fadeIn:{opacity:&quot;show&quot;},fadeOut:{opacity:&quot;hide&quot;},fadeToggle:{opacity:&quot;toggle&quot;}},function(e,r){S.fn[e]=function(e,t,n){return this.animate(r,e,t,n)}}),S.timers=[],S.fx.tick=function(){var e,t=0,n=S.timers;for(Ze=Date.now();t&lt;n.length;t++)(e=n[t])()||n[t]!==e||n.splice(t--,1);n.length||S.fx.stop(),Ze=void 0},S.fx.timer=function(e){S.timers.push(e),S.fx.start()},S.fx.interval=13,S.fx.start=function(){et||(et=!0,ot())},S.fx.stop=function(){et=null},S.fx.speeds={slow:600,fast:200,_default:400},S.fn.delay=function(r,e){return r=S.fx&amp;&amp;S.fx.speeds[r]||r,e=e||&quot;fx&quot;,this.queue(e,function(e,t){var n=C.setTimeout(e,r);t.stop=function(){C.clearTimeout(n)}})},tt=E.createElement(&quot;input&quot;),nt=E.createElement(&quot;select&quot;).appendChild(E.createElement(&quot;option&quot;)),tt.type=&quot;checkbox&quot;,y.checkOn=&quot;&quot;!==tt.value,y.optSelected=nt.selected,(tt=E.createElement(&quot;input&quot;)).value=&quot;t&quot;,tt.type=&quot;radio&quot;,y.radioValue=&quot;t&quot;===tt.value;var ct,ft=S.expr.attrHandle;S.fn.extend({attr:function(e,t){return $(this,S.attr,e,t,1&lt;arguments.length)},removeAttr:function(e){return this.each(function(){S.removeAttr(this,e)})}}),S.extend({attr:function(e,t,n){var r,i,o=e.nodeType;if(3!==o&amp;&amp;8!==o&amp;&amp;2!==o)return&quot;undefined&quot;==typeof e.getAttribute?S.prop(e,t,n):(1===o&amp;&amp;S.isXMLDoc(e)||(i=S.attrHooks[t.toLowerCase()]||(S.expr.match.bool.test(t)?ct:void 0)),void 0!==n?null===n?void S.removeAttr(e,t):i&amp;&amp;&quot;set&quot;in i&amp;&amp;void 0!==(r=i.set(e,n,t))?r:(e.setAttribute(t,n+&quot;&quot;),n):i&amp;&amp;&quot;get&quot;in i&amp;&amp;null!==(r=i.get(e,t))?r:null==(r=S.find.attr(e,t))?void 0:r)},attrHooks:{type:{set:function(e,t){if(!y.radioValue&amp;&amp;&quot;radio&quot;===t&amp;&amp;A(e,&quot;input&quot;)){var n=e.value;return e.setAttribute(&quot;type&quot;,t),n&amp;&amp;(e.value=n),t}}}},removeAttr:function(e,t){var n,r=0,i=t&amp;&amp;t.match(P);if(i&amp;&amp;1===e.nodeType)while(n=i[r++])e.removeAttribute(n)}}),ct={set:function(e,t,n){return!1===t?S.removeAttr(e,n):e.setAttribute(n,n),n}},S.each(S.expr.match.bool.source.match(/\w+/g),function(e,t){var a=ft[t]||S.find.attr;ft[t]=function(e,t,n){var r,i,o=t.toLowerCase();return n||(i=ft[o],ft[o]=r,r=null!=a(e,t,n)?o:null,ft[o]=i),r}});var pt=/^(?:input|select|textarea|button)$/i,dt=/^(?:a|area)$/i;function ht(e){return(e.match(P)||[]).join(&quot; &quot;)}function gt(e){return e.getAttribute&amp;&amp;e.getAttribute(&quot;class&quot;)||&quot;&quot;}function vt(e){return Array.isArray(e)?e:&quot;string&quot;==typeof e&amp;&amp;e.match(P)||[]}S.fn.extend({prop:function(e,t){return $(this,S.prop,e,t,1&lt;arguments.length)},removeProp:function(e){return this.each(function(){delete this[S.propFix[e]||e]})}}),S.extend({prop:function(e,t,n){var r,i,o=e.nodeType;if(3!==o&amp;&amp;8!==o&amp;&amp;2!==o)return 1===o&amp;&amp;S.isXMLDoc(e)||(t=S.propFix[t]||t,i=S.propHooks[t]),void 0!==n?i&amp;&amp;&quot;set&quot;in i&amp;&amp;void 0!==(r=i.set(e,n,t))?r:e[t]=n:i&amp;&amp;&quot;get&quot;in i&amp;&amp;null!==(r=i.get(e,t))?r:e[t]},propHooks:{tabIndex:{get:function(e){var t=S.find.attr(e,&quot;tabindex&quot;);return t?parseInt(t,10):pt.test(e.nodeName)||dt.test(e.nodeName)&amp;&amp;e.href?0:-1}}},propFix:{&quot;for&quot;:&quot;htmlFor&quot;,&quot;class&quot;:&quot;className&quot;}}),y.optSelected||(S.propHooks.selected={get:function(e){var t=e.parentNode;return t&amp;&amp;t.parentNode&amp;&amp;t.parentNode.selectedIndex,null},set:function(e){var t=e.parentNode;t&amp;&amp;(t.selectedIndex,t.parentNode&amp;&amp;t.parentNode.selectedIndex)}}),S.each([&quot;tabIndex&quot;,&quot;readOnly&quot;,&quot;maxLength&quot;,&quot;cellSpacing&quot;,&quot;cellPadding&quot;,&quot;rowSpan&quot;,&quot;colSpan&quot;,&quot;useMap&quot;,&quot;frameBorder&quot;,&quot;contentEditable&quot;],function(){S.propFix[this.toLowerCase()]=this}),S.fn.extend({addClass:function(t){var e,n,r,i,o,a,s,u=0;if(m(t))return this.each(function(e){S(this).addClass(t.call(this,e,gt(this)))});if((e=vt(t)).length)while(n=this[u++])if(i=gt(n),r=1===n.nodeType&amp;&amp;&quot; &quot;+ht(i)+&quot; &quot;){a=0;while(o=e[a++])r.indexOf(&quot; &quot;+o+&quot; &quot;)&lt;0&amp;&amp;(r+=o+&quot; &quot;);i!==(s=ht(r))&amp;&amp;n.setAttribute(&quot;class&quot;,s)}return this},removeClass:function(t){var e,n,r,i,o,a,s,u=0;if(m(t))return this.each(function(e){S(this).removeClass(t.call(this,e,gt(this)))});if(!arguments.length)return this.attr(&quot;class&quot;,&quot;&quot;);if((e=vt(t)).length)while(n=this[u++])if(i=gt(n),r=1===n.nodeType&amp;&amp;&quot; &quot;+ht(i)+&quot; &quot;){a=0;while(o=e[a++])while(-1&lt;r.indexOf(&quot; &quot;+o+&quot; &quot;))r=r.replace(&quot; &quot;+o+&quot; &quot;,&quot; &quot;);i!==(s=ht(r))&amp;&amp;n.setAttribute(&quot;class&quot;,s)}return this},toggleClass:function(i,t){var o=typeof i,a=&quot;string&quot;===o||Array.isArray(i);return&quot;boolean&quot;==typeof t&amp;&amp;a?t?this.addClass(i):this.removeClass(i):m(i)?this.each(function(e){S(this).toggleClass(i.call(this,e,gt(this),t),t)}):this.each(function(){var e,t,n,r;if(a){t=0,n=S(this),r=vt(i);while(e=r[t++])n.hasClass(e)?n.removeClass(e):n.addClass(e)}else void 0!==i&amp;&amp;&quot;boolean&quot;!==o||((e=gt(this))&amp;&amp;Y.set(this,&quot;__className__&quot;,e),this.setAttribute&amp;&amp;this.setAttribute(&quot;class&quot;,e||!1===i?&quot;&quot;:Y.get(this,&quot;__className__&quot;)||&quot;&quot;))})},hasClass:function(e){var t,n,r=0;t=&quot; &quot;+e+&quot; &quot;;while(n=this[r++])if(1===n.nodeType&amp;&amp;-1&lt;(&quot; &quot;+ht(gt(n))+&quot; &quot;).indexOf(t))return!0;return!1}});var yt=/\r/g;S.fn.extend({val:function(n){var r,e,i,t=this[0];return arguments.length?(i=m(n),this.each(function(e){var t;1===this.nodeType&amp;&amp;(null==(t=i?n.call(this,e,S(this).val()):n)?t=&quot;&quot;:&quot;number&quot;==typeof t?t+=&quot;&quot;:Array.isArray(t)&amp;&amp;(t=S.map(t,function(e){return null==e?&quot;&quot;:e+&quot;&quot;})),(r=S.valHooks[this.type]||S.valHooks[this.nodeName.toLowerCase()])&amp;&amp;&quot;set&quot;in r&amp;&amp;void 0!==r.set(this,t,&quot;value&quot;)||(this.value=t))})):t?(r=S.valHooks[t.type]||S.valHooks[t.nodeName.toLowerCase()])&amp;&amp;&quot;get&quot;in r&amp;&amp;void 0!==(e=r.get(t,&quot;value&quot;))?e:&quot;string&quot;==typeof(e=t.value)?e.replace(yt,&quot;&quot;):null==e?&quot;&quot;:e:void 0}}),S.extend({valHooks:{option:{get:function(e){var t=S.find.attr(e,&quot;value&quot;);return null!=t?t:ht(S.text(e))}},select:{get:function(e){var t,n,r,i=e.options,o=e.selectedIndex,a=&quot;select-one&quot;===e.type,s=a?null:[],u=a?o+1:i.length;for(r=o&lt;0?u:a?o:0;r&lt;u;r++)if(((n=i[r]).selected||r===o)&amp;&amp;!n.disabled&amp;&amp;(!n.parentNode.disabled||!A(n.parentNode,&quot;optgroup&quot;))){if(t=S(n).val(),a)return t;s.push(t)}return s},set:function(e,t){var n,r,i=e.options,o=S.makeArray(t),a=i.length;while(a--)((r=i[a]).selected=-1&lt;S.inArray(S.valHooks.option.get(r),o))&amp;&amp;(n=!0);return n||(e.selectedIndex=-1),o}}}}),S.each([&quot;radio&quot;,&quot;checkbox&quot;],function(){S.valHooks[this]={set:function(e,t){if(Array.isArray(t))return e.checked=-1&lt;S.inArray(S(e).val(),t)}},y.checkOn||(S.valHooks[this].get=function(e){return null===e.getAttribute(&quot;value&quot;)?&quot;on&quot;:e.value})}),y.focusin=&quot;onfocusin&quot;in C;var mt=/^(?:focusinfocus|focusoutblur)$/,xt=function(e){e.stopPropagation()};S.extend(S.event,{trigger:function(e,t,n,r){var i,o,a,s,u,l,c,f,p=[n||E],d=v.call(e,&quot;type&quot;)?e.type:e,h=v.call(e,&quot;namespace&quot;)?e.namespace.split(&quot;.&quot;):[];if(o=f=a=n=n||E,3!==n.nodeType&amp;&amp;8!==n.nodeType&amp;&amp;!mt.test(d+S.event.triggered)&amp;&amp;(-1&lt;d.indexOf(&quot;.&quot;)&amp;&amp;(d=(h=d.split(&quot;.&quot;)).shift(),h.sort()),u=d.indexOf(&quot;:&quot;)&lt;0&amp;&amp;&quot;on&quot;+d,(e=e[S.expando]?e:new S.Event(d,&quot;object&quot;==typeof e&amp;&amp;e)).isTrigger=r?2:3,e.namespace=h.join(&quot;.&quot;),e.rnamespace=e.namespace?new RegExp(&quot;(^|\\.)&quot;+h.join(&quot;\\.(?:.*\\.|)&quot;)+&quot;(\\.|$)&quot;):null,e.result=void 0,e.target||(e.target=n),t=null==t?[e]:S.makeArray(t,[e]),c=S.event.special[d]||{},r||!c.trigger||!1!==c.trigger.apply(n,t))){if(!r&amp;&amp;!c.noBubble&amp;&amp;!x(n)){for(s=c.delegateType||d,mt.test(s+d)||(o=o.parentNode);o;o=o.parentNode)p.push(o),a=o;a===(n.ownerDocument||E)&amp;&amp;p.push(a.defaultView||a.parentWindow||C)}i=0;while((o=p[i++])&amp;&amp;!e.isPropagationStopped())f=o,e.type=1&lt;i?s:c.bindType||d,(l=(Y.get(o,&quot;events&quot;)||Object.create(null))[e.type]&amp;&amp;Y.get(o,&quot;handle&quot;))&amp;&amp;l.apply(o,t),(l=u&amp;&amp;o[u])&amp;&amp;l.apply&amp;&amp;V(o)&amp;&amp;(e.result=l.apply(o,t),!1===e.result&amp;&amp;e.preventDefault());return e.type=d,r||e.isDefaultPrevented()||c._default&amp;&amp;!1!==c._default.apply(p.pop(),t)||!V(n)||u&amp;&amp;m(n[d])&amp;&amp;!x(n)&amp;&amp;((a=n[u])&amp;&amp;(n[u]=null),S.event.triggered=d,e.isPropagationStopped()&amp;&amp;f.addEventListener(d,xt),n[d](),e.isPropagationStopped()&amp;&amp;f.removeEventListener(d,xt),S.event.triggered=void 0,a&amp;&amp;(n[u]=a)),e.result}},simulate:function(e,t,n){var r=S.extend(new S.Event,n,{type:e,isSimulated:!0});S.event.trigger(r,null,t)}}),S.fn.extend({trigger:function(e,t){return this.each(function(){S.event.trigger(e,t,this)})},triggerHandler:function(e,t){var n=this[0];if(n)return S.event.trigger(e,t,n,!0)}}),y.focusin||S.each({focus:&quot;focusin&quot;,blur:&quot;focusout&quot;},function(n,r){var i=function(e){S.event.simulate(r,e.target,S.event.fix(e))};S.event.special[r]={setup:function(){var e=this.ownerDocument||this.document||this,t=Y.access(e,r);t||e.addEventListener(n,i,!0),Y.access(e,r,(t||0)+1)},teardown:function(){var e=this.ownerDocument||this.document||this,t=Y.access(e,r)-1;t?Y.access(e,r,t):(e.removeEventListener(n,i,!0),Y.remove(e,r))}}});var bt=C.location,wt={guid:Date.now()},Tt=/\?/;S.parseXML=function(e){var t,n;if(!e||&quot;string&quot;!=typeof e)return null;try{t=(new C.DOMParser).parseFromString(e,&quot;text/xml&quot;)}catch(e){}return n=t&amp;&amp;t.getElementsByTagName(&quot;parsererror&quot;)[0],t&amp;&amp;!n||S.error(&quot;Invalid XML: &quot;+(n?S.map(n.childNodes,function(e){return e.textContent}).join(&quot;\n&quot;):e)),t};var Ct=/\[\]$/,Et=/\r?\n/g,St=/^(?:submit|button|image|reset|file)$/i,kt=/^(?:input|select|textarea|keygen)/i;function At(n,e,r,i){var t;if(Array.isArray(e))S.each(e,function(e,t){r||Ct.test(n)?i(n,t):At(n+&quot;[&quot;+(&quot;object&quot;==typeof t&amp;&amp;null!=t?e:&quot;&quot;)+&quot;]&quot;,t,r,i)});else if(r||&quot;object&quot;!==w(e))i(n,e);else for(t in e)At(n+&quot;[&quot;+t+&quot;]&quot;,e[t],r,i)}S.param=function(e,t){var n,r=[],i=function(e,t){var n=m(t)?t():t;r[r.length]=encodeURIComponent(e)+&quot;=&quot;+encodeURIComponent(null==n?&quot;&quot;:n)};if(null==e)return&quot;&quot;;if(Array.isArray(e)||e.jquery&amp;&amp;!S.isPlainObject(e))S.each(e,function(){i(this.name,this.value)});else for(n in e)At(n,e[n],t,i);return r.join(&quot;&amp;&quot;)},S.fn.extend({serialize:function(){return S.param(this.serializeArray())},serializeArray:function(){return this.map(function(){var e=S.prop(this,&quot;elements&quot;);return e?S.makeArray(e):this}).filter(function(){var e=this.type;return this.name&amp;&amp;!S(this).is(&quot;:disabled&quot;)&amp;&amp;kt.test(this.nodeName)&amp;&amp;!St.test(e)&amp;&amp;(this.checked||!pe.test(e))}).map(function(e,t){var n=S(this).val();return null==n?null:Array.isArray(n)?S.map(n,function(e){return{name:t.name,value:e.replace(Et,&quot;\r\n&quot;)}}):{name:t.name,value:n.replace(Et,&quot;\r\n&quot;)}}).get()}});var Nt=/%20/g,jt=/#.*$/,Dt=/([?&amp;])_=[^&amp;]*/,qt=/^(.*?):[ \t]*([^\r\n]*)$/gm,Lt=/^(?:GET|HEAD)$/,Ht=/^\/\//,Ot={},Pt={},Rt=&quot;*/&quot;.concat(&quot;*&quot;),Mt=E.createElement(&quot;a&quot;);function It(o){return function(e,t){&quot;string&quot;!=typeof e&amp;&amp;(t=e,e=&quot;*&quot;);var n,r=0,i=e.toLowerCase().match(P)||[];if(m(t))while(n=i[r++])&quot;+&quot;===n[0]?(n=n.slice(1)||&quot;*&quot;,(o[n]=o[n]||[]).unshift(t)):(o[n]=o[n]||[]).push(t)}}function Wt(t,i,o,a){var s={},u=t===Pt;function l(e){var r;return s[e]=!0,S.each(t[e]||[],function(e,t){var n=t(i,o,a);return&quot;string&quot;!=typeof n||u||s[n]?u?!(r=n):void 0:(i.dataTypes.unshift(n),l(n),!1)}),r}return l(i.dataTypes[0])||!s[&quot;*&quot;]&amp;&amp;l(&quot;*&quot;)}function Ft(e,t){var n,r,i=S.ajaxSettings.flatOptions||{};for(n in t)void 0!==t[n]&amp;&amp;((i[n]?e:r||(r={}))[n]=t[n]);return r&amp;&amp;S.extend(!0,e,r),e}Mt.href=bt.href,S.extend({active:0,lastModified:{},etag:{},ajaxSettings:{url:bt.href,type:&quot;GET&quot;,isLocal:/^(?:about|app|app-storage|.+-extension|file|res|widget):$/.test(bt.protocol),global:!0,processData:!0,async:!0,contentType:&quot;application/x-www-form-urlencoded; charset=UTF-8&quot;,accepts:{&quot;*&quot;:Rt,text:&quot;text/plain&quot;,html:&quot;text/html&quot;,xml:&quot;application/xml, text/xml&quot;,json:&quot;application/json, text/javascript&quot;},contents:{xml:/\bxml\b/,html:/\bhtml/,json:/\bjson\b/},responseFields:{xml:&quot;responseXML&quot;,text:&quot;responseText&quot;,json:&quot;responseJSON&quot;},converters:{&quot;* text&quot;:String,&quot;text html&quot;:!0,&quot;text json&quot;:JSON.parse,&quot;text xml&quot;:S.parseXML},flatOptions:{url:!0,context:!0}},ajaxSetup:function(e,t){return t?Ft(Ft(e,S.ajaxSettings),t):Ft(S.ajaxSettings,e)},ajaxPrefilter:It(Ot),ajaxTransport:It(Pt),ajax:function(e,t){&quot;object&quot;==typeof e&amp;&amp;(t=e,e=void 0),t=t||{};var c,f,p,n,d,r,h,g,i,o,v=S.ajaxSetup({},t),y=v.context||v,m=v.context&amp;&amp;(y.nodeType||y.jquery)?S(y):S.event,x=S.Deferred(),b=S.Callbacks(&quot;once memory&quot;),w=v.statusCode||{},a={},s={},u=&quot;canceled&quot;,T={readyState:0,getResponseHeader:function(e){var t;if(h){if(!n){n={};while(t=qt.exec(p))n[t[1].toLowerCase()+&quot; &quot;]=(n[t[1].toLowerCase()+&quot; &quot;]||[]).concat(t[2])}t=n[e.toLowerCase()+&quot; &quot;]}return null==t?null:t.join(&quot;, &quot;)},getAllResponseHeaders:function(){return h?p:null},setRequestHeader:function(e,t){return null==h&amp;&amp;(e=s[e.toLowerCase()]=s[e.toLowerCase()]||e,a[e]=t),this},overrideMimeType:function(e){return null==h&amp;&amp;(v.mimeType=e),this},statusCode:function(e){var t;if(e)if(h)T.always(e[T.status]);else for(t in e)w[t]=[w[t],e[t]];return this},abort:function(e){var t=e||u;return c&amp;&amp;c.abort(t),l(0,t),this}};if(x.promise(T),v.url=((e||v.url||bt.href)+&quot;&quot;).replace(Ht,bt.protocol+&quot;//&quot;),v.type=t.method||t.type||v.method||v.type,v.dataTypes=(v.dataType||&quot;*&quot;).toLowerCase().match(P)||[&quot;&quot;],null==v.crossDomain){r=E.createElement(&quot;a&quot;);try{r.href=v.url,r.href=r.href,v.crossDomain=Mt.protocol+&quot;//&quot;+Mt.host!=r.protocol+&quot;//&quot;+r.host}catch(e){v.crossDomain=!0}}if(v.data&amp;&amp;v.processData&amp;&amp;&quot;string&quot;!=typeof v.data&amp;&amp;(v.data=S.param(v.data,v.traditional)),Wt(Ot,v,t,T),h)return T;for(i in(g=S.event&amp;&amp;v.global)&amp;&amp;0==S.active++&amp;&amp;S.event.trigger(&quot;ajaxStart&quot;),v.type=v.type.toUpperCase(),v.hasContent=!Lt.test(v.type),f=v.url.replace(jt,&quot;&quot;),v.hasContent?v.data&amp;&amp;v.processData&amp;&amp;0===(v.contentType||&quot;&quot;).indexOf(&quot;application/x-www-form-urlencoded&quot;)&amp;&amp;(v.data=v.data.replace(Nt,&quot;+&quot;)):(o=v.url.slice(f.length),v.data&amp;&amp;(v.processData||&quot;string&quot;==typeof v.data)&amp;&amp;(f+=(Tt.test(f)?&quot;&amp;&quot;:&quot;?&quot;)+v.data,delete v.data),!1===v.cache&amp;&amp;(f=f.replace(Dt,&quot;$1&quot;),o=(Tt.test(f)?&quot;&amp;&quot;:&quot;?&quot;)+&quot;_=&quot;+wt.guid+++o),v.url=f+o),v.ifModified&amp;&amp;(S.lastModified[f]&amp;&amp;T.setRequestHeader(&quot;If-Modified-Since&quot;,S.lastModified[f]),S.etag[f]&amp;&amp;T.setRequestHeader(&quot;If-None-Match&quot;,S.etag[f])),(v.data&amp;&amp;v.hasContent&amp;&amp;!1!==v.contentType||t.contentType)&amp;&amp;T.setRequestHeader(&quot;Content-Type&quot;,v.contentType),T.setRequestHeader(&quot;Accept&quot;,v.dataTypes[0]&amp;&amp;v.accepts[v.dataTypes[0]]?v.accepts[v.dataTypes[0]]+(&quot;*&quot;!==v.dataTypes[0]?&quot;, &quot;+Rt+&quot;; q=0.01&quot;:&quot;&quot;):v.accepts[&quot;*&quot;]),v.headers)T.setRequestHeader(i,v.headers[i]);if(v.beforeSend&amp;&amp;(!1===v.beforeSend.call(y,T,v)||h))return T.abort();if(u=&quot;abort&quot;,b.add(v.complete),T.done(v.success),T.fail(v.error),c=Wt(Pt,v,t,T)){if(T.readyState=1,g&amp;&amp;m.trigger(&quot;ajaxSend&quot;,[T,v]),h)return T;v.async&amp;&amp;0&lt;v.timeout&amp;&amp;(d=C.setTimeout(function(){T.abort(&quot;timeout&quot;)},v.timeout));try{h=!1,c.send(a,l)}catch(e){if(h)throw e;l(-1,e)}}else l(-1,&quot;No Transport&quot;);function l(e,t,n,r){var i,o,a,s,u,l=t;h||(h=!0,d&amp;&amp;C.clearTimeout(d),c=void 0,p=r||&quot;&quot;,T.readyState=0&lt;e?4:0,i=200&lt;=e&amp;&amp;e&lt;300||304===e,n&amp;&amp;(s=function(e,t,n){var r,i,o,a,s=e.contents,u=e.dataTypes;while(&quot;*&quot;===u[0])u.shift(),void 0===r&amp;&amp;(r=e.mimeType||t.getResponseHeader(&quot;Content-Type&quot;));if(r)for(i in s)if(s[i]&amp;&amp;s[i].test(r)){u.unshift(i);break}if(u[0]in n)o=u[0];else{for(i in n){if(!u[0]||e.converters[i+&quot; &quot;+u[0]]){o=i;break}a||(a=i)}o=o||a}if(o)return o!==u[0]&amp;&amp;u.unshift(o),n[o]}(v,T,n)),!i&amp;&amp;-1&lt;S.inArray(&quot;script&quot;,v.dataTypes)&amp;&amp;S.inArray(&quot;json&quot;,v.dataTypes)&lt;0&amp;&amp;(v.converters[&quot;text script&quot;]=function(){}),s=function(e,t,n,r){var i,o,a,s,u,l={},c=e.dataTypes.slice();if(c[1])for(a in e.converters)l[a.toLowerCase()]=e.converters[a];o=c.shift();while(o)if(e.responseFields[o]&amp;&amp;(n[e.responseFields[o]]=t),!u&amp;&amp;r&amp;&amp;e.dataFilter&amp;&amp;(t=e.dataFilter(t,e.dataType)),u=o,o=c.shift())if(&quot;*&quot;===o)o=u;else if(&quot;*&quot;!==u&amp;&amp;u!==o){if(!(a=l[u+&quot; &quot;+o]||l[&quot;* &quot;+o]))for(i in l)if((s=i.split(&quot; &quot;))[1]===o&amp;&amp;(a=l[u+&quot; &quot;+s[0]]||l[&quot;* &quot;+s[0]])){!0===a?a=l[i]:!0!==l[i]&amp;&amp;(o=s[0],c.unshift(s[1]));break}if(!0!==a)if(a&amp;&amp;e[&quot;throws&quot;])t=a(t);else try{t=a(t)}catch(e){return{state:&quot;parsererror&quot;,error:a?e:&quot;No conversion from &quot;+u+&quot; to &quot;+o}}}return{state:&quot;success&quot;,data:t}}(v,s,T,i),i?(v.ifModified&amp;&amp;((u=T.getResponseHeader(&quot;Last-Modified&quot;))&amp;&amp;(S.lastModified[f]=u),(u=T.getResponseHeader(&quot;etag&quot;))&amp;&amp;(S.etag[f]=u)),204===e||&quot;HEAD&quot;===v.type?l=&quot;nocontent&quot;:304===e?l=&quot;notmodified&quot;:(l=s.state,o=s.data,i=!(a=s.error))):(a=l,!e&amp;&amp;l||(l=&quot;error&quot;,e&lt;0&amp;&amp;(e=0))),T.status=e,T.statusText=(t||l)+&quot;&quot;,i?x.resolveWith(y,[o,l,T]):x.rejectWith(y,[T,l,a]),T.statusCode(w),w=void 0,g&amp;&amp;m.trigger(i?&quot;ajaxSuccess&quot;:&quot;ajaxError&quot;,[T,v,i?o:a]),b.fireWith(y,[T,l]),g&amp;&amp;(m.trigger(&quot;ajaxComplete&quot;,[T,v]),--S.active||S.event.trigger(&quot;ajaxStop&quot;)))}return T},getJSON:function(e,t,n){return S.get(e,t,n,&quot;json&quot;)},getScript:function(e,t){return S.get(e,void 0,t,&quot;script&quot;)}}),S.each([&quot;get&quot;,&quot;post&quot;],function(e,i){S[i]=function(e,t,n,r){return m(t)&amp;&amp;(r=r||n,n=t,t=void 0),S.ajax(S.extend({url:e,type:i,dataType:r,data:t,success:n},S.isPlainObject(e)&amp;&amp;e))}}),S.ajaxPrefilter(function(e){var t;for(t in e.headers)&quot;content-type&quot;===t.toLowerCase()&amp;&amp;(e.contentType=e.headers[t]||&quot;&quot;)}),S._evalUrl=function(e,t,n){return S.ajax({url:e,type:&quot;GET&quot;,dataType:&quot;script&quot;,cache:!0,async:!1,global:!1,converters:{&quot;text script&quot;:function(){}},dataFilter:function(e){S.globalEval(e,t,n)}})},S.fn.extend({wrapAll:function(e){var t;return this[0]&amp;&amp;(m(e)&amp;&amp;(e=e.call(this[0])),t=S(e,this[0].ownerDocument).eq(0).clone(!0),this[0].parentNode&amp;&amp;t.insertBefore(this[0]),t.map(function(){var e=this;while(e.firstElementChild)e=e.firstElementChild;return e}).append(this)),this},wrapInner:function(n){return m(n)?this.each(function(e){S(this).wrapInner(n.call(this,e))}):this.each(function(){var e=S(this),t=e.contents();t.length?t.wrapAll(n):e.append(n)})},wrap:function(t){var n=m(t);return this.each(function(e){S(this).wrapAll(n?t.call(this,e):t)})},unwrap:function(e){return this.parent(e).not(&quot;body&quot;).each(function(){S(this).replaceWith(this.childNodes)}),this}}),S.expr.pseudos.hidden=function(e){return!S.expr.pseudos.visible(e)},S.expr.pseudos.visible=function(e){return!!(e.offsetWidth||e.offsetHeight||e.getClientRects().length)},S.ajaxSettings.xhr=function(){try{return new C.XMLHttpRequest}catch(e){}};var Bt={0:200,1223:204},$t=S.ajaxSettings.xhr();y.cors=!!$t&amp;&amp;&quot;withCredentials&quot;in $t,y.ajax=$t=!!$t,S.ajaxTransport(function(i){var o,a;if(y.cors||$t&amp;&amp;!i.crossDomain)return{send:function(e,t){var n,r=i.xhr();if(r.open(i.type,i.url,i.async,i.username,i.password),i.xhrFields)for(n in i.xhrFields)r[n]=i.xhrFields[n];for(n in i.mimeType&amp;&amp;r.overrideMimeType&amp;&amp;r.overrideMimeType(i.mimeType),i.crossDomain||e[&quot;X-Requested-With&quot;]||(e[&quot;X-Requested-With&quot;]=&quot;XMLHttpRequest&quot;),e)r.setRequestHeader(n,e[n]);o=function(e){return function(){o&amp;&amp;(o=a=r.onload=r.onerror=r.onabort=r.ontimeout=r.onreadystatechange=null,&quot;abort&quot;===e?r.abort():&quot;error&quot;===e?&quot;number&quot;!=typeof r.status?t(0,&quot;error&quot;):t(r.status,r.statusText):t(Bt[r.status]||r.status,r.statusText,&quot;text&quot;!==(r.responseType||&quot;text&quot;)||&quot;string&quot;!=typeof r.responseText?{binary:r.response}:{text:r.responseText},r.getAllResponseHeaders()))}},r.onload=o(),a=r.onerror=r.ontimeout=o(&quot;error&quot;),void 0!==r.onabort?r.onabort=a:r.onreadystatechange=function(){4===r.readyState&amp;&amp;C.setTimeout(function(){o&amp;&amp;a()})},o=o(&quot;abort&quot;);try{r.send(i.hasContent&amp;&amp;i.data||null)}catch(e){if(o)throw e}},abort:function(){o&amp;&amp;o()}}}),S.ajaxPrefilter(function(e){e.crossDomain&amp;&amp;(e.contents.script=!1)}),S.ajaxSetup({accepts:{script:&quot;text/javascript, application/javascript, application/ecmascript, application/x-ecmascript&quot;},contents:{script:/\b(?:java|ecma)script\b/},converters:{&quot;text script&quot;:function(e){return S.globalEval(e),e}}}),S.ajaxPrefilter(&quot;script&quot;,function(e){void 0===e.cache&amp;&amp;(e.cache=!1),e.crossDomain&amp;&amp;(e.type=&quot;GET&quot;)}),S.ajaxTransport(&quot;script&quot;,function(n){var r,i;if(n.crossDomain||n.scriptAttrs)return{send:function(e,t){r=S(&quot;&lt;script>&quot;).attr(n.scriptAttrs||{}).prop({charset:n.scriptCharset,src:n.url}).on(&quot;load error&quot;,i=function(e){r.remove(),i=null,e&amp;&amp;t(&quot;error&quot;===e.type?404:200,e.type)}),E.head.appendChild(r[0])},abort:function(){i&amp;&amp;i()}}});var _t,zt=[],Ut=/(=)\?(?=&amp;|$)|\?\?/;S.ajaxSetup({jsonp:&quot;callback&quot;,jsonpCallback:function(){var e=zt.pop()||S.expando+&quot;_&quot;+wt.guid++;return this[e]=!0,e}}),S.ajaxPrefilter(&quot;json jsonp&quot;,function(e,t,n){var r,i,o,a=!1!==e.jsonp&amp;&amp;(Ut.test(e.url)?&quot;url&quot;:&quot;string&quot;==typeof e.data&amp;&amp;0===(e.contentType||&quot;&quot;).indexOf(&quot;application/x-www-form-urlencoded&quot;)&amp;&amp;Ut.test(e.data)&amp;&amp;&quot;data&quot;);if(a||&quot;jsonp&quot;===e.dataTypes[0])return r=e.jsonpCallback=m(e.jsonpCallback)?e.jsonpCallback():e.jsonpCallback,a?e[a]=e[a].replace(Ut,&quot;$1&quot;+r):!1!==e.jsonp&amp;&amp;(e.url+=(Tt.test(e.url)?&quot;&amp;&quot;:&quot;?&quot;)+e.jsonp+&quot;=&quot;+r),e.converters[&quot;script json&quot;]=function(){return o||S.error(r+&quot; was not called&quot;),o[0]},e.dataTypes[0]=&quot;json&quot;,i=C[r],C[r]=function(){o=arguments},n.always(function(){void 0===i?S(C).removeProp(r):C[r]=i,e[r]&amp;&amp;(e.jsonpCallback=t.jsonpCallback,zt.push(r)),o&amp;&amp;m(i)&amp;&amp;i(o[0]),o=i=void 0}),&quot;script&quot;}),y.createHTMLDocument=((_t=E.implementation.createHTMLDocument(&quot;&quot;).body).innerHTML=&quot;&lt;form>&lt;/form>&lt;form>&lt;/form>&quot;,2===_t.childNodes.length),S.parseHTML=function(e,t,n){return&quot;string&quot;!=typeof e?[]:(&quot;boolean&quot;==typeof t&amp;&amp;(n=t,t=!1),t||(y.createHTMLDocument?((r=(t=E.implementation.createHTMLDocument(&quot;&quot;)).createElement(&quot;base&quot;)).href=E.location.href,t.head.appendChild(r)):t=E),o=!n&amp;&amp;[],(i=N.exec(e))?[t.createElement(i[1])]:(i=xe([e],t,o),o&amp;&amp;o.length&amp;&amp;S(o).remove(),S.merge([],i.childNodes)));var r,i,o},S.fn.load=function(e,t,n){var r,i,o,a=this,s=e.indexOf(&quot; &quot;);return-1&lt;s&amp;&amp;(r=ht(e.slice(s)),e=e.slice(0,s)),m(t)?(n=t,t=void 0):t&amp;&amp;&quot;object&quot;==typeof t&amp;&amp;(i=&quot;POST&quot;),0&lt;a.length&amp;&amp;S.ajax({url:e,type:i||&quot;GET&quot;,dataType:&quot;html&quot;,data:t}).done(function(e){o=arguments,a.html(r?S(&quot;&lt;div>&quot;).append(S.parseHTML(e)).find(r):e)}).always(n&amp;&amp;function(e,t){a.each(function(){n.apply(this,o||[e.responseText,t,e])})}),this},S.expr.pseudos.animated=function(t){return S.grep(S.timers,function(e){return t===e.elem}).length},S.offset={setOffset:function(e,t,n){var r,i,o,a,s,u,l=S.css(e,&quot;position&quot;),c=S(e),f={};&quot;static&quot;===l&amp;&amp;(e.style.position=&quot;relative&quot;),s=c.offset(),o=S.css(e,&quot;top&quot;),u=S.css(e,&quot;left&quot;),(&quot;absolute&quot;===l||&quot;fixed&quot;===l)&amp;&amp;-1&lt;(o+u).indexOf(&quot;auto&quot;)?(a=(r=c.position()).top,i=r.left):(a=parseFloat(o)||0,i=parseFloat(u)||0),m(t)&amp;&amp;(t=t.call(e,n,S.extend({},s))),null!=t.top&amp;&amp;(f.top=t.top-s.top+a),null!=t.left&amp;&amp;(f.left=t.left-s.left+i),&quot;using&quot;in t?t.using.call(e,f):c.css(f)}},S.fn.extend({offset:function(t){if(arguments.length)return void 0===t?this:this.each(function(e){S.offset.setOffset(this,t,e)});var e,n,r=this[0];return r?r.getClientRects().length?(e=r.getBoundingClientRect(),n=r.ownerDocument.defaultView,{top:e.top+n.pageYOffset,left:e.left+n.pageXOffset}):{top:0,left:0}:void 0},position:function(){if(this[0]){var e,t,n,r=this[0],i={top:0,left:0};if(&quot;fixed&quot;===S.css(r,&quot;position&quot;))t=r.getBoundingClientRect();else{t=this.offset(),n=r.ownerDocument,e=r.offsetParent||n.documentElement;while(e&amp;&amp;(e===n.body||e===n.documentElement)&amp;&amp;&quot;static&quot;===S.css(e,&quot;position&quot;))e=e.parentNode;e&amp;&amp;e!==r&amp;&amp;1===e.nodeType&amp;&amp;((i=S(e).offset()).top+=S.css(e,&quot;borderTopWidth&quot;,!0),i.left+=S.css(e,&quot;borderLeftWidth&quot;,!0))}return{top:t.top-i.top-S.css(r,&quot;marginTop&quot;,!0),left:t.left-i.left-S.css(r,&quot;marginLeft&quot;,!0)}}},offsetParent:function(){return this.map(function(){var e=this.offsetParent;while(e&amp;&amp;&quot;static&quot;===S.css(e,&quot;position&quot;))e=e.offsetParent;return e||re})}}),S.each({scrollLeft:&quot;pageXOffset&quot;,scrollTop:&quot;pageYOffset&quot;},function(t,i){var o=&quot;pageYOffset&quot;===i;S.fn[t]=function(e){return $(this,function(e,t,n){var r;if(x(e)?r=e:9===e.nodeType&amp;&amp;(r=e.defaultView),void 0===n)return r?r[i]:e[t];r?r.scrollTo(o?r.pageXOffset:n,o?n:r.pageYOffset):e[t]=n},t,e,arguments.length)}}),S.each([&quot;top&quot;,&quot;left&quot;],function(e,n){S.cssHooks[n]=Fe(y.pixelPosition,function(e,t){if(t)return t=We(e,n),Pe.test(t)?S(e).position()[n]+&quot;px&quot;:t})}),S.each({Height:&quot;height&quot;,Width:&quot;width&quot;},function(a,s){S.each({padding:&quot;inner&quot;+a,content:s,&quot;&quot;:&quot;outer&quot;+a},function(r,o){S.fn[o]=function(e,t){var n=arguments.length&amp;&amp;(r||&quot;boolean&quot;!=typeof e),i=r||(!0===e||!0===t?&quot;margin&quot;:&quot;border&quot;);return $(this,function(e,t,n){var r;return x(e)?0===o.indexOf(&quot;outer&quot;)?e[&quot;inner&quot;+a]:e.document.documentElement[&quot;client&quot;+a]:9===e.nodeType?(r=e.documentElement,Math.max(e.body[&quot;scroll&quot;+a],r[&quot;scroll&quot;+a],e.body[&quot;offset&quot;+a],r[&quot;offset&quot;+a],r[&quot;client&quot;+a])):void 0===n?S.css(e,t,i):S.style(e,t,n,i)},s,n?e:void 0,n)}})}),S.each([&quot;ajaxStart&quot;,&quot;ajaxStop&quot;,&quot;ajaxComplete&quot;,&quot;ajaxError&quot;,&quot;ajaxSuccess&quot;,&quot;ajaxSend&quot;],function(e,t){S.fn[t]=function(e){return this.on(t,e)}}),S.fn.extend({bind:function(e,t,n){return this.on(e,null,t,n)},unbind:function(e,t){return this.off(e,null,t)},delegate:function(e,t,n,r){return this.on(t,e,n,r)},undelegate:function(e,t,n){return 1===arguments.length?this.off(e,&quot;**&quot;):this.off(t,e||&quot;**&quot;,n)},hover:function(e,t){return this.mouseenter(e).mouseleave(t||e)}}),S.each(&quot;blur focus focusin focusout resize scroll click dblclick mousedown mouseup mousemove mouseover mouseout mouseenter mouseleave change select submit keydown keypress keyup contextmenu&quot;.split(&quot; &quot;),function(e,n){S.fn[n]=function(e,t){return 0&lt;arguments.length?this.on(n,null,e,t):this.trigger(n)}});var Xt=/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g;S.proxy=function(e,t){var n,r,i;if(&quot;string&quot;==typeof t&amp;&amp;(n=e[t],t=e,e=n),m(e))return r=s.call(arguments,2),(i=function(){return e.apply(t||this,r.concat(s.call(arguments)))}).guid=e.guid=e.guid||S.guid++,i},S.holdReady=function(e){e?S.readyWait++:S.ready(!0)},S.isArray=Array.isArray,S.parseJSON=JSON.parse,S.nodeName=A,S.isFunction=m,S.isWindow=x,S.camelCase=X,S.type=w,S.now=Date.now,S.isNumeric=function(e){var t=S.type(e);return(&quot;number&quot;===t||&quot;string&quot;===t)&amp;&amp;!isNaN(e-parseFloat(e))},S.trim=function(e){return null==e?&quot;&quot;:(e+&quot;&quot;).replace(Xt,&quot;&quot;)},&quot;function&quot;==typeof define&amp;&amp;define.amd&amp;&amp;define(&quot;jquery&quot;,[],function(){return S});var Vt=C.jQuery,Gt=C.$;return S.noConflict=function(e){return C.$===S&amp;&amp;(C.$=Gt),e&amp;&amp;C.jQuery===S&amp;&amp;(C.jQuery=Vt),S},&quot;undefined&quot;==typeof e&amp;&amp;(C.jQuery=C.$=S),S});


/* Demo page specific styles */
.demo-headline {
    background-color: #110D95;
    background-image: url(&quot; , &quot;'&quot; , &quot;https://phptravels.com/assets/img/head.webp&quot; , &quot;'&quot; , &quot;);
    background-position: bottom right;
    background-repeat: no-repeat;
    background-size: contain;
    color: white;
    padding: 5rem 0;
    margin-bottom: 3rem;
    position: relative;
    overflow: hidden;
}
.demo-headline-content {
    position: relative;
    z-index: 2;
    max-width: 60%;
}
.demo-headline-small {
    font-size: 0.9rem;
    font-weight: 500;
    letter-spacing: 2px;
    text-transform: uppercase;
    margin-bottom: 1rem;
    opacity: 0.9;
}
.demo-headline h1 {
    font-size: 3.5rem;
    font-weight: 700;
    letter-spacing: -2px;
    margin-bottom: 1.5rem;
    line-height: 1.1;
}
.demo-headline p {
    font-size: 1.3rem;
    opacity: 0.9;
    font-weight: 300;
    margin-top: 0;
    line-height: 1.5;
    margin-bottom: 2rem;
}
.demo-section {
    padding: 4rem 0;
    background: #f8f9fa;
}
.demo-form-container {
    background: white;
    border-radius: 16px;
    border: 1px solid #e8e8e8;
    padding: 2rem;
    height: 100%;
    /* box-shadow removed */
}
.demo-form-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 1.5rem;
}
.demo-form-divider {
    border-color: #e8e8e8;
    margin: 1.5rem 0;
}
.demo-form-group {
    margin-bottom: 1rem;
}
.demo-form-control {
    border: 1px solid #e8e8e8;
    border-radius: 8px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
}
.demo-form-control:focus {
    border-color: #110D95;
    box-shadow: none;
}
.demo-submit-btn {
    background: #007bff;
    border: none;
    border-radius: 8px;
    padding: 1rem 2rem;
    font-weight: 600;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
}
.demo-submit-btn:hover {
    background: #0056b3;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,123,255,0.3);
}
.demo-submit-btn:disabled {
    background: #6c757d;
    transform: none;
    box-shadow: none;
}
.demo-captcha-container {
    background: #f8f9fa;
    border-radius: 8px;
    padding: 1rem;
    text-align: center;
    margin-bottom: 1rem;
}
.demo-captcha-result {
    border: 1px solid #e8e8e8;
    border-radius: 8px;
    height: 58px;
    font-size: 1.1rem;
    text-align: center;
}
.demo-image-container {
    background: rgba(124, 145, 161, 0.11);
    border-radius: 16px;
    padding: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
}
.demo-image-container img {
    max-width: 100%;
    height: auto;
    object-fit: cover;
    border-radius: 8px;
}
.demo-success-container {
    text-align: center;
    padding: 2rem 0;
}
.demo-success-icon {
    margin-bottom: 1.5rem;
}
.demo-success-title {
    font-size: 2rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 1rem;
}
.demo-success-text {
    color: #666;
    line-height: 1.6;
    font-size: 1rem;
}
.demo-faq-section {
    padding: 4rem 0;
    background: white;
}
.demo-faq-title {
    font-size: 2rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 2rem;
}
.demo-faq-subtitle {
    font-size: 1rem;
    color: #666;
    margin-bottom: 3rem;
}
.demo-accordion-item {
    border: 1px solid #e8e8e8;
    border-radius: 12px !important;
    margin-bottom: 1rem;
    overflow: hidden;
}
.demo-accordion-button {
    background: white;
    border: none;
    padding: 1.25rem 1.5rem;
    font-weight: 600;
    color: #1a1a1a;
    font-size: 1rem;
}
.demo-accordion-button:not(.collapsed) {
    background: #f8f9fa;
    color: #110D95;
    box-shadow: none;
}
.demo-accordion-button:focus {
    box-shadow: 0 0 0 0.2rem rgba(0,123,255,0.25);
}
.demo-accordion-body {
    padding: 1.5rem;
    background: white;
    color: #666;
    line-height: 1.6;
}
.demo-loading-container {
    text-align: center;
    padding: 3rem 0;
}
.demo-alert {
    border-radius: 8px;
    padding: 0.75rem 1rem;
    margin-top: 1rem;
    border: none;
    background: #f8d7da;
    color: #721c24;
}
/* Responsive adjustments */
@media (max-width: 768px) {
    .demo-headline-content {
        max-width: 100%;
        text-align: center;
    }
    .demo-headline {
        padding: 2.5rem 0;
    }
    .demo-headline h1 {
        font-size: 2.2rem;
    }
    .demo-headline p {
        font-size: 1rem;
    }
    .demo-section {
        padding: 2rem 0;
    }
    .demo-form-container {
        padding: 1.5rem;
        margin-bottom: 2rem;
    }
}
@media (max-width: 576px) {
    .demo-headline h1 {
        font-size: 1.8rem;
    }
    .demo-form-container {
        padding: 1rem;
    }
    .demo-faq-section {
        padding: 2rem 0;
    }
}
/* Animation styles for success checkmark */
#check-group {
    animation: 0.32s ease-in-out 1.03s check-group;
    transform-origin: center;
}
#check-group #check {
    animation: 0.34s cubic-bezier(0.65, 0, 1, 1) 0.8s forwards check;
    stroke-dasharray: 0, 75px;
    stroke-linecap: round;
    stroke-linejoin: round;
}
#check-group #outline {
    animation: 0.38s ease-in outline;
    transform: rotate(0deg);
    transform-origin: center;
}
#check-group #white-circle {
    animation: 0.35s ease-in 0.35s forwards circle;
    transform: none;
    transform-origin: center;
}
@keyframes outline {
    from { stroke-dasharray: 0, 345.576px; }
    to { stroke-dasharray: 345.576px, 345.576px; }
}
@keyframes circle {
    from { transform: scale(1); }
    to { transform: scale(0); }
}
@keyframes check {
    from { stroke-dasharray: 0, 75px; }
    to { stroke-dasharray: 75px, 75px; }
}
@keyframes check-group {
    from { transform: scale(1); }
    50% { transform: scale(1.09); }
    to { transform: scale(1); }
}
/* Utility classes */
.no-spin-buttons {
    -webkit-appearance: none;
    -moz-appearance: textfield;
}
input[type=&quot;number&quot;]::-webkit-inner-spin-button,
input[type=&quot;number&quot;]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
}



    
        
            Demo
            Live Demonstration
            Test drive our complete travel booking platform with full features and functionality
        
    



    
        
            
                
                    Request Demo Access
                    

                    
                        
                            
                                
                                    
                                    First Name
                                
                            
                            
                                
                                    
                                    Last Name
                                
                            
                        

                        
                            
                                
                                    
                                            Country
    Afghanistan +93
    Albania +355
    Algeria +213
    American Samoa +1684
    Andorra +376
    Angola +244
    Anguilla +1264
    Antarctica +0
    Antigua and Barbuda +1268
    Argentina +54
    Armenia +374
    Aruba +297
    Australia +61
    Austria +43
    Azerbaijan +994
    Bahamas +1242
    Bahrain +973
    Bangladesh +880
    Barbados +1246
    Belarus +375
    Belgium +32
    Belize +501
    Benin +229
    Bermuda +1441
    Bhutan +975
    Bolivia +591
    Bosnia and Herzegovina +387
    Botswana +267
    Brazil +55
    British Indian Ocean Territory +246
    Brunei Darussalam +673
    Bulgaria +359
    Burkina Faso +226
    Burundi +257
    Cambodia +855
    Cameroon +237
    Canada +1
    Cape Verde +238
    Cayman Islands +1345
    Central African Republic +236
    Chad +235
    Chile +56
    China +86
    Christmas Island +61
    Cocos (Keeling) Islands +672
    Colombia +57
    Comoros +269
    Congo +243
    Congo, the Democratic Republic of the +242
    Cook Islands +682
    Costa Rica +506
    Cote D&quot; , &quot;'&quot; , &quot;Ivoire +225
    Croatia +385
    Cuba +53
    Cyprus +357
    Czech Republic +420
    Denmark +45
    Djibouti +253
    Dominica +1767
    Dominican Republic +1809
    Ecuador +593
    Egypt +20
    El Salvador +503
    Equatorial Guinea +240
    Eritrea +291
    Estonia +372
    Ethiopia +251
    Falkland Islands (Malvinas) +500
    Faroe Islands +298
    Fiji +679
    Finland +358
    France +33
    French Guiana +594
    French Polynesia +689
    French Southern Territories +0
    Gabon +241
    Gambia +220
    Georgia +995
    Germany +49
    Ghana +233
    Gibraltar +350
    Greece +30
    Greenland +299
    Grenada +1473
    Guadeloupe +590
    Guam +1671
    Guatemala +502
    Guinea +224
    Guinea-Bissau +245
    Guyana +592
    Haiti +509
    Holy See (Vatican City State) +39
    Honduras +504
    Hong Kong +852
    Hungary +36
    Iceland +354
    India +91
    Indonesia +62
    Iran, Islamic Republic of +98
    Iraq +964
    Ireland +353
    Israel +972
    Italy +39
    Jamaica +1876
    Japan +81
    Jordan +962
    Kazakhstan +7
    Kenya +254
    Kiribati +686
    Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of +850
    Korea, Republic of +82
    Kuwait +965
    Kyrgyzstan +996
    Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic +856
    Latvia +371
    Lebanon +961
    Lesotho +266
    Liberia +231
    Libyan Arab Jamahiriya +218
    Liechtenstein +423
    Lithuania +370
    Luxembourg +352
    Macao +853
    Macedonia, the Former Yugoslav Republic of +389
    Madagascar +261
    Malawi +265
    Malaysia +60
    Maldives +960
    Mali +223
    Malta +356
    Marshall Islands +692
    Martinique +596
    Mauritania +222
    Mauritius +230
    Mayotte +269
    Mexico +52
    Micronesia, Federated States of +691
    Moldova, Republic of +373
    Monaco +377
    Mongolia +976
    Montserrat +1664
    Morocco +212
    Mozambique +258
    Myanmar +95
    Namibia +264
    Nauru +674
    Nepal +977
    Netherlands +31
    Netherlands Antilles +599
    New Caledonia +687
    New Zealand +64
    Nicaragua +505
    Niger +227
    Nigeria +234
    Niue +683
    Norfolk Island +672
    Northern Mariana Islands +1670
    Norway +47
    Oman +968
    Pakistan +92
    Palau +680
    Palestinian Territory, Occupied +970
    Panama +507
    Papua New Guinea +675
    Paraguay +595
    Peru +51
    Philippines +63
    Pitcairn +0
    Poland +48
    Portugal +351
    Puerto Rico +1787
    Qatar +974
    Reunion +262
    Romania +40
    Russia +70
    Rwanda +250
    Saint Kitts and Nevis +1869
    Saint Lucia +1758
    Saint Pierre and Miquelon +508
    Saint Vincent and the Grenadines +1784
    Samoa +684
    San Marino +378
    Sao Tome and Principe +239
    Saudi Arabia +966
    Senegal +221
    Serbia and Montenegro +381
    Seychelles +248
    Sierra Leone +232
    Singapore +65
    Slovakia +421
    Slovenia +386
    Solomon Islands +677
    Somalia +252
    South Africa +27
    South Georgia and the South Sandwich Islands +0
    Spain +34
    Sri Lanka +94
    Sudan +249
    Swaziland +268
    Sweden +46
    Switzerland +41
    Syrian Arab Republic +963
    Taiwan, Province of China +886
    Tajikistan +992
    Tanzania, United Republic of +255
    Thailand +66
    Timor-Leste +670
    Togo +228
    Tokelau +690
    Tonga +676
    Trinidad and Tobago +1868
    Tunisia +216
    Turkey +90
    Turkmenistan +7370
    Turks and Caicos Islands +1649
    Tuvalu +688
    Uganda +256
    Ukraine +380
    United Arab Emirates +971
    United Kingdom +44
    United States +1
    Uruguay +598
    Uzbekistan +998
    Vanuatu +678
    Venezuela +58
    Viet Nam +84
    Virgin Islands, British +1284
    Virgin Islands, U.s. +1340
    Wallis and Futuna +681
    Western Sahara +212
    Yemen +967
    Zambia +260
    Zimbabwe +263
    Serbia +381
    Asia / Pacific Region +0
    Montenegro +382
    Aland Islands +358
    Curacao +599
    Guernsey +44
    Isle of Man +44
    Jersey +44
    Kosovo +381
    Saint Barthelemy +590
    Saint Martin +590
    Sint Maarten +1
    South Sudan +211



var requestUrl = &quot;https://ipwhois.app/json/&quot;;
fetch(requestUrl)
.then(function(response) { return response.json(); })
.then(function(c) {
var user_country = c[&quot; , &quot;'&quot; , &quot;country_phone&quot; , &quot;'&quot; , &quot;];
user_country = user_country.replace(&quot; , &quot;'&quot; , &quot;+&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
$(&quot;[data-country-phonecode=&quot; , &quot;'&quot; , &quot;&quot; + user_country + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).prop(&quot;selected&quot;, true);
// $(&quot;[name=&quot; , &quot;'&quot; , &quot;country_id&quot; , &quot;'&quot; , &quot;]&quot;).val(user_country)
console.log(user_country);
});
                                    
                                    Select Country
                                
                            
                            
                                
                                    
                                    WhatsApp Number
                                
                            
                        

                        
                            
                            Business Name
                        

                        
                            
                            Email Address
                        

                        
                            
                                
                                    Submit
                                    
                                        
                                    
                                

                                
                                    
                                
                            
                            
                                
                                    
                                        
                                            
                                                4 + 10 = ?
                                            
                                        
                                    
                                    
                                
                            
                        

                        
                            The whatsapp number is not valid. avoid adding country number, Zero or + signs before the number
                        

                        
                            
                                
                                    
                                
                                
                                    
                                
                                
                                    
                                
                            
                        
                    
                    
                    
                        
                        
                            
                                
                                    
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                         Thank you!
                        We have sent your demo credentials to your email please check your mailbox. if demo credentials not found inbox please check spam folder
                    
                
            
            
                
                    
                
            
        
    



    
        Frequently Asked Questions
        Find answers to common questions about our demo platform and testing process.
        
                            
                    
                        
                            How can I test the PHPTRAVELS demo?                        
                    
                    
                        
                            Submit the demo request form to receive complete access credentials. You&quot; , &quot;'&quot; , &quot;ll be able to test everything from the admin panel to the client interface on our live demo platform.                        
                    
                
                            
                    
                        
                            Can I save content during testing?                        
                    
                    
                        
                            Yes, but please note that the database content resets every few minutes to maintain demo integrity. However, all content can be fully customized in your purchased version.                        
                    
                
                            
                    
                        
                            What features are included in the demo?                        
                    
                    
                        
                            Our demo includes all features needed to run your travel business. Test the complete application from backend administration to frontend booking experience at no cost.                        
                    
                
                            
                    
                        
                            Do you charge for demo testing?                        
                    
                    
                        
                            No, we offer completely free application testing. Access our demo anytime by submitting the request form - no hidden charges or commitments.                        
                    
                
                            
                    
                        
                            Can I test the application on my own hosting?                        
                    
                    
                        
                            Demo testing is available exclusively on our servers for security reasons. Once satisfied with the demo, you can purchase and install on your hosting with our complete support.                        
                    
                
                            
                    
                        
                            Is the application mobile responsive?                        
                    
                    
                        
                            Yes, all themes and interfaces are fully responsive and optimized for desktop, laptop, tablet, and mobile devices. We recommend testing on multiple devices to experience the complete responsiveness.                        
                    
                
                    
    



// Generate random captcha numbers
numb1 = Math.floor((Math.random() * 10) + 1);
numb2 = Math.floor((Math.random() * 10) + 1);
document.getElementById(&quot;numb1&quot;).innerHTML = numb1;
document.getElementById(&quot;numb2&quot;).innerHTML = numb2;

// Form submission handler
$(&quot;#demo&quot;).click(function() {
    var country_id = $(&quot; , &quot;'&quot; , &quot;.country_id&quot; , &quot;'&quot; , &quot;).val();
    let numbers = numb1 + numb2;
    let number = $(&quot; , &quot;'&quot; , &quot;#number&quot; , &quot;'&quot; , &quot;).val();

    // Validation
    if ($(&quot;.first_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your first name&quot;);
        return;
    }
    if ($(&quot;.last_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your last name&quot;);
        return;
    }
    if ($(&quot;.company_name&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your business name&quot;);
        return;
    }
    if ($(&quot;.whatsapp_number&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your WhatsApp number&quot;);
        return;
    }
    if (country_id === &quot;&quot;) {
        alert(&quot;Please select your country&quot;);
        return;
    }
    if ($(&quot;.email&quot;).val() === &quot;&quot;) {
        alert(&quot;Please enter your email address&quot;);
        return;
    }
    if (number == &quot;&quot;) {
        alert(&quot; , &quot;'&quot; , &quot;Please solve the math problem&quot; , &quot;'&quot; , &quot;);
        return;
    }
    
    if (numbers == number) {
        let first_name = $(&quot;.first_name&quot;).val();
        let last_name = $(&quot;.last_name&quot;).val();
        let company_name = $(&quot;.company_name&quot;).val();
        let whatsapp_number = $(&quot;.whatsapp_number&quot;).val();
        let email = $(&quot;.email&quot;).val();

        $(&quot;.btn_submit&quot;).hide();
        $(&quot;.alert_msg&quot;).hide();
        $(&quot;.btn_loading&quot;).show();

        var form = new FormData();
        form.append(&quot;first_name&quot;, first_name);
        form.append(&quot;last_name&quot;, last_name);
        form.append(&quot;email&quot;, email);
        form.append(&quot;company_name&quot;, company_name);
        form.append(&quot;whatsapp_number&quot;, whatsapp_number);
        form.append(&quot;country_id&quot;, country_id);
        form.append(&quot;lead_type&quot;, &quot;demo&quot;);

        var settings = {
            &quot;url&quot;: &quot;https://app.phptravels.com/api_demo_signup.php&quot;,
            &quot;method&quot;: &quot;POST&quot;,
            &quot;timeout&quot;: 0,
            &quot;headers&quot;: {},
            &quot;processData&quot;: false,
            &quot;mimeType&quot;: &quot;multipart/form-data&quot;,
            &quot;contentType&quot;: false,
            &quot;data&quot;: form
        };

        $.ajax(settings).done(function(response) {
            $(&quot;.btn_submit&quot;).show();
            $(&quot;.btn_loading&quot;).hide();

            try {
                var result;
                if (typeof response === &quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;) {
                    result = JSON.parse(response);
                } else {
                    result = response;
                }
                // Show thank you block if success
                if (result.status === true || result.status === &quot;true&quot; || result.status == true) {
                    $(&quot;.from_box&quot;).hide();
                    $(&quot;.completed&quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
                 } else {
                    $(&quot;.alert_msg&quot;).show();
                    $(&quot;.alert_msg p&quot;).text(result.message || &quot;An error occurred&quot;);
                }
            } catch (e) {
                $(&quot;.alert_msg&quot;).show();
                $(&quot;.alert_msg p&quot;).text(&quot;An error occurred. Please try again.&quot;);
            }
        }).fail(function(xhr, status, error) {
            $(&quot;.btn_submit&quot;).show();
            $(&quot;.btn_loading&quot;).hide();
            $(&quot;.alert_msg&quot;).show();
            $(&quot;.alert_msg p&quot;).text(&quot;Connection error. Please try again.&quot;);
        });
    } else {
        alert(&quot;The math result is incorrect. Please try again.&quot;);
    }
});





  
  
  




Aggregated Solution
With our multiple channel aggregation feature now we can get inventory from different API&quot; , &quot;'&quot; , &quot;s with realtime pricing and booking.




  GDS &amp; OTA Integration
  Realtime API&quot; , &quot;'&quot; , &quot;s and Dashboards
  100% Opensource Platform Structure
  Highly Scalable and Extensive
  Secure by Additinal Layers
  Latest Technology Implemented
  Self-Hosted Structure
  Developer Friendly Documentation
  Live Testing Demonstration








    We are Growing!

        
            
                
                
                    
                

                
                
                    
                        
                            
                                
                                    4K+
                                
                                Live Websites
                            
                        
                        
                        
                                
                                    180K+
                                
                                Users
                            
                        
                        
                        
                                
                                    6M+
                                
                                Bookings
                            
                        

                    
                

                
                const counters = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;.counter&quot; , &quot;'&quot; , &quot;);
                // Main function
                for(let n of counters) {
                  const updateCount = () => {
                    const target = + n.getAttribute(&quot; , &quot;'&quot; , &quot;data-target&quot; , &quot;'&quot; , &quot;);
                    const count = + n.innerText;
                    const speed = 500; // change animation speed here
                    const inc = target / speed;
                    if(count &lt; target) {
                      n.innerText = Math.ceil(count + inc);
                      setTimeout(updateCount, 1);
                    } else {
                      n.innerText = target;
                    }
                  }
                  updateCount();
                }
                

                
                
                    
                        
                        
                    

                    

                    
                        
                            
                        
                        
                            
                        
                    
                
            
    

    



      

        
        
          
            
              
                
                
              
            
            
              
                
                  
                    24FLIGHTS
                  
                
                
                  Nancy - 24fights.com
                  
                    PHPTRAVELS Transformed our Business with Exceptional
                    Solution &amp; Service.
                  
                  
                    View Website
                    
                      
                        
                      
                    
                  
                
              
            
          
        

        
        
          
            
              
                
                
              
            
            
              
                
                  TAZKIRA
                
                
                  M. Saeed - Tazkira.ae
                  
                      PHPTRAVELS is the real Secret Behind Tazkira&quot; , &quot;'&quot; , &quot;s 80% of Online Sales &amp; Bookings
                  

                  View Website
                      
                          
                      
                  
                
              
            
          
        

        
        
          
            
              
                
                
              
            
            
              
                
                    BOOKING JORDAN
                
                
                    Khalid Nawalah - Bookingjordan.com
                  
                      Our Achievements are Powered by PHPTRAVELS &amp; Their Super Supportive Team.
                  

                  
                    View Website
                    
                        
                    
                
                
              
            
          
        

        
        
          
          
            
              
            
          
          
            
              
            
          
        
      
    




// ALL SLIDERS
const sliderElements = document.querySelectorAll(&quot;.slider--container&quot;);
const totalSliderElements = sliderElements.length;

// THIS INDICATE THE THE CURRENTLY SHOW SLIDER VALUE
let currentSlider = 1;

// SLIDER CONTROLS
const sliderControls = document.querySelectorAll(&quot;.slider--control&quot;);

sliderControls.forEach((sliderControl) => {
  sliderControl.addEventListener(&quot;click&quot;, function () {
    // THIS WILL GTE THE VALUE(DATA_VALUE) ACCORDING TO WHICH SLIDER MOIVE ForWARD OR REVERSE
    const _sliderValue = Number(this.getAttribute(&quot;data-value&quot;));

    // SET THE THE VALUE(DATA_HIDDEN) TO TRUE OF THE CUrRENTLY ACTIVE SLIDER
    sliderElements[currentSlider - 1].setAttribute(&quot;data-hidden&quot;, &quot;true&quot;);

    // INCREASE OR DECREASE THE SLIDER VALUE ACCORDINGLY
    currentSlider = currentSlider + _sliderValue;

    // CHECK IF THE CURRENT VALUE IS GREATE THEN THE TOTAL NUMBER OF THE AVAILABLE SLIDER
    // OR EQUAL TO ZERO
    if (currentSlider - 1 === totalSliderElements) {
      currentSlider = 1;
    } else if (currentSlider === 0) {
      currentSlider = totalSliderElements;
    }

    // REQUESTED SLIDER
    sliderElements[currentSlider - 1].setAttribute(&quot;data-hidden&quot;, &quot;false&quot;);
    sliderElements[currentSlider - 1].classList.add(&quot;custom--animation&quot;);
  });
});

// WHEN CLICK ON FIRST CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client1() {
  document.getElementById(&quot;client1&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube1&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}

// WHEN CLICK ON SECOND CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client2() {
  document.getElementById(&quot;client2&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube2&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}

// WHEN CLICK ON THIRD CLIENT IMAGE THIS WILL LOAD THE APPROPIATE VIDEO
function client3() {
  document.getElementById(&quot;client3&quot;).style.display = &quot;none&quot;;
  document.getElementById(
    &quot;youtube3&quot;
  ).innerHTML = `&lt;iframe allow=&quot;autoplay&quot; id=&quot;client1Iframe&quot; width=&quot;100%&quot; height=&quot;410&quot; src=&quot;https://www.youtube.com/embed/FELPQq0ws3o?autoplay=1&amp;rel=0&quot; frameborder=&quot;0&quot; allowfullscreen=&quot;true&quot; style=&quot;margin-bottom:-16px&quot;>&lt;/iframe>`;
}






// const myCarouselElement = document.querySelector(&quot;#autoSlider&quot;);

// myCarouselElement.addEventListener(&quot;slide.bs.carousel&quot;, (event) => {
//   document.querySelector(&quot;.carousel-item.active&quot;).classList.remove(&quot;show&quot;);
// });

// myCarouselElement.addEventListener(&quot;slid.bs.carousel&quot;, (event) => {
//   document.querySelector(&quot;.carousel-item.active&quot;).classList.add(&quot;show&quot;);
// });

// FIRST SLDIER CARD DATA
const firstSlider = [

  {
    url: &quot;https://tazkira.ae/&quot;,
    imgSrc: &quot;&quot;
  },


];

// THIS TEMPLATE CONATINS THE HTML FOR EACH SLDIE CARD
// const sliderCard = document.querySelector(&quot;#sliderCard&quot;);
// GETTING THE SLIDER ONE WRAPPER
// const carouselItem1 = document.querySelector(&quot;.carousel--item-1 > div.d-flex&quot;);
// CALLING THE CREATE SLDIER CARD FUNCTION
// createSliderCard(firstSlider, carouselItem1);

// SECOND SLIDER CARD DATA
// const sectSlider = [
//   {

//   }

// ];

// GETTING THE SECOND SLIDER WRAPPER
// const carouselItem2 = document.querySelector(&quot;.carousel--item-2 > div.d-flex&quot;);
// CALLING THE CREATE SLDIER CARD FUNCTION
// createSliderCard(sectSlider, carouselItem2);

// THIS FUNCTION WILL CRETAE THE CARD FOR EACH SLIDER AND APPEND IN THE SPECIFIC SLIDER WRAPPER
// function createSliderCard(cardData, cardPID) {
//   let _slideItem;
//   cardData.forEach((_slide) => {
//     _slideItem = sliderCard.content.cloneNode(true);

//     _slideItem.querySelector(&quot;a&quot;).setAttribute(&quot;href&quot;, _slide.url);
//     _slideItem.querySelector(&quot;a > img&quot;).setAttribute(&quot;src&quot;, _slide.imgSrc);
//     _slideItem.querySelector(&quot;a > img&quot;).setAttribute(&quot;href&quot;, _slide.alt);

//     cardPID.appendChild(_slideItem);
//   });
// }




/* Modern Newsletter Section */
.modern-newsletter {
    background: linear-gradient(135deg, #1a1a1a 0%, #2d2d2d 100%);
    padding: 4rem 0;
    position: relative;
    overflow: hidden;
}

.modern-newsletter::before {
    content: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    position: absolute;
    top: 0;
    right: 0;
    width: 100%;
    height: 100%;
    background: url(&quot; , &quot;'&quot; , &quot;data:image/svg+xml,&lt;svg xmlns=&quot;http://www.w3.org/2000/svg&quot; viewBox=&quot;0 0 100 100&quot;>&lt;defs>&lt;pattern id=&quot;grid&quot; width=&quot;10&quot; height=&quot;10&quot; patternUnits=&quot;userSpaceOnUse&quot;>&lt;path d=&quot;M 10 0 L 0 0 0 10&quot; fill=&quot;none&quot; stroke=&quot;rgba(255,255,255,0.05)&quot; stroke-width=&quot;1&quot;/>&lt;/pattern>&lt;/defs>&lt;rect width=&quot;100&quot; height=&quot;100&quot; fill=&quot;url(%23grid)&quot;/>&lt;/svg>&quot; , &quot;'&quot; , &quot;) repeat;
    pointer-events: none;
}

.newsletter-content {
    position: relative;
    z-index: 2;
}

.newsletter-icon {
    width: 60px;
    height: 60px;
    margin-bottom: 1rem;
}

.newsletter-title {
    font-size: 2rem;
    font-weight: 700;
    color: white;
    margin-bottom: 0.5rem;
}

.newsletter-subtitle {
    color: rgba(255,255,255,0.8);
    font-size: 1.1rem;
    margin-bottom: 2rem;
}

.newsletter-form {
    display: flex;
    gap: 1rem;
    max-width: 500px;
    margin: 0 auto;
}

.newsletter-input {
    flex: 1;
    padding: 1rem 1.5rem;
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 8px;
    background: rgba(255,255,255,0.1);
    color: white;
    font-size: 1rem;
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
}

.newsletter-input::placeholder {
    color: rgba(255,255,255,0.6);
}

.newsletter-input:focus {
    outline: none;
    border-color: #007bff;
    background: rgba(255,255,255,0.15);
}

.newsletter-btn {
    padding: 1rem 2rem;
    background: #007bff;
    border: none;
    border-radius: 8px;
    color: white;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;
}

.newsletter-btn:hover {
    background: #0056b3;
    transform: translateY(-2px);
}

/* Modern Footer */
.modern-footer {
    background: #0a0a0a;
    color: white;
    padding: 4rem 0 0;
    position: relative;
}

.footer-main {
    padding-bottom: 3rem;
    border-bottom: 1px solid rgba(255,255,255,0.1);
}

.footer-brand {
    margin-bottom: 2rem;
}

.brand-logo-container {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
}

.brand-logo {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    object-fit: cover;
}

.brand-text h3 {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
    color: white;
}

.brand-text p {
    font-size: 0.9rem;
    color: rgba(255,255,255,0.7);
    margin: 0;
}

.brand-description {
    color: rgba(255,255,255,0.7);
    line-height: 1.6;
    margin-bottom: 2rem;
    max-width: 350px;
}

.social-links {
    display: flex;
    gap: 1rem;
}

.social-link {
    width: 44px;
    height: 44px;
    background: rgba(255,255,255,0.1);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    text-decoration: none;
}

.social-link:hover {
    background: #007bff;
    transform: translateY(-2px);
}

.social-link svg {
    width: 20px;
    height: 20px;
    fill: white;
}

.footer-section {
    margin-bottom: 2rem;
}

.footer-section h4 {
    font-size: 1.1rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: white;
    position: relative;
}

.footer-section h4::after {
    content: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    position: absolute;
    bottom: -8px;
    left: 0;
    width: 30px;
    height: 2px;
    background: #007bff;
}

.footer-links {
    list-style: none;
    padding: 0;
    margin: 0;
}

.footer-links li {
    margin-bottom: 0.75rem;
}

.footer-links a {
    color: rgba(255,255,255,0.7);
    text-decoration: none;
    transition: color 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
}

.footer-links a:hover {
    color: #007bff;
}

.hiring-badge {
    background: linear-gradient(45deg, #ff4757, #ff6b7a);
    color: white;
    font-size: 0.7rem;
    padding: 0.2rem 0.5rem;
    border-radius: 12px;
    font-weight: 600;
    text-transform: uppercase;
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
}

.payment-section {
    background: rgba(255,255,255,0.03);
    padding: 2rem;
    border-radius: 12px;
    margin-bottom: 2rem;
}

.payment-title {
    font-weight: 600;
    margin-bottom: 1rem;
    color: white;
}

.payment-description {
    color: rgba(255,255,255,0.7);
    line-height: 1.6;
    margin-bottom: 1.5rem;
    font-size: 0.9rem;
}

.payment-gateway-image {
    max-width: 100%;
    height: auto;
    opacity: 0.8;
    transition: opacity 0.3s ease;
}

.payment-gateway-image:hover {
    opacity: 1;
}

.certifications {
    display: flex;
    gap: 2rem;
    align-items: center;
}

.cert-item img {
    /* max-width: 100px; */
    height: auto;
    opacity: 0.7;
    transition: opacity 0.3s ease;
}

.cert-item img:hover {
    opacity: 1;
}

.footer-bottom {
    padding: 2rem 0;
    background: rgba(0,0,0,0.5);
}

.footer-bottom-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
}

.copyright {
    color: rgba(255,255,255,0.6);
    font-size: 0.9rem;
}

.legal-links {
    display: flex;
    gap: 2rem;
    flex-wrap: wrap;
}

.legal-links a {
    color: rgba(255,255,255,0.6);
    text-decoration: none;
    font-size: 0.9rem;
    transition: color 0.3s ease;
}

.legal-links a:hover {
    color: #007bff;
}

.whatsapp-float {
    position: fixed;
    bottom: 2rem;
    right: 2rem;
    z-index: 1000;
    animation: float 3s ease-in-out infinite;
}

.whatsapp-btn {
    width: 60px;
    height: 60px;
    background: linear-gradient(135deg, #25d366, #128c7e);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
    box-shadow: 0 4px 20px rgba(37, 211, 102, 0.4);
    transition: all 0.3s ease;
}

.whatsapp-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 6px 25px rgba(37, 211, 102, 0.6);
}

.whatsapp-btn svg {
    width: 30px;
    height: 30px;
    fill: white;
}

@keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
}

/* Responsive Design */
@media (max-width: 768px) {
    .modern-newsletter {
        padding: 3rem 0;
        text-align: center;
    }
    
    .newsletter-title {
        font-size: 1.5rem;
    }
    
    .newsletter-form {
        flex-direction: column;
    }
    
    .modern-footer {
        padding: 3rem 0 0;
    }
    
    .footer-bottom-content {
        flex-direction: column;
        text-align: center;
    }
    
    .legal-links {
        justify-content: center;
    }
    
    .certifications {
        flex-direction: column;
        gap: 1rem;
    }
    
    .social-links {
        justify-content: center;
    }
}

@media (max-width: 576px) {
    .brand-logo-container {
        flex-direction: column;
        text-align: center;
    }
    
    .legal-links {
        flex-direction: column;
        gap: 1rem;
    }
    
    .whatsapp-float {
        bottom: 1rem;
        right: 1rem;
    }
    
    .whatsapp-btn {
        width: 50px;
        height: 50px;
    }
    
    .whatsapp-btn svg {
        width: 25px;
        height: 25px;
    }
}




    
        
            
                
            
            Stay Updated
            Get the latest updates on travel technology and industry insights
            
                
                Subscribe
            
        
    




    
        
            
                
                
                    
                        
                            
                            
                                PHPTRAVELS
                                Travel Tech Partner
                            
                        
                        Leading travel technology solutions for modern businesses. Build, customize, and scale your travel platform with our comprehensive booking engine.
                        
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                            
                                                                            
                                                                    
                                                    
                    
                

                
                                    
                        
                            Product
                            
                                                                    
                                        
                                            Demo                                                                                    
                                    
                                                                    
                                        
                                            Mobile Apps                                                                                    
                                    
                                                                    
                                        
                                            Pricing                                                                                    
                                    
                                                                    
                                        
                                            Features                                                                                    
                                    
                                                                    
                                        
                                            Technology                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Resources
                            
                                                                    
                                        
                                            Changelog                                                                                    
                                    
                                                                    
                                        
                                            Updates                                                                                    
                                    
                                                                    
                                        
                                            Requirements                                                                                    
                                    
                                                                    
                                        
                                            Affiliate                                                                                    
                                    
                                                                    
                                        
                                            Road Map                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Services
                            
                                                                    
                                        
                                            Product Services                                                                                    
                                    
                                                                    
                                        
                                            Customizations                                                                                    
                                    
                                                                    
                                        
                                            EAN Integration                                                                                    
                                    
                                                                    
                                        
                                            Cloud Hosting                                                                                    
                                    
                                                                    
                                        
                                            Support                                                                                    
                                    
                                                            
                        
                    
                                    
                        
                            Company
                            
                                                                    
                                        
                                            About Us                                                                                    
                                    
                                                                    
                                        
                                            Blog                                                                                    
                                    
                                                                    
                                        
                                            The Team                                                                                    
                                    
                                                                    
                                        
                                            Contact Us                                                                                    
                                    
                                                                    
                                        
                                            Jobs                                                                                            We&quot; , &quot;'&quot; , &quot;re Hiring!
                                                                                    
                                    
                                                            
                        
                    
                            

            
            
                
                    
                        Accepted Payment Methods
                        
                            We support all major payment gateways including PayPal, Credit Cards, Western Union, Skrill, and Transferwise for your convenience.
                        
                        
                    
                
                
                    
                        
                            
                        
                        
                            
                        
                    
                
            
        
    

    
    
        
            
                
                    © 2023 PHPTRAVELS. All Rights Reserved.
                
                
                                            
                            Privacy Policy                        
                                            
                            Terms of Service                        
                                            
                            Abuse Policy                        
                                            
                            Live Chat                        
                                            
                            Content Program                        
                                    
            
        
    

 









   
   
   

   

   
   setTimeout(function() {
      window.__lc = window.__lc || {};
      window.__lc.license = 4618001;
      ;(function(n,t,c){function i(n){return e._h?e._h.apply(null,n):e._q.push(n)}var e={_q:[],_h:null,_v:&quot;2.0&quot;,on:function(){i([&quot;on&quot;,c.call(arguments)])},once:function(){i([&quot;once&quot;,c.call(arguments)])},off:function(){i([&quot;off&quot;,c.call(arguments)])},get:function(){if(!e._h)throw new Error(&quot;[LiveChatWidget] You can&quot; , &quot;'&quot; , &quot;t use getters before load.&quot;);return i([&quot;get&quot;,c.call(arguments)])},call:function(){i([&quot;call&quot;,c.call(arguments)])},init:function(){var n=t.createElement(&quot;script&quot;);n.async=!0,n.type=&quot;text/javascript&quot;,n.src=&quot;https://cdn.livechatinc.com/tracking.js&quot;,t.head.appendChild(n)}};!n.__lc.asyncInit&amp;&amp;e.init(),n.LiveChatWidget=n.LiveChatWidget||e}(window,document,[].slice))
   }, 5000);
   





// import Swup from &quot; , &quot;'&quot; , &quot;https://unpkg.com/swup@4?module&quot; , &quot;'&quot; , &quot;;
// const swup = new Swup();







   // document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, function () {
   //    let exitIntentShown = false;

   //    // Check if the user has opted out of seeing the modal for the next 90 days
   //    const isOptedOut = localStorage.getItem(&quot; , &quot;'&quot; , &quot;exitIntentOptOut&quot; , &quot;'&quot; , &quot;);
   //    const optOutExpiration = localStorage.getItem(&quot; , &quot;'&quot; , &quot;optOutExpiration&quot; , &quot;'&quot; , &quot;);

   //    if (isOptedOut &amp;&amp; optOutExpiration) {
   //       const expirationDate = new Date(optOutExpiration);
   //       const currentDate = new Date();
   //       if (currentDate &lt; expirationDate) {
   //          // User has opted out and the 90-day period has not expired
   //          exitIntentShown = true;
   //       }
   //    }

   //    document.addEventListener(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function (e) {
   //       if (e.clientY &lt; 0 &amp;&amp; !exitIntentShown) {
   //          const exitModal = new bootstrap.Modal(document.getElementById(&quot; , &quot;'&quot; , &quot;exitModal&quot; , &quot;'&quot; , &quot;));
   //          exitModal.show();
   //          exitIntentShown = true;

   //          // Reset the exit intent flag when modal is closed
   //          document.getElementById(&quot; , &quot;'&quot; , &quot;exitModal&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;hidden.bs.modal&quot; , &quot;'&quot; , &quot;, function () {
   //             exitIntentShown = false;
   //          });
   //       }
   //    });

   //    document.getElementById(&quot; , &quot;'&quot; , &quot;dontShowAgainBtn&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
   //       // Set a flag in localStorage to indicate that the user opted out
   //       const expirationDate = new Date();
   //       expirationDate.setDate(expirationDate.getDate() + 90); // Set expiration date for 90 days
   //       localStorage.setItem(&quot; , &quot;'&quot; , &quot;exitIntentOptOut&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;);
   //       localStorage.setItem(&quot; , &quot;'&quot; , &quot;optOutExpiration&quot; , &quot;'&quot; , &quot;, expirationDate.toISOString()); // Store the expiration date
   //       // Hide the modal
   //       const exitModal = new bootstrap.Modal(document.getElementById(&quot; , &quot;'&quot; , &quot;exitModal&quot; , &quot;'&quot; , &quot;));
   //       exitModal.hide();
   //    });
   // });





// Get the current URL parameters
function getQueryParam(param) {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(param);
}

// Update all links with the referral ID
function updateLinksWithRef(refID) {
    if (!refID) return; // If no valid refID, don&quot; , &quot;'&quot; , &quot;t update links

    const links = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;);

    links.forEach(link => {
        const url = new URL(link.href);

        // Append the ref parameter to the URL only if it doesn&quot; , &quot;'&quot; , &quot;t already exist
        if (!url.searchParams.has(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;)) {
            url.searchParams.append(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;, refID);
        }

        link.href = url.toString();
    });
}

// Check if the URL has a ref parameter
const refID = getQueryParam(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;);
if (refID &amp;&amp; refID !== &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;) {
    // Store the refID in sessionStorage so it persists across page loads
    sessionStorage.setItem(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;, refID);
}

// Ensure that links include the ref parameter only if it&quot; , &quot;'&quot; , &quot;s present in the session or the URL
const storedRef = sessionStorage.getItem(&quot; , &quot;'&quot; , &quot;ref&quot; , &quot;'&quot; , &quot;);
if (storedRef &amp;&amp; storedRef !== &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;) {
    updateLinksWithRef(storedRef);
}






// SHOW - HIDE MENU ON MOBILE
function togglemenu() { var element = document.getElementById(&quot;responsive-menu&quot;); element.classList.toggle(&quot;show-menu&quot;);  }
function anchor() { var element = document.getElementById(&quot;responsive-menu&quot;); element.classList.remove(&quot;show-menu&quot;); }

/*! Waves v0.7.6  */
(function(window,factory){&quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;if(typeof define===&quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;&amp;&amp;define.amd){define([],function(){window.Waves=factory.call(window);return window.Waves})}else if(typeof exports===&quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;){module.exports=factory.call(window)}else{window.Waves=factory.call(window)}})(typeof global===&quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;?global:this,function(){&quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;var Waves=Waves||{};var $$=document.querySelectorAll.bind(document);var toString=Object.prototype.toString;var isTouchAvailable=&quot; , &quot;'&quot; , &quot;ontouchstart&quot; , &quot;'&quot; , &quot; in window;function isWindow(obj){return obj!==null&amp;&amp;obj===obj.window}
function getWindow(elem){return isWindow(elem)?elem:elem.nodeType===9&amp;&amp;elem.defaultView}
function isObject(value){var type=typeof value;return type===&quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;||type===&quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;&amp;&amp;!!value}
function isDOMNode(obj){return isObject(obj)&amp;&amp;obj.nodeType>0}
function getWavesElements(nodes){var stringRepr=toString.call(nodes);if(stringRepr===&quot; , &quot;'&quot; , &quot;[object String]&quot; , &quot;'&quot; , &quot;){return $$(nodes)}else if(isObject(nodes)&amp;&amp;/^\[object (Array|HTMLCollection|NodeList|Object)\]$/.test(stringRepr)&amp;&amp;nodes.hasOwnProperty(&quot; , &quot;'&quot; , &quot;length&quot; , &quot;'&quot; , &quot;)){return nodes}else if(isDOMNode(nodes)){return[nodes]}
return[]}
function offset(elem){var docElem,win,box={top:0,left:0},doc=elem&amp;&amp;elem.ownerDocument;docElem=doc.documentElement;if(typeof elem.getBoundingClientRect!==typeof undefined){box=elem.getBoundingClientRect()}
win=getWindow(doc);return{top:box.top+win.pageYOffset-docElem.clientTop,left:box.left+win.pageXOffset-docElem.clientLeft}}
function convertStyle(styleObj){var style=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;for(var prop in styleObj){if(styleObj.hasOwnProperty(prop)){style+=(prop+&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;+styleObj[prop]+&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;)}}
return style}
var Effect={duration:750,delay:200,show:function(e,element,velocity){if(e.button===2){return!1}
element=element||this;var ripple=document.createElement(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;);ripple.className=&quot; , &quot;'&quot; , &quot;waves-ripple waves-rippling&quot; , &quot;'&quot; , &quot;;element.appendChild(ripple);var pos=offset(element);var relativeY=0;var relativeX=0;if(&quot; , &quot;'&quot; , &quot;touches&quot; , &quot;'&quot; , &quot; in e&amp;&amp;e.touches.length){relativeY=(e.touches[0].pageY-pos.top);relativeX=(e.touches[0].pageX-pos.left)}else{relativeY=(e.pageY-pos.top);relativeX=(e.pageX-pos.left)}
relativeX=relativeX>=0?relativeX:0;relativeY=relativeY>=0?relativeY:0;var scale=&quot; , &quot;'&quot; , &quot;scale(&quot; , &quot;'&quot; , &quot;+((element.clientWidth/100)*3)+&quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot;;var translate=&quot; , &quot;'&quot; , &quot;translate(0,0)&quot; , &quot;'&quot; , &quot;;if(velocity){translate=&quot; , &quot;'&quot; , &quot;translate(&quot; , &quot;'&quot; , &quot;+(velocity.x)+&quot; , &quot;'&quot; , &quot;px, &quot; , &quot;'&quot; , &quot;+(velocity.y)+&quot; , &quot;'&quot; , &quot;px)&quot; , &quot;'&quot; , &quot;}
ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-hold&quot; , &quot;'&quot; , &quot;,Date.now());ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-x&quot; , &quot;'&quot; , &quot;,relativeX);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-y&quot; , &quot;'&quot; , &quot;,relativeY);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-scale&quot; , &quot;'&quot; , &quot;,scale);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;data-translate&quot; , &quot;'&quot; , &quot;,translate);var rippleStyle={top:relativeY+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;,left:relativeX+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;};ripple.classList.add(&quot; , &quot;'&quot; , &quot;waves-notransition&quot; , &quot;'&quot; , &quot;);ripple.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,convertStyle(rippleStyle));ripple.classList.remove(&quot; , &quot;'&quot; , &quot;waves-notransition&quot; , &quot;'&quot; , &quot;);rippleStyle[&quot; , &quot;'&quot; , &quot;-webkit-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle[&quot; , &quot;'&quot; , &quot;-moz-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle[&quot; , &quot;'&quot; , &quot;-ms-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle[&quot; , &quot;'&quot; , &quot;-o-transform&quot; , &quot;'&quot; , &quot;]=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle.transform=scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate;rippleStyle.opacity=&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;var duration=e.type===&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;?2500:Effect.duration;rippleStyle[&quot; , &quot;'&quot; , &quot;-webkit-transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;rippleStyle[&quot; , &quot;'&quot; , &quot;-moz-transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;rippleStyle[&quot; , &quot;'&quot; , &quot;-o-transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;rippleStyle[&quot; , &quot;'&quot; , &quot;transition-duration&quot; , &quot;'&quot; , &quot;]=duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;;ripple.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,convertStyle(rippleStyle))},hide:function(e,element){element=element||this;var ripples=element.getElementsByClassName(&quot; , &quot;'&quot; , &quot;waves-rippling&quot; , &quot;'&quot; , &quot;);for(var i=0,len=ripples.length;i&lt;len;i++){removeRipple(e,element,ripples[i])}
if(isTouchAvailable){element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,Effect.hide);element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,Effect.hide)}
element.removeEventListener(&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,Effect.hide);element.removeEventListener(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;,Effect.hide)}};var TagWrapper={input:function(element){var parent=element.parentNode;if(parent.tagName.toLowerCase()===&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;&amp;&amp;parent.classList.contains(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)){return}
var wrapper=document.createElement(&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;);wrapper.className=element.className+&quot; , &quot;'&quot; , &quot; waves-input-wrapper&quot; , &quot;'&quot; , &quot;;element.className=&quot; , &quot;'&quot; , &quot;waves-button-input&quot; , &quot;'&quot; , &quot;;parent.replaceChild(wrapper,element);wrapper.appendChild(element);var elementStyle=window.getComputedStyle(element,null);var color=elementStyle.color;var backgroundColor=elementStyle.backgroundColor;wrapper.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;color:&quot; , &quot;'&quot; , &quot;+color+&quot; , &quot;'&quot; , &quot;;background:&quot; , &quot;'&quot; , &quot;+backgroundColor);element.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;background-color:rgba(0,0,0,0);&quot; , &quot;'&quot; , &quot;)},img:function(element){var parent=element.parentNode;if(parent.tagName.toLowerCase()===&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;&amp;&amp;parent.classList.contains(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)){return}
var wrapper=document.createElement(&quot; , &quot;'&quot; , &quot;i&quot; , &quot;'&quot; , &quot;);parent.replaceChild(wrapper,element);wrapper.appendChild(element)}};function removeRipple(e,el,ripple){if(!ripple){return}
ripple.classList.remove(&quot; , &quot;'&quot; , &quot;waves-rippling&quot; , &quot;'&quot; , &quot;);var relativeX=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-x&quot; , &quot;'&quot; , &quot;);var relativeY=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-y&quot; , &quot;'&quot; , &quot;);var scale=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-scale&quot; , &quot;'&quot; , &quot;);var translate=ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-translate&quot; , &quot;'&quot; , &quot;);var diff=Date.now()-Number(ripple.getAttribute(&quot; , &quot;'&quot; , &quot;data-hold&quot; , &quot;'&quot; , &quot;));var delay=350-diff;if(delay&lt;0){delay=0}
if(e.type===&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;){delay=150}
var duration=e.type===&quot; , &quot;'&quot; , &quot;mousemove&quot; , &quot;'&quot; , &quot;?2500:Effect.duration;setTimeout(function(){var style={top:relativeY+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;,left:relativeX+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;,opacity:&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-webkit-transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-moz-transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-o-transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;transition-duration&quot; , &quot;'&quot; , &quot;:duration+&quot; , &quot;'&quot; , &quot;ms&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;-webkit-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;-moz-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;-ms-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;-o-transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate,&quot; , &quot;'&quot; , &quot;transform&quot; , &quot;'&quot; , &quot;:scale+&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+translate};ripple.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,convertStyle(style));setTimeout(function(){try{el.removeChild(ripple)}catch(e){return!1}},duration)},delay)}
var TouchHandler={touches:0,allowEvent:function(e){var allow=!0;if(/^(mousedown|mousemove)$/.test(e.type)&amp;&amp;TouchHandler.touches){allow=!1}
return allow},registerEvent:function(e){var eType=e.type;if(eType===&quot; , &quot;'&quot; , &quot;touchstart&quot; , &quot;'&quot; , &quot;){TouchHandler.touches+=1}else if(/^(touchend|touchcancel)$/.test(eType)){setTimeout(function(){if(TouchHandler.touches){TouchHandler.touches-=1}},500)}}};function getWavesEffectElement(e){if(TouchHandler.allowEvent(e)===!1){return null}
var element=null;var target=e.target||e.srcElement;while(target.parentElement){if((!(target instanceof SVGElement))&amp;&amp;target.classList.contains(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)){element=target;break}
target=target.parentElement}
return element}
function showEffect(e){var element=getWavesEffectElement(e);if(element!==null){if(element.disabled||element.getAttribute(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;)||element.classList.contains(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;)){return}
TouchHandler.registerEvent(e);if(e.type===&quot; , &quot;'&quot; , &quot;touchstart&quot; , &quot;'&quot; , &quot;&amp;&amp;Effect.delay){var hidden=!1;var timer=setTimeout(function(){timer=null;Effect.show(e,element)},Effect.delay);var hideEffect=function(hideEvent){if(timer){clearTimeout(timer);timer=null;Effect.show(e,element)}
if(!hidden){hidden=!0;Effect.hide(hideEvent,element)}
removeListeners()};var touchMove=function(moveEvent){if(timer){clearTimeout(timer);timer=null}
hideEffect(moveEvent);removeListeners()};element.addEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;,touchMove,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,hideEffect,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,hideEffect,!1);var removeListeners=function(){element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;,touchMove);element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,hideEffect);element.removeEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,hideEffect)}}else{Effect.show(e,element);if(isTouchAvailable){element.addEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,Effect.hide,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,Effect.hide,!1)}
element.addEventListener(&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,Effect.hide,!1);element.addEventListener(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;,Effect.hide,!1)}}}
Waves.init=function(options){var body=document.body;options=options||{};if(&quot; , &quot;'&quot; , &quot;duration&quot; , &quot;'&quot; , &quot; in options){Effect.duration=options.duration}
if(&quot; , &quot;'&quot; , &quot;delay&quot; , &quot;'&quot; , &quot; in options){Effect.delay=options.delay}
if(isTouchAvailable){body.addEventListener(&quot; , &quot;'&quot; , &quot;touchstart&quot; , &quot;'&quot; , &quot;,showEffect,!1);body.addEventListener(&quot; , &quot;'&quot; , &quot;touchcancel&quot; , &quot;'&quot; , &quot;,TouchHandler.registerEvent,!1);body.addEventListener(&quot; , &quot;'&quot; , &quot;touchend&quot; , &quot;'&quot; , &quot;,TouchHandler.registerEvent,!1)}
body.addEventListener(&quot; , &quot;'&quot; , &quot;mousedown&quot; , &quot;'&quot; , &quot;,showEffect,!1)};Waves.attach=function(elements,classes){elements=getWavesElements(elements);if(toString.call(classes)===&quot; , &quot;'&quot; , &quot;[object Array]&quot; , &quot;'&quot; , &quot;){classes=classes.join(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;)}
classes=classes?&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;+classes:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;var element,tagName;for(var i=0,len=elements.length;i&lt;len;i++){element=elements[i];tagName=element.tagName.toLowerCase();if([&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;img&quot; , &quot;'&quot; , &quot;].indexOf(tagName)!==-1){TagWrapper[tagName](element);element=element.parentElement}
if(element.className.indexOf(&quot; , &quot;'&quot; , &quot;waves-effect&quot; , &quot;'&quot; , &quot;)===-1){element.className+=&quot; , &quot;'&quot; , &quot; waves-effect&quot; , &quot;'&quot; , &quot;+classes}}};Waves.ripple=function(elements,options){elements=getWavesElements(elements);var elementsLen=elements.length;options=options||{};options.wait=options.wait||0;options.position=options.position||null;if(elementsLen){var element,pos,off,centre={},i=0;var mousedown={type:&quot; , &quot;'&quot; , &quot;mousedown&quot; , &quot;'&quot; , &quot;,button:1};var hideRipple=function(mouseup,element){return function(){Effect.hide(mouseup,element)}};for(;i&lt;elementsLen;i++){element=elements[i];pos=options.position||{x:element.clientWidth/2,y:element.clientHeight/2};off=offset(element);centre.x=off.left+pos.x;centre.y=off.top+pos.y;mousedown.pageX=centre.x;mousedown.pageY=centre.y;Effect.show(mousedown,element);if(options.wait>=0&amp;&amp;options.wait!==null){var mouseup={type:&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,button:1};setTimeout(hideRipple(mouseup,element),options.wait)}}}};Waves.calm=function(elements){elements=getWavesElements(elements);var mouseup={type:&quot; , &quot;'&quot; , &quot;mouseup&quot; , &quot;'&quot; , &quot;,button:1};for(var i=0,len=elements.length;i&lt;len;i++){Effect.hide(mouseup,elements[i])}};Waves.displayEffect=function(options){console.error(&quot; , &quot;'&quot; , &quot;Waves.displayEffect() has been deprecated and will be removed in future version. Please use Waves.init() to initialize Waves effect&quot; , &quot;'&quot; , &quot;);Waves.init(options)};return Waves})

Waves.init();
Waves.attach(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;);
Waves.attach(&quot; , &quot;'&quot; , &quot;.logo&quot; , &quot;'&quot; , &quot;);

// CONSOLE
console.log(&quot;%cHi there! 👋 We are phptravels. we are disrupting the travel industry world-wide.&quot;, &quot;font-size:14px&quot;);
console.log(&quot;%cAny doubts, get in touch at info@phptravels.com&quot;, &quot;font-size:12px&quot;);



(function(){function c(){var b=a.contentDocument||a.contentWindow.document;if(b){var d=b.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);d.innerHTML=&quot;window.__CF$cv$params={r:&quot; , &quot;'&quot; , &quot;95d87a4f391d712c&quot; , &quot;'&quot; , &quot;,t:&quot; , &quot;'&quot; , &quot;MTc1MjIzODc2My4wMDAwMDA=&quot; , &quot;'&quot; , &quot;};var a=document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);a.nonce=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;a.src=&quot; , &quot;'&quot; , &quot;/cdn-cgi/challenge-platform/scripts/jsd/main.js&quot; , &quot;'&quot; , &quot;;document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(a);&quot;;b.getElementsByTagName(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;)[0].appendChild(d)}}if(document.body){var a=document.createElement(&quot; , &quot;'&quot; , &quot;iframe&quot; , &quot;'&quot; , &quot;);a.height=1;a.width=1;a.style.position=&quot; , &quot;'&quot; , &quot;absolute&quot; , &quot;'&quot; , &quot;;a.style.top=0;a.style.left=0;a.style.border=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;a.style.visibility=&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;;document.body.appendChild(a);if(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState)c();else if(window.addEventListener)document.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;,c);else{var e=document.onreadystatechange||function(){};document.onreadystatechange=function(b){e(b);&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;!==document.readyState&amp;&amp;(document.onreadystatechange=e,c())}}}})();

  /html[1]&quot;))]</value>
      <webElementGuid>8d762075-8de0-46b4-b33d-773fb9bdbf3d</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
