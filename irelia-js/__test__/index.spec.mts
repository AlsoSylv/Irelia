import {LcuClient} from "../index";
import test from "ava";

test("test lcu client", t => {
    let client = LcuClient.new(false);
    client.get("example/endpoint/").then(json => {
        console.log(json);
        
    });
})