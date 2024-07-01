use std::fmt::format;
use strum::IntoEnumIterator;
use z3::{ast, Context, FuncDecl, Solver, Sort};
use z3::ast::{Ast, Bool, Datatype};
use crate::model::Degree::Bachelor;
use crate::model::{Degree, Module, Speciality};


pub struct Z3Module<'ctx> {
    pub used: Bool<'ctx>,
    pub semester: ast::Int<'ctx>,
    pub degree: Datatype<'ctx>,
    pub associated_specialty: Datatype<'ctx>,
    pub ects: ast::Int<'ctx>,
    pub identifier: &'static str,
}

impl<'ctx> Z3Module<'ctx> {
    pub fn from_module(context: &'ctx Context, module: &Module, degree_sort: &'ctx Sort, bachelor_tester: &FuncDecl, specialty_sort: &'ctx Sort, specialty_values: &[Datatype], solver: &Solver) -> Z3Module<'ctx> {
        let used = Bool::new_const(context, format!("used_{}_{}", module.name, module.identifier));
        let semester = ast::Int::new_const(context, format!("semester_{}_{}", module.name, module.identifier));
        let ects = ast::Int::from_i64(context, module.half_ects as i64);
        let specialty = Datatype::new_const(context, format!("specialty_{}_{}", module.name, module.identifier), specialty_sort);
        if let Degree::Master(specialties) = &module.degree {
            let specialty_values: Vec<_> = Speciality::iter().enumerate()
                .filter(|(_, entry)| specialties.contains(entry))
                .map(|(index, _)| &specialty_values[index])
                .collect();
            
            let matches_specialty = specialty_values.iter().map(|specialty_value| specialty_value._eq(&specialty)).collect::<Vec<_>>();
            let matches_specialty: Vec<_> = matches_specialty.iter().collect();
            let matches_any = Bool::or(context, matches_specialty.as_slice());
            solver.assert_and_track(&matches_any, &Bool::new_const(context, format!("{}_{} must match its specialties", module.name, module.identifier)))
        }

        let degree = Datatype::new_const(context, format!("degree_{}_{}", module.name, module.identifier), degree_sort);
        if module.degree == Bachelor {
            let is_bachelor = bachelor_tester.apply(&[&degree]).as_bool().unwrap();
            solver.assert_and_track(&is_bachelor, &Bool::new_const(context, format!("is_bachelor_{}_{}", module.name, module.identifier)));
        }
        
        if module.force {
            solver.assert_and_track(&used, &Bool::new_const(context, format!("Module {}_{} was forced", module.name, module.identifier)))
        }

        Z3Module {
            used,
            semester,
            degree,
            associated_specialty: specialty,
            ects,
            identifier: module.identifier,
        }
    }
}