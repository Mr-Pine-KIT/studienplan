use crate::model::{Module, Plan, PlanModuleView, Semester, SemesterDegree};
use crate::model::Degree::{Bachelor, Master};
use crate::model::ModuleType::{Lab, Lecture, Seminar};
use crate::model::SemesterType::{Summer, Unknown, Winter};
use crate::model::Speciality::{AiOverlords, Algorithms, ComputerGraphics, Parallelism, Robotics, Security, SoftwareEngineering, SystemArchitecture, Theoretics};

mod model;
mod z3model;

const PROGRAMMING_ID: &str = "M-INFO-101174";
const GBI_ID: &str = "M-INFO-101170";
const LA1_ID: &str = "T-MATH-103215";
const HM1_ID: &str = "T-MATH-102232";

fn get_bachelor_semester_1() -> Semester {
    let programming = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Bachelor,
        name: "Programmieren",
        identifier: PROGRAMMING_ID,
        semesters: vec![Winter],
        requirements: vec![],
        force: false,
    };
    let gbi = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "GBI",
        identifier: GBI_ID,
        semesters: vec![Winter],
        requirements: vec![],
        force: false,
    };
    let la1 = Module {
        module_type: Lecture { is_root: false },
        half_ects: 18,
        degree: Bachelor,
        name: "LA1",
        identifier: LA1_ID,
        semesters: vec![Winter],
        requirements: vec![],
        force: false,
    };
    let hm1 = Module {
        module_type: Lecture { is_root: false },
        half_ects: 18,
        degree: Bachelor,
        name: "HM1",
        identifier: HM1_ID,
        semesters: vec![Winter],
        requirements: vec![],
        force: false,
    };
    Semester {
        degrees: vec![SemesterDegree::Bachelor],
        modules: vec![programming, gbi, la1, hm1],
        semester_type: Winter,
        number: 1,
        ects: 40..29 * 2,
    }
}

const DT_ID: &str = "24007";
const ALGO1_ID: &str = "M-INFO-100030";
const LA2_ID: &str = "T-MATH-102241";

fn get_bachelor_semester_2() -> Semester {
    let swt = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "Softwaretechnik I",
        identifier: "M-INFO-101175",
        semesters: vec![Summer],
        requirements: vec![PROGRAMMING_ID],
        force: false,
    };
    let algo = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "Algo 1",
        identifier: ALGO1_ID,
        semesters: vec![Summer],
        requirements: vec![GBI_ID],
        force: false,
    };
    let dt = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "DT",
        identifier: DT_ID,
        semesters: vec![Summer],
        requirements: vec![],
        force: false,
    };
    let la2 = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Bachelor,
        name: "LA2",
        identifier: LA2_ID,
        semesters: vec![Summer],
        requirements: vec![LA1_ID],
        force: false,
    };
    let hm2 = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "HM2",
        identifier: "T-MATH-102233",
        semesters: vec![Summer],
        requirements: vec![],
        force: false,
    };
    Semester {
        degrees: vec![SemesterDegree::Bachelor],
        modules: vec![algo, swt, dt, hm2, la2],
        semester_type: Summer,
        number: 2,
        ects: 50..58,
    }
}

const OS_ID: &str = "M-INFO-101177";
const TGI_ID: &str = "M-INFO-101172";
const WT_ID: &str = "T-MATH-102244";

fn get_bachelor_semester_3() -> Semester {
    let ro = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "RO",
        identifier: "24502",
        semesters: vec![Winter],
        requirements: vec![DT_ID],
        force: false,
    };
    let tgi = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "TGI",
        identifier: TGI_ID,
        semesters: vec![Winter],
        requirements: vec![ALGO1_ID],
        force: false,
    };
    let os = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "OS",
        identifier: OS_ID,
        semesters: vec![Winter],
        requirements: vec![],
        force: false,
    };
    let wt = Module {
        module_type: Lecture { is_root: false },
        half_ects: 9,
        degree: Bachelor,
        name: "WT",
        identifier: WT_ID,
        semesters: vec![Winter],
        requirements: vec![],
        force: false,
    };
    let pse = Module {
        module_type: Lab,
        half_ects: 14,
        degree: Bachelor,
        name: "PSE",
        identifier: "M-INFO-101176",
        semesters: vec![Winter],
        requirements: vec![],
        force: false,
    };
    Semester {
        degrees: vec![SemesterDegree::Bachelor],
        modules: vec![ro, tgi, os, wt, pse],
        semester_type: Winter,
        number: 3,
        ects: 50..64,
    }
}

const INFOSEC_ID: &str = "M-INFO-106015";
fn get_bachelor_semester_4() -> Semester {
    let infosec = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Bachelor,
        name: "Infosec",
        identifier: INFOSEC_ID,
        semesters: vec![Summer],
        requirements: vec![],
        force: false,
    };
    let rn = Module {
        module_type: Lecture { is_root: false },
        half_ects: 8,
        degree: Bachelor,
        name: "Rechnernetze",
        identifier: "T-INFO-102015",
        semesters: vec![Summer],
        requirements: vec![],
        force: false,
    };
    let dbs = Module {
        module_type: Lecture { is_root: false },
        half_ects: 8,
        degree: Bachelor,
        name: "DBS",
        identifier: "T-INFO-101497 ",
        semesters: vec![Summer],
        requirements: vec![],
        force: false,
    };
    let numerik = Module {
        module_type: Lecture { is_root: false },
        half_ects: 9,
        degree: Bachelor,
        name: "Numerik",
        identifier: "T-MATH-102242",
        semesters: vec![Summer],
        requirements: vec![],
        force: false,
    };
    let osdev = Module {
        module_type: Lab,
        half_ects: 8,
        degree: Bachelor,
        name: "OSDev",
        identifier: "OSDev",
        semesters: vec![Summer],
        requirements: vec![],
        force: false,
    };
    Semester {
        degrees: vec![SemesterDegree::Bachelor],
        modules: vec![infosec, rn, dbs, numerik, osdev],
        semester_type: Summer,
        number: 4,
        ects: 40..46,
    }
}

const FORMSYS_ID: &str = "M-INFO-100799";

fn get_bachelor_semester_5() -> Semester {
    let propa = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Bachelor,
        name: "Propa",
        identifier: "M-INFO-101179",
        semesters: vec![Winter],
        requirements: vec![TGI_ID],
        force: false,
    };

    let gki = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Bachelor,
        name: "GKI",
        identifier: "M-INFO-106014",
        semesters: vec![Winter],
        requirements: vec![LA2_ID, WT_ID],
        force: false,
    };
    let proseminar = Module {
        module_type: Seminar { is_pro: true },
        half_ects: 6,
        degree: Bachelor,
        name: "Proseminar",
        identifier: "proseminar",
        semesters: vec![Winter, Summer],
        requirements: vec![],
        force: false,
    };
    let formsys = Module {
        module_type: Lecture { is_root: true },
        half_ects: 12,
        degree: Bachelor,//Master(vec![Theoretics]),
        name: "Formsys",
        identifier: FORMSYS_ID,
        semesters: vec![Winter],
        requirements: vec![TGI_ID],
        force: false,
    };
    Semester {
        degrees: vec![SemesterDegree::Bachelor, SemesterDegree::Master],
        modules: vec![propa, gki, proseminar, formsys],
        semester_type: Winter,
        number: 5,
        ects: 36..56 - 12,
    }
}

fn get_bachelor_semester_6() -> Semester {
    let algo_pg = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Bachelor,
        name: "AlgoPG",
        identifier: "M-INFO-101220",
        semesters: vec![Summer],
        requirements: vec![ALGO1_ID, TGI_ID],
        force: false,
    };
    Semester {
        degrees: vec![SemesterDegree::Bachelor, SemesterDegree::Master],
        modules: vec![algo_pg],
        semester_type: Summer,
        number: 6,
        ects: 40..60,
    }
}

fn get_bachelor_semester_7() -> Semester {
    Semester {
        degrees: vec![SemesterDegree::Bachelor, SemesterDegree::Master],
        modules: vec![],
        semester_type: Winter,
        number: 7,
        ects: 10..25,
    }
}

fn get_master_semester_1() -> Semester {
    Semester {
        degrees: vec![SemesterDegree::Master],
        modules: vec![],
        semester_type: Summer,
        number: 8,
        ects: 36..64,
    }
}

fn get_master_semester_2() -> Semester {
    Semester {
        degrees: vec![SemesterDegree::Master],
        modules: vec![],
        semester_type: Winter,
        number: 9,
        ects: 40..64,
    }
}

fn get_master_semester_3() -> Semester {
    Semester {
        degrees: vec![SemesterDegree::Master],
        modules: vec![],
        semester_type: Summer,
        number: 10,
        ects: 40..64,
    }
}

fn get_master_semester_4() -> Semester {
    Semester {
        degrees: vec![SemesterDegree::Master],
        modules: vec![],
        semester_type: Winter,
        number: 11,
        ects: 40..64,
    }
}

fn main() {
    let semesters = [
        get_bachelor_semester_1(),
        get_bachelor_semester_2(),
        get_bachelor_semester_3(),
        get_bachelor_semester_4(),
        get_bachelor_semester_5(),
        get_bachelor_semester_6(),
        get_bachelor_semester_7(),
        get_master_semester_1(),
        get_master_semester_2(),
        get_master_semester_3(),
        get_master_semester_4(),
    ];

    const CG_ID: &str = "M-INFO-100856";
    let cg = Module {
        module_type: Lecture { is_root: true },
        half_ects: 12,
        degree: Master(vec![ComputerGraphics]),
        name: "Computergrafik",
        identifier: CG_ID,
        semesters: vec![Winter],
        requirements: vec![LA2_ID],
        force: false,
    };

    const ALGO2_ID: &str = "M-INFO-101173";
    let algo2 = Module {
        module_type: Lecture { is_root: true },
        half_ects: 12,
        degree: Master(vec![Theoretics, Algorithms]),
        name: "Algo II",
        identifier: ALGO2_ID,
        semesters: vec![Winter],
        requirements: vec![ALGO1_ID],
        force: false,
    };

    const ITSEC_ID: &str = "M-INFO-106315";
    let itsec = Module {
        module_type: Lecture { is_root: true },
        half_ects: 12,
        degree: Master(vec![Security]),
        name: "ITSec",
        identifier: ITSEC_ID,
        semesters: vec![Winter],
        requirements: vec![INFOSEC_ID],
        force: false,
    };

    const ROBOTICS_ID: &str = "M-INFO-100893";
    let robotics = Module {
        module_type: Lecture { is_root: true },
        half_ects: 12,
        degree: Master(vec![Robotics]),
        name: "Robotik",
        identifier: ROBOTICS_ID,
        semesters: vec![Winter],
        requirements: vec![LA2_ID],
        force: false,
    };

    let formsys2_therory = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Theoretics]),
        name: "Formsys2: Theorie",
        identifier: "M-INFO-100841",
        semesters: vec![Summer],
        requirements: vec![FORMSYS_ID],
        force: false,
    };
    let formsys2_application = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Theoretics, SoftwareEngineering]),
        name: "Formsys2: Anwendung",
        identifier: "M-INFO-100744",
        semesters: vec![Summer],
        requirements: vec![FORMSYS_ID],
        force: false,
    };

    const PRACTICAL_SAT_ID: &str = "M-INFO-102825";
    let practical_sat_solving = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Theoretics]),
        name: "SAT Solving in der Praxis",
        identifier: PRACTICAL_SAT_ID,
        semesters: vec![Unknown],
        requirements: vec![FORMSYS_ID],
        force: false,
    };

    let algorithm_engineering = Module {
        module_type: Seminar { is_pro: false },
        half_ects: 8,
        degree: Master(vec![Theoretics, Algorithms, Parallelism]),
        name: "Algorithm Engineering",
        identifier: "M-INFO-106086",
        semesters: vec![Unknown],
        requirements: vec![ALGO2_ID],
        force: false,
    };

    let clogic = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Theoretics, SoftwareEngineering]),
        name: "Constructive logic / anderes Modul ( :( )von Platzer (Compilerbau) :pray:",
        identifier: "M-INFO-106256",
        semesters: vec![Summer],
        requirements: vec![FORMSYS_ID],
        force: true,
    };

    let cps_logical_foundations = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Theoretics, SoftwareEngineering]),
        name: "Logical foundations of cyber-physical systems",
        identifier: "M-INFO-106102",
        semesters: vec![Winter],
        requirements: vec![FORMSYS_ID],
        force: true,
    };

    let complexity_theory = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Master(vec![Theoretics, Algorithms]),
        name: "Fine-grained complexity theory and algorithms",
        identifier: "M-INFO-106644",
        semesters: vec![Unknown],
        requirements: vec![],
        force: false,
    };

    let parameterized_algos = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Master(vec![Theoretics, Algorithms]),
        name: "Parametrisierte Algorithmen",
        identifier: "M-INFO-105621",
        semesters: vec![Unknown],
        requirements: vec![ALGO1_ID],
        force: true,
    };

    let seminar_complexity_theory = Module {
        module_type: Seminar { is_pro: false },
        half_ects: 8,
        degree: Master(vec![Theoretics, Algorithms]),
        name: "Seminar: Fine-grained complexity theory and algorithms",
        identifier: "M-INFO-106645",
        semesters: vec![Unknown],
        requirements: vec![],
        force: false,
    };

    let advanced_sat_solving = Module {
        module_type: Seminar { is_pro: false },
        half_ects: 6,
        degree: Master(vec![Theoretics, Algorithms]),
        name: "Fortgeschrittene Themen zu SAT Solving",
        identifier: "M-INFO-106085",
        semesters: vec![Winter],
        requirements: vec![PRACTICAL_SAT_ID],
        force: false,
    };

    let fuzzy_sets = Module {
        module_type: Lecture { is_root: false },
        half_ects: 12,
        degree: Master(vec![Theoretics, Robotics, AiOverlords]),
        name: "Unscharfe Mengen",
        identifier: "M-INFO-100839",
        semesters: vec![Summer],
        requirements: vec![FORMSYS_ID],
        force: false,
    };

    let route_planning = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Algorithms]),
        name: "Routenplanung",
        identifier: "M-INFO-100031",
        semesters: vec![Summer],
        requirements: vec![ALGO2_ID],
        force: true,
    };

    let algorithmic_graph_theory = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Theoretics, Algorithms]),
        name: "Algorithmische Graphentheorie",
        identifier: "M-INFO-100762",
        semesters: vec![Unknown],
        requirements: vec![ALGO2_ID],
        force: false,
    };

    let parallel_algorithms = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Algorithms, Parallelism]),
        name: "Parallele Algorithmen",
        identifier: "M-INFO-100796",
        semesters: vec![Winter],
        requirements: vec![ALGO2_ID],
        force: false,
    };

    let randomized_algorithmic = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Theoretics, Algorithms]),
        name: "Randomisierte Algorithmik",
        identifier: "M-INFO-106469",
        semesters: vec![Winter],
        requirements: vec![WT_ID, ALGO2_ID],
        force: false,
    };

    let crypto_foundations = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![Security]),
        name: "Theoretische Grundlagen der Kryptographie",
        identifier: "M-INFO-105584",
        semesters: vec![Winter],
        requirements: vec![ITSEC_ID],
        force: false,
    };

    let appsec = Module {
        module_type: Lab,
        half_ects: 8,
        degree: Master(vec![Security]),
        name: "Appsec",
        identifier: "M-INFO-103166",
        semesters: vec![Winter],
        requirements: vec![],
        force: true,
    };

    let cryptanalysis = Module {
        module_type: Seminar { is_pro: false },
        half_ects: 6,
        degree: Master(vec![Security]),
        name: "Kryptoanalyse",
        identifier: "M-INFO-105337",
        semesters: vec![Summer],
        requirements: vec![ITSEC_ID],
        force: false,
    };

    let cg2 = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![ComputerGraphics]),
        name: "CG2",
        identifier: "M-INFO-106685",
        semesters: vec![Summer],
        requirements: vec![CG_ID],
        force: false,
    };

    const FOTO_BS_ID: &str = "M-INFO-100731";
    let foto_bs = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![ComputerGraphics]),
        name: "FotoBS",
        identifier: FOTO_BS_ID,
        semesters: vec![Winter],
        requirements: vec![CG_ID],
        force: true,
    };

    const VISUALIZATION_ID: &str = "M-INFO-100738";
    let visualization = Module {
        module_type: Lecture { is_root: false },
        half_ects: 10,
        degree: Master(vec![ComputerGraphics]),
        name: "Visualisierung",
        identifier: VISUALIZATION_ID,
        semesters: vec![Summer],
        requirements: vec![CG_ID],
        force: false,
    };

    let scientific_visualization = Module {
        module_type: Lab,
        half_ects: 12,
        degree: Master(vec![ComputerGraphics]),
        name: "Scientific Visualization",
        identifier: "M-INFO-106686",
        semesters: vec![Winter, Summer],
        requirements: vec![VISUALIZATION_ID, CG_ID],
        force: false,
    };

    let rendering = Module {
        module_type: Lab,
        half_ects: 12,
        degree: Master(vec![ComputerGraphics]),
        name: "Rendering in CGI",
        identifier: "M-INFO-106687",
        semesters: vec![Winter, Summer],
        requirements: vec![CG_ID, FOTO_BS_ID],
        force: false,
    };

    let virtual_systems = Module {
        module_type: Lecture { is_root: false },
        half_ects: 6,
        degree: Master(vec![Security, SystemArchitecture]),
        name: "Virtuelle Systeme",
        identifier: "M-INFO-108867",
        semesters: vec![Winter],
        requirements: vec![OS_ID],
        force: true,
    };

    let os_seminar = Module {
        module_type: Seminar { is_pro: false },
        half_ects: 6,
        degree: Master(vec![SystemArchitecture]),
        name: "Seminar Betriebssysteme",
        identifier: "M-INFO-101540",
        semesters: vec![Unknown],
        requirements: vec![OS_ID],
        force: false,
    };

    let advanced_os_seminar = Module {
        module_type: Seminar { is_pro: false },
        half_ects: 12,
        degree: Master(vec![SystemArchitecture]),
        name: "Seminar Betriebssysteme für fortgeschrittene",
        identifier: "M-INFO-100849",
        semesters: vec![Summer],
        requirements: vec![OS_ID],
        force: false,
    };

    let modules = vec![cg, algo2, itsec, robotics, formsys2_therory, formsys2_application, practical_sat_solving, algorithm_engineering, algorithmic_graph_theory, clogic, cps_logical_foundations, complexity_theory, parameterized_algos, parallel_algorithms, seminar_complexity_theory, advanced_sat_solving, fuzzy_sets, route_planning, randomized_algorithmic, crypto_foundations, cryptanalysis, appsec, cg2, foto_bs, visualization, scientific_visualization, rendering, virtual_systems, os_seminar, advanced_os_seminar];

    let plan = Plan::from_semesters_with_modules(&semesters, &modules);
    let solutions = plan.get_solutions();
    let mut interesting_solutions: Vec<_> = solutions.iter().map(|plan| PlanModuleView(plan.clone())).collect();
    interesting_solutions.sort();
    interesting_solutions.dedup();
    println!("{}", interesting_solutions.iter().map(|plan| format!("{}", plan.0)).collect::<Vec<_>>().join("\n\n\n\n\n"));
    println!("{}, {}", solutions.len(), interesting_solutions.len())
}
