import type { NextPage } from 'next'
import Head from 'next/head'
import { useEffect, useState } from 'react'
import { Creation } from '../components/Creation'
import { Navbar } from '../components/Navbar'
import { TaskList } from '../components/TaskList'
import { createTask, getTasks } from '../src/api'
import { Task, TaskStatus } from '../src/types'

const toStatus = (code: number): TaskStatus => {
	switch (code) {
		case 1:
			return "open"
		case 2:
			return "closed"
		case 3:
			return "open"
		default:
			throw new Error("Invalid status code received.")
	}
}

const Home: NextPage = () => {
	const [tasks, setTasks] = useState<Task[]>([]);

	const [isCreating, setCreation] = useState(false);

	useEffect(() => {
		(async () => {
				const response = await getTasks();		
				
				if(response.data.tasks) {
					setTasks(response.data.tasks.map(task => {
							return { ...task, status: toStatus(task.status_code)}
						}));
				}
				else console.debug(response.data)
				})();
	}, [])

	const handleCreate = async (data: Task) => {
		try {
			const response = await createTask(data);
			if(response.data) {
				setTasks([...tasks, { ...response.data.task, status: toStatus(response.data.task.status_code)}])
			}
			else console.error("Missing task data.")
		}
		catch(error) {
			console.error("Failed to create task.")
		}
	}

  return (
    <div className="flex flex-col w-screen h-screen">
      <Head>
        <title>To-do</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

			<Navbar onClick={() => setCreation(!isCreating)} disabled={isCreating}/>
			{isCreating ? 
				<Creation onCreate={handleCreate}/> :	
				<TaskList tasks={tasks}/>
			}	
    </div>
  )
}

export default Home
