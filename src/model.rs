use serde::{Deserialize, Serialize};
use z3::{Context, FuncDecl, Sort, Symbol};
use crate::model::SemesterDegree::{Bachelor, Master};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
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
        let names = [Bachelor, Master].map(|degree| toml::to_string(&degree).unwrap()).map(Symbol::from);
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

#[derive(Clone)]
pub struct Semester {
    pub(crate) number: i32,
    pub(crate) degrees: Vec<SemesterDegree>,
    pub(crate) max_ects: i32,
    pub(crate) modules: Vec<Module>,
    pub(crate) semester_type: SemesterType,
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
}