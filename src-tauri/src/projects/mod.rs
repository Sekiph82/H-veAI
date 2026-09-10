mod detection;
mod paths;
mod registry;

pub use registry::{
    archive_project, ensure_scrubbots_agent_preference, fetch_project, list_projects,
    refresh_repository_metadata, register_project, remove_project, repair_project_path,
    update_project_settings, ProjectListQuery, ProjectRecord, RegisterProjectRequest,
    RepairProjectPathRequest, UpdateProjectSettingsRequest,
};
