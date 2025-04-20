import init, { new_repo } from "./pkg/testing.js";

let repo = null;

function postTodos() {
  if (!repo) {
    return;
  }
  postMessage({
    command: "todos",
    todos: repo.get_todos().map((todo) => ({
      id: todo.id(),
      text: todo.text(),
    })),
  });
}

async function run() {
  await init();
  const newRepo = await new_repo();
  newRepo.migrate();
  repo = newRepo;
  postTodos();
}

function addTodo(text) {
  if (!repo) {
    return;
  }
  repo.put_todo(text);
  postTodos();
}

function deleteTodo(id) {
  if (!repo) {
    return;
  }
  repo.delete_todo(id);
  postTodos();
}

onmessage = (msg) => {
  if (msg.data.command === "init") {
    run();
  } else if (msg.data.command === "new") {
    addTodo(msg.data.text);
  } else if (msg.data.command === "delete") {
    deleteTodo(msg.data.id);
  } else {
    console.log(`Unrecognized command: ${msg.data.command}`)
  }
};
