function outer(n) {
    const actuallySharedArray = [];

    return function inner(a) {
        const result = a + n;
        actuallySharedArray.push(result);
        return result;
    };
}

const adder = outer(1);
for (let i = 0; i < 1_000_000_000; i++) {
    console.log(adder(i));
}
