"use client"
import { RootState } from '@/app/redux/store';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { useSelector } from 'react-redux';

export default function Viewer() {
  const [fileContent, setFileContent] = useState('');
  
  const url = useSelector((state: RootState) => state.fsSliceReducer.url);
  const type = useSelector((state: RootState) => state.fsSliceReducer.type);
  
  useEffect(() => {
    invoke('get_file_content', { fullPath: url })
    .then((content) => {
        console.log('then');
        console.log(content);
        setFileContent(content);
    })
    .catch((error) => {
        console.error(error);
    });
  }, [url, type]);


  if (type === 'pdf') {
    return (
      <div>
        <h1>PDF Viewer</h1>
        <embed width={750} height={550} src={`data:application/pdf;base64,${fileContent}`} />
      </div>
    );
  } else if (type === 'jpg' || type === 'jpeg' || type === 'png'|| type === 'txt') {
    return (
      <div>
        <h1>Image Viewer</h1>
        <img width={750} height={550} src={`data:image/${type};base64,${fileContent}`} alt="File" />
      </div>
    );
  } else if (type === '') {
    return <div></div>;
  } else {
    return <div>Unsupported file type</div>;
  }
};