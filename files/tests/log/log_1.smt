(set-option :print-success false)
; resolve_perf truncated k=1
; ---------- Constant ----------
(declare-const i Int)
(declare-const prop Bool)
; ---------- State 0 ----------
(declare-const j__0 Int)
; _0 = (F (j = (i)))
(declare-const _0__0 Bool)
; ---------- State 1 ----------
(declare-const j__1 Int)
(declare-const _0__1 Bool)
; ---------- Init ----------
(assert (and (>= i 0) (>= j__0 0) (< j__0 i)))
(assert (= prop _0__0))
; ---------- Invariant 0 ----------
; ---------- Invariant 1 ----------
; ---------- Transition 0 ----------
(assert (= j__1 (+ j__0 i)))
; ---------- LTL 0 ----------
(assert (= _0__0 (or (= j__0 (* i)) _0__1)))
; ---------- LTL finite 1 ----------
(assert (= _0__1 (= j__1 (* i))))
; ---------- Add Property ----------
(assert prop)
; ---------- Check Sat ----------
(check-sat-using (then (repeat (then propagate-ineqs simplify propagate-values solve-eqs elim-uncnstr)) smt))
(eval i)
(eval prop)
(eval j__0)
(eval j__1)
(exit)
