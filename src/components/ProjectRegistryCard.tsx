import { Archive, GitBranch, GitFork, MapPin, MoreHorizontal, Trash2 } from 'lucide-react';
import type { ProjectRecord } from '../projectRegistry';
import '../registry-card.css';

function statusLabel(status: ProjectRecord['status']) {
  return status === 'ACTIVE' ? 'Active' : status === 'MISSING' ? 'Path missing' : 'Archived';
}

function workspaceAction(project: ProjectRecord) {
  const title = project.status === 'MISSING'
    ? 'Repair local workspace'
    : project.normalizedPath.trim() ? 'Change local workspace' : 'Attach local workspace';
  const accessibleLabel = project.status === 'MISSING'
    ? 'Repair local workspace'
    : project.normalizedPath.trim() ? 'Change local workspace' : 'Attach local workspace';
  return { label: 'Local workspace', title, accessibleLabel };
}

export function ProjectRegistryCard({ project, onOpen, onArchive, onRemove, onRepair, onPriority }: {
  project: ProjectRecord;
  onOpen: () => void;
  onArchive: () => void;
  onRemove: () => void;
  onRepair: () => void;
  onPriority: (priority: number) => void;
}) {
  const repository = project.repository;
  const localWorkspace = workspaceAction(project);
  return <article data-testid={`registry-card-${project.id}`} className={`registry-card registry-card-${project.status.toLowerCase()}`}>
    <div className="registry-card-top"><div className="registry-project-mark">{project.name.slice(0, 2).toUpperCase()}</div><div className="registry-card-title"><div><h2>{project.name}</h2><span className={`registry-status registry-status-${project.status.toLowerCase()}`}>{statusLabel(project.status)}</span></div><button className="icon-button" type="button" aria-label={`More actions for ${project.name}`}><MoreHorizontal size={17} /></button></div></div>
    <div className="registry-path"><MapPin size={14} aria-hidden="true" /><span title={project.originalPath}>{project.originalPath}</span></div>
    <div className="registry-meta-grid"><div><span>Repository</span><strong>{repository?.isGitRepository ? <><GitFork size={13} />Git repository</> : 'Non-Git folder'}</strong></div><div><span>Branch</span><strong>{repository?.currentBranch ? <><GitBranch size={13} />{repository.currentBranch}</> : 'Not detected'}</strong></div><div><span>Priority</span><select aria-label={`Priority for ${project.name}`} value={project.priority} onChange={event => onPriority(Number(event.target.value))}><option value={0}>Normal</option><option value={1}>High</option><option value={2}>Critical</option></select></div></div>
    <div className="registry-remote">{repository?.preferredRemoteUrl ? <><GitFork size={13} /><span title={repository.preferredRemoteUrl}>{repository.githubOwner && repository.githubRepo ? `${repository.githubOwner}/${repository.githubRepo}` : repository.preferredRemoteUrl}</span></> : <><span className="registry-dot" />No remote detected</>}</div>
    <div className="registry-settings"><span>Builder <b>{project.preferredBuilder ?? 'Unassigned'}</b></span><span>Auditor <b>{project.preferredAuditor ?? 'Unassigned'}</b></span></div>
    <div className="registry-card-foot" data-testid={`registry-card-footer-${project.id}`}><button className="secondary-button" type="button" onClick={onOpen}>Open cockpit</button><div className="registry-icon-actions"><button className="secondary-button registry-workspace-action" type="button" onClick={onRepair} aria-label={`${localWorkspace.accessibleLabel} for ${project.name}`} title={localWorkspace.title}><MapPin size={15} />{localWorkspace.label}</button>{project.status !== 'ARCHIVED' ? <button className="icon-button" type="button" onClick={onArchive} aria-label={`Archive ${project.name}`} title="Archive"><Archive size={15} /></button> : null}<button className="icon-button registry-danger" type="button" onClick={onRemove} aria-label={`Remove ${project.name} from registry`} title="Remove from registry"><Trash2 size={15} /></button></div></div>
  </article>;
}
