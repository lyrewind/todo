import { Task } from "../src/types"

interface TaskListProps {
	tasks: Task[];	
}

export const TaskList: React.FC<TaskListProps> = ({ tasks }) => {
	return <div>
		<ul className="flex flex-col gap-2 p-4 overflow-y-scroll max-h-[92vh]">
			{tasks.map(task => (
				<li className="flex flex-col gap-1 border-2 border-gray-200 rounded p-2" key={task.id}>
					<div className="flex gap-2">
						<strong>{task.title}</strong>
						<p className="px-1 bg-blue-800 font-bold rounded">{task.status}</p>
					</div>
					<i>{task.details}</i>
				</li>
			))}
		</ul>
	</div>
}
