// CURE Foundation
// Coalition for Unified Research and Education in Biotech

#[derive(Debug, Clone)]
pub struct CureProject {
    pub name: String,
    pub disease_target: String,
    pub funding: f64,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone)]
pub enum ProjectStatus {
    Planning,
    Active,
    Clinical,
    Approved,
}

impl CureProject {
    pub fn new(name: &str, disease_target: &str, funding: f64) -> Self {
        CureProject {
            name: name.to_string(),
            disease_target: disease_target.to_string(),
            funding,
            status: ProjectStatus::Planning,
        }
    }

    pub fn advance_status(&mut self) {
        self.status = match self.status {
            ProjectStatus::Planning => ProjectStatus::Active,
            ProjectStatus::Active => ProjectStatus::Clinical,
            ProjectStatus::Clinical => ProjectStatus::Approved,
            ProjectStatus::Approved => ProjectStatus::Approved,
        };
    }
}

pub struct CureFoundation {
    projects: Vec<CureProject>,
    total_funding: f64,
}

impl Default for CureFoundation {
    fn default() -> Self {
        Self::new()
    }
}

impl CureFoundation {
    pub fn new() -> Self {
        CureFoundation {
            projects: Vec::new(),
            total_funding: 0.0,
        }
    }

    pub fn add_project(&mut self, project: CureProject) {
        self.total_funding += project.funding;
        self.projects.push(project);
    }

    pub fn get_projects(&self) -> &[CureProject] {
        &self.projects
    }

    pub fn total_funding(&self) -> f64 {
        self.total_funding
    }

    pub fn display(&self) -> String {
        format!(
            "CURE Foundation: {} projects, ${:.2}M total funding",
            self.projects.len(),
            self.total_funding / 1_000_000.0
        )
    }
}

pub fn initialize_cure_foundation() -> CureFoundation {
    let mut foundation = CureFoundation::new();
    
    foundation.add_project(CureProject::new(
        "Alzheimer's Research Initiative",
        "Alzheimer's Disease",
        5_000_000.0,
    ));
    
    foundation.add_project(CureProject::new(
        "Cancer Immunotherapy Program",
        "Various Cancers",
        10_000_000.0,
    ));
    
    foundation.add_project(CureProject::new(
        "Rare Disease Gene Therapy",
        "Rare Genetic Disorders",
        3_000_000.0,
    ));

    foundation
}
