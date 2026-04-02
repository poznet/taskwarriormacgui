export interface Task {
	id: number;
	uuid: string;
	description: string;
	status: 'pending' | 'completed' | 'deleted' | 'waiting' | 'recurring';
	project?: string;
	priority?: 'H' | 'M' | 'L';
	due?: string;
	tags?: string[];
	entry: string;
	modified: string;
	end?: string;
	urgency: number;
	annotations?: Annotation[];
	depends?: string;
	recur?: string;
}

export interface Annotation {
	entry: string;
	description: string;
}
