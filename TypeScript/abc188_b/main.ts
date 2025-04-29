async function readAllInput(): Promise<string[]> {
    const input = await new Response(Deno.stdin.readable).text();
    return input.trim().split('\n');
}

function parseLineToNumbers(line: string): number[] {
    return line.trim().split(' ').map(Number);
}

function solve(aArr: number[], bArr: number[]): string {
    let sum = 0;
    for (let i = 0; i < aArr.length; i++) {
        sum += aArr[i] * bArr[i];
    }

    if (sum == 0) {
        return "Yes";
    }
    return "No";
}

const lines = await readAllInput();

const _n = Number(lines[0]);
const aArr = parseLineToNumbers(lines[1]);
const bArr = parseLineToNumbers(lines[2]);

console.log(solve(aArr, bArr))

export{};

