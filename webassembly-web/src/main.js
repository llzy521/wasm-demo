import { createApp } from "vue";
import App from "./App.vue";
import { greet} from "webassembly-rust";

// ...
console.debug(greet);
greet();


createApp(App).mount("#app");
