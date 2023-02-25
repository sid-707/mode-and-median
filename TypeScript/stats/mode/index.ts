const getMode = (list: number[]): number => {
    let countsMap = new Map();

    list.forEach(i => {
        if (countsMap.has(i)) countsMap.set(i, countsMap.get(i) + 1);
        else countsMap.set(i, 1);
    });

    return ([...countsMap.entries()].reduce((a, b) => a[1] > b[1] ? a : b)[0]);
}

export { getMode };
