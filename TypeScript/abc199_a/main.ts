const input = await new Response(Deno.stdin.readable).text();
const lines = input.trim().split('\n');

const [a, b, c] = lines[0].split(' ').map(Number);
console.log(a * a + b * b < c * c ? 'Yes' : 'No');

export{};
