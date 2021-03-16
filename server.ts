import { serve } from "https://deno.land/std/http/server.ts";
import { greet } from './pkg/bsv_rs.js';

type Resp = {
    body: string;
}

const s = serve({ port: 8000 });
console.log("http://localhost:8000/");
for await (const req of s) {
  let r = {} as Resp;
  console.log(greet())
  r.body = "Done";
  req.respond(r);
}