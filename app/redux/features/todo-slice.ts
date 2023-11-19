import { createSlice } from "@reduxjs/toolkit";

type DirectoryState = {
  list: string[];
  loading: boolean;
};

type FileState = {
  url: string;
  type: string;
};

const initialState: DirectoryState = {
  list: [],
  loading: true,
};

const initialFileState: FileState = {
  url: "",
  type: "",
};

export const todo = createSlice({
  name: "todo",
  initialState,
  reducers: {
    updateTodo: (state, action) => {
      state.list = action.payload;
      state.loading = false;
    },
  },
});

export const fsSlice = createSlice({
  name: "fSlice",
  initialState: initialFileState,
  reducers: {
    updateFile: (state, action) => {
      state.url = action.payload.url;
      state.type = action.payload.type;
    },
  },
});

export const { updateTodo} = todo.actions;
export const { updateFile} = fsSlice.actions;

export const todoReducer = todo.reducer;
export const fsSliceReducer = fsSlice.reducer;