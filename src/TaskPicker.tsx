import type { WorkflowTask } from "./workflow";

const TASK_TITLE_LIMIT = 72;

function taskLabel(task: WorkflowTask) {
  const title = task.title.length > TASK_TITLE_LIMIT ? `${task.title.slice(0, TASK_TITLE_LIMIT - 1)}...` : task.title;
  return `${title} · ${task.currentState}`;
}

export function TaskPicker({
  tasks,
  value,
  onChange,
  disabled,
  ariaLabel,
}: {
  tasks: WorkflowTask[];
  value: string;
  onChange: (value: string) => void;
  disabled: boolean;
  ariaLabel: string;
}) {
  const taskList = tasks ?? [];
  const selected = taskList.find((task) => task.taskId === value);
  const fullTitle = selected?.title ?? "Freeform project operation";
  const detailId = `${ariaLabel.toLowerCase().replaceAll(" ", "-")}-title-detail`;
  return <label className="prompt-task-picker">Task <span className="agent-field-note">optional for a project operation</span><select aria-label={ariaLabel} aria-describedby={detailId} title={fullTitle} value={value} onChange={(event) => onChange(event.target.value)} disabled={disabled}><option value="">Freeform project operation</option>{taskList.map((task) => <option key={task.taskId} value={task.taskId} title={task.title}>{taskLabel(task)}</option>)}</select><span id={detailId} className="sr-only">Full task title: {fullTitle}</span></label>;
}
