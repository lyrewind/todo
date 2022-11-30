import { ErrorMessage, Field, Form, Formik } from "formik"
import { Task, TaskStatus } from "../src/types"

interface CreationProps {
	onCreate: (task: Omit<Task, "id">) => void;
}

export const Creation: React.FC<CreationProps> = ({ onCreate }) => {
	return <Formik
		initialValues={{
				title: '',
				details: '',
				status: 'open' as TaskStatus
			}}
		onSubmit={(values) => {
				onCreate(values);
			}}
	>
		{() => (
				<Form className="flex flex-col w-[50vw] m-auto">
					<div className="flex flex-col">
						<label className="text-sm">TITLE</label>
						<Field type="text" name="title"
							className="text-black text-lg py-1 px-2"
						/>
						<ErrorMessage name="title"/>
					</div>

					<div className="flex flex-col mt-2">
						<label className="text-sm">DETAILS</label>
						<Field type="text" name="details"
							className="text-black text-lg py-1 px-2"
						/>
						<ErrorMessage name="details"/>
					</div>

					<div className="flex gap-2 w-full justify-evenly my-2.5">
						<label>
							<Field type="radio" name="status" value="open"/>
							Open
						</label>


						<label>
							<Field type="radio" name="status" value="paused"/>
							Paused
						</label>

						
						<label>
							<Field type="radio" name="status" value="closed"/>
							Closed
						</label>
					</div>
					<button 
						className="font-bold uppercase border-white rounded border-[1px] py-1.5 hover:text-black hover:bg-white transition ease-in duration-500"
						type="submit"
					>Create</button>
				</Form>
		)}
	</Formik>
}
