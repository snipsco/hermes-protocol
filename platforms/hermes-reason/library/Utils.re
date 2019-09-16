let suffixes = [".so", ".dylib", ".exe", ""];

let openDynamicLibrary = (~flags=[Dl.RTLD_LAZY], path) => {
    suffixes |> List.exists(suff => {
    let path = path ++ suff;
    try ({
        Dl.dlopen(~filename=path, ~flags) |> ignore;
        true
    }) {
        | _ => false
    }
    });

}