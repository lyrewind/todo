export type TaskStatus = 'open' | 'closed' | 'paused';

export interface Task {
	id?: number;
	title: string;
	details: string;
	status: TaskStatus;
}
