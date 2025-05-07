async function readAllInput(): Promise<string[]> {
    const input = await new Response(Deno.stdin.readable).text();
    return input.trim().split('\n');
}

function parseLineToNumbers(line: string): number[] {
    return line.trim().split(' ').map(Number);
}

function solve(nums: number[], x: number): string {
    let ret = nums.filter(a => a != x).join(" ");
    return ret;
}

const line = await readAllInput();
const nAndX = parseLineToNumbers(line[0]);
const _n = nAndX[0];
const x = nAndX[1];
const a = parseLineToNumbers(line[1]);

const result = solve(a, x);
console.log(result);

export{};

