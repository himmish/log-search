'use client'
import {FiFolderMinus, FiFolderPlus} from "react-icons/fi";
import {FaFileAlt} from "react-icons/fa";
import {FaFilePdf} from "react-icons/fa6";
import {BsFileWordFill, BsFiletypePng, BsFiletypeXml} from "react-icons/bs";
import { VscJson } from "react-icons/vsc";
import { TbFileTypePpt } from "react-icons/tb";
import { AiOutlineFileJpg } from "react-icons/ai";

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
        return <FaFileAlt size={size} style={{ color: "grey"}} />
    }
}

const DirectoryIconMapper = (expanded: boolean) => {
    if(expanded == true) {
        return <FiFolderMinus size={size} style={{ color: "grey"}} />
    } else {
        return <FiFolderPlus size={size} style={{ color: "grey"}} />
    }
}

export {IconMapper, DirectoryIconMapper};