(set-option :print-success false)
; resolve_perf truncated k=0
; ---------- Constant ----------
(declare-const i Int)
(declare-const prop Bool)
; ---------- State 0 ----------
(declare-const j__0 Int)
; _0 = (F (j = (i)))
(declare-const _0__0 Bool)
; ---------- Init ----------
(assert (and (>= i 0) (>= j__0 0) (< j__0 i)))
(assert (= prop _0__0))
; ---------- Invariant 0 ----------
; ---------- LTL finite 0 ----------
(assert (= _0__0 (= j__0 (* i))))
; ---------- Add Property ----------
(assert prop)
; ---------- Check Sat ----------
(check-sat-using (then (repeat (then propagate-ineqs simplify propagate-values solve-eqs elim-uncnstr)) smt))
(exit)
