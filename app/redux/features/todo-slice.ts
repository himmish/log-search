import { createSlice } from "@reduxjs/toolkit";

type DirectoryState = {
  list: string[];
  loading: boolean;
};

const initialState: DirectoryState = {
  list: [],
  loading: true,
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

export const { updateTodo} = todo.actions;
export default todo.reducer;