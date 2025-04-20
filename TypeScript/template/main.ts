async function getInputToNumberArray(): Promise<number[]> {
    const input = await new Response(Deno.stdin.readable).text();
    const lines = input.trim().split('\n');

    const numbers = lines[0].split(' ').map(Number);
    return numbers;
}

async function getInputToString(): Promise<string> {
    const input = await new Response(Deno.stdin.readable).text();
    return input.trim();
}

function solve(): string {
    // Todo: implemented me!
    throw new Error("Not implemented!")
}

console.log(solve())

export{};

