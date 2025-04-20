async function getInputToNumberArray(): Promise<number[]> {
    const input = await new Response(Deno.stdin.readable).text();
    const lines = input.trim().split('\n');

    const numbers = lines[0].split(' ').map(Number);
    return numbers;
}

const [a, b, c] = await getInputToNumberArray();

function solve(a: number, b: number, c: number): string {
    return a * a + b * b < c * c ? 'Yes' : 'No';
}

console.log(solve(a, b, c))

export{};
