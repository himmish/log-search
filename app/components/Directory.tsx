'use client'

import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/tauri'
import { AppDispatch, RootState } from "../redux/store";
import { useDispatch, useSelector } from "react-redux";

import React from "react";
import { updateFile } from "../redux/features/todo-slice";
import { IconMapper, DirectoryIconMapper } from "./IconMapper";

const { list_files } = require('@tauri-apps/api/fs');

export interface FileProps {
    name: string;
    extension: string;
    url: string;
}

export interface FilesProps {
    name: string;
    files: FileProps[];
    expanded?: boolean
}


export default function Directory() {
    const [files, setFiles] = useState<Array<FilesProps>>([]);

    const todoList = useSelector((state: RootState) => state.todoReducer.list);
    const isLoading = useSelector((state: RootState) => state.todoReducer.loading);

    useEffect(() => {
        invoke('list_files', { folderPath: todoList[0] })
        .then((files1) => {
            files1.sort(function(a: FilesProps, b: FilesProps) {
                var x = a.name; var y = b.name;
                return ((x < y) ? -1 : ((x > y) ? 1 : 0));
            });
            console.log(files1);
            setFiles(files1);
        })
        .catch((error) => {
            console.error(error);
        });
    }, [isLoading, todoList]);


    const dispatch = useDispatch<AppDispatch>();

    const handleFileOpen = async (url: string, type: string) => {
        console.log(url);
        dispatch(
            updateFile({url, type})
        );
    }
    const handleDirectoryExpand = (f: FilesProps, e: any) => {
        console.log("clicked unexpand for " + f.name);
        let copyFiles: Array<FilesProps> = files.map(elem => {
            if (elem.name.localeCompare(f.name) === 0) {
              return {...elem, expanded: !directoryExpandedStatus(elem.expanded)};
            }
            return elem;
        });
        console.log(copyFiles);
        setFiles(copyFiles);
    }
    function directoryExpandedStatus (val?: boolean): boolean {
        if(val !== undefined && val == false) {
            return false;
        }
        return true;
    }

    if(!files?.length){
        return(
            <div className="relative px-0">
                <div className="relative px-6 py-4 flex items-center space-x-3 focus-within:ring-0 backdrop-blur-sm">
                    <h3 className="text-grey">Directory</h3>
                </div>
            </div>
        );
    } else {
        return (
            <div className="relative px-0">
                {(files.map((f: FilesProps) => (
                <div key={f.name}>
                    <div className="bg-dark-accent-2 text-sm py-3 font-bold text-black uppercase">
                        <button className="relative flex items-center space-x-3 focus-within:ring-0" onClick={(e) => handleDirectoryExpand(f, e)}>
                            {DirectoryIconMapper(directoryExpandedStatus(f.expanded))}
                            <h3>{f.name}</h3>
                        </button>
                    </div>
                    {!directoryExpandedStatus(f.expanded) ? <></> :
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
                    }
                </div>
                )))
                }
            </div>
        );
    }
}