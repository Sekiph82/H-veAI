use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(not(test))]
use tauri::{webview::PageLoadEvent, Emitter, Manager};
#[cfg(not(test))]
use tauri_plugin_log::{Target, TargetKind};

mod agent_session_center;
mod audit_engine;
mod codex_adapter;
mod command_center;
mod control_plane;
mod db;
mod external_browser;
mod final_response;
mod git_engine;
mod github_tracking;
mod process_policy;
mod project_cockpit;
mod project_dashboard;
mod projects;
mod prompt_engine;
mod runtime;
mod stream_sanitizer;
mod task_intelligence;
mod task_sources;
mod time;
mod watcher;
mod workflow;
#[cfg(not(test))]
use agent_session_center::{
    AgentRetryRequest, AgentSession, AgentSessionCenter, AgentStartRequest, ProviderReadiness,
    SessionEvent,
};
#[cfg(not(test))]
use audit_engine::{AuditInput, AuditInputRequest, AuditRun};
#[cfg(not(test))]
use codex_adapter::{AgentAdapter, CodexAdapter, CodexReadiness, CodexSession, CodexStartRequest};
#[cfg(not(test))]
use command_center::CommandCenterSnapshot;
#[cfg(not(test))]
use db::{DatabaseState, DatabaseStatus};
#[cfg(not(test))]
use github_tracking::GitHubTrackingManager;
#[cfg(not(test))]
use project_cockpit::ProjectCockpitSnapshot;
#[cfg(not(test))]
use project_dashboard::ProjectDashboardResolution;
#[cfg(not(test))]
use projects::{
    ensure_scrubbots_agent_preference, ProjectListQuery, ProjectRecord, RegisterProjectRequest,
    RepairProjectPathRequest, UpdateProjectSettingsRequest,
};
#[cfg(not(test))]
use prompt_engine::{
    ContextManifest, PromptApproveRequest, PromptContextRequest, PromptDispatchRequest,
    PromptDispatchResult, PromptEditRequest, PromptGenerateRequest, PromptRecord, PromptVersion,
};
#[cfg(not(test))]
use runtime::{RuntimeStatus, RuntimeSupervisor};
#[cfg(not(test))]
use task_intelligence::TaskIntelligenceSnapshot;
#[cfg(not(test))]
use task_sources::{
    CustomPathRequest, CustomPathUpdateRequest, CustomSourcePath, DiscoveredProjectSource,
};
#[cfg(not(test))]
use watcher::{ProjectWatcherStatus, WatcherManager, WatcherStatusSummary};
#[cfg(not(test))]
use workflow::{
    WorkflowEvent, WorkflowHistoryQuery, WorkflowOverrideRequest, WorkflowProjectList,
    WorkflowProjectListQuery, WorkflowTask, WorkflowTransitionRequest,
};

#[cfg(not(test))]
use git_engine::{GitDiff, GitDiffRequest, GitSnapshot, GitSnapshotRequest, MutationStatus};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NativeStatus {
    product_name: String,
    identifier: String,
    version: String,
    platform: String,
    app_data_dir: Option<String>,
    log_dir: Option<String>,
}

#[derive(Default)]
struct StartupIntroState {
    claimed: AtomicBool,
}

impl StartupIntroState {
    fn claim(&self) -> bool {
        !self.claimed.swap(true, Ordering::AcqRel)
    }
}

fn claim_startup_intro(state: &StartupIntroState) -> bool {
    state.claim()
}

#[cfg(not(test))]
mod app_commands {
    use super::*;

    #[tauri::command]
    fn hiveai_native_status(app: tauri::AppHandle) -> NativeStatus {
        log::info!("H!veAI native status requested.");

        let config = app.config();
        let package = app.package_info();
        let app_data_dir = app
            .path()
            .app_data_dir()
            .ok()
            .map(|path| path.to_string_lossy().into_owned());
        let log_dir = app
            .path()
            .app_log_dir()
            .ok()
            .map(|path| path.to_string_lossy().into_owned());

        NativeStatus {
            product_name: config
                .product_name
                .clone()
                .unwrap_or_else(|| "H!veAI".to_string()),
            identifier: config.identifier.clone(),
            version: package.version.to_string(),
            platform: std::env::consts::OS.to_string(),
            app_data_dir,
            log_dir,
        }
    }

    #[tauri::command]
    fn hiveai_request_restart(app: tauri::AppHandle) {
        log::info!("H!veAI restart requested through native foundation command.");
        let _ = app.emit("hiveai-restart-requested", ());
        app.request_restart();
    }

    #[tauri::command]
    fn hiveai_frontend_ready() {
        log::info!("HIVEAI_FRONTEND_READY");
    }

    #[tauri::command]
    fn hiveai_open_akilta() -> Result<(), String> {
        external_browser::open_akilta()
    }

    #[tauri::command]
    fn hiveai_open_external_url(url: String) -> Result<(), String> {
        external_browser::open_external_url(&url)
    }

    #[tauri::command]
    fn hiveai_startup_intro_claim(state: tauri::State<'_, StartupIntroState>) -> bool {
        claim_startup_intro(&state)
    }

    #[tauri::command]
    fn hiveai_runtime_status(supervisor: tauri::State<'_, RuntimeSupervisor>) -> RuntimeStatus {
        log::info!("H!veAI runtime status requested.");
        supervisor.status()
    }

    #[tauri::command]
    fn hiveai_database_status(database: tauri::State<'_, DatabaseState>) -> DatabaseStatus {
        log::info!("H!veAI database status requested.");
        database.status()
    }

    #[tauri::command]
    fn hiveai_command_center_snapshot(
        database: tauri::State<'_, DatabaseState>,
    ) -> Result<CommandCenterSnapshot, String> {
        github_tracking::ensure_portfolio(&database)?;
        command_center::snapshot(&database)
    }

    #[tauri::command]
    fn hiveai_github_tracking_select_project(
        manager: tauri::State<'_, GitHubTrackingManager>,
        project_id: Option<String>,
    ) {
        manager.select_project(project_id);
    }

    #[tauri::command]
    fn hiveai_github_tracking_refresh(
        manager: tauri::State<'_, GitHubTrackingManager>,
    ) -> Result<usize, String> {
        manager.refresh_now()
    }

    #[tauri::command]
    fn hiveai_project_dashboard_resolve(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<ProjectDashboardResolution, String> {
        github_tracking::ensure_portfolio(&database)?;
        command_center::resolve_project(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_project_cockpit_snapshot(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<ProjectCockpitSnapshot, String> {
        github_tracking::ensure_portfolio(&database)?;
        project_cockpit::snapshot(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_control_plane_snapshot(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<control_plane::ControlPlaneSnapshot, String> {
        github_tracking::ensure_portfolio(&database)?;
        control_plane::snapshot(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_control_plane_adopt(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<control_plane::ControlPlaneSnapshot, String> {
        github_tracking::ensure_portfolio(&database)?;
        let project = projects::fetch_project(&database, &project_id)?;
        if github_tracking::is_github_tasks_project(&project) {
            return control_plane::snapshot(&database, &project_id);
        }
        control_plane::adopt(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_control_plane_upgrade(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<control_plane::ControlPlaneSnapshot, String> {
        github_tracking::ensure_portfolio(&database)?;
        let project = projects::fetch_project(&database, &project_id)?;
        if github_tracking::is_github_tasks_project(&project) {
            return control_plane::snapshot(&database, &project_id);
        }
        if !control_plane::upgrade_control_plane(&project)?
            && !std::path::Path::new(&project.normalized_path)
                .join(control_plane::PROJECT_JSON)
                .is_file()
        {
            return control_plane::adopt(&database, &project_id);
        }
        control_plane::snapshot(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_control_plane_set_auto_fast_forward(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        enabled: bool,
    ) -> Result<(), String> {
        control_plane::set_auto_fast_forward(&database, &project_id, enabled)
    }

    #[tauri::command]
    fn hiveai_control_plane_sync_remote(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        auto_fast_forward_enabled: bool,
    ) -> Result<control_plane::GitSyncPlan, String> {
        let project = projects::fetch_project(&database, &project_id)?;
        if github_tracking::is_github_tasks_project(&project) {
            return Err("GitHub-tracked projects use GitHub + root TASKS.md state".into());
        }
        control_plane::sync_remote(&database, &project_id, auto_fast_forward_enabled)
    }

    #[tauri::command]
    fn hiveai_control_plane_local_repair_plan(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<control_plane::LocalRepairPlan, String> {
        let project = projects::fetch_project(&database, &project_id)?;
        Ok(control_plane::local_repair_plan(&project))
    }

    #[tauri::command]
    async fn hiveai_codex_readiness() -> Result<CodexReadiness, String> {
        tauri::async_runtime::spawn_blocking(|| CodexAdapter::default().readiness())
            .await
            .map_err(|error| format!("Codex readiness task failed: {error}"))
    }

    #[tauri::command]
    fn hiveai_codex_sessions_list(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<Vec<CodexSession>, String> {
        let adapter = CodexAdapter::default();
        let _ = adapter.provider();
        adapter.list(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_codex_start(
        adapter: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        request: CodexStartRequest,
    ) -> Result<CodexSession, String> {
        adapter.start(&database, request)
    }

    #[tauri::command]
    fn hiveai_codex_stop(
        adapter: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        session_id: String,
    ) -> Result<CodexSession, String> {
        adapter.stop(&database, &session_id)
    }

    #[tauri::command]
    fn hiveai_codex_resume(
        adapter: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        session_id: String,
    ) -> Result<CodexSession, String> {
        adapter.resume(&database, &session_id)
    }

    #[tauri::command]
    fn hiveai_agent_readiness() -> Vec<ProviderReadiness> {
        agent_session_center::readiness()
    }

    #[tauri::command]
    fn hiveai_agent_sessions_list(
        center: tauri::State<'_, AgentSessionCenter>,
        codex: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<Vec<AgentSession>, String> {
        agent_session_center::list(&center, &codex, &database, &project_id)
    }

    #[tauri::command]
    fn hiveai_agent_start(
        center: tauri::State<'_, AgentSessionCenter>,
        codex: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        request: AgentStartRequest,
    ) -> Result<AgentSession, String> {
        agent_session_center::start(&center, &codex, &database, request)
    }

    #[tauri::command]
    fn hiveai_agent_stop(
        center: tauri::State<'_, AgentSessionCenter>,
        codex: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        session_id: String,
    ) -> Result<AgentSession, String> {
        agent_session_center::stop(&center, &codex, &database, &project_id, &session_id)
    }

    #[tauri::command]
    fn hiveai_agent_retry(
        center: tauri::State<'_, AgentSessionCenter>,
        codex: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        request: AgentRetryRequest,
    ) -> Result<AgentSession, String> {
        agent_session_center::retry(&center, &codex, &database, request)
    }

    #[tauri::command]
    fn hiveai_agent_resume(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        session_id: String,
    ) -> Result<AgentSession, String> {
        agent_session_center::resume(&database, &project_id, &session_id)
    }

    #[tauri::command]
    fn hiveai_agent_session_events(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        session_id: String,
    ) -> Result<Vec<SessionEvent>, String> {
        agent_session_center::session_events(&database, &project_id, &session_id)
    }

    #[tauri::command]
    fn hiveai_agent_resize(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        session_id: String,
        rows: u16,
        columns: u16,
    ) -> Result<(), String> {
        agent_session_center::resize(&database, &project_id, &session_id, rows, columns)
    }

    #[tauri::command]
    fn hiveai_agent_permission_decision(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        session_id: String,
        permission_id: String,
        decision: String,
    ) -> Result<(), String> {
        let _ = agent_session_center::session_events(&database, &project_id, &session_id)?;
        let _ = (permission_id, decision);
        Err("AGENT_PERMISSION_PROVIDER_MANAGED_UNSUPPORTED".into())
    }

    #[tauri::command]
    fn hiveai_prompt_context_collect(
        database: tauri::State<'_, DatabaseState>,
        request: PromptContextRequest,
    ) -> Result<ContextManifest, String> {
        prompt_engine::collect_context(&database, request)
    }

    #[tauri::command]
    fn hiveai_prompt_generate(
        database: tauri::State<'_, DatabaseState>,
        request: PromptGenerateRequest,
    ) -> Result<PromptVersion, String> {
        prompt_engine::generate(&database, request)
    }

    #[tauri::command]
    fn hiveai_prompts_list(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<Vec<PromptRecord>, String> {
        prompt_engine::list(&database, project_id)
    }

    #[tauri::command]
    fn hiveai_prompt_versions(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        prompt_id: String,
    ) -> Result<Vec<PromptVersion>, String> {
        prompt_engine::versions_for_project(&database, project_id, prompt_id)
    }

    #[tauri::command]
    fn hiveai_prompt_edit(
        database: tauri::State<'_, DatabaseState>,
        request: PromptEditRequest,
    ) -> Result<PromptVersion, String> {
        prompt_engine::edit(&database, request)
    }

    #[tauri::command]
    fn hiveai_prompt_approve(
        database: tauri::State<'_, DatabaseState>,
        request: PromptApproveRequest,
    ) -> Result<PromptVersion, String> {
        prompt_engine::approve(&database, request)
    }

    #[tauri::command]
    fn hiveai_prompt_dispatch(
        center: tauri::State<'_, AgentSessionCenter>,
        codex: tauri::State<'_, CodexAdapter>,
        database: tauri::State<'_, DatabaseState>,
        request: PromptDispatchRequest,
    ) -> Result<PromptDispatchResult, String> {
        prompt_engine::dispatch(&center, &codex, &database, request)
    }

    #[tauri::command]
    fn hiveai_audit_input_collect(
        database: tauri::State<'_, DatabaseState>,
        request: AuditInputRequest,
    ) -> Result<AuditInput, String> {
        audit_engine::collect_input(&database, request)
    }

    #[tauri::command]
    fn hiveai_audit_run(
        database: tauri::State<'_, DatabaseState>,
        request: AuditInputRequest,
    ) -> Result<AuditRun, String> {
        audit_engine::run(&database, request)
    }

    #[tauri::command]
    fn hiveai_audits_list(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<Vec<AuditRun>, String> {
        audit_engine::list(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_audit_get(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        audit_id: String,
    ) -> Result<AuditRun, String> {
        audit_engine::get(&database, &project_id, &audit_id)
    }

    #[tauri::command]
    fn hiveai_audit_create_remediation_prompt(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        audit_id: String,
        finding_ids: Vec<String>,
        title: String,
        summary: String,
    ) -> Result<PromptVersion, String> {
        audit_engine::create_remediation_prompt(
            &database,
            &project_id,
            &audit_id,
            finding_ids,
            title,
            summary,
        )
    }

    #[tauri::command]
    fn hiveai_audit_link_remediation_session(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        audit_id: String,
        session_id: String,
    ) -> Result<(), String> {
        audit_engine::link_remediation_session(&database, &project_id, &audit_id, &session_id)
    }

    #[tauri::command]
    fn hiveai_projects_list(
        database: tauri::State<'_, DatabaseState>,
        query: Option<ProjectListQuery>,
    ) -> Result<Vec<ProjectRecord>, String> {
        github_tracking::ensure_portfolio(&database)?;
        projects::list_projects(&database, query.unwrap_or_default())
    }

    #[tauri::command]
    fn hiveai_project_register(
        database: tauri::State<'_, DatabaseState>,
        request: RegisterProjectRequest,
    ) -> Result<ProjectRecord, String> {
        projects::register_project(&database, request)
    }

    #[tauri::command]
    fn hiveai_project_get(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<ProjectRecord, String> {
        projects::fetch_project(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_project_update_settings(
        database: tauri::State<'_, DatabaseState>,
        request: UpdateProjectSettingsRequest,
    ) -> Result<ProjectRecord, String> {
        projects::update_project_settings(&database, request)
    }

    #[tauri::command]
    fn hiveai_project_archive(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<ProjectRecord, String> {
        projects::archive_project(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_project_remove_from_registry(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<(), String> {
        projects::remove_project(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_project_repair_path(
        database: tauri::State<'_, DatabaseState>,
        request: RepairProjectPathRequest,
    ) -> Result<ProjectRecord, String> {
        projects::repair_project_path(&database, request)
    }

    #[tauri::command]
    fn hiveai_git_snapshot(
        database: tauri::State<'_, DatabaseState>,
        request: GitSnapshotRequest,
    ) -> Result<GitSnapshot, String> {
        git_engine::snapshot(&database, request)
    }

    #[tauri::command]
    fn hiveai_git_diff(
        database: tauri::State<'_, DatabaseState>,
        request: GitDiffRequest,
    ) -> Result<GitDiff, String> {
        git_engine::diff(&database, request)
    }

    #[tauri::command]
    fn hiveai_git_mutation_status() -> MutationStatus {
        git_engine::mutation_status()
    }

    #[tauri::command]
    fn hiveai_watcher_status(watcher: tauri::State<'_, WatcherManager>) -> WatcherStatusSummary {
        watcher.status()
    }

    #[tauri::command]
    fn hiveai_watcher_project_status(
        watcher: tauri::State<'_, WatcherManager>,
        project_id: String,
    ) -> Result<ProjectWatcherStatus, String> {
        watcher.project_status(&project_id)
    }

    #[tauri::command]
    fn hiveai_watcher_refresh_set(
        watcher: tauri::State<'_, WatcherManager>,
    ) -> Result<WatcherStatusSummary, String> {
        watcher.refresh_from_registry()
    }

    #[tauri::command]
    fn hiveai_watcher_rescan(
        watcher: tauri::State<'_, WatcherManager>,
        project_id: String,
    ) -> Result<ProjectWatcherStatus, String> {
        watcher.rescan_project(&project_id)
    }

    #[tauri::command]
    fn hiveai_task_sources_discover(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<Vec<DiscoveredProjectSource>, String> {
        task_sources::discover(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_task_sources_list(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<Vec<DiscoveredProjectSource>, String> {
        task_sources::list(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_task_source_custom_paths_list(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<Vec<CustomSourcePath>, String> {
        task_sources::custom_paths_list(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_task_source_custom_path_add(
        database: tauri::State<'_, DatabaseState>,
        request: CustomPathRequest,
    ) -> Result<Vec<CustomSourcePath>, String> {
        task_sources::custom_path_add(&database, request)
    }

    #[tauri::command]
    fn hiveai_task_source_custom_path_remove(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
        path_or_id: String,
    ) -> Result<Vec<CustomSourcePath>, String> {
        task_sources::custom_path_remove(&database, &project_id, &path_or_id)
    }

    #[tauri::command]
    fn hiveai_task_source_custom_path_update(
        database: tauri::State<'_, DatabaseState>,
        request: CustomPathUpdateRequest,
    ) -> Result<Vec<CustomSourcePath>, String> {
        task_sources::custom_path_update(&database, request)
    }

    #[tauri::command]
    fn hiveai_task_intelligence_parse(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<TaskIntelligenceSnapshot, String> {
        task_intelligence::parse(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_task_intelligence_list(
        database: tauri::State<'_, DatabaseState>,
        project_id: String,
    ) -> Result<TaskIntelligenceSnapshot, String> {
        task_intelligence::list(&database, &project_id)
    }

    #[tauri::command]
    fn hiveai_workflow_task_get(
        database: tauri::State<'_, DatabaseState>,
        task_id: String,
    ) -> Result<WorkflowTask, String> {
        workflow::task_get(&database, task_id)
    }

    #[tauri::command]
    fn hiveai_workflow_project_list(
        database: tauri::State<'_, DatabaseState>,
        query: WorkflowProjectListQuery,
    ) -> Result<WorkflowProjectList, String> {
        workflow::project_list(&database, query)
    }

    #[tauri::command]
    fn hiveai_workflow_history(
        database: tauri::State<'_, DatabaseState>,
        query: WorkflowHistoryQuery,
    ) -> Result<Vec<WorkflowEvent>, String> {
        workflow::history(&database, query)
    }

    #[tauri::command]
    fn hiveai_workflow_transition(
        database: tauri::State<'_, DatabaseState>,
        request: WorkflowTransitionRequest,
    ) -> Result<WorkflowEvent, String> {
        workflow::transition(&database, request)
    }

    #[tauri::command]
    fn hiveai_workflow_override(
        database: tauri::State<'_, DatabaseState>,
        request: WorkflowOverrideRequest,
    ) -> Result<WorkflowEvent, String> {
        workflow::override_state(&database, request)
    }

    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    pub fn run() {
        tauri::Builder::default()
            .on_page_load(|_, payload| {
                if payload.event() == PageLoadEvent::Finished {
                    log::info!("HIVEAI_FRONTEND_READY");
                }
            })
            .plugin(
                tauri_plugin_log::Builder::new()
                    .target(Target::new(TargetKind::LogDir {
                        file_name: Some("hiveai".into()),
                    }))
                    .build(),
            )
            .plugin(tauri_plugin_notification::init())
            .invoke_handler(tauri::generate_handler![
                hiveai_native_status,
                hiveai_frontend_ready,
                hiveai_open_akilta,
                hiveai_open_external_url,
                hiveai_startup_intro_claim,
                hiveai_request_restart,
                hiveai_runtime_status,
                hiveai_database_status,
                hiveai_command_center_snapshot,
                hiveai_github_tracking_select_project,
                hiveai_github_tracking_refresh,
                hiveai_project_dashboard_resolve,
                hiveai_project_cockpit_snapshot,
                hiveai_control_plane_snapshot,
                hiveai_control_plane_adopt,
                hiveai_control_plane_upgrade,
                hiveai_control_plane_set_auto_fast_forward,
                hiveai_control_plane_sync_remote,
                hiveai_control_plane_local_repair_plan,
                hiveai_codex_readiness,
                hiveai_codex_sessions_list,
                hiveai_codex_start,
                hiveai_codex_stop,
                hiveai_codex_resume,
                hiveai_agent_readiness,
                hiveai_agent_sessions_list,
                hiveai_agent_start,
                hiveai_agent_stop,
                hiveai_agent_retry,
                hiveai_agent_resume,
                hiveai_agent_session_events,
                hiveai_agent_resize,
                hiveai_agent_permission_decision,
                hiveai_prompt_context_collect,
                hiveai_prompt_generate,
                hiveai_prompts_list,
                hiveai_prompt_versions,
                hiveai_prompt_edit,
                hiveai_prompt_approve,
                hiveai_prompt_dispatch,
                hiveai_audit_input_collect,
                hiveai_audit_run,
                hiveai_audits_list,
                hiveai_audit_get,
                hiveai_audit_create_remediation_prompt,
                hiveai_audit_link_remediation_session,
                hiveai_projects_list,
                hiveai_project_register,
                hiveai_project_get,
                hiveai_project_update_settings,
                hiveai_project_archive,
                hiveai_project_remove_from_registry,
                hiveai_project_repair_path,
                hiveai_git_snapshot,
                hiveai_git_diff,
                hiveai_git_mutation_status,
                hiveai_watcher_status,
                hiveai_watcher_project_status,
                hiveai_watcher_refresh_set,
                hiveai_watcher_rescan,
                hiveai_task_sources_discover,
                hiveai_task_sources_list,
                hiveai_task_source_custom_paths_list,
                hiveai_task_source_custom_path_add,
                hiveai_task_source_custom_path_remove,
                hiveai_task_source_custom_path_update,
                hiveai_task_intelligence_parse,
                hiveai_task_intelligence_list,
                hiveai_workflow_task_get,
                hiveai_workflow_project_list,
                hiveai_workflow_history,
                hiveai_workflow_transition,
                hiveai_workflow_override
            ])
            .setup(|app| {
                app.manage(StartupIntroState::default());
                app.manage(RuntimeSupervisor::new());
                let app_data_dir = app
                    .path()
                    .app_data_dir()
                    .map_err(|error| format!("resolve H!veAI app-data directory: {error}"))?;
                let database =
                    DatabaseState::initialize(app_data_dir.clone()).map_err(|error| {
                        format!("H!veAI persistence initialization failed: {error}")
                    })?;
                ensure_scrubbots_agent_preference(&database).map_err(|error| {
                    format!("H!veAI agent preference initialization failed: {error}")
                })?;
                let github_tracking_manager =
                    GitHubTrackingManager::start(database.clone(), app.handle().clone()).map_err(
                        |error| format!("H!veAI GitHub tracking scheduler failed: {error}"),
                    )?;
                workflow::recover_stale(&database)
                    .map_err(|error| format!("H!veAI workflow recovery failed: {error}"))?;
                let codex_adapter = CodexAdapter::default();
                codex_adapter
                    .reconcile(&database)
                    .map_err(|error| format!("H!veAI Codex recovery failed: {error}"))?;
                let agent_session_center = AgentSessionCenter::default();
                agent_session_center::reconcile(&database)
                    .map_err(|error| format!("H!veAI agent session recovery failed: {error}"))?;
                let database_status = database.status();
                let watcher_manager = WatcherManager::initialize_with_app_handle(
                    database.clone(),
                    app.handle().clone(),
                )
                .map_err(|error| format!("H!veAI watcher initialization failed: {error}"))?;
                app.manage(database);
                app.manage(github_tracking_manager);
                app.manage(codex_adapter);
                app.manage(agent_session_center);
                app.manage(watcher_manager);

                log::info!(
                    "H!veAI Tauri 2 foundation started; app_data_dir={}; schema_version={}",
                    app_data_dir.to_string_lossy(),
                    database_status.schema_version
                );
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running H!veAI Tauri application");
    }
}

#[cfg(not(test))]
pub use app_commands::run;

#[cfg(test)]
mod startup_intro_tests {
    use super::{claim_startup_intro, StartupIntroState};

    #[test]
    fn fresh_state_first_claim_is_true() {
        assert!(claim_startup_intro(&StartupIntroState::default()));
    }

    #[test]
    fn same_state_second_claim_is_false() {
        let state = StartupIntroState::default();
        assert!(claim_startup_intro(&state));
        assert!(!claim_startup_intro(&state));
    }

    #[test]
    fn separately_constructed_state_claims_fresh_lifecycle() {
        let first = StartupIntroState::default();
        let second = StartupIntroState::default();
        assert!(claim_startup_intro(&first));
        assert!(claim_startup_intro(&second));
    }

    #[test]
    fn production_claim_path_uses_native_state() {
        let state = StartupIntroState::default();
        assert!(claim_startup_intro(&state));
        assert!(!claim_startup_intro(&state));
    }
}

#[cfg(test)]
mod capability_tests {
    #[test]
    fn main_window_allows_the_registered_project_cockpit_command() {
        let capability = include_str!("../capabilities/default.json");
        assert!(capability.contains("allow-project-cockpit"));
        assert!(capability.contains("allow-codex-adapter"));
        let permissions = include_str!("../permissions/foundation.toml");
        assert!(permissions.contains("hiveai_project_cockpit_snapshot"));
        assert!(permissions.contains("hiveai_codex_start"));
    }

    #[test]
    fn main_window_allows_only_the_narrow_prompt_engine_permission() {
        let capability = include_str!("../capabilities/default.json");
        assert!(capability.contains("\"allow-prompt-engine\""));
        let permissions = include_str!("../permissions/foundation.toml");
        let block = permissions
            .split("[[permission]]")
            .find(|entry| entry.contains("identifier = \"allow-prompt-engine\""))
            .expect("prompt engine permission exists");
        for command in [
            "hiveai_prompt_context_collect",
            "hiveai_prompt_generate",
            "hiveai_prompts_list",
            "hiveai_prompt_versions",
            "hiveai_prompt_edit",
            "hiveai_prompt_approve",
            "hiveai_prompt_dispatch",
        ] {
            assert!(
                block.contains(command),
                "missing Prompt Engine command {command}"
            );
        }
        assert!(!block.contains("hiveai_agent_start"));
        assert!(!block.contains("shell"));
        assert!(!block.contains("process"));
    }
}
