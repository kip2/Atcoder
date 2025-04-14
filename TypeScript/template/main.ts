const input = await new Response(Deno.stdin.readable).text();
const lines = input.trim().split('\n');

const [a, b, c] = lines[0].split(' ').map(Number);

function solve(a: number, b: number, c: number): string {
    // Todo: implemented me!
    throw new Error("Not implemented!")
}

console.log(solve(a, b, c))

export{};

