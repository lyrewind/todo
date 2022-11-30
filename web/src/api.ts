import axios from 'axios'
import { Task } from './types'

export type APITask = Omit<Task, "status"> & {
	status_code: number;
}

export const instance = axios.create({
	baseURL: "http://localhost:8080",
	timeout: 1000
})

export async function getTasks() {
	return await instance.get<{ tasks: APITask[] }>("/task")
}

export async function createTask(data: Task) {
	return await instance.post<{ task: APITask }>("/task", data)
}
