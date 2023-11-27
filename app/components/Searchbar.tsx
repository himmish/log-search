'use client';
import { FormEvent } from "react";
import { BiSearchAlt } from "react-icons/bi";

export default function Search() {
    async function onSubmit(event: FormEvent<HTMLFormElement>) {
        event.preventDefault()
        const formData = new FormData(event.currentTarget)
        // for (const value of formData.values()) {
        //     console.log(value);
        // }
    }

    
  return (
      <div className="w-full max-px-40 px-20 py-4">
        <div className="max-w-md space-y-4 animate-in fade-in slide-in-from-bottom-4 duration-1200 ease-in-out">
          
          <form onSubmit={onSubmit} className="bg-black rounded-xl shadow-lg h-fit flex flex-row px-1 items-center w-full">
            <input
                type="text"
                name="prompt"
                placeholder="Search"
                className="bg-transparent text-white placeholder:text-gray-400 ring-0 outline-none resize-none py-2.5 px-2 font-mono text-sm h-10 w-full transition-all duration-300"
            />
            
            <button type="submit" className="text-white rounded-lg hover:bg-white/25 focus:bg-white/25 w-8 h-8 aspect-square flex items-center justify-center ring-0 outline-0">
                <BiSearchAlt />
            </button>           
          </form>
        </div>
        
      </div>
  )
}