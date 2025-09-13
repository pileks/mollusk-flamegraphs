import path from "path";
import { generateIdl } from "@metaplex-foundation/shank-js";

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "hello_world_program",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "hello-world"),
});
