async function readAllInput(): Promise<string[]> {
    const input = await new Response(Deno.stdin.readable).text();
    return input.trim().split('\n');
}

function parseLineToNumbers(line: string): number[] {
    return line.trim().split(' ').map(Number);
}

function solve(): string {
    // Todo: implemented me!
    throw new Error("Not implemented!")
}

console.log(solve())

export{};

