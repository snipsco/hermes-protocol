module.exports.activeSubPaths = function(flowPath, subPath) {
    return subPath.reduce((activePaths, path) => {
        const { subPath } = path
        let activePosition

        for(let cursor = subPath.length - 1; cursor >= 0; cursor--) {
            if(subPath[cursor] !== flowPath[flowPath.length - 1])
                continue

            activePosition = cursor

            for(let flowCursor = flowPath.length - 1; flowCursor >= 0 && cursor >= 0; flowCursor--, cursor--) {
                if(flowPath[flowCursor] !== subPath[cursor]) {
                    return activePaths
                }
            }

            activePaths.push({
                path,
                trigger: activePosition === subPath.length - 1,
                position: activePosition
            })
            break
        }

        return activePaths
    }, [])
}