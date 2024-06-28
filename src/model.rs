use std::f64;
use std::fmt::{Display, format, Formatter, write};
use serde::{Deserialize, Serialize};
use z3::{ast, Config, Context, FuncDecl, SatResult, Solver, Sort, Symbol};
use z3::ast::{Ast, Bool, Int};
use crate::model::SemesterDegree::{Bachelor, Master};
use crate::z3model::Z3Module;

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Ord, PartialOrd)]
pub enum Speciality {
    Theoretics,
    Algorithms,
    Security,
    Parallelism,
    SoftwareEngineering,
    Embedded,
    Telematics,
    InformationSystems,
    ComputerGraphics,
    Robotics,
    AiOverlords,
    SystemArchitecture,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum Degree {
    Bachelor,
    Master(Vec<Speciality>),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum SemesterDegree {
    Bachelor,
    Master,
}

impl SemesterDegree {
    pub fn z3_enum(ctx: &Context) -> (Sort, Vec<FuncDecl>, Vec<FuncDecl>) {
        let names = ["Bachelor", "Master"].map(Symbol::from);
        let names = &names[..];
        Sort::enumeration(ctx, Symbol::from("Degree"), names)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemesterType {
    Summer,
    Winter,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Semester {
    pub(crate) number: i32,
    pub(crate) degrees: Vec<SemesterDegree>,
    pub(crate) max_ects: i32,
    pub(crate) modules: Vec<Module>,
    pub(crate) semester_type: SemesterType,
}

impl Display for Semester {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut modules = self.modules.clone();
        modules.sort_by(|module, other| module.degree.cmp(&other.degree));
        let modules = modules.iter().map(|module| format!("{}", module)).collect::<Vec<_>>().join("\n\t");
        let total_ects: i32 = self.modules.iter().map(|module| module.half_ects).sum();
        let total_ects = f64::from(total_ects) / 2.0;
        write!(f, "Semester {} ({:?}) - total {} ECTS:\n\t{}", self.number, self.semester_type, total_ects, modules)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModuleType {
    Lecture { is_root: bool },
    Lab,
    Seminar { is_pro: bool },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    pub(crate) module_type: ModuleType,
    pub(crate) half_ects: i32,
    pub(crate) degree: Degree,
    pub(crate) name: &'static str,
    pub(crate) identifier: &'static str,
    pub(crate) requirements: Vec<&'static str>,
    pub(crate) semesters: Vec<SemesterType>,
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}] as {:?} with {} ECTS", self.name, self.identifier, self.degree, f64::from(self.half_ects) / 2.0)
    }
}

#[derive(Clone, Debug)]
pub struct Plan {
    modules: Vec<Module>,
    semesters: Vec<Semester>,
}

impl Plan {
    pub fn from_semesters_with_modules(semesters: &[Semester], remaining_modules: &[Module]) -> Plan {
        let collected_modules = semesters.iter().flat_map(|semester| semester.modules.iter());
        let modules: Vec<_> = remaining_modules.iter().chain(collected_modules).cloned().collect();

        let plan = Plan {
            modules,
            semesters: semesters.to_vec(),
        };
        plan.check_basic();

        plan
    }

    pub fn check_basic(&self) {
        let invalid_root_modules = self.modules.iter().filter(|&module| {
            matches!(module.module_type, ModuleType::Lecture { is_root: true }) && module.half_ects != 12
        }).collect::<Vec<_>>();
        assert_eq!(invalid_root_modules, Vec::<&Module>::new(), "Stammmodule müssen 6 ECTS haben")
    }

    pub fn get_solutions(self) -> Option<Plan> {
        let context = Context::new(&Config::new());
        let solver = Solver::new(&context);

        let semester_count = self.semesters.len();
        let (degree_sort, degree_values, degree_testers) = SemesterDegree::z3_enum(&context);
        let [bachelor, master] = &degree_values[..]
        else { panic!("aaaa") };
        let bachelor = bachelor.apply(&[]);
        let master = master.apply(&[]);

        let [bachelor_tester, master_tester] = &degree_testers[..]
        else { panic!("aaaa") };

        let zero = Int::from_i64(&context, 0);
        let one = Int::from_i64(&context, 1);
        let semester_count_z3 = Int::from_i64(&context, semester_count as i64);

        let z3_modules: Vec<_> = self.modules.iter().map(|module| Z3Module::from_module(&context, module, &degree_sort, &bachelor_tester, &solver)).collect();
        for z3_module in &z3_modules {
            solver.assert_and_track(&z3_module.semester.ge(&zero), &Bool::new_const(&context, format!("Semester_number >= 0 [{}]", z3_module.identifier)));
            solver.assert_and_track(&z3_module.semester.lt(&semester_count_z3), &Bool::new_const(&context, format!("Semester_number < {} [{}]", semester_count, z3_module.identifier)));

            if let Some((index, _)) = self.semesters.iter().enumerate().find(|(_, element)|
                element.modules.iter().any(|module| module.identifier == z3_module.identifier)
            ) {
                solver.assert_and_track(&z3_module.semester._eq(&Int::from_i64(&context, index as i64)), &Bool::new_const(&context, format!("Fixed semester number {} [{}]", index, z3_module.identifier)));
                solver.assert_and_track(&z3_module.used, &Bool::new_const(&context, format!("{} has to be used", z3_module.identifier)));
            }
        }

        // Max ects:
        for (semester_index, semester) in self.semesters.iter().enumerate() {
            let mut semester_sum = Int::from_i64(&context, 0);
            for z3_module in &z3_modules {
                let counted = &z3_module.used & &z3_module.semester._eq(&Int::from_i64(&context, semester_index as i64));
                semester_sum += counted.ite(&z3_module.ects, &zero);
            }

            solver.assert_and_track(&semester_sum.le(&Int::from_i64(&context, semester.max_ects as i64)), &Bool::new_const(&context, format!("Sum for semester {} <= {}", semester_index, semester.max_ects)))
        }

        // Totals
        let bachelor_min = 180 - 6 - 15 - 12;
        let bachelor_max = Int::from_i64(&context, (bachelor_min + 4) * 2);
        let bachelor_min = Int::from_i64(&context, bachelor_min * 2);

        let master_min = 120 - 41;
        let master_max = Int::from_i64(&context, (master_min + 4) * 2);
        let master_min = Int::from_i64(&context, (master_min) * 2);

        let mut bachelor_sum = Int::from_i64(&context, 0);
        let mut master_sum = Int::from_i64(&context, 0);
        for z3_module in &z3_modules {
            let is_bachelor = bachelor_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_bachelor = is_bachelor & &z3_module.used;
            bachelor_sum += is_bachelor.ite(&z3_module.ects, &zero);

            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_sum += is_master.ite(&z3_module.ects, &zero);
        }

        solver.assert_and_track(&bachelor_sum.ge(&bachelor_min), &Bool::new_const(&context, "Bachelor minimum ECTS"));
        solver.assert_and_track(&bachelor_sum.le(&bachelor_max), &Bool::new_const(&context, "Bachelor maximum ECTS"));


        // Stammmodule
        let root_modules: Vec<_> = self.modules.iter().filter(|module| matches!(module.module_type, ModuleType::Lecture {is_root: true})).collect();
        let mut bachelor_root_module_count = Int::from_i64(&context, 0);
        let mut master_root_module_count = Int::from_i64(&context, 0);
        for z3_module in z3_modules.iter().filter(|z3_module| root_modules.iter().any(|module| module.identifier == z3_module.identifier)) {
            let is_bachelor = bachelor_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_bachelor = is_bachelor & &z3_module.used;
            bachelor_root_module_count += is_bachelor.ite(&one, &zero);

            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_root_module_count += is_master.ite(&one, &zero);
        }

        solver.assert_and_track(&bachelor_root_module_count.ge(&one), &Bool::new_const(&context, "Bachelor root module count"));
        solver.assert_and_track(&master_root_module_count.ge(&Int::from_i64(&context, 4)), &Bool::new_const(&context, "Master root module count"));

        // Master praktika
        let lab_modules: Vec<_> = self.modules.iter().filter(|module| matches!(module.module_type, ModuleType::Lab)).collect();
        let mut master_lab_module_count = Int::from_i64(&context, 0);
        for z3_module in z3_modules.iter().filter(|z3_module| lab_modules.iter().any(|module| module.identifier == z3_module.identifier)) {
            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_lab_module_count += is_master.ite(&z3_module.ects, &zero);
        }

        solver.assert_and_track(&master_lab_module_count.ge(&Int::from_i64(&context, 6)), &Bool::new_const(&context, "Master lab module count"));

        // Master seminare
        let seminar_modules: Vec<_> = self.modules.iter().filter(|module| matches!(module.module_type, ModuleType::Seminar {is_pro: _})).collect();
        let mut master_seminar_module_count = Int::from_i64(&context, 0);
        for z3_module in z3_modules.iter().filter(|z3_module| seminar_modules.iter().any(|module| module.identifier == z3_module.identifier)) {
            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_seminar_module_count += is_master.ite(&z3_module.ects, &zero);
        }

        solver.assert_and_track(&master_seminar_module_count.ge(&Int::from_i64(&context, 3)), &Bool::new_const(&context, "Master seminar module count"));

        let lab_seminar_sum = master_seminar_module_count + master_lab_module_count;
        solver.assert_and_track(&lab_seminar_sum.ge(&Int::from_i64(&context, 3)), &Bool::new_const(&context, "Master lab + seminar module count"));

        // Master sum adjusted for not-counted seminars/labs
        let overlap = Int::from_i64(&context, 18) - lab_seminar_sum;
        let overlap = overlap.ge(&one).ite(&overlap, &zero);
        master_sum -= overlap;
        solver.assert_and_track(&master_sum.le(&master_max), &Bool::new_const(&context, "Master max ects"));
        solver.assert_and_track(&master_sum.ge(&master_min), &Bool::new_const(&context, "Master min ects"));

        if solver.check() == SatResult::Unsat {
            println!("Unsat :(");
            dbg!(&solver.get_unsat_core());
            return None
        }

        let model = solver.get_model().unwrap();
        let semesters = self.semesters.iter().enumerate().map(|(index, semester)| {
            let mut modules: Vec<_> = z3_modules.iter().filter(|z3_module| {
                let is_used = model.eval(&z3_module.used, true).unwrap().as_bool().unwrap();
                let semester = model.eval(&z3_module.semester, true).unwrap().as_i64().unwrap();
                is_used && semester == index as i64
            }).map(|z3_module| {
                let is_bachelor = model.eval(&z3_module.degree, true).unwrap().eq(&bachelor.as_datatype().unwrap());
                let degree = if is_bachelor { Degree::Bachelor } else { Degree::Master(vec![]) }; // TODO: Welches Vertiefungsfach?

                (z3_module, degree)
            }).map(|(z3_module, degree)| {
                let module = self.modules.iter().find(|module| module.identifier == z3_module.identifier).unwrap();
                let mut module = module.clone();
                module.degree = degree;

                module
            }).collect();

            let mut semester = semester.clone();
            semester.modules.clear();
            semester.modules.append(&mut modules);

            semester
        }).collect::<Vec<_>>();

        Some(Plan {
            semesters,
            modules: vec![],
        })
    }
}

impl Display for Plan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let semesters= self.semesters.iter().map(|semester| format!("{}\n", semester)).collect::<String>();
        write!(f, "{}", semesters)
    }
}