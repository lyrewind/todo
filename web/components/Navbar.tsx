import { useState } from "react"

interface NavbarProps {
	onClick: () => void;
	disabled?: boolean;
}

export const Navbar: React.FC<NavbarProps> = ({ onClick, disabled }) => {
	const [query, setQuery] = useState<string>("");

	return <div className="grid grid-rows-1 grid-cols-[1fr_8fr] w-full items-center py-2 px-4 mt-2 gap-1">
		<button 
			className="text-white py-2 px-6 border-gray-200 border-2 rounded hover:bg-white hover:text-black transition-all duration-500 ease-in" 
			onClick={onClick}
		 >{disabled ? "Cancel" : "Create"}</button>
		<input
			className="py-2 px-6 text-black"
			type="text"
			placeholder="Filter..."
			value={query}
			onChange={e => setQuery(e.target.value)}
			disabled={disabled}
		/>
	</div>
}
