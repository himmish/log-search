"use client"
import { RootState } from '@/app/redux/store';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { useSelector } from 'react-redux';
import Image from 'next/image';
import doodle from '../../../public/doodle1.jpg';

let width = 900;
let height = 500;

export function JsonViewer({content}: any) {
  return (
    <div className='container w-full bg-gray-100 h-4/5 overflow-y-auto'>
      <pre><code>{content}</code></pre>
    </div>
  );
}

export function PdfViewer({content}: any) {
  return (
    <div>
      <embed width={width} height={height} src={`data:application/pdf;base64,${content}`} />
    </div>
  );
}

export function TextViewer({content}: String) {
  return (
    <div className='container w-full bg-gray-100 h-4/5 overflow-y-auto'>
      <pre>{content}</pre>
    </div>
  );
}

export function ImageViewer({content, type}: any) {
  return (
    <div>
      <Image width={width} height={height} src={`data:image/${type};base64,${content}`} alt="File" />
    </div>
  );
}

export function UnSupportedViewer() {
  return (
    <div className="p-20 text-grey text-center">
      Unsupported file type
    </div>
  );
}

export function XmlViewer({content}: String) {
  return (
  <div className='container w-full bg-gray-100 h-4/5 overflow-y-auto'>
    <pre>{content}</pre>
  </div>
  );
}

export function DefaultViewer() {
  return (
    <div className="ml-10 mt-20">
      <Image width={500} height={300} src={doodle} alt="Home" />
    </div>
  );
}

export default function Viewer() {
  const [fileContent, setFileContent] = useState('');
  
  const url = useSelector((state: RootState) => state.fsSliceReducer.url);
  const type = useSelector((state: RootState) => state.fsSliceReducer.type);
  
  useEffect(() => {
    invoke('get_file_content', { fullPath: url })
    .then((content) => {
        console.log(content);
        setFileContent(content);
    })
    .catch((error) => {
        console.error(error);
    });
  }, [url, type]);

  function mapToViewer(type: String, fileContent: any) {
    if (type === 'pdf') {
      return <PdfViewer content={fileContent} />
    } else if (type === 'json') {
      return <JsonViewer content={fileContent} />;
    } else if (type === 'xml') {
      return <XmlViewer content={fileContent} />;
    } else if (type === 'txt') {
      return <TextViewer content={fileContent} />;
    } else if (type === 'jpg' || type === 'jpeg' || type === 'png') {
      return <ImageViewer type={type} content={fileContent} />
    } else if (type === '') {
      return <DefaultViewer />
    } else {
      return <UnSupportedViewer />;
    }
  }
  return(
    <div className="container p-10">
      {mapToViewer(type, fileContent)}
    </div>
  );
};