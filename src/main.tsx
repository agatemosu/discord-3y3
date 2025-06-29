import { render } from "preact";
import { App } from "./app.tsx";

const app = document.querySelector("#app");
if (!app) {
	throw new Error("App element not found");
}

render(<App />, app);
