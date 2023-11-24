use super::*;
use crate::error::*;
use crate::expr::*;
use crate::search::*;
use crate::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Model {
    enumerates: Vec<Enumerate>,
    intervals: Vec<Interval>,
    declarations: Vec<Declaration>,
    definitions: Vec<Definition>,
    fun_decs: Vec<FunDec>,
    fun_defs: Vec<FunDef>,
    //
    initials: Vec<Initial>,
    invariants: Vec<Invariant>,
    transitions: Vec<Transition>,
    //
    property: Option<Expr>,
    //
    search: Search,
    //
    ltl_variables: Vec<LTLVariable>,
}

impl Model {
    pub fn empty() -> Self {
        Self {
            enumerates: Vec::new(),
            intervals: Vec::new(),
            declarations: Vec::new(),
            definitions: Vec::new(),
            fun_decs: Vec::new(),
            fun_defs: Vec::new(),
            initials: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            property: None,
            search: Search::new(
                TransitionNumber::new(0, None),
                PathType::Initial,
                SearchType::Solve,
            ),
            ltl_variables: Vec::new(),
        }
    }

    //---------- Enumerate ----------

    pub fn enumerates(&self) -> &Vec<Enumerate> {
        &self.enumerates
    }

    pub fn add_enumerate(&mut self, enumerate: Enumerate) -> EnumerateId {
        let id = EnumerateId(self.enumerates.len());
        let mut enumerate = enumerate;
        enumerate.set_id(id);
        self.enumerates.push(enumerate);
        id
    }

    //---------- Interval ----------

    pub fn intervals(&self) -> &Vec<Interval> {
        &self.intervals
    }

    pub fn add_interval(&mut self, interval: Interval) -> IntervalId {
        let id = IntervalId(self.intervals.len());
        let mut interval = interval;
        interval.set_id(id);
        self.intervals.push(interval);
        id
    }

    //---------- Declaration ----------

    pub fn declarations(&self) -> &Vec<Declaration> {
        &self.declarations
    }

    pub fn add_declaration(&mut self, declaration: Declaration) -> DeclarationId {
        let id = DeclarationId(self.declarations.len());
        let mut declaration = declaration;
        declaration.set_id(id);
        self.declarations.push(declaration);
        id
    }

    pub fn cst_declaration_ids(&self) -> Vec<DeclarationId> {
        self.declarations()
            .iter()
            .filter_map(|d| if d.is_constant() { Some(d.id()) } else { None })
            .collect()
    }

    pub fn var_declaration_ids(&self) -> Vec<DeclarationId> {
        self.declarations()
            .iter()
            .filter_map(|d| if d.is_constant() { None } else { Some(d.id()) })
            .collect()
    }

    pub fn var_function_ids(&self) -> Vec<FunDecId> {
        self.fun_decs()
            .iter()
            .filter_map(|f| if f.is_constant() { None } else { Some(f.id()) })
            .collect()
    }

    //---------- Definition ----------

    pub fn definitions(&self) -> &Vec<Definition> {
        &self.definitions
    }

    pub fn add_definition(&mut self, definition: Definition) -> DefinitionId {
        let id = DefinitionId(self.definitions.len());
        let mut definition = definition;
        definition.set_id(id);
        self.definitions.push(definition);
        id
    }

    //---------- FunDec ----------

    pub fn fun_decs(&self) -> &Vec<FunDec> {
        &self.fun_decs
    }

    pub fn add_fun_dec(&mut self, fun_dec: FunDec) -> FunDecId {
        let id = FunDecId(self.fun_decs.len());
        let mut fun_dec = fun_dec;
        fun_dec.set_id(id);
        self.fun_decs.push(fun_dec);
        id
    }

    //---------- FunDef ----------

    pub fn fun_defs(&self) -> &Vec<FunDef> {
        &self.fun_defs
    }

    pub fn add_fun_def(&mut self, fun_def: FunDef) -> FunDefId {
        let id = FunDefId(self.fun_defs.len());
        let mut fun_def = fun_def;
        fun_def.set_id(id);
        self.fun_defs.push(fun_def);
        id
    }

    //---------- Initial ----------

    pub fn initials(&self) -> &Vec<Initial> {
        &self.initials
    }

    pub fn add_initial(&mut self, initial: Initial) -> InitialId {
        let id = InitialId(self.initials.len());
        let mut initial = initial;
        initial.set_id(id);
        self.initials.push(initial);
        id
    }

    //---------- Invariant ----------

    pub fn invariants(&self) -> &Vec<Invariant> {
        &self.invariants
    }

    pub fn add_invariant(&mut self, invariant: Invariant) -> InvariantId {
        let id = InvariantId(self.invariants.len());
        let mut invariant = invariant;
        invariant.set_id(id);
        self.invariants.push(invariant);
        id
    }

    //---------- Transition ----------

    pub fn transitions(&self) -> &Vec<Transition> {
        &self.transitions
    }

    pub fn add_transition(&mut self, transition: Transition) -> TransitionId {
        let id = TransitionId(self.transitions.len());
        let mut transition = transition;
        transition.set_id(id);
        self.transitions.push(transition);
        id
    }

    //---------- Property ----------

    pub fn property(&self) -> &Option<Expr> {
        &self.property
    }

    pub fn set_property(&mut self, expr: Expr) {
        self.property = Some(expr)
    }

    //---------- Search ----------

    pub fn search(&self) -> &Search {
        &self.search
    }

    pub fn set_search(&mut self, search: Search) {
        self.search = search
    }

    //---------- LTL Variable ----------

    pub fn ltl_variables(&self) -> &Vec<LTLVariable> {
        &self.ltl_variables
    }

    pub fn insert_ltl_variable(&mut self, expr: Expr) -> LTLVariableId {
        if let Some(v) = self.ltl_variables.iter().find(|v| v.expr().is_same(&expr)) {
            return v.id();
        }
        // Variable
        let index = self.ltl_variables.len();
        let v = LTLVariable::new(index, expr);
        let id = v.id();
        self.ltl_variables.push(v);
        id
    }

    pub fn get_ltl_expr(&self, expr: Expr) -> Expr {
        if let Some(v) = self.ltl_variables.iter().find(|v| v.expr().is_same(&expr)) {
            return v.id().into();
        }
        expr
    }

    pub fn flatten_ltl(&mut self) {
        if let Some(phi) = self.property.clone() {
            // ----- Non Loop -----
            let phi = phi.flatten_ltl(self);
            // Dec
            let dec_id = DeclarationId(self.declarations.len());
            let mut dec = Declaration::new(true, "prop", Type::Bool, None);
            dec.set_id(dec_id);
            self.declarations.push(dec);
            // Init
            let init_id = InitialId(self.initials.len());
            let expr = Expr::eq(dec_id.into(), phi);
            let mut init = Initial::new("prop", expr, None);
            init.set_id(init_id);
            self.initials.push(init);
            //
            self.property = Some(dec_id.into());
            // -----Loop -----
            let mut list = vec![];
            for v in self.ltl_variables.iter() {
                match v.expr().expression() {
                    Expression::LTLunary(op, kid) => match op {
                        LTLUnaryOperator::X => {}
                        LTLUnaryOperator::F => {
                            let e = LTLUnaryOperator::_F_.new(*kid.clone());
                            list.push(e.into());
                        }
                        LTLUnaryOperator::G => {
                            let e = LTLUnaryOperator::_G_.new(*kid.clone());
                            list.push(e.into());
                        }
                        LTLUnaryOperator::_F_ => {}
                        LTLUnaryOperator::_G_ => {}
                    },
                    Expression::LTLbinary(left, op, right) => match op {
                        LTLBinaryOperator::U => {
                            let e = LTLBinaryOperator::_U_.new(*left.clone(), *right.clone());
                            list.push(e.into());
                        }
                        LTLBinaryOperator::R => {
                            let e = LTLBinaryOperator::_U_.new(*left.clone(), *right.clone());
                            list.push(e.into());
                        }
                        LTLBinaryOperator::_U_ => {}
                        LTLBinaryOperator::_R_ => {}
                    },
                    _ => {}
                }
            }
            for e in list {
                self.insert_ltl_variable(e);
            }
        }
    }

    //========================= =========================

    //---------- Check Interval ----------

    pub fn check_intervals(&self) -> Result<(), Error> {
        for i in self.intervals.iter() {
            i.check_interval()?;
        }
        Ok(())
    }

    //---------- Naming ----------

    pub fn namings(&self) -> Vec<Naming> {
        let mut v = Vec::new();
        v.extend(self.enumerates.iter().flat_map(|x| x.namings()));
        v.extend(self.intervals.iter().map(|x| x.naming()));
        v.extend(self.declarations.iter().map(|x| x.naming()));
        v.extend(self.definitions.iter().map(|x| x.naming()));
        v.extend(self.fun_decs.iter().map(|x| x.naming()));
        v.extend(self.fun_defs.iter().map(|x| x.naming()));
        //
        v.extend(self.initials.iter().map(|x| x.naming()));
        v.extend(self.invariants.iter().map(|x| x.naming()));
        v.extend(self.transitions.iter().map(|x| x.naming()));
        //
        v
    }

    //---------- Unicity ----------

    pub fn check_unicity(&self) -> Result<(), Error> {
        check_unicity(self.namings())?;
        for fun in self.fun_decs.iter() {
            fun.check_unicity()?;
        }
        for fun in self.fun_defs.iter() {
            fun.check_unicity()?;
        }
        Ok(())
    }

    //---------- Types ----------

    pub fn types(&self) -> HashMap<String, Type> {
        let mut map = HashMap::new();
        // Enumerate
        for x in self.enumerates.iter() {
            map.insert(x.name().to_string(), Type::Enumerate(x.id()));
        }
        // Interval
        for x in self.intervals.iter() {
            map.insert(x.name().to_string(), Type::Interval(x.id()));
        }
        //
        map
    }

    //---------- Resolve Types ----------

    pub fn resolve_type(&mut self) -> Result<(), Error> {
        let types = self.types();
        // Declaration
        for x in self.declarations.iter_mut() {
            x.resolve_type(&types)?;
        }
        // Definitions
        for x in self.definitions.iter_mut() {
            x.resolve_type(&types)?;
        }
        // FunDec
        for x in self.fun_decs.iter_mut() {
            x.resolve_type(&types)?;
        }
        // FunDef
        for x in self.fun_defs.iter_mut() {
            x.resolve_type(&types)?;
        }
        // Init
        for x in self.initials.iter_mut() {
            x.resolve_type(&types)?;
        }
        // Inv
        for x in self.invariants.iter_mut() {
            x.resolve_type(&types)?;
        }
        // Trans
        for x in self.transitions.iter_mut() {
            x.resolve_type(&types)?;
        }
        //
        Ok(())
    }

    //---------- Entries ----------

    pub fn entries(&self) -> Vec<Entry> {
        let mut entries = Vec::new();
        // Enumerate
        for x in self.enumerates.iter() {
            entries.extend(x.entries());
        }
        // Declaration
        for x in self.declarations.iter() {
            entries.push(x.into());
        }
        // Definition
        for x in self.definitions.iter() {
            entries.push(x.into());
        }
        // FunDec
        for x in self.fun_decs.iter() {
            entries.push(x.into());
        }
        // FunDef
        for x in self.fun_defs.iter() {
            entries.push(x.into());
        }
        //
        entries
    }

    //---------- Resolve Expr ----------

    pub fn resolve_expr(&mut self) -> Result<(), Error> {
        let entries = self.entries();
        // Definition
        let mut definitions = Vec::new();
        for x in self.definitions.iter() {
            let y = x.resolve_expr(self, &entries)?;
            definitions.push(y);
        }
        self.definitions = definitions;
        // FunDef
        let mut fun_defs = Vec::new();
        for x in self.fun_defs.iter() {
            let y = x.resolve_expr(self, &entries)?;
            fun_defs.push(y);
        }
        self.fun_defs = fun_defs;
        // Initial
        let mut initials = Vec::new();
        for x in self.initials.iter() {
            let y = x.resolve_expr(self, &entries)?;
            initials.push(y);
        }
        self.initials = initials;
        // Invariant
        let mut invariants = Vec::new();
        for x in self.invariants.iter() {
            let y = x.resolve_expr(self, &entries)?;
            invariants.push(y);
        }
        self.invariants = invariants;
        // Transition
        let mut transitions = Vec::new();
        for x in self.transitions.iter() {
            let y = x.resolve_expr(self, &entries)?;
            transitions.push(y);
        }
        self.transitions = transitions;
        // Property
        if let Some(phi) = &self.property {
            let phi = phi.resolve(self, &entries)?;
            self.property = Some(phi)
        }
        // Search
        let search = self.search.resolve_expr(self, &entries)?;
        self.search = search;
        //
        Ok(())
    }

    //---------- Typing ----------

    pub fn check_type(&self) -> Result<(), Error> {
        // Definition
        for x in self.definitions.iter() {
            x.check_type(self)?;
        }
        // FunDef
        for x in self.fun_defs.iter() {
            x.check_type(self)?;
        }
        // Initial
        for x in self.initials.iter() {
            x.check_type(self)?;
        }
        // Invariant
        for x in self.invariants.iter() {
            x.check_type(self)?;
        }
        // Transition
        for x in self.transitions.iter() {
            x.check_type(self)?;
        }
        // Property
        if let Some(phi) = &self.property {
            phi.check_type(self)?;
        }
        // Search
        self.search.check_type(self)?;
        //
        Ok(())
    }

    //---------- Time ----------

    pub fn check_time(&self) -> Result<(), Error> {
        // Definition
        for x in self.definitions.iter() {
            x.check_time()?;
        }
        // FunDef
        for x in self.fun_defs.iter() {
            x.check_time()?;
        }
        // Initial
        for x in self.initials.iter() {
            x.check_time()?;
        }
        // Invariant
        for x in self.invariants.iter() {
            x.check_time()?;
        }
        // Transition
        for x in self.transitions.iter() {
            x.check_time(self)?;
        }
        // Property
        if let Some(phi) = &self.property {
            phi.check_time(self)?;
        }
        // Search
        self.search.check_time()?;
        //
        Ok(())
    }

    //---------- Propagate Expr ----------

    pub fn propagate_expr(&mut self) {
        // Definition
        let mut definitions = Vec::new();
        for x in self.definitions.iter() {
            let y = x.propagate_expr(self);
            definitions.push(y);
        }
        self.definitions = definitions;
        // FunDef
        let mut fun_defs = Vec::new();
        for x in self.fun_defs.iter() {
            let y = x.propagate_expr(self);
            fun_defs.push(y);
        }
        self.fun_defs = fun_defs;
        // Initial
        let mut initials = Vec::new();
        for x in self.initials.iter() {
            let y = x.propagate_expr(self);
            initials.push(y);
        }
        self.initials = initials;
        // Invariant
        let mut invariants = Vec::new();
        for x in self.invariants.iter() {
            let y = x.propagate_expr(self);
            invariants.push(y);
        }
        self.invariants = invariants;
        // Transition
        let mut transitions = Vec::new();
        for x in self.transitions.iter() {
            let y = x.propagate_expr(self);
            transitions.push(y);
        }
        self.transitions = transitions;
        // Property
        if let Some(phi) = &self.property {
            let phi = phi.propagate(self);
            self.property = Some(phi)
        }
        // Search
        let search = self.search.propagate_expr(self);
        self.search = search;
    }

    //---------- Check Bounded ----------

    pub fn check_cst_fun_bounded_paramters(&self) -> Result<(), Error> {
        for fun in self.fun_decs.iter() {
            if fun.is_constant() {
                fun.check_bounded_parameters(self)?;
            }
        }
        Ok(())
    }

    pub fn check_var_fun_bounded_paramters(&self) -> Result<(), Error> {
        for fun in self.fun_decs.iter() {
            if !fun.is_constant() {
                fun.check_bounded_parameters(self)?;
            }
        }
        Ok(())
    }

    pub fn check_var_dec_bounded_type(&self, pretty: &mut d_stuff::Pretty) {
        let mut v = vec![];
        for dec in self.declarations.iter() {
            if !dec.is_constant() {
                if !dec.get_type(self).is_bounded() {
                    v.push(dec.id())
                }
            }
        }
        let entry = Warning::UnboundedDec(v).to_entry(self);
        pretty.add(entry)
    }

    pub fn check_var_fun_bounded_type(&self, _pretty: &mut d_stuff::Pretty) {
        todo!()
        // for fun in self.fun_decs.iter() {
        //     if !fun.is_constant() {
        //         fun.check_bounded_parameters(self)?;
        //     }
        // }
        // Ok(())
    }

    //==================== ====================

    //---------- To Entry ----------

    pub fn to_entry(&self) -> d_stuff::Entry {
        d_stuff::Entry::new(
            d_stuff::Status::Info,
            d_stuff::Text::new(
                "Model",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![d_stuff::Message::new(
                None,
                d_stuff::Text::new(
                    format!("{}", self),
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                ),
            )],
        )
    }

    pub fn to_debug_entry(&self) -> d_stuff::Entry {
        d_stuff::Entry::new(
            d_stuff::Status::Info,
            d_stuff::Text::new(
                "Model",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![d_stuff::Message::new(
                None,
                d_stuff::Text::new(
                    self.debug(),
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                ),
            )],
        )
    }

    pub fn debug(&self) -> String {
        let mut res = "".to_string();
        // ----- Enumerate -----
        for x in self.enumerates.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- Interval -----
        for x in self.intervals.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- Declaration -----
        for x in self.declarations.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- Definition -----
        for x in self.definitions.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- FunDec -----
        for x in self.fun_decs.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- FunDef -----
        for x in self.fun_defs.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- Initial -----
        for x in self.initials.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- Invariant -----
        for x in self.invariants.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- Transition -----
        for x in self.transitions.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        // ----- Property -----
        if let Some(phi) = &self.property {
            res.push_str(&format!("prop = {}\n", phi.to_debug(self)));
        }
        // ----- Search -----
        res.push_str(&self.search.to_lang(self));
        //
        // ----- LTL Variables -----
        for x in self.ltl_variables.iter() {
            res.push_str(&format!("{}\n", x.to_debug(self)));
        }
        //
        res
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ----- Enumerate -----
        for x in self.enumerates.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- Interval -----
        for x in self.intervals.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- Declaration -----
        for x in self.declarations.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- Definition -----
        for x in self.definitions.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- FunDec -----
        for x in self.fun_decs.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- FunDef -----
        for x in self.fun_defs.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- Initial -----
        for x in self.initials.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- Invariant -----
        for x in self.invariants.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- Transition -----
        for x in self.transitions.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }
        // ----- Property -----
        if let Some(phi) = &self.property {
            write!(f, "prop = {}\n", phi.to_lang(self))?;
        }
        // ----- Search -----
        write!(f, "{}\n", self.search.to_lang(self))?;

        // ----- LTL Variables -----
        for x in self.ltl_variables.iter() {
            write!(f, "{}\n", x.to_lang(self))?;
        }

        // for x in self.structures.iter() {
        //     write!(f, "// {:?}\n", x.id())?;
        //     write!(f, "{}\n", x.to_lang(self))?;
        //     let instances = x.instances(self);
        //     if let Some((first, others)) = instances.split_first() {
        //         write!(f, "inst {}", self.get(*first).unwrap().name())?;
        //         for i in others.iter() {
        //             write!(f, ", {}", self.get(*i).unwrap().name())?;
        //         }
        //         write!(f, ": {}\n", x.name())?;
        //     }
        // }
        // for x in self.classes.iter() {
        //     write!(f, "// {:?}\n", x.id())?;
        //     write!(f, "{}\n", x.to_lang(self))?;
        //     let instances = x.instances(self);
        //     if let Some((first, others)) = instances.split_first() {
        //         write!(f, "inst {}", self.get(*first).unwrap().name())?;
        //         for i in others.iter() {
        //             write!(f, ", {}", self.get(*i).unwrap().name())?;
        //         }
        //         write!(f, ": {}\n", x.name())?;
        //     }
        // }
        // for x in self.variables.iter() {
        //     write!(f, "{}\n", x.to_lang(self))?;
        // }
        // for x in self.functions.iter() {
        //     write!(f, "{}\n", x.to_lang(self))?;
        // }
        // for x in self.constraints.iter() {
        //     write!(f, "{}\n", x.to_lang(self))?;
        // }
        // write!(f, "{}\n", self.search.to_lang(self))?;
        Ok(())
    }
}

//------------------------- Id Get -------------------------

impl GetFromId<EnumerateId, Enumerate> for Model {
    fn get(&self, id: EnumerateId) -> Option<&Enumerate> {
        self.enumerates.get(id.index())
    }
}
impl GetFromId<EnumerateElementId, EnumerateElement> for Model {
    fn get(&self, id: EnumerateElementId) -> Option<&EnumerateElement> {
        if let Some(e) = self.get(id.enumerate_id()) {
            e.get(id)
        } else {
            None
        }
    }
}

impl GetFromId<IntervalId, Interval> for Model {
    fn get(&self, id: IntervalId) -> Option<&Interval> {
        self.intervals.get(id.index())
    }
}

impl GetFromId<DeclarationId, Declaration> for Model {
    fn get(&self, id: DeclarationId) -> Option<&Declaration> {
        self.declarations.get(id.index())
    }
}

impl GetFromId<DefinitionId, Definition> for Model {
    fn get(&self, id: DefinitionId) -> Option<&Definition> {
        self.definitions.get(id.index())
    }
}

impl GetFromId<FunDecId, FunDec> for Model {
    fn get(&self, id: FunDecId) -> Option<&FunDec> {
        self.fun_decs.get(id.index())
    }
}

impl GetFromId<FunDefId, FunDef> for Model {
    fn get(&self, id: FunDefId) -> Option<&FunDef> {
        self.fun_defs.get(id.index())
    }
}

impl GetFromId<InitialId, Initial> for Model {
    fn get(&self, id: InitialId) -> Option<&Initial> {
        self.initials.get(id.index())
    }
}

impl GetFromId<InvariantId, Invariant> for Model {
    fn get(&self, id: InvariantId) -> Option<&Invariant> {
        self.invariants.get(id.index())
    }
}

impl GetFromId<TransitionId, Transition> for Model {
    fn get(&self, id: TransitionId) -> Option<&Transition> {
        self.transitions.get(id.index())
    }
}

impl GetFromId<LTLVariableId, LTLVariable> for Model {
    fn get(&self, id: LTLVariableId) -> Option<&LTLVariable> {
        self.ltl_variables.get(id.index())
    }
}

//------------------------- From Name -------------------------

impl FromName<Enumerate> for Model {
    fn from_name(&self, name: &str) -> Option<&Enumerate> {
        for e in self.enumerates.iter() {
            if e.name() == name {
                return Some(e);
            }
        }
        None
    }
}

impl FromName<EnumerateElement> for Model {
    fn from_name(&self, name: &str) -> Option<&EnumerateElement> {
        for e in self.enumerates.iter() {
            let elt = e.from_name(name);
            if elt.is_some() {
                return elt;
            }
        }
        None
    }
}

impl FromName<Declaration> for Model {
    fn from_name(&self, name: &str) -> Option<&Declaration> {
        for e in self.declarations.iter() {
            if e.name() == name {
                return Some(e);
            }
        }
        None
    }
}
