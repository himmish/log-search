'use client'


import { updateTodo } from "../redux/features/todo-slice";
import { AppDispatch } from "../redux/store";
import { useDispatch } from "react-redux";

import { useState } from "react";
import { open } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';

export function Navbar(props: any) {
    const [fileDropdown, setFileDropdown] = useState<boolean>(false);

    const dispatch = useDispatch<AppDispatch>();

    const handleFileSelect = async () => {
        console.log("button clicked");

        setFileDropdown(false);

        const selected = await open({
            directory: true,
            multiple: true,
            defaultPath: await appDataDir(),
        });

        console.log(selected);
        if(Array.isArray(selected)) {
            console.log("multiple");
            dispatch(
                updateTodo(selected)
            );
        } else if(selected === null) {
            console.log("cancelled");
        } else {
            console.log("single");
            dispatch(
                updateTodo(selected)
            );
        }
    
    }

    return (
    <div className="container">
    <header className="bg-white">
    <nav className="flex gap-x-8 max-w-full p-2 lg:px-8" aria-label="Global">
    
        <div className="relative">
            <button type="button" className="flex items-center gap-x-1 text-sm font-semibold leading-6 text-gray-900" aria-expanded="false" onClick={(e) => setFileDropdown(!fileDropdown)}>
            File
            <svg className="h-5 w-5 flex-none text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fillRule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
            </svg>
            </button>
            {fileDropdown ? (
            <div className="absolute -left-8 top-full z-10 mt-3 w-screen max-w-sm overflow-hidden rounded-3xl bg-white shadow-lg ring-1 ring-gray-900/5">
            <div className="p-4">
                <div className="group relative flex items-center gap-x-6 rounded-lg p-4 text-sm leading-6 hover:bg-gray-50">
                    <button type="button" className="flex-auto block font-semibold text-gray-900" onClick={handleFileSelect} >
                    Open
                    <span className="absolute inset-0"></span>
                    <p className="mt-1 text-gray-600">Open Directory & load files..</p>
                    </button>
                </div>

                <div className="group relative flex items-center gap-x-6 rounded-lg p-4 text-sm leading-6 hover:bg-gray-50">
                <div className="flex-auto">
                    <button type="button" className="block font-semibold text-gray-900">
                    Close
                    <span className="absolute inset-0"></span>
                    <p className="mt-1 text-gray-600">Close Directory and it&apos;s files</p>
                    </button>
                </div>
                </div>

                <div className="group relative flex items-center gap-x-6 rounded-lg p-4 text-sm leading-6 hover:bg-gray-50">
                <div className="flex-auto">
                    <a href="#" className="block font-semibold text-gray-900">
                    Security
                    <span className="absolute inset-0"></span>
                    </a>
                    <p className="mt-1 text-gray-600">Your customers’ data will be safe and secure</p>
                </div>
                </div>

                <div className="group relative flex items-center gap-x-6 rounded-lg p-4 text-sm leading-6 hover:bg-gray-50">
                <div className="flex-auto">
                    <a href="#" className="block font-semibold text-gray-900">
                    Automations
                    <span className="absolute inset-0"></span>
                    </a>
                    <p className="mt-1 text-gray-600">Build strategic funnels that will convert</p>
                </div>
                </div>
            </div>
            </div>
            ) : ""}
        </div>

        <div className="items-center gap-x-1 text-sm font-semibold leading-6 text-gray-900">
        Edit
        </div>
        
        <div className="items-center gap-x-1 text-sm font-semibold leading-6 text-gray-900">
        View
        </div>
        
        <div className="items-center gap-x-1 text-sm font-semibold leading-6 text-gray-900">
        Settings
        </div>

        <div className="hidden lg:flex lg:flex-1 lg:justify-end">
        <a href="#" className="text-sm font-semibold leading-6 text-gray-900">Log in <span aria-hidden="true">&rarr;</span></a>
        </div>
    </nav>
    
    </header>
    </div>
  );
}