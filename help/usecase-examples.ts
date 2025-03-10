// A. OPTION
//  A1. Postponed assignment
// https://codesandbox.io/p/sandbox/hv36rt?file=%2Fsrc%2Findex.ts%3A14%2C1
let value: string | undefined;

const shouldUpdate: boolean = checkIfUpdate();

if (shouldUpdate) {
    value = "Updated value!";
}
 console.log('Value:', value);


// B. RESULT
//  B1. Optional value using null/undefined
function divide(a: number, b: number): number | null {
    if (b === 0) {
        return null;
    }
    return a / b;
}

const result = divide(10, 2);
if (result === null) {
    console.error('Error: Division by zero');
} else {
    console.log('Result:', result);
}



//  B2. Try-catch (Exception-based OR Promise<>)
function tryDivide(a: number, b: number): number {
    if (b === 0) {
        throw new Error('Division by zero');
    }
    return a / b;
}

(function() {
    try {
        const result = tryDivide(10, 0);
        console.log('Result:', result);
    } catch (error) {
        console.error('Error:', error.message);
    }

})();


