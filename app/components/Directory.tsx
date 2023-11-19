'use client'
import {FaFileAlt} from "react-icons/fa";
import {FaFilePdf} from "react-icons/fa6";
import {BsFileWordFill, BsFiletypePng, BsFiletypeXml} from "react-icons/bs";
import { VscJson } from "react-icons/vsc";
import { TbFileTypePpt } from "react-icons/tb";
import { AiOutlineFileJpg } from "react-icons/ai";


import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri'
import { AppDispatch, RootState } from "../redux/store";
import { useDispatch, useSelector } from "react-redux";

import React from "react";
import { updateFile } from "../redux/features/todo-slice";

const { list_files } = require('@tauri-apps/api/fs');

export interface FileProps {
    name: string;
    extension: string;
    url: string;
}

export interface FilesProps {
    name: string;
    files: FileProps[];
}
let size = 20;
const IconMapper = (extension: string) => {
    if(extension === 'pdf') {
        return <FaFilePdf size={size} style={{ color: "maroon"}} />;
    } else if(extension === 'word') {
        return <BsFileWordFill size={size} />;
    } else if(extension === 'json') {
        return <VscJson size={size} style={{ color: "orange"}} />;
    } else if(extension === 'ppt') {
        return <TbFileTypePpt size={size} style={{ color: "orange"}} />;
    } else if(extension === 'jpg' || extension === 'jpeg') {
        return <AiOutlineFileJpg size={size} style={{ color: "orange"}} />;
    } else if(extension === 'png') {
        return <BsFiletypePng size={size} style={{ color: "blue"}} />;
    } else if(extension === 'xml') {
        return <BsFiletypeXml size={size} style={{ color: "green"}} />;
    } else {
        return <FaFileAlt size={size} style={{ color: "black"}} />
    }
}

export function DirectoryResults({files} : Array<FilesProps>) {
    const dispatch = useDispatch<AppDispatch>();

    const handleFileOpen = async (url: string, type: string) => {
        console.log(url);
        dispatch(
            updateFile({url, type})
        );
    }


    if(!files?.length){
        return(
          <div className="relative px-6 py-4 flex items-center space-x-3 focus-within:ring-0">
            <h1>No files found</h1>
          </div>
        );
    }

    return (
        <>
        {(files.map((f: FilesProps) => (
            <>
            <div className="bg-dark-accent-2 text-sm py-3 font-bold text-black uppercase">
                <h3>{f.name}</h3>
            </div>
            <div>
                {(f.files.map((file: FileProps) => (
                    <div key={file.name} className="relative px-4 py-2 flex items-center space-x-3 focus-within:ring-0">
                        <button className="relative flex items-center space-x-3 focus-within:ring-0" onClick={(e) => handleFileOpen(file.url, file.extension)}>
                        <div className="flex-shrink-0 h-4 w-4 ">
                            {IconMapper(file.extension)}
                        </div>
                        <p className="text-sm font-medium text-black truncate hover:text-clip">
                            {file.name}
                        </p>
                        </button>
                    </div>
                )))}
            </div>
            </>
            )))}
        </>
    );
} 


export default function Directory() {
    const [f, setF] = useState<FilesProps>([]);

    const todoList = useSelector((state: RootState) => state.todoReducer.list);
    const isLoading = useSelector((state: RootState) => state.todoReducer.loading);

    useEffect(() => {
        invoke('list_files', { folderPath: todoList[0] })
        .then((files) => {
            files.sort();
            console.log(files);
            setF(files);
        })
        .catch((error) => {
            console.error(error);
        });
    }, [isLoading, todoList]);

    return (
        <div className="relative px-0">
            <DirectoryResults files={f} />
        </div>
    );
}