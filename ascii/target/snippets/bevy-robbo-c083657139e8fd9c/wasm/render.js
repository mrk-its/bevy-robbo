var cnt = 0;
export function render(screws, keys, ammo, level, board) {
    document.getElementById("inventory").innerText=`screws: ${screws}, keys: ${keys}, ammo: ${ammo}, level: ${level}`;
    document.getElementById("board").innerText = board;
}
