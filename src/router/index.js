import { createRouter, createMemoryHistory } from "vue-router";

import TaskView from "../views/tasks.view.vue";
import UserView from "../views/user.view.vue";

const router = createRouter({
    history: createMemoryHistory(),
    routes: [
        {path: "/", component: TaskView},
        {path: "/users", component: UserView}
    ],
})

export default router;