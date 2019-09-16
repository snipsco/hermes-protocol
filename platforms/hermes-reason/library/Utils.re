let suffixes = [".so", ".dylib", ".exe", ""];

let openDynamicLibrary = path => {
    suffixes |> List.exists(suff => {
    let path = path ++ suff;
    try ({
        Dl.dlopen(~filename=path, ~flags=[RTLD_NOW]) |> ignore;
        true
    }) {
        | _ => {
        /* Ignore */
        /* Temp: print it */
        Console.log("Dynamic library not found at path :" ++ path);
        false
        };
    }
    });

}