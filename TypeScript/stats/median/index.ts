const getMedian = (list: number[]): number => {
 
    const length = list.length;

    if (length % 2 == 0) {
        let middle_index = (length / 2) - 1;
        let middle_value = list[middle_index];
        let next_middle_value = list[(middle_index + 1)]; 
        return (middle_value + next_middle_value) / 2.0;
    }

    let middle_index = (length - 1) / 2;

    return list[middle_index];
}

export { getMedian };
