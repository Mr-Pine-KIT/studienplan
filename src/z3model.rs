use z3::{ast, Context, FuncDecl, Solver, Sort};
use z3::ast::Bool;
use crate::model::Degree::Bachelor;
use crate::model::Module;

pub struct Z3Module<'ctx> {
    pub used: Bool<'ctx>,
    pub semester: ast::Int<'ctx>,
    pub degree: ast::Datatype<'ctx>,
    pub ects: ast::Int<'ctx>,
    pub identifier: &'static str
}

impl <'ctx> Z3Module<'ctx> {
    pub fn from_module(context: &'ctx Context, module: &Module, degree_sort: &'ctx Sort, bachelor_tester: &FuncDecl, solver: &Solver) -> Z3Module<'ctx> {
        let used = Bool::new_const(context, format!("used_{}_{}", module.name, module.identifier));
        let semester = ast::Int::new_const(context, format!("semester_{}_{}", module.name, module.identifier));
        let ects = ast::Int::from_i64(context, module.half_ects as i64);
        let degree = ast::Datatype::new_const(context, format!("defree_{}_{}", module.name, module.identifier), degree_sort);
        if module.degree == Bachelor {
            let is_bachelor = bachelor_tester.apply(&[&degree]).as_bool().unwrap();
            solver.assert_and_track(&is_bachelor, &Bool::new_const(context, format!("is_bachelor_{}_{}", module.name, module.identifier)));
        }
        
        Z3Module {
            used, semester, degree, ects, identifier: module.identifier
        }
    }
}