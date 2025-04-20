async function getInputToString(): Promise<string> {
    const input = await new Response(Deno.stdin.readable).text();
    return input.trim();
}

function solve(input: string): string {
    const first = input.slice(0,1);
    const rest = input.slice(1);
    return rest + first;
}

const inputText = await getInputToString();

console.log(solve(inputText));

export{};

