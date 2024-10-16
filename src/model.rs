use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64;
use std::fmt::{write, Display, Formatter};
use std::hash::Hash;
use std::ops::Range;
use std::str::FromStr;

use crate::model::Degree::Bachelor;
use crate::model::SemesterType::Unknown;
use crate::model::Speciality::Telematics;
use crate::z3model::Z3Module;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use z3::ast::{Ast, Bool, Datatype, Int};
use z3::{Config, Context, FuncDecl, SatResult, Solver, Sort, Symbol};

#[derive(
    Debug,
    Display,
    Eq,
    PartialEq,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
    EnumIter,
    EnumString,
    Hash,
)]
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

impl Speciality {
    pub fn z3_enum(ctx: &Context) -> (Sort, Vec<FuncDecl>, Vec<FuncDecl>) {
        let names: Vec<_> = Self::iter()
            .map(|entry| entry.to_string())
            .map(Symbol::from)
            .collect();
        let names = &names[..];
        Sort::enumeration(ctx, Symbol::from("Specialty"), names)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum Degree {
    Bachelor,
    Master(Vec<Speciality>),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Hash)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SemesterType {
    Summer,
    Winter,
    Unknown,
}

#[derive(Clone, Debug, Hash)]
pub struct Semester {
    pub(crate) number: i32,
    pub(crate) degrees: Vec<SemesterDegree>,
    pub(crate) ects: Range<i32>,
    pub(crate) modules: Vec<Module>,
    pub(crate) semester_type: SemesterType,
}

impl Display for Semester {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut modules = self.modules.clone();
        modules.sort_by(|module, other| module.degree.cmp(&other.degree));
        let modules = modules
            .iter()
            .map(|module| format!("{}", module))
            .collect::<Vec<_>>()
            .join("\n\t");
        let total_ects: i32 = self.modules.iter().map(|module| module.half_ects).sum();
        let total_ects = f64::from(total_ects) / 2.0;
        write!(
            f,
            "Semester {} ({:?}) - total {} ECTS:\n\t{}",
            self.number, self.semester_type, total_ects, modules
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ModuleType {
    Lecture { is_root: bool },
    Lab,
    Seminar { is_pro: bool },
}

impl ModuleType {
    pub fn prefix(&self) -> &'static str {
        match &self {
            ModuleType::Lecture { is_root: _ } => "",
            ModuleType::Lab => "Praktikum ",
            ModuleType::Seminar { is_pro: true } => "Proseminar ",
            ModuleType::Seminar { is_pro: false } => "Seminar ",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Module {
    pub(crate) module_type: ModuleType,
    pub(crate) half_ects: i32,
    pub(crate) degree: Degree,
    pub(crate) name: &'static str,
    pub(crate) identifier: &'static str,
    pub(crate) requirements: Vec<&'static str>,
    pub(crate) semesters: Vec<SemesterType>,
    pub(crate) force: bool,
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} [{}] as {:?} with {} ECTS",
            self.module_type.prefix(),
            self.name,
            self.identifier,
            self.degree,
            f64::from(self.half_ects) / 2.0
        )
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Plan {
    modules: Vec<Module>,
    semesters: Vec<Semester>,
    specialties: [Option<Speciality>; 2],
}

impl Plan {
    pub fn from_semesters_with_modules(
        semesters: &[Semester],
        remaining_modules: &[Module],
    ) -> Plan {
        let collected_modules = semesters
            .iter()
            .flat_map(|semester| semester.modules.iter());
        let modules: Vec<_> = remaining_modules
            .iter()
            .chain(collected_modules)
            .cloned()
            .collect();

        let plan = Plan {
            modules,
            semesters: semesters.to_vec(),
            specialties: [None, None],
        };
        plan.check_basic();

        plan
    }

    pub fn check_basic(&self) {
        let invalid_root_modules = self
            .modules
            .iter()
            .filter(|&module| {
                matches!(module.module_type, ModuleType::Lecture { is_root: true })
                    && module.half_ects != 12
            })
            .collect::<Vec<_>>();
        assert_eq!(
            invalid_root_modules,
            Vec::<&Module>::new(),
            "Stammmodule müssen 6 ECTS haben"
        )
    }

    pub fn get_solutions(self) -> Vec<Plan> {
        let context = Context::new(&Config::new());
        let solver = Solver::new(&context);

        let semester_count = self.semesters.len();
        let (degree_sort, degree_values, degree_testers) = SemesterDegree::z3_enum(&context);
        let [bachelor, master] = &degree_values[..] else {
            panic!("aaaa")
        };
        let bachelor = bachelor.apply(&[]);
        let master = master.apply(&[]);

        let [bachelor_tester, master_tester] = &degree_testers[..] else {
            panic!("aaaa")
        };

        let (speciality_sort, speciality_values, _speciality_testers) =
            Speciality::z3_enum(&context);
        let speciality_values = speciality_values
            .iter()
            .map(|val| val.apply(&[]).as_datatype().unwrap())
            .collect::<Vec<_>>();

        let first_specialty = Datatype::new_const(&context, "First specialty", &speciality_sort);
        let second_specialty = Datatype::new_const(&context, "Second specialty", &speciality_sort);

        solver.assert_and_track(
            &Datatype::distinct(&context, &[&first_specialty, &second_specialty]),
            &Bool::new_const(&context, "Must be two specialties"),
        );

        let zero = Int::from_i64(&context, 0);
        let one = Int::from_i64(&context, 1);
        let semester_count_z3 = Int::from_i64(&context, semester_count as i64);

        let z3_modules: Vec<_> = self
            .modules
            .iter()
            .map(|module| {
                Z3Module::from_module(
                    &context,
                    module,
                    &degree_sort,
                    bachelor_tester,
                    &speciality_sort,
                    &speciality_values,
                    &solver,
                )
            })
            .collect();
        for z3_module in &z3_modules {
            solver.assert_and_track(
                &z3_module.semester.ge(&zero),
                &Bool::new_const(
                    &context,
                    format!("Semester_number >= 0 [{}]", z3_module.identifier),
                ),
            );
            solver.assert_and_track(
                &z3_module.semester.lt(&semester_count_z3),
                &Bool::new_const(
                    &context,
                    format!(
                        "Semester_number < {} [{}]",
                        semester_count, z3_module.identifier
                    ),
                ),
            );

            if let Some((index, _)) = self.semesters.iter().enumerate().find(|(_, element)| {
                element
                    .modules
                    .iter()
                    .any(|module| module.identifier == z3_module.identifier)
            }) {
                solver.assert_and_track(
                    &z3_module
                        .semester
                        ._eq(&Int::from_i64(&context, index as i64)),
                    &Bool::new_const(
                        &context,
                        format!("Fixed semester number {} [{}]", index, z3_module.identifier),
                    ),
                );
                solver.assert_and_track(
                    &z3_module.used,
                    &Bool::new_const(&context, format!("{} has to be used", z3_module.identifier)),
                );
            }
        }

        // Max ects:
        for (semester_index, semester) in self.semesters.iter().enumerate() {
            let mut semester_sum = Int::from_i64(&context, 0);
            for z3_module in &z3_modules {
                let counted = &z3_module.used
                    & &z3_module
                        .semester
                        ._eq(&Int::from_i64(&context, semester_index as i64));
                semester_sum += counted.ite(&z3_module.ects, &zero);
            }

            solver.assert_and_track(
                &semester_sum.le(&Int::from_i64(&context, semester.ects.end as i64)),
                &Bool::new_const(
                    &context,
                    format!(
                        "Sum for semester {} <= {}",
                        semester_index, semester.ects.end
                    ),
                ),
            );
            solver.assert_and_track(
                &semester_sum.ge(&Int::from_i64(&context, semester.ects.start as i64)),
                &Bool::new_const(
                    &context,
                    format!(
                        "Sum for semester {} >= {}",
                        semester_index, semester.ects.start
                    ),
                ),
            );
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
            let is_bachelor = bachelor_tester
                .apply(&[&z3_module.degree])
                .as_bool()
                .unwrap();
            let is_bachelor = is_bachelor & &z3_module.used;
            bachelor_sum += is_bachelor.ite(&z3_module.ects, &zero);

            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_sum += is_master.ite(&z3_module.ects, &zero);
        }

        solver.assert_and_track(
            &bachelor_sum.ge(&bachelor_min),
            &Bool::new_const(&context, "Bachelor minimum ECTS"),
        );
        solver.assert_and_track(
            &bachelor_sum.le(&bachelor_max),
            &Bool::new_const(&context, "Bachelor maximum ECTS"),
        );

        // Stammmodule
        let root_modules: Vec<_> = self
            .modules
            .iter()
            .filter(|module| matches!(module.module_type, ModuleType::Lecture { is_root: true }))
            .collect();
        let mut bachelor_root_module_count = Int::from_i64(&context, 0);
        let mut master_root_module_count = Int::from_i64(&context, 0);
        for z3_module in z3_modules.iter().filter(|z3_module| {
            root_modules
                .iter()
                .any(|module| module.identifier == z3_module.identifier)
        }) {
            let is_bachelor = bachelor_tester
                .apply(&[&z3_module.degree])
                .as_bool()
                .unwrap();
            let is_bachelor = is_bachelor & &z3_module.used;
            bachelor_root_module_count += is_bachelor.ite(&one, &zero);

            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_root_module_count += is_master.ite(&one, &zero);
        }

        solver.assert_and_track(
            &bachelor_root_module_count.ge(&one),
            &Bool::new_const(&context, "Bachelor root module count"),
        );
        solver.assert_and_track(
            &master_root_module_count.ge(&Int::from_i64(&context, 4)),
            &Bool::new_const(&context, "Master root module count"),
        );

        // Master praktika
        let lab_modules: Vec<_> = self
            .modules
            .iter()
            .filter(|module| matches!(module.module_type, ModuleType::Lab))
            .collect();
        let mut master_lab_module_count = Int::from_i64(&context, 0);
        for z3_module in z3_modules.iter().filter(|z3_module| {
            lab_modules
                .iter()
                .any(|module| module.identifier == z3_module.identifier)
        }) {
            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_lab_module_count += is_master.ite(&z3_module.ects, &zero);
        }

        solver.assert_and_track(
            &master_lab_module_count.ge(&Int::from_i64(&context, 6 * 2)),
            &Bool::new_const(&context, "Master lab module count"),
        );

        // Master seminare
        let seminar_modules: Vec<_> = self
            .modules
            .iter()
            .filter(|module| matches!(module.module_type, ModuleType::Seminar { is_pro: _ }))
            .collect();
        let mut master_seminar_module_count = Int::from_i64(&context, 0);
        for z3_module in z3_modules.iter().filter(|z3_module| {
            seminar_modules
                .iter()
                .any(|module| module.identifier == z3_module.identifier)
        }) {
            let is_master = master_tester.apply(&[&z3_module.degree]).as_bool().unwrap();
            let is_master = is_master & &z3_module.used;
            master_seminar_module_count += is_master.ite(&z3_module.ects, &zero);
        }

        solver.assert_and_track(
            &master_seminar_module_count.ge(&Int::from_i64(&context, 2 * 3)),
            &Bool::new_const(&context, "Master seminar module count"),
        );

        let lab_seminar_sum = master_seminar_module_count + master_lab_module_count;
        solver.assert_and_track(
            &lab_seminar_sum.ge(&Int::from_i64(&context, 2 * 3)),
            &Bool::new_const(&context, "Master lab + seminar module count"),
        );

        // Master sum adjusted for not-counted seminars/labs
        let overlap = Int::from_i64(&context, 18 * 2) - lab_seminar_sum;
        let overlap = overlap.ge(&one).ite(&overlap, &zero);
        master_sum -= overlap;
        solver.assert_and_track(
            &master_sum.le(&master_max),
            &Bool::new_const(&context, "Master max ects"),
        );
        solver.assert_and_track(
            &master_sum.ge(&master_min),
            &Bool::new_const(&context, "Master min ects"),
        );

        // Ensure proper order
        for z3_module in &z3_modules {
            let module = self
                .modules
                .iter()
                .find(|module| module.identifier == z3_module.identifier)
                .unwrap();
            for &requirement in &module.requirements {
                let required_z3_module = z3_modules
                    .iter()
                    .find(|module| module.identifier == requirement)
                    .unwrap_or_else(|| panic!("Did not find identifier {}", requirement));

                solver.assert_and_track(
                    &z3_module.semester.gt(&required_z3_module.semester),
                    &Bool::new_const(
                        &context,
                        format!(
                            "{} is a requirement of {}_{} (semester)",
                            required_z3_module.identifier, module.name, z3_module.identifier
                        ),
                    ),
                );
                solver.assert_and_track(
                    &z3_module.used.implies(&required_z3_module.used),
                    &Bool::new_const(
                        &context,
                        format!(
                            "{} is a requirement of {}_{} (usage)",
                            required_z3_module.identifier, module.name, z3_module.identifier
                        ),
                    ),
                );
            }
        }

        // Ensure proseminar in bachelor
        let proseminars: Vec<Bool> = self
            .modules
            .iter()
            .filter(|module| matches!(module.module_type, ModuleType::Seminar { is_pro: true }))
            .map(|module| {
                z3_modules
                    .iter()
                    .find(|z3_module| module.identifier == z3_module.identifier)
                    .unwrap()
            })
            .map(|z3_module| {
                let is_bachelor = bachelor_tester
                    .apply(&[&z3_module.degree])
                    .as_bool()
                    .unwrap();
                is_bachelor & &z3_module.used
            })
            .collect::<Vec<_>>();
        let proseminars = proseminars.iter().collect::<Vec<_>>();
        solver.assert_and_track(
            &Bool::or(&context, &proseminars[..]),
            &Bool::new_const(&context, "Bachelor needs at least one proseminar"),
        );

        // Ensure modules with fixed semester type are in a semester of that type
        let yearly_modules = self
            .modules
            .iter()
            .filter(|module| module.semesters.len() == 1 && module.semesters != vec![Unknown])
            .map(|module| {
                (
                    &module.semesters[0],
                    z3_modules
                        .iter()
                        .find(|z3_module| module.identifier == z3_module.identifier)
                        .unwrap(),
                )
            })
            .map(|(semester_type, module)| {
                let semesters: Vec<_> = self
                    .semesters
                    .iter()
                    .enumerate()
                    .filter(|(_, semester)| semester.semester_type == *semester_type)
                    .map(|(index, _)| index)
                    .collect();
                let conditions: Vec<_> = semesters
                    .iter()
                    .map(|semester_index| {
                        (module
                            .semester
                            ._eq(&Int::from_i64(&context, *semester_index as i64)))
                    })
                    .collect();
                let conditions: Vec<_> = conditions.iter().collect();

                (
                    Bool::or(&context, &conditions[..]),
                    Bool::new_const(
                        &context,
                        format!(
                            "Module {} has to be in one of {:?}",
                            module.identifier, semesters
                        ),
                    ),
                )
            });
        for (module, tracker) in yearly_modules {
            solver.assert_and_track(&module, &tracker);
        }

        // Check degree requirements
        for (index, semester) in self.semesters.iter().enumerate() {
            let matcher = match semester.degrees.as_slice() {
                [SemesterDegree::Bachelor] => bachelor_tester,
                [SemesterDegree::Master] => master_tester,
                _ => continue,
            };

            for z3_module in &z3_modules {
                let is_semester = z3_module
                    .semester
                    ._eq(&Int::from_i64(&context, index as i64));
                let matches_degree = matcher.apply(&[&z3_module.degree]).as_bool().unwrap();
                let condition = is_semester.implies(&matches_degree);
                solver.assert_and_track(
                    &condition,
                    &Bool::new_const(
                        &context,
                        format!(
                            "If {} is in semester {index} it has to be {:?}",
                            z3_module.identifier, semester.degrees[0]
                        ),
                    ),
                );
            }
        }

        let mut specialty_counts = [zero.clone(), zero.clone()];
        let mut specialty_counts_no_root = [zero.clone(), zero.clone()];
        // Check specialty ects requirements
        for (index, specialty) in [&first_specialty, &second_specialty]
            .into_iter()
            .enumerate()
        {
            let mut total_sum = Int::from_i64(&context, 0);
            let mut without_root = Int::from_i64(&context, 0);

            for z3_module in &z3_modules {
                let is_specialty = z3_module.associated_specialty._eq(specialty);
                let is_master = z3_module.degree._eq(&master.as_datatype().unwrap());
                let is_relevant = is_specialty & &z3_module.used & is_master;
                let total_count = is_relevant.ite(&z3_module.ects, &zero);
                total_sum += total_count;

                let module = self
                    .modules
                    .iter()
                    .find(|module| module.identifier == z3_module.identifier)
                    .unwrap();
                let is_root = matches!(module.module_type, ModuleType::Lecture { is_root: true });
                let without_root_count = (is_relevant & !is_root).ite(&z3_module.ects, &zero);
                without_root += without_root_count
            }

            specialty_counts[index] = total_sum;
            solver.assert_and_track(
                &specialty_counts[index].ge(&Int::from_i64(&context, 15 * 2)),
                &Bool::new_const(&context, "Specialty min ects (total)"),
            );

            let (telematics_index, _) = Speciality::iter()
                .enumerate()
                .find(|(_, entry)| *entry == Telematics)
                .unwrap();
            let telematics_value = &speciality_values[telematics_index];
            let is_telematics = specialty._eq(telematics_value);
            let min_without_root = is_telematics.ite(
                &Int::from_i64(&context, 8 * 2),
                &Int::from_i64(&context, 10 * 2),
            );
            specialty_counts_no_root[index] = without_root;
            solver.assert_and_track(
                &specialty_counts_no_root[index].ge(&min_without_root),
                &Bool::new_const(&context, "Specialty min ects (without root)"),
            );
        }

        let mut solutions = vec![];
        let mut solution_views = HashSet::new();

        if solver.check() == SatResult::Unsat {
            println!("Unsat :(");
            dbg!(&solver.get_unsat_core());
            return solutions;
        }

        while solver.check() == SatResult::Sat {
            let model = solver.get_model().unwrap();
            let semesters = self
                .semesters
                .iter()
                .enumerate()
                .map(|(index, semester)| {
                    let mut modules: Vec<_> = z3_modules
                        .iter()
                        .filter(|z3_module| {
                            let is_used = model
                                .eval(&z3_module.used, true)
                                .unwrap()
                                .as_bool()
                                .unwrap();
                            let semester = model
                                .eval(&z3_module.semester, true)
                                .unwrap()
                                .as_i64()
                                .unwrap();
                            is_used && semester == index as i64
                        })
                        .map(|z3_module| {
                            let is_bachelor = model
                                .eval(&z3_module.degree, true)
                                .unwrap()
                                .eq(&bachelor.as_datatype().unwrap());

                            let specialty = model
                                .eval(&z3_module.associated_specialty, true)
                                .unwrap()
                                .to_string();
                            let specialty = Speciality::from_str(&specialty).unwrap();

                            let degree = if is_bachelor {
                                Degree::Bachelor
                            } else {
                                Degree::Master(vec![specialty])
                            };

                            (z3_module, degree)
                        })
                        .map(|(z3_module, degree)| {
                            let module = self
                                .modules
                                .iter()
                                .find(|module| module.identifier == z3_module.identifier)
                                .unwrap();
                            let mut module = module.clone();
                            module.degree = degree;

                            module
                        })
                        .collect();

                    let mut semester = semester.clone();
                    semester.modules.clear();
                    semester.modules.append(&mut modules);

                    semester
                })
                .collect::<Vec<_>>();

            let specialties = [&first_specialty, &second_specialty]
                .map(|specialty| model.eval(specialty, true).unwrap().to_string())
                .map(|name| Speciality::from_str(&name).ok());

            let plan = Plan {
                semesters,
                modules: vec![],
                specialties,
            };
            solutions.push(plan.clone());
            let view = PlanModuleView(plan);
            solution_views.insert(view);
            print!("\rFound {} solutions so far ^C to cancel search and show them all", solution_views.len());

            let differences: Vec<_> = z3_modules
                .iter()
                .map(|module| &module.used)
                .map(|property| !property._eq(&model.eval(property, true).unwrap()))
                .collect();
            let differences: Vec<_> = differences.iter().collect();
            solver.assert_and_track(
                &Bool::or(&context, differences.as_slice()),
                &Bool::new_const(&context, "Ensure new solution"),
            )
        }
        println!();
        solutions
    }
}

impl Display for Plan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for semester in &self.semesters {
            let _ = writeln!(f, "{}", semester);
        }
        let _ = writeln!(
            f,
            "\nSpecialties: {}, {}",
            self.specialties[0]
                .map(|specialty| specialty.to_string())
                .or_else(|| Some("unknown".to_string()))
                .unwrap(),
            self.specialties[1]
                .map(|specialty| specialty.to_string())
                .or_else(|| Some("unknown".to_string()))
                .unwrap()
        );
        let bachelor_sum: i32 = self
            .semesters
            .iter()
            .flat_map(|semester| &semester.modules)
            .filter(|module| module.degree == Bachelor)
            .map(|module| module.half_ects)
            .sum();
        let bachelor_sum = f64::from(bachelor_sum) / 2.0;
        let master_sum: i32 = self
            .semesters
            .iter()
            .flat_map(|semester| &semester.modules)
            .filter(|module| matches!(module.degree, Degree::Master(_)))
            .map(|module| module.half_ects)
            .sum();
        let master_sum = f64::from(master_sum) / 2.0;
        write!(
            f,
            "ECTS Sum Bachelor {} Master: {}",
            bachelor_sum, master_sum
        )
    }
}

#[derive(Hash)]
pub struct PlanModuleView(pub Plan);

impl PlanModuleView {
    fn get_module_set(&self) -> HashSet<&Module> {
        let self_modules: Vec<_> = self
            .0
            .semesters
            .iter()
            .flat_map(|semester| &semester.modules)
            .collect();
        let mut module_set = HashSet::new();
        module_set.extend(self_modules);
        module_set
    }
}

impl PartialEq for PlanModuleView {
    fn eq(&self, other: &Self) -> bool {
        self.get_module_set() == other.get_module_set()
    }
}

impl Eq for PlanModuleView {}

impl Ord for PlanModuleView {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut module_list: Vec<_> = self.get_module_set().iter().copied().collect();
        module_list.sort();
        let mut other_module_list: Vec<_> = other.get_module_set().iter().copied().collect();
        other_module_list.sort();
        module_list.cmp(&other_module_list)
    }
}

impl PartialOrd for PlanModuleView {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

