'use client'
import {FaFileAlt} from "react-icons/fa";
import {FaFilePdf} from "react-icons/fa6";
import {BsFileWordFill} from "react-icons/bs";
import { VscJson } from "react-icons/vsc";
import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri'
import { RootState } from "../redux/store";
import { useSelector } from "react-redux";

const { list_files } = require('@tauri-apps/api/fs');

export interface FileProps {
    name: string;
    extension: string;
    url: string;
}

export interface FilesProps {
    files: FileProps[];
}

export function DirectoryResults({files}: FilesProps) {
    let size = 20;
    if(!files?.length){
        return(
          <div className="relative px-6 py-4 flex items-center space-x-3 focus-within:ring-0">
            <h1>No files found</h1>
          </div>
        );
    }

    const IconMapper = (extension: String) => {
        if(extension == "pdf") {
            return <FaFilePdf size={size} style={{ color: "maroon"}} />;
        } else if(extension == "word") {
            return <BsFileWordFill size={size} />;
        } else if(extension == "json") {
            return <VscJson size={size} style={{ color: "orange"}} />;
        } else {
            return <FaFileAlt size={size} style={{ color: "black"}} />
        }
    }

    return (
        <div>
            {(files.map((file: FileProps) => (
            <div key={file.name} className="relative px-6 py-4 flex items-center space-x-3 focus-within:ring-0">
                <div className="flex-shrink-0 h-4 w-4 ">
                    {IconMapper(file.extension)}
                </div>
                <p className="text-sm font-medium text-black truncate hover:text-clip">
                    {file.name}
                </p>
            </div>
            )))}
        </div>
    );
} 


export default function Directory() {
    const [f, setF] = useState<FilesProps>([]);

    const todoList = useSelector((state: RootState) => state.todoReducer.list);
    const isLoading = useSelector((state: RootState) => state.todoReducer.loading);

    useEffect(() => {
        invoke('list_files', { folderPath: todoList[0] })
        .then((files) => {
            console.log(files);
            setF(files);
        })
        .catch((error) => {
            console.error(error);
        });
    }, [isLoading, todoList]);

    return (
        <div className="relative px-0">
            <div className="bg-dark-accent-2 text-sm font-bold text-black uppercase">
            <h3>Downloads</h3>
            </div>
            <DirectoryResults files={f} />
        </div>
    );
}