
             fn main(){let mut
         t=0.;use std::print as P;
       let q=|x:f32|x.sin_cos();P!("
     \x1b[2J");loop{let mut b=[(0.,32)
   ;1760];P!("\x1b[H");let((e,g),(n,m))=
 (q(t*2.),q(t));for v in 0..90{let(f,d)=
 q(v as f32/14.);for u in 0..314{let h=2.+
 d;let(c,l)=q(u as f32/50.);let(k,j)=(1./(
 c*h*e+f*g+5.),c         *h*g-f*e);let w=|
r,p,q|(r*k*(l*h           *p+j*q));let(x,y)
 =(40+w(30.,m,-n)         as i32,12+w(15.,
 n,m)as i32);let i=(x+80*y)as usize;if 22>
 y&&y|x>0&&80>x&&k>b[i].0{let j=(4.9*((f*
  e-c*d*g)*m-c*d*e-f*g-l*d*n)).max(0.) as
   usize;b[i]=(k,b" ':riaM"[j]);}}}for i
     in 0..1760{P!("{}",if i%80>0{b[i]
       .1}else{10}as char);}t+=0.002
         ;}}/* port by Rachel K */
             /*    2021     */

