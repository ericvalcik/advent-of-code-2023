// result: 32000000 signals
pub const EXAMPLE1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

// button -low-> broadcaster
// broadcaster -low-> a
// broadcaster -low-> b
// broadcaster -low-> c
// a -high-> b
// b -high-> c
// c -high-> inv
// inv -low-> a
// a -low-> b
// b -low-> c
// c -low-> inv
// inv -high-> a

// result: 11687500 signals
pub const EXAMPLE2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";


// button -low-> broadcaster k
// broadcaster -low-> a k
// a -high-> inv k
// a -high-> con k
// inv -low-> b
// con -high-> output
// b -high-> con
// con -low-> output

pub const INPUT: &str = "\
%jf -> cr, dn
%fd -> hm, jx
%mb -> bq, cr
&mr -> qt
%qd -> cr, dg
%rs -> hh
%xl -> gq, vj
%zl -> qn
%tj -> cr, qd
%fn -> hn
%qc -> tf
%fh -> jj, vj
&kk -> qt
%qn -> jx, fn
%hm -> jx, fq
%cm -> vj, fh
%jj -> vj, lp
%dr -> vj, qc
broadcaster -> db, hd, cm, xf
%fq -> jx, zk
%hd -> jx, zl
&qt -> rx
&vj -> bb, qc, cm
%tt -> bd
%sf -> jx
%rg -> nl, hr
%zk -> jx, sf
%lp -> cz, vj
%xf -> mb, cr
&cr -> dg, bq, kk, xf, gm
%nb -> vj, dr
%dg -> vm
%ql -> nl
&gl -> qt
&nl -> db, hr, mr, hh, hk, rs, bn
%ff -> ql, nl
%rb -> cr
%lc -> vj
%vm -> gm, cr
%tf -> vj, xl
%hr -> kf
%kf -> xx, nl
&bb -> qt
%ml -> nl, bn
%bq -> tj
%db -> ml, nl
%hn -> jx, tt
%dn -> cr, rb
%gm -> qs
%gq -> lc, vj
%hh -> rg
%bd -> fd
%xx -> nl, ff
%qs -> jf, cr
&jx -> fn, bd, tt, gl, zl, hd
%cz -> nb, vj
%bn -> hk
%hk -> rs";