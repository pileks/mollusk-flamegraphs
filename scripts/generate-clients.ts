import { createFromIdls, renderJavaScriptVisitor } from "@metaplex-foundation/kinobi";
import path from "path";

// Instantiate Kinobi.
const kinobi = createFromIdls([
  path.join(__dirname, "idls", "hello_world_program.json"),
]);

// Render JavaScript.
const jsDir = path.join(__dirname, "clients", "js", "src", "generated");
// kinobi.accept(renderJavaScriptVisitor(jsDir));